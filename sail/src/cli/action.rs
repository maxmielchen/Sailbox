use crate::cli::dsl::{Cli, Tool, User};
use crate::lib;
use crate::lib::users;

pub fn validate(parse : &Cli)
{
    match &parse.tool
    {
        Some(Tool::User {user}) => {
            match user
            {
                Some(User::Create { username, password, root, sudo})  => {

                    let username_encoded;
                    let password_encoded;

                    match &username {
                        None => {
                            username_encoded = lib::console::native::read_input("Username: ").unwrap()
                        }
                        _ => {
                            username_encoded = username.clone().unwrap();
                        }
                    }

                    match users::exits_user(&username_encoded) {
                        Ok(exist) => {
                            if exist
                            {
                                println!("User already exist!");
                                return;
                            }
                        }
                        Err(e) => {
                            println!("{}", e);
                            return;
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
                        Err(e) => {
                            println!("{}", e);
                            return;
                        }
                    }
                    if *root
                    {
                        match users::root_user(&username_encoded) {
                            Ok(_) => println!("Successfully rooting user!"),
                            Err(e) => {
                                println!("{}", e);
                                return;
                            }
                        }
                    }
                    if *sudo
                    {
                        match users::sudo_user(&username_encoded) {
                            Ok(_) => println!("Successfully give user sudo access!"),
                            Err(e) => {
                                println!("{}", e);
                                return;
                            }
                        }
                    }
                    println!("Please reboot the Sailbox to initialize the user complete!")
                },
                Some(User::Delete {username}) => {

                    let username_encoded;

                    match &username {
                        None => {
                            username_encoded = lib::console::native::read_input("Username: ").unwrap()
                        }
                        _ => {
                            username_encoded = username.clone().unwrap();
                        }
                    }

                    match users::exits_user(&username_encoded) {
                        Ok(exist) => {
                            if !exist
                            {
                                println!("User does not exist!");
                                return;
                            }
                        }
                        Err(e) => {
                            println!("{}", e);
                            return;
                        }
                    }

                    match users::delete_user(&username_encoded) {
                        Ok(_) => println!("Successfully delete user!"),
                        Err(e) => {
                            println!("{}", e);
                            return;
                        }
                    }

                    println!("Please reboot the Sailbox to remove the user complete!")
                },
                None => {
                    println!("Too few arguments use # sail user --help")
                }
            }
        },
        None => {
            println!("Too few arguments use # sail --help")
        }
    }
}