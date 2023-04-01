use std::fs;

pub fn add_project(owner : &String, name : &String)
{
    fs::create_dir(format!("/home/{}/projects/{}", &owner, &name)).unwrap()
}

pub fn remove_project(owner : &String, name : &String)
{
    fs::remove_dir_all(format!("/home/{}/projects/{}", &owner, &name)).unwrap()
}