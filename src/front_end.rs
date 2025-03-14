//  //  //  //  //  //  //  //
use std::sync::RwLock;

pub struct AppState {
    backend: RwLock<crate::back_end::Backend>,
}

impl AppState {
    pub fn new() -> Self {
        let back = crate::back_end::Backend::default();
        //runner.insert("one-1").unwrap();
        //runner.insert("two-2").unwrap();
        println!("\n[+]AppState\n");
        Self {
            backend: RwLock::new(back),
        }
    }
}

//  //  //  //  //  //  //  //
//  //  //  //  //  //  //  //

pub mod methods {
    use super::AppState;
    //use actix_web::{delete, get, post, web, Responder};
    use actix_web::{get, web, Responder};

    #[derive(serde::Deserialize)]
    struct ItemID {
        id: Option<usize>,
    }

    #[get("/list")]
    async fn get_list(state: web::Data<AppState>, query: web::Query<ItemID>) -> impl Responder {
        println!("get list {:?}", query.id);
        let backend = state.backend.read().unwrap();
        backend.list()
    }
}
//use crate::error::RunnerError;
//use actix_web::Result;
//use actix_web::http::StatusCode;

/*
//  //  //  //  //  //  //  //

#[get("/task/{id}")]
async fn get_item(
    state: web::Data<AppState>,
    path: web::Path<usize>,
) -> Result<impl Responder, RunnerError> {
    let id = path.into_inner();
    println!("get item {}", id);
    let runner = state.runner.read().unwrap();
    Ok(runner.get(id)?.clone())
}

#[post("/")]
async fn insert_item(
    state: web::Data<AppState>,
    info: String,
) -> Result<(impl Responder, StatusCode), RunnerError> {
    println!("insert item\n'{}'", info);
    let mut runner = state.runner.write().unwrap();
    let new_id = runner.insert(&info)?;
    Ok((format!("{}", new_id), StatusCode::CREATED))
}

#[delete("/task/{id}")]
async fn delete_item(
    state: web::Data<AppState>,
    path: web::Path<usize>,
) -> Result<impl Responder, RunnerError> {
    let id = path.into_inner();
    println!("DELETE item {}", id);
    let mut runner = state.runner.write().unwrap();
    Ok(runner.remove(id)?.clone())
}
*/
