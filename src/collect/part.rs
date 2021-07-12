pub enum PartKind {
    Short,
    Full,
}

pub enum PartStatus {
    Default,
    Wip,
    Mvp,
    Archived,
}

pub struct Part {
    kind: PartKind,
    language: String,
    status: PartStatus,
    repo: String,
    tags: Vec<String>,
    markdown_text: String,
}

impl Part {
    fn parse(kind: PartKind, source: &str) -> Self {
        unimplemented!()
    }
}
