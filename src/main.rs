use checkmeout::{git::Git, prompt::Prompt};

fn main() {
    let fetch = Git::new(vec!["fetch", "--prune", "--all"]).exec();

    println!("{}", fetch.out);

    let branches = Git::new(vec!["branch", "-a"]).exec();
    println!("{:?}", branches)
}
