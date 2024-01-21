use dsp_domain::metadata::entity::project::Project;
use extracted_project_attributes::ExtractedProjectAttributes;
use extracted_project_blocks::ExtractedProjectBlocks;

use crate::api::convert::hcl::hcl_block::HclBlock;
use crate::error::DspMetaError;

pub(crate) mod extracted_project_attributes;
pub(crate) mod extracted_project_blocks;

impl<'a> TryInto<Project> for HclBlock<'a> {
    type Error = DspMetaError;

    fn try_into(self) -> Result<Project, Self::Error> {
        if self.0.identifier.as_str() != "project" {
            return Err(DspMetaError::ParseProject(
                format!(
                    "Parse error: project block needs to be named 'project', however got '{}' instead.",
                    self.0.identifier.as_str()
                )
                .to_string(),
            ));
        }

        // extract the project attributes
        // created_at, created_by, shortcode, name, teaser_text, how_to_cite, start_date, end_date,
        // datasets (1-n), funders, grants

        let attributes: Vec<&hcl::Attribute> = self.0.body.attributes().collect();

        let extracted_attributes = ExtractedProjectAttributes::try_from(attributes)?;

        let id = extracted_attributes.id.ok_or(DspMetaError::ParseProject(
            "Parse error: project needs to have an id.".to_string(),
        ))?;

        let created_at = extracted_attributes
            .created_at
            .ok_or(DspMetaError::ParseProject(
                "Parse error: project needs to have a created_at value.".to_string(),
            ))?;

        let created_by = extracted_attributes
            .created_by
            .ok_or(DspMetaError::ParseProject(
                "Parse error: project needs to have a created_by value.".to_string(),
            ))?;

        let shortcode = extracted_attributes
            .shortcode
            .ok_or(DspMetaError::ParseProject(
                "Parse error: project needs to have a shortcode.".to_string(),
            ))?;

        let name = extracted_attributes.name.ok_or(DspMetaError::ParseProject(
            "Parse error: project needs to have a name.".to_string(),
        ))?;

        let teaser_text = extracted_attributes
            .teaser_text
            .ok_or(DspMetaError::ParseProject(
                "Parse error: project needs to have a teaser_text.".to_string(),
            ))?;

        let how_to_cite = extracted_attributes
            .how_to_cite
            .ok_or(DspMetaError::ParseProject(
                "Parse error: project needs to have a how_to_cite.".to_string(),
            ))?;

        let start_date = extracted_attributes
            .start_date
            .ok_or(DspMetaError::ParseProject(
                "Parse error: project needs to have a start_date.".to_string(),
            ))?;

        let end_date = extracted_attributes.end_date;

        let status = extracted_attributes
            .status
            .ok_or(DspMetaError::ParseProject(
                "Parse error: project needs to have a status.".to_string(),
            ))?;

        let contact_point = extracted_attributes.contact_point;

        let datasets = extracted_attributes.datasets;

        // extract the project blocks
        // alternative_names (0-n), description (1), url (1), keywords (1-n), disciplines (1-n),
        // publications (0-n)

        let blocks: Vec<&hcl::Block> = self.0.body.blocks().collect();
        let extracted_blocks = ExtractedProjectBlocks::try_from(blocks)?;

        let alternative_names = extracted_blocks.alternative_names;
        let description = extracted_blocks
            .description
            .ok_or(DspMetaError::ParseProject(
                "Parse error: project needs to have a description.".to_string(),
            ))?;
        let url = extracted_blocks.url.ok_or(DspMetaError::ParseProject(
            "Parse error: project needs to have a url.".to_string(),
        ))?;
        let keywords = if !extracted_blocks.keywords.is_empty() {
            Ok(extracted_blocks.keywords)
        } else {
            Err(DspMetaError::ParseProject(
                "Parse error: project needs to have at least one keyword.".to_string(),
            ))
        }?;
        let disciplines = if !extracted_blocks.disciplines.is_empty() {
            Ok(extracted_blocks.disciplines)
        } else {
            Err(DspMetaError::ParseProject(
                "Parse error: project needs to have at least one discipline.".to_string(),
            ))
        }?;
        let publications = extracted_blocks.publications;

        let project = Project {
            id,
            created_at,
            created_by,
            shortcode,
            name,
            alternative_names,
            teaser_text,
            description,
            url,
            how_to_cite,
            start_date,
            end_date,
            status,
            contact_point,
            keywords,
            disciplines,
            publications,
            datasets,
        };

        Ok(project)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use dsp_domain::metadata::entity::project::Project;
    use dsp_domain::metadata::value::description::Description;
    use dsp_domain::metadata::value::discipline::Discipline;
    use dsp_domain::metadata::value::identifier::{DatasetId, ProjectId};
    use dsp_domain::metadata::value::iso_code::IsoCode;
    use dsp_domain::metadata::value::keyword::Keyword;
    use dsp_domain::metadata::value::ref_data::RefData;
    use dsp_domain::metadata::value::url::Url;
    use dsp_domain::metadata::value::{
        ContactPoint, CreatedAt, CreatedBy, EndDate, HowToCite, Name, Shortcode, StartDate,
        TeaserText,
    };
    use hcl::block;
    use tracing_test::traced_test;

    use super::*;
    use crate::api::convert::hcl::hcl_block::HclBlock;

    #[traced_test]
    #[test]
    fn test_convert_project_block() {
        let input_project_block = block!(
            project {
                id = "p1"
                created_at = 1630601274523025000u64 // FIXME: is there a more readable way to write an i64?
                created_by  = "dsp-metadata-gui"
                shortcode = "0803"
                name = "The German Family Panel (pairfam)"
                alternative_name "1" {
                    de = "Der deutsche Familienpanel (pairfam)"
                    en = "The German Family Panel (pairfam)"
                }
                teaser_text = "The German Family Panel (pairfam) is a multidisciplinary, longitudinal study."
                description {
                    de = "Der deutsche Familienpanel (pairfam) ist eine multidisziplinäre, längsschnittliche Studie."
                    en = "The German Family Panel (pairfam) is a multidisciplinary, longitudinal study."
                }
                url {
                    href = "https://admin.dasch.swiss/project/081C"
                    label = "Discover Project Data"
                }
                how_to_cite = "Huinink, Johannes; Schröder, Carolin; Castiglioni, Laura; Feldhaus, Michael"
                start_date  = "2009-04-01"
                end_date    = "2012-03-31"
                status      = "Ongoing"
                keyword {
                    en = "Bern"
                }
                discipline snf {
                    ref_id = "10302"
                    description = "Schweizer Geschichte"
                    url = "https://www.snf.ch/SiteCollectionDocuments/allg_disziplinenliste.pdf"
                }
                contact_point = "project_organization"
                datasets = ["ds1", "ds2"]
            }
        );
        let project: Project = HclBlock(&input_project_block).try_into().unwrap();
        assert_eq!(project.id, ProjectId(String::from("p1")));
        assert_eq!(project.created_at, CreatedAt(1630601274523025000));
        assert_eq!(
            project.created_by,
            CreatedBy(String::from("dsp-metadata-gui"))
        );
        assert_eq!(project.shortcode, Shortcode(String::from("0803")));
        assert_eq!(
            project.name,
            Name(String::from("The German Family Panel (pairfam)"))
        );
        assert_eq!(
            project.teaser_text,
            TeaserText(String::from(
                "The German Family Panel (pairfam) is a multidisciplinary, longitudinal study."
            ))
        );
        assert_eq!(
            project.description,
            Description(
                HashMap::from_iter(
                    [
                        (IsoCode::DE, "Der deutsche Familienpanel (pairfam) ist eine multidisziplinäre, längsschnittliche Studie.".to_string()),
                        (IsoCode::EN, "The German Family Panel (pairfam) is a multidisciplinary, longitudinal study.".to_string())
                    ]
                )
            )
        );
        assert_eq!(
            project.url,
            Url {
                href: url::Url::parse("https://admin.dasch.swiss/project/081C").unwrap(),
                label: "Discover Project Data".to_string()
            }
        );
        assert_eq!(
            project.how_to_cite,
            HowToCite(String::from(
                "Huinink, Johannes; Schröder, Carolin; Castiglioni, Laura; Feldhaus, Michael"
            ))
        );
        assert_eq!(project.start_date, StartDate(String::from("2009-04-01")));
        assert_eq!(project.end_date, Some(EndDate(String::from("2012-03-31"))));
        assert_eq!(
            project.status,
            dsp_domain::metadata::value::status::Status::Ongoing
        );
        assert_eq!(
            project.keywords,
            vec![Keyword(HashMap::from_iter([(
                IsoCode::EN,
                "Bern".to_string()
            )]))]
        );
        assert_eq!(
            project.disciplines,
            vec![Discipline::Snf(RefData {
                ref_id: "10302".to_string(),
                description: "Schweizer Geschichte".to_string(),
                url: url::Url::parse(
                    "https://www.snf.ch/SiteCollectionDocuments/allg_disziplinenliste.pdf"
                )
                .unwrap()
            })]
        );
        assert_eq!(
            project.contact_point,
            Some(ContactPoint(String::from("project_organization")))
        );
        assert_eq!(
            project.datasets,
            vec![DatasetId("ds1".to_string()), DatasetId("ds2".to_string())]
        );
    }
}
