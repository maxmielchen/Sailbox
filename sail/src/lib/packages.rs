use std::process::Command;

pub fn add_package(name : &String) -> Result<(), &'static str>
{
    return match Command::new("apt")
        .arg("install")
        .arg("-y")
        .arg(&name)
        .output()
    {
        Err(_) => Err("Could not install package!"),
        Ok(_) => Ok(())
    }
}

pub fn remove_package(name : &String) -> Result<(), &'static str>
{
    return match Command::new("apt")
        .arg("purge")
        .arg("-y")
        .arg(&name)
        .output()
    {
        Err(_) => Err("Could not remove package!"),
        Ok(_) => Ok(())
    }
}

pub fn update_packages() -> Result<(), &'static str>
{
    match Command::new("apt")
        .arg("update")
        .output()
    {
        Err(_) => return Err("Could not update packages!"),
        _ => {}
    }

    return match Command::new("apt")
        .arg("upgrade")
        .arg("-y")
        .output()
    {
        Err(_) => Err("Could not upgrade packages!"),
        Ok(_) => Ok(())
    }
}