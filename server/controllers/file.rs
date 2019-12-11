use actix_files::NamedFile;
use actix_web::Result;

pub fn script() -> Result<NamedFile> {
    Ok(NamedFile::open("static/script.js")?)
}

pub fn front_register() -> Result<NamedFile> {
    Ok(NamedFile::open("static/front/register.html")?)
}

pub fn front_login() -> Result<NamedFile> {
    Ok(NamedFile::open("static/front/login.html")?)
}

pub fn front_create_website() -> Result<NamedFile> {
    Ok(NamedFile::open("static/front/create_website.html")?)
}
