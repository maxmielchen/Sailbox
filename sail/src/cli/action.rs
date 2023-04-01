use std::io;
use crate::cli::dsl::{Cli, Project, Tool, User};
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

                    let mut usernameEncoded = String::new();
                    let mut passwordEncoded = String::new();

                    match &username {
                        None => {
                            print!("Username: ");
                            io::stdin().read_line(&mut usernameEncoded).unwrap();
                        }
                        _ => {
                            usernameEncoded = username.clone().unwrap();
                        }
                    }

                    match &password {
                        None => {
                            print!("Password: ");
                            io::stdin().read_line(&mut passwordEncoded).unwrap();
                        }
                        _ => {
                            passwordEncoded = password.clone().unwrap();
                        }
                    }

                    match users::add_user(&usernameEncoded, &passwordEncoded) {
                        Ok(_) => println!("Successfully create user!"),
                        Err(e) => println!("{}", e)
                    }
                    if *root
                    {
                        match users::root_user(&usernameEncoded) {
                            Ok(_) => println!("Successfully rooting user!"),
                            Err(e) => println!("{}", e)
                        }
                    }
                    if *sudo
                    {
                        match users::sudo_user(&usernameEncoded) {
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