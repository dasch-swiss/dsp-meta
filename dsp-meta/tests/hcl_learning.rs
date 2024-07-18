use hcl::Body;
use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq)]
pub struct User {
    name: String,
    email: String,
}

#[derive(Deserialize, Debug, PartialEq)]
pub enum Discipline {
    skos(RefData),
    snf(RefData),
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct Disciplines(Vec<Discipline>);

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
    assert_eq!(
        metadata,
        ProjectMetadata {
            version: Version(1),
            discipline: Discipline::skos(RefData {
                ref_id: "foo".to_string()
            }),
            user: User {
                name: "John Doe".to_string(),
                email: "john@doe.tld".to_string()
            }
        }
    );
}
