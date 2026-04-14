use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct CommandArgs {

    #[clap(short, long = "repo")]
    pub repo_path: String,

    #[clap(short, long = "file")]
    pub file_path: String,

    #[clap(short, long)]
    pub phrase: String,

    #[clap(short, long, default_value = "5")] //, default_value = "true"
    pub max_count: usize,
    
    #[clap(short, long)]
    pub verbose: bool
}