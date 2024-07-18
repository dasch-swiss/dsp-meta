use std::collections::HashMap;

use serde::Deserialize;
use url::Url;

#[derive(Deserialize, Debug, PartialEq)]
struct ProjectMetadata {
    version: Version,
    project: Project,
}

#[derive(Deserialize, Debug, PartialEq)]
struct Project {
    keyword: Option<Vec<Keyword>>,
    #[serde(rename = "discipline")]
    disciplines: Vec<Discipline>,
}

#[derive(Deserialize, Debug, PartialEq)]
struct Version(usize);

#[derive(Deserialize, Debug, PartialEq)]
struct Keyword(HashMap<String, String>);

#[derive(Deserialize, Debug, PartialEq)]
struct Discipline {
    skos: Option<Vec<RefData>>,
    snf: Option<Vec<RefData>>,
    text: Option<Vec<LangTextData>>,
}

#[derive(Deserialize, Debug, PartialEq)]
struct RefData {
    ref_id: String,
    description: Option<String>,
    url: Option<Url>,
}

#[derive(Deserialize, Debug, PartialEq)]
struct LangTextData(HashMap<String, String>);

#[test]
fn test_toml() {
    let input = r#"
        version = 1

        [project]
        [[project.keyword]] 
        en = "English"
        de = "German"

        [[project.keyword]]
        fr = "French"

        [[project.discipline]]
        [[project.discipline.skos]]
        ref_id = "foo"
        description = "foo description"
        url = "http://example.com/foo"

        [[project.discipline.skos]]
        ref_id = "bar"
        description = "bar description"
        
        [[project.discipline.text]]
        age = "old"
        "#;
    let actual = toml::from_str::<ProjectMetadata>(input).unwrap();

    let input2 = r#"
        version = 1

        [project]
        keyword = [
            { en = "English", de = "German" },
            { fr = "French" }
        ]

        [[project.discipline]]
        skos = [
            { ref_id = "foo", description = "foo description", url = "http://example.com/foo" },
            { ref_id = "bar", description = "bar description" }
        ]

        [[project.discipline.text]]
        age = "old"
    "#;

    let actual2 = toml::from_str::<ProjectMetadata>(input2).unwrap();

    let exp_keywords = Some(vec![
        Keyword(
            [
                ("en".to_string(), "English".to_string()),
                ("de".to_string(), "German".to_string()),
            ]
            .iter()
            .cloned()
            .collect(),
        ),
        Keyword(
            [("fr".to_string(), "French".to_string())]
                .iter()
                .cloned()
                .collect(),
        ),
    ]);

    let exp_discipline = vec![Discipline {
        skos: Some(vec![
            RefData {
                ref_id: "foo".to_string(),
                description: Some("foo description".to_string()),
                url: Some(Url::parse("http://example.com/foo").unwrap()),
            },
            RefData {
                ref_id: "bar".to_string(),
                description: Some("bar description".to_string()),
                url: None
            },
        ]),
        snf: None,
        text: Some(vec![LangTextData(
            [("age".to_string(), "old".to_string())]
                .iter()
                .cloned()
                .collect(),
        )]),
    }];

    let exp_project = Project {
        keyword: exp_keywords,
        disciplines: exp_discipline,
    };
    let expected_metadata = ProjectMetadata {
        version: Version(1),
        project: exp_project,
    };
    assert_eq!(actual, expected_metadata);
    assert_eq!(actual2, expected_metadata);
}
