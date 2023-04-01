use std::process::Command;

pub fn add_package(name : &String) -> Result<(), &'static str>
{
    return match Command::new("apt")
        .arg("install")
        .arg("-y")
        .arg(&name)
        .output()
    {
        Err(e) => Err("Could not install package!"),
        Ok(()) => Ok(())
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
        Err(e) => Err("Could not remove package!"),
        Ok(()) => Ok(())
    }
}

pub fn update_packages() -> Result<(), &'static str>
{
    match Command::new("apt")
        .arg("update")
        .output()
    {
        Err(e) => return Err("Could not update packages!"),
        _ => {}
    }

    return match Command::new("apt")
        .arg("upgrade")
        .arg("-y")
        .output()
    {
        Err(e) => Err("Could not upgrade packages!"),
        Ok(()) => Ok(())
    }
}