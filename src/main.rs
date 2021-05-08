use actix_multipart::Multipart;
use actix_web::{middleware, web, App, Error, HttpResponse, HttpServer};
use async_std::prelude::*;
use futures::{StreamExt, TryStreamExt};
use std::env;
use actix_files::Files;

async fn save_file(mut payload: Multipart) -> Result<HttpResponse, Error> {
    // iterate over multipart stream
    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_type = field
            .content_disposition()
            .ok_or_else(|| actix_web::error::ParseError::Incomplete)?;
        let filename = content_type
            .get_filename()
            .ok_or_else(|| actix_web::error::ParseError::Incomplete)?;
        let filepath = format!("./tmp/{}", sanitize_filename::sanitize(&filename));
        let mut f = async_std::fs::File::create(filepath).await?;

        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            f.write_all(&data).await?;
        }
    }
    Ok(HttpResponse::Ok().into())
}

fn index() -> HttpResponse {
    let html = r#"<html>
        <head><title>Upload Test</title></head>
        <body>
            <form target="/" method="post" enctype="multipart/form-data">
                <input type="file" multiple name="file"/>
                <button type="submit">Submit</button>
            </form>
        </body>
    </html>"#;

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "actix_server=debug,actix_web=debug");
    async_std::fs::create_dir_all("./tmp").await?;

    let port = env::var("PORT")
        .unwrap_or_else(|_| "5000".to_string())
        .parse().unwrap();

    let ip = "0.0.0.0";

    HttpServer::new(|| {
        App::new()
            .wrap(
                middleware::Logger::default())
            .service(
                    web::resource("/upload")
                    .route(web::get().to(index))
                    .route(web::post().to(save_file)))
                .service(Files::new("/images", "tmp/").show_files_listing())
            .service(Files::new("/", "./frontend/build/").index_file("index.html"))
    })
    .bind((ip, port))?
    .run()
    .await
}