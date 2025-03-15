mod config;
mod front_end;
use front_end::{AppState, methods::*};
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
            .service(get_list)
            .service(display_screenshot)
            .service(window_screenshot)
            .service(
                tester
            )
    })
    .bind((config.bind, config.port))?
    .run()
    .await
}
