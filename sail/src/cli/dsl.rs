use clap::Subcommand;
use clap::Parser;


/// Tool to manage a Sailbox
#[derive(Parser)]
#[command()]
pub struct Cli
{
    #[command(subcommand)]
    pub tool : Option<Tool>,
}

#[derive(Subcommand)]
#[command()]
pub enum Tool
{
    User
    {
        #[command(subcommand)]
        user : Option<User>,
    }
}

#[derive(Subcommand)]
#[command()]
pub enum User
{
    Create
    {
        /// Your username
        #[arg(short, long, required = false)]
        username : Option<String>,

        /// Your password
        #[arg(short, long, required = false)]
        password : Option<String>,

        /// Root rights
        #[arg(short, long, default_value_t = false)]
        root : bool,

        /// Sudo rights
        #[arg(short, long, default_value_t = false)]
        sudo : bool,
    },

    Delete
    {
        /// Your username
        #[arg(short, long, required = false)]
        username : Option<String>,
    }
}