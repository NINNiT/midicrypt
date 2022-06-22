use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct CliArgs {
    #[clap(short, long, value_parser)]
    port: String,
}
