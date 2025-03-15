//  //  //  //  //  //  //  //
use std::sync::RwLock;

pub struct AppState {
    backend: RwLock<crate::back_end::Backend>,
}

impl AppState {
    pub fn new() -> Self {
        let back = crate::back_end::Backend::default();
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
    use actix_web::http::{header::ContentType, StatusCode};
    use actix_web::{get, web, HttpRequest, HttpResponse, Responder};

    #[derive(serde::Deserialize)]
    struct ItemID {
        id: Option<usize>,
    }

    #[get("/test/{AA}/{BB}/")]
    pub async fn tester(state: web::Data<AppState>, 
        //path: web::Path<String>,
        req: HttpRequest,
    ) -> impl Responder {
        //let (a, b, c) = path.into_inner();
        //println!("test: {:?}-{:?}-{:?}", a, b, c);
        //println!("test: {:?}", path);
        let v1: String = req.match_info().get("AA").unwrap().parse().unwrap();
        let v2: String = req.match_info().query("BB").parse().unwrap();
        let (v3,v4): (String, String) = req.match_info().load().unwrap();
        println!("!! {} {} {} {}", v1,v2,v3,v4);

        "<- * ->".to_owned()
    }

    #[get("/list")]
    async fn get_list(state: web::Data<AppState>, query: web::Query<ItemID>) -> impl Responder {
        println!("get list {:?}", query.id);
        let backend = state.backend.read().unwrap();
        backend.list()
    }

    #[get("/display")]
    async fn display_screenshot(state: web::Data<AppState>) -> Option<impl Responder> {
        println!("display");
        let backend = state.backend.read().unwrap();
        let Some(img) = backend.display_screenshot() else {
            return None;
        };
        Some(
            HttpResponse::Ok()
                .content_type(ContentType::jpeg())
                .body(img),
        )
    }

    #[get("/window/{hwnd}")]
    async fn window_screenshot(
        state: web::Data<AppState>,
        path: web::Path<usize>,
    ) -> Option<impl Responder> {
        let hwnd = path.into_inner() as isize;
        println!("get item {}", hwnd);
        let backend = state.backend.read().unwrap();
        let Some(img) = backend.window_screenshot(hwnd) else {
            return None;
        };
        Some(
            HttpResponse::Ok()
                .content_type(ContentType::jpeg())
                .body(img),
        )
    }
}
//use crate::error::RunnerError;

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
