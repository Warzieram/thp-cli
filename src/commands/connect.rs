use std::process;

use clap::Args;
use regex::Regex;

#[derive(Args, Debug)]
pub struct ConnectArgs{
    pub email: String,
    pub password: String
}

pub fn run(args: ConnectArgs){
    println!("Connection a THP...");
    if !check_valid_email(&args.email) {
        eprintln!("Please use a valid email address");
        process::exit(1);
    }
    login(args);
}

pub fn login(args: ConnectArgs){
    println!("{}, {}", args.email, args.password);
}

pub fn check_valid_email(mail: &str) -> bool {
    let re = Regex::new(r"(^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+$)").unwrap();
    re.is_match(mail)
}

