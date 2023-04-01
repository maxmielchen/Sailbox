use crate::cli::dsl::{Cli, Project, Tool, User};
use crate::lib::projects;
use crate::lib::users;

pub fn validate(parse : &Cli)
{
    match &parse.command
    {
        Some(User::Create { username, password, root, sudo})  => {
            match users::add_user(&username, &password) {
                Ok(_) => println!("Successfully create user!"),
                Err(e) => println!(e)
            }
            if root
            {
                match users::root_user(&username) {
                    Ok(_) => println!("Successfully rooting user!"),
                    Err(e) => println!(e)
                }
            }
            if sudo
            {
                match users::sudo_user(&username) {
                    Ok(_) => println!("Successfully give user sudo access!"),
                    Err(e) => println!(e)
                }
            }
        },
        Some(User::Delete {username}) => {
            match users::delete_user(&username) {
                Ok(_) => println!("Successfully delete user!"),
                Err(e) => println!(e)
            }
        },

        Some(Project::Create {owner, name}) => {
            match projects::add_project(&owner, &name) {
                Ok(_) => println!("Successfully create project!"),
                Err(e) => println!(e)
            }
        }

        Some(Project::Delete {owner, name}) => {
            match projects::remove_project(&owner, &name) {
                Ok(_) => println!("Successfully delete project!"),
                Err(e) => println!(e)
            }
        }

        None => {}
    }
}