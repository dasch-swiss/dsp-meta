use serde::Serialize;

use crate::metadata::value::access::Access;
use crate::metadata::value::attribution::Attribution;
use crate::metadata::value::data_type::DataType;
use crate::metadata::value::identifier::DatasetId;
use crate::metadata::value::language::Language;
use crate::metadata::value::license::License;
use crate::metadata::value::r#abstract::Abstract;
use crate::metadata::value::status::Status;
use crate::metadata::value::url::Url;
use crate::metadata::value::{CreatedAt, CreatedBy, DatePublished, HowToCite, Title};

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Dataset {
    pub id: DatasetId,                         // (1)
    pub created_at: CreatedAt,                 // (1)
    pub created_by: CreatedBy,                 // (1)
    pub title: Title,                          // (1)
    pub status: Status,                        // (1)
    pub access_conditions: Access,             // (1)
    pub how_to_cite: HowToCite,                // (1)
    pub date_published: Option<DatePublished>, // (0-1)
    pub type_of_data: Vec<DataType>,           // (1-n)
    pub alternative_titles: Vec<Title>,        // (0-n)
    pub abstracts: Vec<Abstract>,              // (1-n)
    pub licenses: Vec<License>,                // (1-n)
    pub languages: Vec<Language>,              // (1-n)
    pub attributions: Vec<Attribution>,        // (1-n)

    pub urls: Vec<Url>, // (0-n)
}
