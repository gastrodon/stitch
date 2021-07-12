pub enum ProjectKind {
    Short,
    Full,
}

pub enum ProjectStatus {
    Default,
    Wip,
    Mvp,
    Archived,
}

pub struct Project {
    kind: ProjectKind,
    language: String,
    status: ProjectStatus,
    repo: String,
    tags: Vec<String>,
    markdown_text: String,
}

impl Project {
    fn parse(kind: ProjectKind, source: &str) -> Self {
        unimplemented!()
    }
}
