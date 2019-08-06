#![feature(proc_macro_hygiene)]

use home_zircon_api::*;
use rocket_cors::{AllowedHeaders, AllowedOrigins, Error, Guard, Responder};
use rocket::{routes};
use rocket::http::Method;

fn main() {
    let allowed_origins = AllowedOrigins::some_exact(&[
        "http://127.0.0.1:8193",
    ]);

    // You can also deserialize this
    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get, Method::Post].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::some(&[
            "Authorization",
            "Accept",
            "Access-Control-Allow-Origin",
        ]),
        allow_credentials: true,
        ..Default::default()
    }
        .to_cors().expect("must be able to init CORS struct");

    rocket::ignite()
        .mount("/v1", routes![authorize, check])
        .mount("/", rocket_cors::catch_all_options_routes())
        .attach(cors)
        .manage(UsersState::new())
        .launch();
}
