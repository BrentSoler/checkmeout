use std::process::{self, Command};

pub struct Git {
    out: Option<String>,
    err: Option<String>,
}

impl Git {
    pub fn new(args: Vec<&str>) -> GitBuilder {
        return GitBuilder { args };
    }
}

pub struct GitBuilder<'a> {
    args: Vec<&'a str>,
}

impl<'a> GitBuilder<'a> {
    pub fn exec(&self) {
        let cmd = Command::new("git")
            .args(&self.args)
            .output()
            .unwrap_or_else(|err| {
                println!("Error: {}", err);
                process::exit(0);
            });
        println!("{:?}", cmd)
    }
}
