mod cli;
mod system;

use cli::entrypoint::Cli;

use clap::Parser;

fn main()
{
    let cli = Cli::parse();
}
