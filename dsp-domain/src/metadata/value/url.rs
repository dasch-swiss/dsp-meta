use serde::Serialize;

/// Represents an HCL attribute which consists of an attribute key and a value expression.
///
/// In HCL syntax this is represented as:
///
/// ```hcl
/// url {
///   href = "https://www.google.com"
///   label = "text describing the link"
/// }
/// ```
///
/// Use [`Attribute::new`] to construct an [`Attribute`] from a value that is convertible to this
/// crate's [`Expression`] type.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct Url {
    pub href: url::Url,
    pub label: String,
}

impl Url {
    pub fn new(url_string: String, label: String) -> Result<Self, String> {
        let maybe_url = url::Url::try_from(url_string.as_str());
        match maybe_url {
            Ok(href) => Ok(Url { href, label }),
            Err(_) => Err(
                "Creating an UrlValue failed because provided value is not a valid URL."
                    .to_string(),
            ),
        }
    }
}

impl Default for Url {
    fn default() -> Self {
        Url {
            href: url::Url::try_from("https://default.xyz").unwrap(),
            label: "Default URL description".to_string(),
        }
    }
}
