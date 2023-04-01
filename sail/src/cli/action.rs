use std::io;
use crate::cli::dsl::{Cli, Project, Tool, User};
use crate::lib;
use crate::lib::projects;
use crate::lib::users;

pub fn validate(parse : &Cli)
{
    match &parse.tool
    {
        Some(Tool::User {user}) => {
            match user
            {
                Some(User::Create { username, password, root, sudo})  => {

                    let mut username_encoded = String::new();
                    let mut password_encoded = String::new();

                    match &username {
                        None => {
                            username_encoded = lib::console::native::read_input("Username: ").unwrap()
                        }
                        _ => {
                            username_encoded = username.clone().unwrap();
                        }
                    }

                    match &password {
                        None => {
                            password_encoded = lib::console::native::read_hidden("Password: ").unwrap();
                        }
                        _ => {
                            password_encoded = password.clone().unwrap();
                        }
                    }

                    match users::add_user(&username_encoded, &password_encoded) {
                        Ok(_) => println!("Successfully create user!"),
                        Err(e) => println!("{}", e)
                    }
                    if *root
                    {
                        match users::root_user(&username_encoded) {
                            Ok(_) => println!("Successfully rooting user!"),
                            Err(e) => println!("{}", e)
                        }
                    }
                    if *sudo
                    {
                        match users::sudo_user(&username_encoded) {
                            Ok(_) => println!("Successfully give user sudo access!"),
                            Err(e) => println!("{}", e)
                        }
                    }
                    println!("Please reboot the Sailbox to initialize the user complete!")
                },
                Some(User::Delete {username}) => {
                    match users::delete_user(&username) {
                        Ok(_) => println!("Successfully delete user!"),
                        Err(e) => println!("{}", e)
                    }
                    println!("Please reboot the Sailbox to remove the user complete!")
                },
                None => {
                    println!("Too few arguments use # sail user --help")
                }
            }
        },
        Some(Tool::Project {project}) => {
            match project {
                Some(Project::Create {owner, name}) => {
                    projects::add_project(&owner, &name);
                    println!("Successfully create project!");
                }

                Some(Project::Delete {owner, name}) => {
                    projects::remove_project(&owner, &name);
                    println!("Successfully delete project!")
                }
                None => {
                    println!("Too few arguments use # sail project --help")
                }
            }
        },
        None => {
            println!("Too few arguments use # sail --help")
        }
    }
}