use serde::Serialize;

use crate::error::DspDomainError;
use crate::error::DspDomainError::CreateValueObject;

/// Represents an HCL block which consists of attribute keys and a value expressions.
///
/// In HCL syntax this is represented as:
///
/// ```hcl
/// license {
///     type = "creative_commons"
///     href = "https://creativecommons.org/licenses/by-nc/4.0"
///     date = "2021-09-02"
///     label = "CC BY-NC 4.0"
/// }
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct License {
    pub license_type: String,
    pub href: url::Url,
    pub date: String,
    pub label: String,
}

impl License {
    pub fn new(
        license_type: String,
        url_string: String,
        date: String,
        label: String,
    ) -> Result<Self, DspDomainError> {
        let maybe_url = url::Url::try_from(url_string.as_str());
        match maybe_url {
            Ok(href) => Ok(License {
                license_type,
                href,
                date,
                label,
            }),
            Err(_) => Err(CreateValueObject(
                "Creating an UrlValue failed because provided value is not a valid URL."
                    .to_string(),
            )),
        }
    }
}

impl Default for License {
    fn default() -> Self {
        License {
            license_type: "creative_commons".to_string(),
            href: url::Url::try_from("https://default.xyz").unwrap(),
            date: "2021-09-02".to_string(),
            label: "Default license label".to_string(),
        }
    }
}
