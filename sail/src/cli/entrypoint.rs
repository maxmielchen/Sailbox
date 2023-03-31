use clap::Subcommand;
use clap::Parser;


/// Tool to manage a Sailbox
#[derive(Parser)]
#[command()]
pub struct Cli
{
    #[command(subcommand)]
    tool : Tool,
}

#[derive(Subcommand)]
#[command()]
pub enum Tool
{
    User
    {
        #[command(subcommand)]
        user : User,
    },

    Project
    {
        #[command(subcommand)]
        project : Project,
    }
}

#[derive(Subcommand)]
#[command()]
pub enum User
{
    Create
    {
        /// Your username
        #[arg(short, long)]
        username : String,

        /// Your password
        #[arg(short, long)]
        password : String,

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
        #[arg(short, long)]
        username : String,
    }
}

#[derive(Subcommand)]
#[command()]
pub enum Project
{
    Create
    {
        /// Project owner and directory entrypoint
        #[arg(short, long)]
        owner : String,

        /// The dir/project name
        #[arg(short, long)]
        name : String,
    },

    Delete
    {
        /// Project owner and directory entrypoint
        #[arg(short, long)]
        owner : String,

        /// The dir/project name
        #[arg(short, long)]
        name : String,
    }
}

