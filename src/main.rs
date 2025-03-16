mod config;
mod front_end;
use front_end::{methods::*, AppState};
mod back_end;
//mod error;

use actix_web::{web, App, HttpServer};
//  //  //  //  //  //  //  //
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = config::parse_cli();
    println!("WebScreen: {}:{}", config.bind, config.port);

    let state = web::Data::new(AppState::new());

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(get_login_page)
            .service(post_auth)
            .service(get_welcome)
            .service(get_watch)
            .service(screenshot)
            .default_service(web::route().to(not_found))
    })
    .bind((config.bind, config.port))?
    .run()
    .await
}
