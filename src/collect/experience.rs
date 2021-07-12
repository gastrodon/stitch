pub struct Experience {
    company: String,
    title: String,
    from: Option<(u32, u32)>,
    to: Option<(u32, u32)>,
    markdown_text: String,
}

impl Experience {
    fn parse(source: &str) -> Self {
        unimplemented!()
    }
}
