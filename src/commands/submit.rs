use clap::Args;
use regex::Regex;
use std::process;

#[derive(Args, Debug)]
pub struct SubmitArgs {
    pub name: String,
    pub link: String,
}

pub fn run(args: SubmitArgs) {
    if check_valid_github_repo_url(&args.link) {
        println!("{}", args.link);
    } else {
        eprint!("Not a valid github repo link !");
        process::exit(1);
    };
    println!("submitting {}", args.name);
    println!("link : {}", args.link);
}

pub fn check_valid_github_repo_url(url: &str) -> bool {
    let re =
        Regex::new(r"^https://github\.com/([a-zA-Z0-9_-]+)/([a-zA-Z0-9_.-]+)(\.git)?$").unwrap();
    re.is_match(url)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn valid_github_repo_link() {
        let link = "https://github.com/Warzieram/thp-cli";
        assert_eq!(true, check_valid_github_repo_url(link));
    }

    #[test]
    fn invalid_github_repo_link() {
        let link = "https://github.com//thp-cli";
        assert_eq!(false, check_valid_github_repo_url(link));
    }
}
