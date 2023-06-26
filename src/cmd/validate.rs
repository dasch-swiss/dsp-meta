use std::fmt::Debug;
use std::fs::Metadata;

/// Read projects definition from .toml
pub fn read_metadata() {
    println! {"Hello from read_metadata!"}

    use serde::{Deserialize, Serialize};

    #[derive(Debug, Deserialize, Serialize)]
    struct Metadata {
        project: Project,
        datasets: Vec<Dataset>,
        persons: Vec<Person>,
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct Project {
        id: String,
        shortcode: String,
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct Dataset {
        id: String,
        title: String,
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct Person {
        id: String,
    }

    let metadata: Metadata = toml::from_str(
        r#"


        [Project]
        id = "0000-project"
        shortcode = "0000"

        [[Dataset]]
        id = "dataset-001"
        title = "Dataset 1"

        [[Dataset]]
        id = "dataset-002"
        title = "Dataset 2"

        [[Person]]
        id = "person-001"

        [[Person]]
        id = "person-002"
    "#,
    )
    .unwrap();

    print!("{metadata:?}")
}
