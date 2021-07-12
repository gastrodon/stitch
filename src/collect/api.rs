use std::fs;
use walkdir::WalkDir;

pub fn collect_raw_parts(root: &str) -> Vec<String> {
    WalkDir::new(root)
        .follow_links(true)
        .into_iter()
        .map(|it| it.unwrap_or_else(|err| panic!("{}", err)).into_path())
        .filter(|it| it.is_file())
        .map(|it| fs::read_to_string(&it).unwrap_or_else(|err| panic!("{}", err)))
        .collect()
}
