use std::collections::HashMap;

use serde::*;

#[derive(Debug, Serialize)]
struct Project {
    id: Iri,
    name: String,
    description: LangString,
    shortcode: String,
    datasets: Vec<Dataset>,
}

#[derive(Debug, Serialize)]
struct Dataset {
    id: Iri,
    title: String,
}

type Iri = String;

#[derive(Debug, Serialize)]
pub struct LangString(pub HashMap<IsoCode, String>);

#[derive(Debug, Default, Serialize, PartialEq, Eq, Hash)]
pub enum IsoCode {
    #[default]
    DE, // German
    EN, // English
    FR, // French
    IT, // Italian
    ES, // Spanish
    PT, // Portuguese
    NL, // Dutch
    PL, // Polish
    RU, // Russian
    JA, // Japanese
    ZH, // Chinese
    AR, // Arabic
    FA, // Persian
}

fn main() {
    let mut name = HashMap::<IsoCode, String>::new();
    name.insert(IsoCode::EN, "Hôtel de Musique Bern".to_string());

    let dataset = Dataset {
        id: "dataset-0".to_string(),
        title: "Hôtel de Musique Bern".to_string(),
    };

    let project = Project {
        id: "https://ark.dasch.swiss/ark:/72163/1/081C".to_string(),
        name: "Hôtel de Musique Bern".to_string(),
        description: LangString(name),
        shortcode: "081C".to_string(),
        datasets: vec![dataset],
    };

    let project_ttl = serde_rdf::to_string(&project).unwrap();

    dbg!(project_ttl);
}
