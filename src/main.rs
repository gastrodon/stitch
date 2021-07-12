mod collect;

use argparse::{ArgumentParser, Collect, StoreOption};
use std::{collections::HashSet, iter::FromIterator};

static PARTS_ROOT_DEFAULT: &str = "./parts";

#[derive(Debug)]
struct Arguments {
    parts_root: Option<String>,
    tags: HashSet<String>,
}

impl Arguments {
    fn new() -> Self {
        Self {
            parts_root: None,
            tags: HashSet::new(),
        }
    }
}

fn collect_args() -> Arguments {
    let mut args = Arguments::new();
    let mut tags_buffer = Vec::<String>::new();

    {
        let mut parser = ArgumentParser::new();

        parser.set_description("Render a resume from some parts");
        parser.refer(&mut args.parts_root).add_option(
            &["-p", "--parts"],
            StoreOption,
            "Markdown parts root directory",
        );
        parser.refer(&mut tags_buffer).add_option(
            &["-t", "--tags"],
            Collect,
            "Tags to optimize for",
        );

        parser.parse_args_or_exit();
    }

    args.tags = HashSet::from_iter(tags_buffer);
    args
}

fn main() {
    let args = collect_args();
    let raw = collect::collect_raw_parts(
        &args
            .parts_root
            .as_ref()
            .unwrap_or(&PARTS_ROOT_DEFAULT.to_string()),
    );
}
