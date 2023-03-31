use std::fmt::format;
use std::process::{Command, Stdio};
use std::io::Write;
use std::fs;

pub fn add_user(&username : String, password: String) -> Result<(), Vec<str>>
{
    // Add user
    Command::new("adduser")
        .arg("--disabled-password")
        .arg("--gecos")
        .arg("''")
        .arg(&username)
        .output().unwrap_or_else(
        {
            Err("Could not create user!")
        });

    // User set password
    Command::new("chpasswd")
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::inherit())
        .spawn().unwrap_or_else({
        Command::new("deluser")
            .arg("--remove-home")
            .arg(&username)
            .output().unwrap();
        Err("Could not create user!")
    })
        .stdin
        .unwrap()
        .write_all(format!("{}:{}", &username, &password).as_bytes())
        .unwrap_or_else({
            Command::new("deluser")
                .arg("--remove-home")
                .arg(&username)
                .output().unwrap();
            Err("Could not create user!")
        })
    ;

    // Add user to ssh config
    Command::new("echo")
        .arg(format!("\"AllowUsers {}\"", &username))
        .arg(">>")
        .arg("/etc/ssh/sshd_config")
    ;

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

pub fn root_user(&username : String) -> Result<(), Vec<str>>
{
    Command::new("usermod")
        .arg("-G")
        .arg("root")
        .arg(&username)
        .output()
        .unwrap_or_else({
            Err(format!("Could not root {}!", &username))
        });
    Ok(())
}

pub fn sudo_user(&username : String) -> Result<(), Vec<str>>
{
    Command::new("usermod")
        .arg("-aG")
        .arg("sudo")
        .arg(&username)
        .output()
        .unwrap_or_else({
            Err(format!("Could not give sudo rights to {}!", &username))
        });
    Ok(())
}

pub fn delete_user(&username : String) -> Result<(), Vec<str>>
{
    Command::new("deluser")
        .arg("--remove-home")
        .arg(&username)
        .output().unwrap_or_else({
            Err(format!("Could not delete {}!", username))
        });
    Command::new("sed")
        .arg("-i")
        .arg(format!("/^AllowUsers {}/d", &username))
        .arg("/etc/ssh/sshd_config")
        .output()
        .unwrap_or_else({
            Err(format!("Could not remove {} from SSH config!", &username))
        });
    Ok(())
}
