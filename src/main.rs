use actix_multipart::Multipart;
use actix_web::{web, App, HttpResponse, HttpServer};
use async_std::prelude::*;
use futures::{StreamExt, TryStreamExt};
use actix_files::NamedFile;
use std::path::PathBuf;

type Resposta = HttpResponse;

async fn download(file_name: web::Path<String>) -> actix_web::Result<NamedFile> {
    let path: PathBuf = format!("C:/Fiorilli/Upload/{}", file_name).parse().unwrap();
    Ok(NamedFile::open(path)?)
}

async fn upload(mut payload: Multipart) -> Resposta {      
    while let Ok(Some(mut field)) = payload.try_next().await {
        let file_name = field
            .content_disposition()            
            .get_filename()
            .unwrap()
            .to_string();

        let file_path = format!("C:/Fiorilli/Upload/{}", file_name);
        let mut file = async_std::fs::File::create(file_path)
            .await
            .unwrap();

        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            file.write_all(&data).await.unwrap();
        }
    }

   upload_response().await
}

async fn index() -> Resposta {
    HttpResponse::Ok().content_type("text/html; charset=utf-8").body(include_str!("index.html"))
}


async fn upload_response() -> Resposta {
    HttpResponse::Ok().content_type("text/html; charset=utf-8").body(include_str!("upload.html"))
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/upload", web::post().to(upload))
            .route("/upload_response", web::get().to(upload_response))
            .route("/download/{file_name}", web::get().to(download))
    })
    .bind("0.0.0.0:8079")?
    .run()
    .await
}