use actix_web::{Result, get};
use actix_files::NamedFile;
use std::path::PathBuf;

#[get("/servefile")]
pub async fn serve_static_file() -> Result<NamedFile> {
    let file: PathBuf = "./static/logo.png".parse().unwrap();
    Ok(NamedFile::open(file)?)
}