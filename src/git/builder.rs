use super::Git;
use std::process::{self, Command};

pub struct GitBuilder<'a> {
    pub args: Vec<&'a str>,
    pub out: Option<String>,
    pub err: Option<String>,
}

impl<'a> GitBuilder<'a> {
    pub fn exec(&mut self) -> &mut Self {
        let cmd = Command::new("git")
            .args(&self.args)
            .output()
            .unwrap_or_else(|err| {
                println!("Error: {}", err);
                process::exit(0);
            });

        self.out = Some(String::from_utf8(cmd.stdout).unwrap());
        self.err = if cmd.stderr.len() != 0 {
            Some(String::from_utf8(cmd.stderr).unwrap())
        } else {
            None
        };

        return self;
    }

    pub fn handle_err(&mut self) -> &Self {
        if let Some(err) = &mut self.err {
            if err.contains("From https://github.com/BrentSoler/checkmeout") {
                println!("{}", err);
                return self;
            }

            if err.contains("Switched") {
                if let Some(out) = &self.out {
                    if out.contains("Your branch is behind") {
                        *err = String::from("Local branch is behind");
                    }
                }

                println!("{}", err);

                return self;
            }

            println!("{}", err);
            process::exit(0)
        }

        return self;
    }

    pub fn build(&self) -> Git {
        return Git {
            out: self.out.clone().unwrap(),
            err: self.err.clone().unwrap_or_else(|| String::new()),
        };
    }
}
