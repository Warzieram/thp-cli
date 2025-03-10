use clap::Args;

#[derive(Args, Debug)]
pub struct SubmitArgs{
    pub name: String,
    pub link: String
}

pub fn run(args: SubmitArgs){
    println!("{}", args.name);
    println!("{}", args.link);
}
