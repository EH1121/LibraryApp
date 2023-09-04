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

/// Nama list utama untuk setor list usernya
pub const USER_LIST: &str = "users_apps";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Debug mode
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let db = Data::new(Database::new("http://127.0.0.1:9200"));

    // Hidupin servernya
    HttpServer::new( move || {
        App::new()
        .wrap(Cors::permissive())
        .service(
            web::scope("")
                .app_data(db.clone())
                .service(
                    // Route untuk user
                    web::scope("/user")
                        .route("", web::post().to(create_new_user))
                        .route("", web::put().to(update_user))   
                        .route("/{user_id}", web::get().to(get_a_user))
                        .route("/{user_id}", web::delete().to(delete_user))
                )

                // Ambil list user
                .route("/users", web::get().to(get_user_list))
                
                // Route untuk genre
                .service(
                    web::scope("/genre/{user_id}")
                        .route("", web::post().to(create_genre))
                        .route("", web::get().to(get_genre))
                        .route("/{genre}", web::delete().to(delete_genre))
                )
                
                // Route untuk ambil buku
                .service(
                    web::scope("/book/{user_id}/{genre}")
                        .route("", web::post().to(create_books))
                        .route("/{book_id}", web::get().to(get_book))
                        .route("/{book_id}", web::put().to(update_book))
                        .route("/{book_id}", web::delete().to(delete_book))
                )

                // Cari
                .route("/search/{user_id}", web::post().to(search_books))
                .route("/search/{user_id}", web::get().to(search_books_get))   
                
                // Upload
                .route("/upload/{user_id}/{genre}", web::post().to(upload_json))
        )
        })
    .bind(("127.0.0.1", 1234))?
    .run()
    .await
}