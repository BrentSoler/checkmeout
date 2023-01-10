use checkmeout;

fn main() {
    let test = checkmeout::git::Git::new(["branch","-a"].to_vec()).exec();
}
