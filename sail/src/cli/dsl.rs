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
    },

    Project
    {
        #[command(subcommand)]
        project : Option<Project>,
    },

    /* Not implemented yet
    Module
    {
        #[command(subcommand)]
        module : Option<Module>,
    }
    */
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

/* Not implemented yet
#[derive(Subcommand)]
#[command()]
pub enum Module
{
    Add
    {
        /// The module name
        #[arg(short, long)]
        name : String,
    },

    Remove
    {
        /// The module name
        #[arg(short, long)]
        name : String,
    },

    Update
}
 */
