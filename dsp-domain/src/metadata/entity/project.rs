use serde::Serialize;

use crate::metadata::value::alternative_name::AlternativeName;
use crate::metadata::value::description::Description;
use crate::metadata::value::discipline::Discipline;
use crate::metadata::value::keyword::Keyword;
use crate::metadata::value::publication::Publication;
use crate::metadata::value::status::Status;
use crate::metadata::value::url::Url;
use crate::metadata::value::{
    ContactPoint, CreatedAt, CreatedBy, EndDate, HowToCite, Name, Shortcode, StartDate, TeaserText,
};

#[derive(Debug, Default, Clone, PartialEq, Serialize)]
pub struct Project {
    pub created_at: CreatedAt,
    pub created_by: CreatedBy,
    pub shortcode: Shortcode,
    pub name: Name,
    pub alternative_names: Vec<AlternativeName>,
    pub teaser_text: TeaserText,
    pub description: Description,
    pub url: Url,
    pub how_to_cite: HowToCite,
    pub start_date: StartDate,
    pub end_date: Option<EndDate>,
    pub status: Status,
    pub contact_point: Option<ContactPoint>,
    pub keywords: Vec<Keyword>,
    pub disciplines: Vec<Discipline>,
    pub publications: Vec<Publication>,
}
