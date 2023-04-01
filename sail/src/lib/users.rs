use std::fmt::format;
use std::process::{Command, Stdio};
use std::io::Write;
use std::fs;

pub fn add_user(username : &String, password: &String) -> Result<(), &'static str>
{
    // Add user
    match Command::new("adduser")
        .arg("--disabled-password")
        .arg("--gecos")
        .arg("''")
        .arg(&username)
        .output() {
        Ok(output) => output,
        Err(_) => return Err("Could not create user!")
    };

    // User set password
    match match Command::new("chpasswd")
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::inherit())
        .spawn() {
            Ok(output) => output,
            Err(_) => {
                Command::new("deluser")
                    .arg("--remove-home")
                    .arg(&username)
                    .output().unwrap();
                return Err("Could not create user!")
            }
        }
        .stdin
        .unwrap()
        .write_all(format!("{}:{}", &username, &password).as_bytes()) {
            Ok(output) => output,
            Err(_) => {
            Command::new("deluser")
                .arg("--remove-home")
                .arg(&username)
                .output().unwrap();
            return Err("Could not create user!")
        }
    };

    // Add user to ssh config
    Command::new("echo")
        .arg(format!("\"AllowUsers {}\"", &username))
        .arg(">>")
        .arg("/etc/ssh/sshd_config")
        .output().unwrap();

    // Add project dir
    fs::create_dir(format!("/home/{}/projects", &username)).unwrap();

    // Give user using rights
    Command::new("chown")
        .arg("-R")
        .arg("-c")
        .arg(&username)
        .arg(format!("/home/{}", &username))
        .output().unwrap();

    return Ok(())
}

pub fn root_user(username : &String) -> Result<(), &'static str>
{
    match Command::new("usermod")
        .arg("-G")
        .arg("root")
        .arg(&username)
        .output() {
            Ok(output) => output,
            Err(_) => return Err("Could not root")
        };
    return Ok(())
}

pub fn sudo_user(username : &String) -> Result<(), &'static str>
{
    match Command::new("usermod")
        .arg("-aG")
        .arg("sudo")
        .arg(&username)
        .output() {
            Ok(output) => output,
            Err(_) => return Err("Could not give sudo rights")
        };
    return Ok(())
}

pub fn delete_user(username : &String) -> Result<(), &'static str>
{
    match Command::new("deluser")
        .arg("--remove-home")
        .arg(&username)
        .output() {
            Ok(output) => output,
            Err(_) => return Err("Could not delete")
    };
    match Command::new("sed")
        .arg("-i")
        .arg(format!("/^AllowUsers {}/d", &username))
        .arg("/etc/ssh/sshd_config")
        .output() {
            Ok(output) => output,
            Err(_) => return Err("Could not remove from SSH-Server")
    };
    return Ok(())
}
