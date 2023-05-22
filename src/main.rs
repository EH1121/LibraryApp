use actix_web::{web::{self, Data}, App, HttpServer};
use actix_cors::Cors;
use database::Database;
use user::*;
use genre::*;
use book::*;

mod database;
mod book;
mod genre;
mod user;
mod structs;
mod libs;

/// Where should the main list be
/// The constant must be in lowercase, without space, and lowercase alphanumeric
pub const USER_LIST: &str = "users_apps";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Debug mode
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let db = Data::new(Database::new("http://127.0.0.1:9200"));

    // Start server
    HttpServer::new( move || {
        App::new()
        .wrap(Cors::permissive())
        .service(
            web::scope("")
                .app_data(db.clone())
                .service(
                    web::scope("/user")
                        .route("", web::post().to(create_new_user))
                        .route("", web::put().to(update_user))   
                        .route("/{user_id}", web::get().to(get_a_user))
                        .route("/{user_id}", web::delete().to(delete_user))
                )

                .route("/users", web::get().to(get_user_list))
                
                .service(
                    web::scope("/genre/{user_id}")
                        .route("", web::post().to(create_genre))
                        .route("", web::get().to(get_genre))
                        .route("/{genre}", web::delete().to(delete_genre))
                )
                
                .service(
                    web::scope("/book/{user_id}/{genre}")
                        .route("", web::post().to(create_books))
                        .route("/{book_id}", web::get().to(get_book))
                        .route("/{book_id}", web::put().to(update_book))
                        .route("/{book_id}", web::delete().to(delete_book))
                        
                )

                .route("/search/{user_id}", web::post().to(search_books))   
        )
        })
    .bind(("127.0.0.1", 1234))?
    .run()
    .await
}