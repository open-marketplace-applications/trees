#[macro_use]
extern crate diesel;
#[macro_use]
extern crate log;

#[macro_use]
extern crate serde_derive;

use actix_multipart::Multipart;
use actix_web::middleware::{errhandlers::ErrorHandlers, Logger};
use actix_web::{get, http, middleware, post, web, App, Error, HttpResponse, HttpServer};

use actix_files::Files;
use async_std::prelude::*;
use futures::{StreamExt, TryStreamExt};
use std::env;

use actix_cors::Cors;

use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use uuid::Uuid;

mod actions;
mod cli_args;
mod database;
mod errors;
mod models;
mod schema;
mod blobstorage;

// type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

/// Finds user by UID.
#[get("/user/{user_id}")]
async fn get_user(
    pool: web::Data<database::Pool>,
    user_uid: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    let user_uid = user_uid.into_inner();
    let conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let user = web::block(move || actions::find_user_by_uid(user_uid, &conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    if let Some(user) = user {
        Ok(HttpResponse::Ok().json(user))
    } else {
        let res = HttpResponse::NotFound().body(format!("No user found with uid: {}", user_uid));
        Ok(res)
    }
}

/// Inserts new user with name defined in form.
#[post("/user")]
async fn add_user(
    pool: web::Data<database::Pool>,
    form: web::Json<models::NewUser>,
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let user = web::block(move || actions::insert_new_user(&form.name, &conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(user))
}

// trees
/// Inserts new user with name defined in form.
#[post("/trees")]
async fn add_tree(
    pool: web::Data<database::Pool>,
    form: web::Json<models::NewTree>,
) -> Result<HttpResponse, Error> {
    println!("Hellow worl!");
    let conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let tree = web::block(move || actions::insert_new_tree(&form.name, &form.genus, &form.lat, &form.lng, &conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    Ok(HttpResponse::Ok().json(tree))
}
/// Returns a list of all trees in a array.
#[get("/trees")]
async fn trees_index(pool: web::Data<database::Pool>) -> Result<HttpResponse, Error> {
    println!("Hellow worl!");
    let conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let trees = web::block(move || actions::get_all_trees(&conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    println!("Hellow worl! {:?}", trees);

    Ok(HttpResponse::Ok().json(trees))
}

/// Finds user by UID.
#[get("/trees/{tree_id}")]
async fn get_tree(
    pool: web::Data<database::Pool>,
    tree_uid: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    let tree_uid = tree_uid.into_inner();
    let conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let tree = web::block(move || actions::find_tree_by_uid(tree_uid, &conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;

    if let Some(tree) = tree {
        Ok(HttpResponse::Ok().json(tree))
    } else {
        let res = HttpResponse::NotFound().body(format!("No tree found with uid: {}", tree_uid));
        Ok(res)
    }
}

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
async fn main() -> anyhow::Result<()> {
    env::set_var("RUST_LOG", "actix_server=debug,actix_web=debug");
    env_logger::init();

    dotenv::dotenv().ok();
    async_std::fs::create_dir_all("./tmp").await?;

    // set up database connection pool
    // let connspec = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    // let manager = ConnectionManager::<SqliteConnection>::new(connspec);
    // let pool = r2d2::Pool::builder()
    //     .build(manager)
    //     .expect("Failed to create pool.");

    let opt = {
        use structopt::StructOpt;
        cli_args::Opt::from_args()
    };
    let pool = web::Data::new(database::pool::establish_connection(opt.clone()));

    let blobstorage = blobstorage::Blobstorage::new()?;

    let port = env::var("PORT")
        .unwrap_or_else(|_| "5000".to_string())
        .parse()
        .unwrap();

    let ip = "0.0.0.0";

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST"])
            //   .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
            .app_data(pool.clone())
            .app_data(blobstorage.clone())
            .wrap(cors)
            .wrap(Logger::default())
            .wrap(middleware::Logger::default())
            .service(
                web::resource("/upload")
                    .route(web::get().to(index))
                    .route(web::post().to(save_file)),
            )
            .service(get_user)
            .service(add_user)
            .service(add_tree)
            .service(get_tree)
            .service(trees_index)
            .service(Files::new("/images", "tmp/").show_files_listing())
    })
    .bind((ip, port))?
    .run()
    .await?;

    Ok(())
}
