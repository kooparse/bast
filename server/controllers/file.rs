use actix_files::NamedFile;
use actix_web::Result;

pub async fn script() -> Result<NamedFile> {
    Ok(NamedFile::open("static/script.js")?)
}

pub async fn front_register() -> Result<NamedFile> {
    Ok(NamedFile::open("static/front/register.html")?)
}

pub async fn front_login() -> Result<NamedFile> {
    Ok(NamedFile::open("static/front/login.html")?)
}

pub async fn front_create_website() -> Result<NamedFile> {
    Ok(NamedFile::open("static/front/create_website.html")?)
}
