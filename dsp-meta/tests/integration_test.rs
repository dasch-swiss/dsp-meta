mod common;

use std::env;

use common::load;

#[test]
fn load_dokubib_config() {
    let file = env::current_dir()
        .unwrap()
        .parent()
        .unwrap()
        .join("data")
        .join("dokubib.hcl");
    let metadata = load(file).unwrap();
    assert_eq!(metadata.version.0, 1);
}

#[test]
fn load_hdm_config() {
    let file = env::current_dir()
        .unwrap()
        .parent()
        .unwrap()
        .join("data")
        .join("hdm.hcl");
    let metadata = load(file).unwrap();
    assert_eq!(metadata.version.0, 1);
}

#[test]
fn load_incunabula_config() {
    let file = env::current_dir()
        .unwrap()
        .parent()
        .unwrap()
        .join("data")
        .join("incunabula.hcl");
    let metadata = load(file).unwrap();
    assert_eq!(metadata.version.0, 1);
}
