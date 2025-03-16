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
    use actix_web::{get, web, HttpResponse, Responder};

    #[derive(Debug, PartialEq)]
    enum AuthStatus {
        Unknown,
        Ok,
    }
    impl AuthStatus {
        fn from_query(query: &web::Query<AuthInfo>) -> Self {
            let Some(ref user) = query.user else {
                return Self::Unknown;
            };
            let Some(ref pin) = query.pin else {
                return Self::Unknown;
            };
            Self::from_user(user, pin)
        }

        fn from_user(user: &str, pin: &str) -> Self {
            match (user, pin) {
                ("tester", "42") => {
                    return Self::Ok;
                }
                _ => {
                    return Self::Unknown;
                }
            }
        }
    }

    #[derive(serde::Deserialize)]
    struct AuthInfo {
        user: Option<String>,
        pin: Option<String>,
    }

    fn auth_failed() -> HttpResponse {
        HttpResponse::build(StatusCode::LOCKED)
            .content_type(ContentType::plaintext())
            .body("no or invalide user/pin pair")
    }

    #[get("/")]
    async fn get_list(state: web::Data<AppState>, query: web::Query<AuthInfo>) -> HttpResponse {
        let status = AuthStatus::from_query(&query);
        println!("{:?} gets status {:?}", query.user, status);
        if AuthStatus::from_query(&query) == AuthStatus::Unknown {
            return auth_failed();
        }

        let backend = state.backend.read().unwrap();
        HttpResponse::Ok()
            .content_type(ContentType::plaintext())
            .body(backend.list())
    }

    #[get("/{target}")]
    async fn screenshot(
        state: web::Data<AppState>,
        path: web::Path<String>,
        query: web::Query<AuthInfo>,
    ) -> HttpResponse {
        let status = AuthStatus::from_query(&query);
        println!("{:?} gets status {:?}", query.user, status);
        if AuthStatus::from_query(&query) == AuthStatus::Unknown {
            return auth_failed();
        }

        let target = path.into_inner();
        if target == "screen" {
            println!("target -> Screen");
        } else {
            println!("target ->> {}", target);
        }
        HttpResponse::Ok()
            .content_type(ContentType::plaintext())
            .body("")
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
