pub mod builder;
use builder::GitBuilder;

#[derive(Debug)]
pub struct Git {
    pub out: String,
    pub err: String,
}

impl Git {
    pub fn new(args: Vec<&str>) -> GitBuilder {
        return GitBuilder {
            args,
            out: None,
            err: None,
        };
    }

    pub fn parse_branch(&self) -> Vec<&str> {
        return self
            .out
            .trim()
            .split("\n")
            .into_iter()
            .map(|branch| branch.trim())
            .collect();
    }
}
