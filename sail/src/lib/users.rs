use std::ffi::CString;
use std::io::{self, Write};
use std::os::unix::io::FromRawFd;
use std::ptr::null_mut;
use libc::{c_char, c_int, c_void};
use std::fs;
use std::path::Path;

pub fn exits_user(username: &str) -> Result<bool, &'static str> {
    let username_cstring = std::ffi::CString::new(username).unwrap();
    let mut args: [std::ffi::CString; 3] = [
        std::ffi::CString::new("id").unwrap(),
        std::ffi::CString::new("-u").unwrap(),
        username_cstring,
    ];
    let mut argv: Vec<*const c_char> = args
        .iter()
        .map(|arg| arg.as_ptr() as *const c_char)
        .chain(std::iter::once(std::ptr::null()))
        .collect();
    let mut status: c_int = 0;
    unsafe {
        let pid = libc::fork();
        if pid == -1 {
            return Err("Could not fork process");
        } else if pid == 0 {
            // Child process
            libc::execvp(argv[0], argv.as_mut_ptr());
            // The above function call never returns if successful
            libc::_exit(1);
        } else {
            // Parent process
            let mut child_status: c_int = 0;
            libc::waitpid(pid, &mut child_status, 0);
            status = child_status;
        }
    }
    if libc::WIFEXITED(status) && libc::WEXITSTATUS(status) == 0 {
        Ok(true)
    } else {
        Err("Could not check if user exists")
    }
}

pub fn add_user(username: &str, password: &str) -> Result<(), &'static str> {
    if Path::new(&format!("/home/{}", username)).exists() {
        return Err("User already exists!");
    }

    // Add user
    if let Err(_) = fs::create_dir(&format!("/home/{}", username)) {
        return Err("Could not create user!");
    }

    // Set password
    let shadow_path = "/etc/shadow";
    let shadow_contents = match fs::read_to_string(shadow_path) {
        Ok(contents) => contents,
        Err(_) => {
            fs::remove_dir(&format!("/home/{}", username)).unwrap();
            return Err("Could not create user!");
        }
    };
    let mut new_shadow_contents = String::new();
    for line in shadow_contents.lines() {
        if line.starts_with(username) {
            let hash = crypt::hash_with_salt(password, &line.split('$').nth(2).unwrap());
            new_shadow_contents.push_str(&line.replace(line.split(':').nth(1).unwrap(), &hash));
        } else {
            new_shadow_contents.push_str(line);
        }
        new_shadow_contents.push_str("\n");
    }
    if let Err(_) = fs::write(shadow_path, new_shadow_contents) {
        fs::remove_dir(&format!("/home/{}", username)).unwrap();
        return Err("Could not create user!");
    }

    // Add user to sudoers
    let sudoers_path = "/etc/sudoers";
    let sudoers_contents = match fs::read_to_string(sudoers_path) {
        Ok(contents) => contents,
        Err(_) => {
            fs::remove_dir(&format!("/home/{}", username)).unwrap();
            return Err("Could not create user!");
        }
    };
    if !sudoers_contents.contains(username) {
        if let Err(_) = fs::write(sudoers_path, format!("{}\n{}\n", sudoers_contents, username)) {
            fs::remove_dir(&format!("/home/{}", username)).unwrap();
            return Err("Could not create user!");
        }
    }

    // Add user to ssh config
    let sshd_config_path = "/etc/ssh/sshd_config";
    let sshd_config_contents = match fs::read_to_string(sshd_config_path) {
        Ok(contents) => contents,
        Err(_) => {
            fs::remove_dir(&format!("/home/{}", username)).unwrap();
            return Err("Could not create user!");
        }
    };
    if !sshd_config_contents.contains(username) {
        if let Err(_) = fs::write(sshd_config_path, format!("{}\nAllowUsers {}\n", sshd_config_contents, username)) {
            fs::remove_dir(&format!("/home/{}", username)).unwrap();
            return Err("Could not create user!");
        }
    }

    Ok(())
}

pub fn root_user(username : &String) -> Result<(), &'static str>
{
    if !exits_user(username).unwrap()
    {
        return Err("User does not exist!")
    }

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
    if !exits_user(username).unwrap()
    {
        return Err("User does not exist!")
    }

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
    if !exits_user(username).unwrap()
    {
        return Err("User does not exist!")
    }

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
