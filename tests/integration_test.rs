use dsp_meta::load;

#[test]
fn load_dokubib_config() {
    let metadata = load("./data/dokubib.hcl").unwrap();
    assert_eq!(metadata.version.0, 1);
}

#[test]
fn load_hdm_config() {
    let metadata = load("./data/hdm.hcl").unwrap();
    assert_eq!(metadata.version.0, 1);
}

#[test]
fn load_incunabula_config() {
    let metadata = load("./data/incunabula.hcl").unwrap();
    assert_eq!(metadata.version.0, 1);
}
