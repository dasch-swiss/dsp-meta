use std::path::{Path, PathBuf};

pub fn load_hcl_file_paths(data_path: &Path) -> Vec<PathBuf> {
    // get paths of HCL files
    std::fs::read_dir(data_path)
        .expect("read directory containing HCL files.")
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("hcl") {
                Some(path)
            } else {
                None
            }
        })
        .collect::<Vec<PathBuf>>()
}
