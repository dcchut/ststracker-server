use std::collections::HashMap;
use std::sync::Mutex;

use actix_cors::Cors;
use actix_web::{
    http::header,
    middleware::Logger,
    web,
    web::{resource, Data},
    App, HttpServer,
};
use redis;

use crate::model::ApplicationState;

mod model;
mod view;

fn main() -> std::io::Result<()> {
    // load the settings file
    let mut settings = config::Config::default();
    settings.merge(config::File::with_name("Settings")).unwrap();

    let mut settings = settings
        .try_into::<HashMap<String, String>>()
        .expect("Failed to load settings");

    // Check that the settings file contains all the settings we require
    let redis_addr = settings
        .remove("redis_addr")
        .expect("Missing setting `redis_addr`");
    let redis_port = settings
        .remove("redis_port")
        .expect("Missing setting `redis_port`");
    let redis_passwd = settings
        .remove("redis_passwd")
        .expect("Missing setting `redis_passwd`");
    let twitch_secret_key = settings
        .remove("twitch_secret_key")
        .expect("Missing setting `twitch_secret_key`");
    let backend_secret_key = settings
        .remove("backend_secret_key")
        .expect("Missing setting `backend_secret_key`");
    let client_id = settings
        .remove("client_id")
        .expect("Missing setting `client_id`");
    let bind_addr = settings
        .remove("bind_addr")
        .expect("Missing setting `bind_addr`");

    // Due to the requirements of the itsdangerous crate, we have to leak
    // our backend secret key here to ensure that it lives for the lifetime of the program
    let backend_secret_key = Box::leak(backend_secret_key.into_boxed_str());

    // Establish a connection to our redis server
    let redis_url = format!("redis://:{}@{}:{}", redis_passwd, redis_addr, redis_port);
    let client = redis::Client::open(redis_url.as_str()).unwrap();
    let con = client.get_connection().unwrap();

    // Establish our initial application state
    let state = Data::new(Mutex::new(ApplicationState::new(
        &client_id,
        twitch_secret_key,
        backend_secret_key,
        con,
    )));

    HttpServer::new(move || {
        App::new()
            .register_data(state.clone())
            .wrap(Logger::default())
            .wrap(
                Cors::new()
                    .allowed_methods(vec!["GET", "POST", "OPTIONS"])
                    .allowed_header(header::AUTHORIZATION)
                    .allowed_header(header::CONTENT_TYPE),
            )
            .service(resource("/update").route(web::post().to_async(crate::view::update)))
            .service(resource("/get").route(web::get().to_async(crate::view::get)))
            .service(
                resource("/view_api_key").route(web::post().to_async(crate::view::view_api_key)),
            )
    })
    .bind(bind_addr)?
    .run()
}
