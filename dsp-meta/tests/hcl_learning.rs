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
    snf: Option<Snf>,
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
}

#[derive(Deserialize, Debug, PartialEq)]
struct Version(usize);

#[test]
fn test_hcl() {
    let input = r#"
        version = 1
        discipline skos {
            ref_id = "foo"
        }
        discipline snf {
            ref_id = "bar"
        }
        user {
            name = "John Doe"
            email = "john@doe.tld"
        }
    "#;

    let body: Body = hcl::from_str(input).expect("Failed to parse");

    let metadata: ProjectMetadata = hcl::from_body(body).expect("Failed to parse");
    let exp = Discipline {
        skos: Some(Skos(RefData {
            ref_id: "foo".to_string(),
        })),
        snf: Some(Snf(RefData {
            ref_id: "bar".to_string(),
        })),
    };
    assert_eq!(
        metadata,
        ProjectMetadata {
            version: Version(1),
            discipline: exp,
            user: User {
                name: "John Doe".to_string(),
                email: "john@doe.tld".to_string()
            }
        }
    );
}
