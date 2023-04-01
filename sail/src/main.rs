mod cli;
mod lib;

use cli::dsl::Cli;
use cli::action::validate;

use clap::Parser;

fn main()
{
    let cli = Cli::parse();

    validate(&cli)
}
