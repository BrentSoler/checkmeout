use checkmeout::{git::Git, prompt::Prompt};
use colored::Colorize;

fn main() {
    println!(
        "{}\n",
        "
 ██████ ██   ██ ███████  ██████ ██   ██ ███    ███ ███████  ██████  ██    ██ ████████ 
██      ██   ██ ██      ██      ██  ██  ████  ████ ██      ██    ██ ██    ██    ██    
██      ███████ █████   ██      █████   ██ ████ ██ █████   ██    ██ ██    ██    ██    
██      ██   ██ ██      ██      ██  ██  ██  ██  ██ ██      ██    ██ ██    ██    ██    
 ██████ ██   ██ ███████  ██████ ██   ██ ██      ██ ███████  ██████   ██████     ██"
            .green()
    );

    let fetch = Git::new(vec!["fetch", "--prune", "--all"])
        .exec()
        .handle_err()
        .build();
    println!("{}", fetch.out);

    let get_branches = Git::new(vec!["branch", "-a"]).exec().handle_err().build();
    let branches = get_branches.parse_branch();

    let selected_branch = Prompt::new("Branches Fetched", branches);

    if selected_branch.input.contains("remotes") {
        let switch_branch = Git::new(vec![
            "switch",
            &selected_branch
                .input
                .split("remotes/origin/")
                .collect::<Vec<&str>>()[1],
        ])
        .exec()
        .handle_err()
        .build();

        if &switch_branch.err == "Local branch is behind" {
            let pull = Git::new(vec!["pull"]).exec().handle_err().build();

            println!("{}", pull.out);
        }

        println!("{}", switch_branch.out);
    } else {
        let checkout = Git::new(vec!["checkout", &selected_branch.input])
            .exec()
            .handle_err()
            .build();

        println!("{}", checkout.out);
    }
}
