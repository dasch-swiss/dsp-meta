#![no_main]

use dsp_domain::metadata::entity::project_metadata::ProjectMetadata;
use dsp_meta::api::convert::hcl::hcl_body::HclBody;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    // fuzzed code goes here

    if let Ok(input) = std::str::from_utf8(data) {
        let ascii_string = if input.is_ascii() {
            input.to_string()
        } else {
            return;
        };

        if let Ok(body) = hcl::from_str(&ascii_string) {
            if let Ok(project_metadata) = HclBody(&body).try_into() {
                let _: ProjectMetadata = project_metadata;
            }
        }
    }
    return;
});
