use std::process::{self, Command};

#[derive(Debug)]
pub struct Git {
    pub out: String,
    pub err: String,
}

impl Git {
    pub fn new<'a>(args: [&'a str]) -> GitBuilder {
        return GitBuilder {
            args,
            out: None,
            err: None,
        };
    }
}

pub struct GitBuilder<'a> {
    args: [&'a str],
    out: Option<String>,
    err: Option<String>,
}

impl<'a> GitBuilder<'a> {
    pub fn exec(&self) -> Self {
        let cmd = Command::new("git")
            .args(&self.args)
            .output()
            .unwrap_or_else(|err| {
                println!("Error: {}", err);
                process::exit(0);
            });

        return Self {
            args: self.args,
            out: Some(String::from_utf8(cmd.stdout).unwrap()),
            err: Some(String::from_utf8(cmd.stderr).unwrap()),
        };
    }
}
