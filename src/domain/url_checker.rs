use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;

use indicatif::{ProgressBar, ProgressStyle};
use log::debug;
use reqwest::Client;
use serde_json::Value;
use tokio::fs;
use tokio::time::timeout;

use crate::domain::model::url_check_result::{UrlCheckReport, UrlCheckResult, UrlStatus};

const DEFAULT_TIMEOUT_SECS: u64 = 10;
const MAX_CONCURRENT_REQUESTS: usize = 5; // Reduced from 10 to avoid rate limiting

pub struct UrlChecker {
    client: Client,
    timeout_duration: Duration,
}

impl UrlChecker {
    pub fn new(timeout_secs: Option<u64>) -> Result<Self, String> {
        let timeout_duration = Duration::from_secs(timeout_secs.unwrap_or(DEFAULT_TIMEOUT_SECS));

        // Use a realistic browser user-agent to avoid being blocked
        let client = Client::builder()
            .timeout(timeout_duration)
            .user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
            .redirect(reqwest::redirect::Policy::limited(5))
            .build()
            .map_err(|e| format!("Failed to create HTTP client: {}", e))?;

        Ok(Self {
            client,
            timeout_duration,
        })
    }

    /// Check if a URL matches known patterns that require browser/JS
    fn requires_browser(url: &str) -> bool {
        url.contains("data.snf.ch/grants")
    }

    /// Check if a URL matches known patterns that block automation but are likely OK
    fn blocks_automation(url: &str) -> bool {
        url.contains("linkedin.com")
            || url.contains("ark.dasch.swiss")
            || url.contains(".edu.eg") // Egyptian universities
            || url.contains("zhdk.ch") // Zurich University of the Arts
            || url.contains("p3.snf.ch") // SNSF project pages
            || url.contains("unine.ch") // University of Neuchâtel
            || url.contains("fwo.be") // Research Foundation Flanders
    }

