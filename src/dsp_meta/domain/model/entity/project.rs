use serde::Serialize;

use crate::domain::model::value::alternative_name::AlternativeName;
use crate::domain::model::value::description::Description;
use crate::domain::model::value::discipline::Discipline;
use crate::domain::model::value::keyword::Keyword;
use crate::domain::model::value::publication::Publication;
use crate::domain::model::value::url::Url;
use crate::domain::model::value::{
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
    pub contact_point: Option<ContactPoint>,
    pub keywords: Vec<Keyword>,
    pub disciplines: Vec<Discipline>,
    pub publications: Vec<Publication>,
}
