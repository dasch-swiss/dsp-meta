use serde::{Deserialize, Serialize};

use crate::domain::value_objects::{
    AlternativeNames, CreatedAt, CreatedBy, Datasets, Description, EndDate, Funders, Grants,
    HowToCite, Name, Shortcode, StartDate, TeaserText, ID,
};

// no need for smart constructors here, as there is no validation happening
#[derive(Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct Project {
    pub id: ID,
    pub created_at: CreatedAt,
    pub created_by: CreatedBy,
    pub shortcode: Shortcode,
    pub name: Name,
    pub alternative_names: AlternativeNames,
    pub teaser_text: TeaserText,
    pub description: Description,
    pub how_to_cite: HowToCite,
    pub start_date: StartDate,
    pub end_date: Option<EndDate>,
    pub datasets: Datasets,
    pub funders: Funders,
    pub grants: Option<Grants>,
}