    /// Check if URL is an authority file/controlled vocabulary (always trusted)
    fn is_authority_file(url: &str) -> Option<&'static str> {
        if url.contains("skos.um.es") {
            Some("UNESCO Thesaurus")
        } else if url.contains("vocab.getty.edu") {
            Some("Getty Vocabularies")
        } else if url.contains("data.perio.do") || url.contains("n2t.net/ark:/99152/") {
            Some("PeriodO")
        } else if url.contains("geonames.org") {
            Some("GeoNames")
        } else if url.contains("doi.org") || url.starts_with("https://doi.org/10.") {
            Some("DOI")
        } else if url.contains("orcid.org") {
            Some("ORCID")
        } else if url.contains("d-nb.info/gnd/") {
            Some("GND")
        } else if url.contains("viaf.org") {
            Some("VIAF")
        } else if url.contains("pleiades.stoa.org") {
            Some("Pleiades")
        } else {
            None
        }
    }

    /// Normalize URL by removing common issues
    fn normalize_url(url: &str) -> String {
        // Strip trailing periods (common copy/paste error)
        url.trim_end_matches('.').to_string()
    }

    pub async fn check_directory(&self, data_dir: &Path) -> Result<UrlCheckReport, String> {
        let json_files = self.find_json_files(data_dir).await?;

        if json_files.is_empty() {
            return Err("No JSON files found in the specified directory".to_string());
        }

        println!("Found {} JSON files to check", json_files.len());

        let mut all_urls = Vec::new();

        for json_file in &json_files {
            match self.extract_urls_from_file(json_file).await {
                Ok(urls) => all_urls.extend(urls),
                Err(e) => {
                    eprintln!("Warning: Failed to process {}: {}", json_file.display(), e);
                }
            }
        }

        println!("Found {} URLs to check", all_urls.len());

        let results = self.check_urls(all_urls).await;

        Ok(UrlCheckReport::new(results))
    }

    async fn find_json_files(&self, dir: &Path) -> Result<Vec<PathBuf>, String> {
        let mut json_files = Vec::new();

        let mut entries = fs::read_dir(dir)
            .await
            .map_err(|e| format!("Failed to read directory {}: {}", dir.display(), e))?;

        while let Some(entry) = entries
            .next_entry()
            .await
            .map_err(|e| format!("Failed to read directory entry: {}", e))?
        {
            let path = entry.path();
            if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("json") {
                json_files.push(path);
            } else if path.is_dir() {
                // Recursively search subdirectories
                let sub_files = Box::pin(self.find_json_files(&path)).await?;
                json_files.extend(sub_files);
            }
        }

        Ok(json_files)
    }

    async fn extract_urls_from_file(
        &self,
        file_path: &Path,
    ) -> Result<Vec<UrlCheckResult>, String> {
        let content = fs::read_to_string(file_path)
            .await
            .map_err(|e| format!("Failed to read file: {}", e))?;

        let json: Value =
            serde_json::from_str(&content).map_err(|e| format!("Failed to parse JSON: {}", e))?;

        let mut urls = Vec::new();
        let file_path_str = file_path.display().to_string();

        Self::extract_urls_recursive(&json, &file_path_str, "root", &mut urls);

        Ok(urls)
    }

    fn extract_urls_recursive(
        value: &Value,
        file_path: &str,
        context: &str,
        urls: &mut Vec<UrlCheckResult>,
    ) {
        match value {
            Value::Object(map) => {
                // Check if this is a URL object with __type: "URL"
                let is_url_object = map
                    .get("__type")
                    .and_then(|v| v.as_str())
                    .map(|t| t == "URL")
                    .unwrap_or(false);

                if is_url_object {
                    // This is a URL object, extract only the url field and skip recursing into
                    // children
                    if let Some(url_str) = map.get("url").and_then(|v| v.as_str()) {
                        let url_type = map
                            .get("type")
                            .and_then(|v| v.as_str())
                            .unwrap_or("unknown");
                        let url_context = format!("{} (type: {})", context, url_type);

                        urls.push(UrlCheckResult {
                            url: url_str.to_string(),
                            status: UrlStatus::Skipped {
                                reason: "Not checked yet".to_string(),
                            },
                            file_path: file_path.to_string(),
                            context: url_context,
                        });
                    }
                    // Don't recurse into URL object children to avoid duplicates
                    return;
                }

                // Recursively check all values in the object
                for (key, val) in map {
                    let new_context = format!("{}.{}", context, key);
                    Self::extract_urls_recursive(val, file_path, &new_context, urls);
                }
            }
            Value::Array(arr) => {
                for (idx, val) in arr.iter().enumerate() {
                    let new_context = format!("{}[{}]", context, idx);
                    Self::extract_urls_recursive(val, file_path, &new_context, urls);
                }
            }
            Value::String(s) => {
                // Check if the string looks like a URL and is not within a URL object
                if (s.starts_with("http://") || s.starts_with("https://"))
                    && !context.contains("__type")
                {
                    urls.push(UrlCheckResult {
                        url: s.clone(),
                        status: UrlStatus::Skipped {
                            reason: "Not checked yet".to_string(),
                        },
                        file_path: file_path.to_string(),
                        context: context.to_string(),
                    });
                }
            }
            _ => {}
        }
    }

    async fn check_urls(&self, urls: Vec<UrlCheckResult>) -> Vec<UrlCheckResult> {
        let pb = ProgressBar::new(urls.len() as u64);
        pb.set_style(
            ProgressStyle::default_bar()
                .template(
                    "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} {msg}",
                )
                .unwrap()
                .progress_chars("#>-"),
        );

        let mut results = Vec::new();
        let semaphore = Arc::new(tokio::sync::Semaphore::new(MAX_CONCURRENT_REQUESTS));

        let mut tasks = Vec::new();

        for mut url_result in urls {
            let client = self.client.clone();
            let timeout_duration = self.timeout_duration;
            let permit = Arc::clone(&semaphore);

            let task = tokio::spawn(async move {
                let _permit = permit.acquire().await.unwrap();

                // Normalize URL first
                let normalized_url = Self::normalize_url(&url_result.url);
                if normalized_url != url_result.url {
                    url_result.url = normalized_url;
                }

                // Skip internal references
                if url_result
                    .url
                    .starts_with("http://ns.dasch.swiss/repository")
                    || url_result
                        .url
                        .starts_with("https://ns.dasch.swiss/repository")
                {
                    url_result.status = UrlStatus::Skipped {
                        reason: "Internal reference".to_string(),
                    };
                    return url_result;
                }

                // Check if it's an authority file (trusted, skip checking)
                if let Some(authority_type) = Self::is_authority_file(&url_result.url) {
                    url_result.status = UrlStatus::AuthorityFile {
                        authority_type: authority_type.to_string(),
                    };
                    return url_result;
                }

                // Check whitelisted patterns
                if Self::requires_browser(&url_result.url) {
                    url_result.status = UrlStatus::RequiresBrowser {
                        reason: "Requires JavaScript/browser".to_string(),
                    };
                    return url_result;
                }

                if Self::blocks_automation(&url_result.url) {
                    url_result.status = UrlStatus::BlocksAutomation {
                        status_code: 405,
                        reason: "Known to block automated requests".to_string(),
                    };
                    return url_result;
                }

                // Try HEAD request first
                let status = match timeout(timeout_duration, client.head(&url_result.url).send())
                    .await
                {
                    Ok(Ok(response)) => {
                        let status_code = response.status().as_u16();
                        if response.status().is_success() {
                            UrlStatus::Ok { status_code }
                        } else if response.status().is_redirection() {
                            let location = response
                                .headers()
                                .get("location")
                                .and_then(|v| v.to_str().ok())
                                .unwrap_or("unknown")
                                .to_string();
                            UrlStatus::Redirect {
                                status_code,
                                location,
                            }
                        } else if status_code == 405 || status_code == 999 {
                            // 405 Method Not Allowed or 999 (LinkedIn) - retry with GET
                            match timeout(timeout_duration, client.get(&url_result.url).send())
                                .await
                            {
                                Ok(Ok(get_response)) => {
                                    let get_status_code = get_response.status().as_u16();
                                    if get_response.status().is_success() {
                                        UrlStatus::BlocksAutomation {
                                            status_code: 405,
                                            reason: "HEAD not allowed but GET works".to_string(),
                                        }
                                    } else {
                                        UrlStatus::Error {
                                            status_code: get_status_code,
                                            message: get_response.status().to_string(),
                                        }
                                    }
                                }
                                _ => UrlStatus::BlocksAutomation {
                                    status_code,
                                    reason: "Blocks HEAD requests".to_string(),
                                },
                            }
                        } else if status_code == 403 || status_code == 400 {
                            // 403 Forbidden or 400 Bad Request - may reject HEAD, try GET
                            match timeout(timeout_duration, client.get(&url_result.url).send())
                                .await
                            {
                                Ok(Ok(get_response)) if get_response.status().is_success() => {
                                    UrlStatus::BlocksAutomation {
                                        status_code,
                                        reason: "HEAD blocked but GET works".to_string(),
                                    }
                                }
                                _ => UrlStatus::Error {
                                    status_code,
                                    message: format!(
                                        "{} (GET retry also failed)",
                                        if status_code == 403 {
                                            "403 Forbidden"
                                        } else {
                                            "400 Bad Request"
                                        }
                                    ),
                                },
                            }
                        } else if status_code == 429 {
                            // 429 Too Many Requests - wait with jitter and retry once
                            let jitter = (std::time::SystemTime::now()
                                .duration_since(std::time::UNIX_EPOCH)
                                .unwrap()
                                .as_millis()
                                % 500) as u64;
                            tokio::time::sleep(Duration::from_millis(2000 + jitter)).await;
                            match timeout(timeout_duration, client.head(&url_result.url).send())
                                .await
                            {
                                Ok(Ok(retry_response)) if retry_response.status().is_success() => {
                                    UrlStatus::Ok {
                                        status_code: retry_response.status().as_u16(),
                                    }
                                }
                                _ => UrlStatus::Error {
                                    status_code,
                                    message: "429 Too Many Requests (retry failed)".to_string(),
                                },
                            }
                        } else if status_code == 410 {
                            // 410 Gone - content permanently removed (critical!)
                            UrlStatus::PermanentlyRemoved {
                                status_code: 410,
                                message: "Content permanently removed by site owner".to_string(),
                            }
                        } else if status_code == 404 {
                            // 404 Not Found - may come back, less critical than 410
                            UrlStatus::Error {
                                status_code: 404,
                                message: "404 Not Found".to_string(),
                            }
                        } else if status_code == 451 {
                            // 451 Unavailable For Legal Reasons - censored/blocked
                            UrlStatus::Error {
                                status_code: 451,
                                message: "451 Unavailable For Legal Reasons (Censored)".to_string(),
                            }
                        } else if (500..600).contains(&status_code) {
                            // 5xx Server Errors - retry once with jitter
                            let jitter = (std::time::SystemTime::now()
                                .duration_since(std::time::UNIX_EPOCH)
                                .unwrap()
                                .as_millis()
                                % 300) as u64;
                            tokio::time::sleep(Duration::from_millis(1000 + jitter)).await;
                            match timeout(timeout_duration, client.head(&url_result.url).send())
                                .await
                            {
                                Ok(Ok(retry_response)) if retry_response.status().is_success() => {
                                    UrlStatus::Ok {
                                        status_code: retry_response.status().as_u16(),
                                    }
                                }
                                _ => UrlStatus::Error {
                                    status_code,
                                    message: format!("{} (retry failed)", response.status()),
                                },
                            }
                        } else {
                            UrlStatus::Error {
                                status_code,
                                message: response.status().to_string(),
                            }
                        }
                    }
                    Ok(Err(e)) => {
                        if e.is_timeout() {
                            // Retry timeout once with longer timeout
                            let extended_timeout = timeout_duration * 2;
                            match timeout(extended_timeout, client.head(&url_result.url).send())
                                .await
                            {
                                Ok(Ok(retry_response)) if retry_response.status().is_success() => {
                                    UrlStatus::Ok {
                                        status_code: retry_response.status().as_u16(),
                                    }
                                }
                                _ => UrlStatus::Timeout,
                            }
                        } else if e.is_connect() || e.is_request() {
                            // Retry network errors once with jitter (might be transient)
                            let jitter = (std::time::SystemTime::now()
                                .duration_since(std::time::UNIX_EPOCH)
                                .unwrap()
                                .as_millis()
                                % 300) as u64;
                            tokio::time::sleep(Duration::from_millis(1000 + jitter)).await;
                            match timeout(timeout_duration, client.head(&url_result.url).send())
                                .await
                            {
                                Ok(Ok(retry_response)) if retry_response.status().is_success() => {
                                    UrlStatus::Ok {
                                        status_code: retry_response.status().as_u16(),
                                    }
                                }
                                _ => UrlStatus::NetworkError {
                                    message: format!("{} (retry failed)", e),
                                },
                            }
                        } else {
                            UrlStatus::InvalidUrl {
                                message: e.to_string(),
                            }
                        }
                    }
                    Err(_) => {
                        // Timeout from tokio::time::timeout - retry once
                        let extended_timeout = timeout_duration * 2;
                        match timeout(extended_timeout, client.head(&url_result.url).send()).await {
                            Ok(Ok(retry_response)) if retry_response.status().is_success() => {
                                UrlStatus::Ok {
                                    status_code: retry_response.status().as_u16(),
                                }
                            }
                            _ => UrlStatus::Timeout,
                        }
                    }
                };

                url_result.status = status;
                url_result
            });

            tasks.push(task);
        }

        for task in tasks {
            match task.await {
                Ok(result) => {
                    pb.inc(1);
                    debug!("Checked: {} - {}", result.url, result.status);
                    results.push(result);
                }
                Err(e) => {
                    eprintln!("Task failed: {}", e);
                    pb.inc(1);
                }
            }
        }

        pb.finish_with_message("Done");
        results
    }
}
