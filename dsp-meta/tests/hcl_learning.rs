use std::collections::HashMap;

use hcl::Body;
use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq)]
pub struct User {
    name: String,
    email: String,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct Discipline {
    skos: Option<Skos>,
    snf: Vec<Snf>,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct Skos(RefData);

#[derive(Deserialize, Debug, PartialEq)]
pub struct Snf(RefData);

#[derive(Deserialize, Debug, PartialEq)]
pub struct RefData {
    ref_id: String,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct ProjectMetadata {
    version: Version,
    discipline: Discipline,
    user: User,
    #[serde(rename = "keyword")]
    keywords: Vec<Keyword>,
}

#[derive(Deserialize, Debug, PartialEq)]
struct Version(usize);

#[derive(Deserialize, Debug, PartialEq)]
struct Keyword(HashMap<String, String>);

#[test]
fn test_hcl() {
    let input = r#"
        version = 1
        discipline skos {
            ref_id = "foo"
        }
        discipline snf {
            ref_id = "snf1"
        }
        discipline snf {
            ref_id = "snf2"
        }
        user {
            name = "John Doe"
            email = "john@doe.tld"
        }
        keyword {
          en ="word"
          de = "wort"
        }
        keyword {
          en ="word2"
        }
    "#;

    let body: Body = hcl::from_str(input).expect("Failed to parse");

    let metadata: ProjectMetadata = hcl::from_body(body).expect("Failed to parse");

    let exp = Discipline {
        skos: Some(Skos(RefData {
            ref_id: "foo".to_string(),
        })),
        snf: vec![
            Snf(RefData {
                ref_id: "snf1".to_string(),
            }),
            Snf(RefData {
                ref_id: "snf2".to_string(),
            }),
        ],
    };

    let mut exp_keywords = Vec::new();
    let keyw1 = Keyword({
        let mut map = HashMap::new();
        map.insert("en".to_string(), "word".to_string());
        map.insert("de".to_string(), "wort".to_string());
        map
    });
    let kew2 = Keyword({
        let mut map = HashMap::new();
        map.insert("en".to_string(), "word2".to_string());
        map
    });
    exp_keywords.append(&mut vec![keyw1, kew2]);

    assert_eq!(
        metadata,
        ProjectMetadata {
            version: Version(1),
            discipline: exp,
            keywords: exp_keywords,
            user: User {
                name: "John Doe".to_string(),
                email: "john@doe.tld".to_string()
            }
        }
    );
}
