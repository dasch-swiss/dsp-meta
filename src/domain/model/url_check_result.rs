use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UrlCheckResult {
    pub url: String,
    pub status: UrlStatus,
    pub file_path: String,
    pub context: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum UrlStatus {
    Ok {
        status_code: u16,
    },
    Redirect {
        status_code: u16,
        location: String,
    },
    Error {
        status_code: u16,
        message: String,
    },
    Timeout,
    NetworkError {
        message: String,
    },
    InvalidUrl {
        message: String,
    },
    Skipped {
        reason: String,
    },
    /// Likely OK but requires browser/JavaScript (e.g., data.snf.ch)
    RequiresBrowser {
        reason: String,
    },
    /// Likely OK but blocks automation (e.g., LinkedIn, HEAD not allowed)
    BlocksAutomation {
        status_code: u16,
        reason: String,
    },
    /// Authority file/controlled vocabulary (trusted, not checked)
    AuthorityFile {
        authority_type: String,
    },
    /// Content permanently removed (410 Gone) - should be fixed immediately
    PermanentlyRemoved {
        status_code: u16,
        message: String,
    },
}

impl UrlStatus {
    pub fn is_ok(&self) -> bool {
        matches!(self, UrlStatus::Ok { .. })
    }

    pub fn is_redirect(&self) -> bool {
        matches!(self, UrlStatus::Redirect { .. })
    }

    pub fn is_error(&self) -> bool {
        matches!(
            self,
            UrlStatus::Error { .. }
                | UrlStatus::Timeout
                | UrlStatus::NetworkError { .. }
                | UrlStatus::InvalidUrl { .. }
                | UrlStatus::PermanentlyRemoved { .. }
        )
    }

    pub fn is_permanently_removed(&self) -> bool {
        matches!(self, UrlStatus::PermanentlyRemoved { .. })
    }

    pub fn is_likely_ok(&self) -> bool {
        matches!(
            self,
            UrlStatus::RequiresBrowser { .. }
                | UrlStatus::BlocksAutomation { .. }
                | UrlStatus::AuthorityFile { .. }
        )
    }

    pub fn is_warning(&self) -> bool {
        self.is_likely_ok() || self.is_redirect()
    }

    pub fn is_authority_file(&self) -> bool {
        matches!(self, UrlStatus::AuthorityFile { .. })
    }
}

impl fmt::Display for UrlStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UrlStatus::Ok { status_code } => write!(f, "OK ({})", status_code),
            UrlStatus::Redirect {
                status_code,
                location,
            } => write!(f, "REDIRECT ({}) -> {}", status_code, location),
            UrlStatus::Error {
                status_code,
                message,
            } => write!(f, "ERROR ({}): {}", status_code, message),
            UrlStatus::Timeout => write!(f, "TIMEOUT"),
            UrlStatus::NetworkError { message } => write!(f, "NETWORK ERROR: {}", message),
            UrlStatus::InvalidUrl { message } => write!(f, "INVALID URL: {}", message),
            UrlStatus::Skipped { reason } => write!(f, "SKIPPED: {}", reason),
            UrlStatus::RequiresBrowser { reason } => {
                write!(f, "LIKELY OK (requires browser): {}", reason)
            }
            UrlStatus::BlocksAutomation {
                status_code,
                reason,
            } => write!(
                f,
                "LIKELY OK ({}, blocks automation): {}",
                status_code, reason
            ),
            UrlStatus::AuthorityFile { authority_type } => {
                write!(f, "AUTHORITY FILE ({})", authority_type)
            }
            UrlStatus::PermanentlyRemoved {
                status_code,
                message,
            } => write!(f, "PERMANENTLY REMOVED ({}): {}", status_code, message),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UrlCheckReport {
    pub results: Vec<UrlCheckResult>,
    pub summary: UrlCheckSummary,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UrlCheckSummary {
    pub total: usize,
    pub ok: usize,
    pub redirect: usize,
    pub error: usize,
    pub timeout: usize,
    pub network_error: usize,
    pub invalid_url: usize,
    pub skipped: usize,
    pub requires_browser: usize,
    pub blocks_automation: usize,
    pub authority_file: usize,
    pub permanently_removed: usize,
}

impl UrlCheckReport {
    pub fn new(results: Vec<UrlCheckResult>) -> Self {
        let mut summary = UrlCheckSummary {
            total: results.len(),
            ok: 0,
            redirect: 0,
            error: 0,
            timeout: 0,
            network_error: 0,
            invalid_url: 0,
            skipped: 0,
            requires_browser: 0,
            blocks_automation: 0,
            authority_file: 0,
            permanently_removed: 0,
        };

        for result in &results {
            match &result.status {
                UrlStatus::Ok { .. } => summary.ok += 1,
                UrlStatus::Redirect { .. } => summary.redirect += 1,
                UrlStatus::Error { .. } => summary.error += 1,
                UrlStatus::Timeout => summary.timeout += 1,
                UrlStatus::NetworkError { .. } => summary.network_error += 1,
                UrlStatus::InvalidUrl { .. } => summary.invalid_url += 1,
                UrlStatus::Skipped { .. } => summary.skipped += 1,
                UrlStatus::RequiresBrowser { .. } => summary.requires_browser += 1,
                UrlStatus::BlocksAutomation { .. } => summary.blocks_automation += 1,
                UrlStatus::AuthorityFile { .. } => summary.authority_file += 1,
                UrlStatus::PermanentlyRemoved { .. } => summary.permanently_removed += 1,
            }
        }

        Self { results, summary }
    }

    pub fn print_text(&self) {
        println!("\n=== URL Check Report ===\n");
        println!("Total URLs checked: {}", self.summary.total);
        println!("  ✓ OK: {}", self.summary.ok);
        if self.summary.redirect > 0 {
            println!("  → Redirects: {}", self.summary.redirect);
        }
        if self.summary.authority_file > 0 {
            println!(
                "  📚 Authority files (trusted): {}",
                self.summary.authority_file
            );
        }
        if self.summary.requires_browser > 0 || self.summary.blocks_automation > 0 {
            let likely_ok = self.summary.requires_browser + self.summary.blocks_automation;
            println!("  ⓘ Likely OK (blocked automation): {}", likely_ok);
            if self.summary.requires_browser > 0 {
                println!(
                    "    - Requires browser/JS: {}",
                    self.summary.requires_browser
                );
            }
            if self.summary.blocks_automation > 0 {
                println!(
                    "    - Blocks HEAD requests: {}",
                    self.summary.blocks_automation
                );
            }
        }
        if self.summary.permanently_removed > 0 {
            println!(
                "  🗑️  Permanently Removed (410): {}",
                self.summary.permanently_removed
            );
        }
        if self.summary.error > 0 {
            println!("  ✗ Errors: {}", self.summary.error);
        }
        if self.summary.timeout > 0 {
            println!("  ⏱ Timeouts: {}", self.summary.timeout);
        }
        if self.summary.network_error > 0 {
            println!("  ⚠ Network Errors: {}", self.summary.network_error);
        }
        if self.summary.invalid_url > 0 {
            println!("  ⚠ Invalid URLs: {}", self.summary.invalid_url);
        }
        if self.summary.skipped > 0 {
            println!("  ⊘ Skipped: {}", self.summary.skipped);
        }

        // Show permanently removed URLs first (highest priority)
        let permanently_removed: Vec<_> = self
            .results
            .iter()
            .filter(|r| r.status.is_permanently_removed())
            .collect();

        if !permanently_removed.is_empty() {
            println!("\n=== Permanently Removed (410 Gone) - FIX IMMEDIATELY ===\n");
            println!("These URLs are intentionally removed and will never return:");
            println!();
            for result in permanently_removed {
                println!("File: {}", result.file_path);
                println!("Context: {}", result.context);
                println!("URL: {}", result.url);
                println!("Status: {}", result.status);
                println!();
            }
        }

        // Show other errors (excluding permanently removed)
        let errors: Vec<_> = self
            .results
            .iter()
            .filter(|r| r.status.is_error() && !r.status.is_permanently_removed())
            .collect();

        if !errors.is_empty() {
            println!("\n=== Broken Links ===\n");
            for result in errors {
                println!("File: {}", result.file_path);
                println!("Context: {}", result.context);
                println!("URL: {}", result.url);
                println!("Status: {}", result.status);
                println!();
            }
        }

        // Show likely OK URLs (false positives) - excluding authority files
        let likely_ok: Vec<_> = self
            .results
            .iter()
            .filter(|r| r.status.is_likely_ok() && !r.status.is_authority_file())
            .collect();

        if !likely_ok.is_empty() {
            println!("\n=== Likely OK (Blocked Automation) ===\n");
            println!("These URLs probably work but block automated checking:");
            println!();
            for result in likely_ok.iter().take(10) {
                println!("File: {}", result.file_path);
                println!("URL: {}", result.url);
                println!("Status: {}", result.status);
                println!();
            }
            if likely_ok.len() > 10 {
                println!("... and {} more", likely_ok.len() - 10);
                println!();
            }
        }

        // Note about authority files
        if self.summary.authority_file > 0 {
            println!(
                "\n=== Authority Files ({}) ===\n",
                self.summary.authority_file
            );
            println!("Authority files (controlled vocabularies) are trusted and not checked.");
            println!(
                "Examples: UNESCO Thesaurus, Getty Vocabularies, PeriodO, GeoNames, DOI, ORCID"
            );
            println!();
        }

        // Show redirects if any
        let redirects: Vec<_> = self
            .results
            .iter()
            .filter(|r| r.status.is_redirect())
            .collect();

        if !redirects.is_empty() {
            println!("\n=== Redirects (consider updating) ===\n");
            for result in redirects {
                println!("File: {}", result.file_path);
                println!("Context: {}", result.context);
                println!("URL: {}", result.url);
                println!("Status: {}", result.status);
                println!();
            }
        }
    }

    pub fn print_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    pub fn print_markdown(&self) {
        println!("# URL Check Report\n");
        println!("## Summary\n");
        println!("| Status | Count |");
        println!("|--------|-------|");
        println!("| Total | {} |", self.summary.total);
        println!("| ✓ OK | {} |", self.summary.ok);
        println!("| → Redirects | {} |", self.summary.redirect);
        println!("| ✗ Errors | {} |", self.summary.error);
        println!("| ⏱ Timeouts | {} |", self.summary.timeout);
        println!("| ⚠ Network Errors | {} |", self.summary.network_error);
        println!("| ⚠ Invalid URLs | {} |", self.summary.invalid_url);
        println!("| ⊘ Skipped | {} |", self.summary.skipped);

        let errors: Vec<_> = self
            .results
            .iter()
            .filter(|r| r.status.is_error())
            .collect();

        if !errors.is_empty() {
            println!("\n## Broken Links\n");
            println!("| File | Context | URL | Status |");
            println!("|------|---------|-----|--------|");
            for result in errors {
                println!(
                    "| {} | {} | {} | {} |",
                    result.file_path, result.context, result.url, result.status
                );
            }
        }

        let redirects: Vec<_> = self
            .results
            .iter()
            .filter(|r| r.status.is_redirect())
            .collect();

        if !redirects.is_empty() {
            println!("\n## Redirects (consider updating)\n");
            println!("| File | Context | URL | Status |");
            println!("|------|---------|-----|--------|");
            for result in redirects {
                println!(
                    "| {} | {} | {} | {} |",
                    result.file_path, result.context, result.url, result.status
                );
            }
        }
    }
}
