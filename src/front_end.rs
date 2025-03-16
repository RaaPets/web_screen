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

    pub fn login(&self, user: &str, pass: &str) -> Option<String> {
        if user != "admin" || pass != "123" {
            return None;
        }
        Some("admin_session".to_owned())
    }

    pub fn check_session(&self, session: &str) -> bool {
        if session == "admin_session" {
            return true;
        }

        false
    }
}

//  //  //  //  //  //  //  //
//  //  //  //  //  //  //  //
pub mod methods {
    use super::*;
    use actix_web::http::header::LOCATION;
    use actix_web::http::{header::ContentType, StatusCode};
    use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};

    //  //  //  //  //  //  //
    #[get("/login")]
    async fn get_login_page() -> HttpResponse {
        HttpResponse::Ok().body(HTML_LOGIN)
    }

    //  //  //  //  //  //  //
    #[post("/auth")]
    async fn post_auth(
        state: web::Data<AppState>,
        form: web::Form<CredentialsInfo>,
    ) -> impl Responder {
        println!("post auth: {:?}", form);
        let Some(session) = state.login(&form.username, &form.password) else {
            return HttpResponse::Forbidden().body("invalid credentials");
        };
        HttpResponse::Found()
            .insert_header((LOCATION, format!("/session/{}", session)))
            .finish()
    }

    //  //  //  //  //  //  //
    #[get("/session/{session}")]
    async fn get_welcome(state: web::Data<AppState>, session: web::Path<String>) -> impl Responder {
        println!("welcome: {:?}", session);
        if !state.check_session(&session) {
            return HttpResponse::Found()
                .insert_header((LOCATION, "/login"))
                .finish();
        }

        let backend = state.backend.read().unwrap();
        HttpResponse::Ok().body(format!("LIST:\n{}", backend.list()))
    }

    //  //  //  //  //  //  //
    #[get("/session/{session}/watch")]
    async fn get_watch(
        state: web::Data<AppState>,
        session: web::Path<String>,
        query: web::Query<MainQuery>,
    ) -> impl Responder {
        println!("WATCH: {:?}\nquery: {:?}", session, query);
        if !state.check_session(&session) {
            return HttpResponse::Found()
                .insert_header((LOCATION, "/login"))
                .finish();
        }

        let backend = state.backend.read().unwrap();
        if let Some(hwnd) = query.hwnd {
            let Some(img) = backend.window_screenshot(hwnd) else {
                return HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(format!("can not get access to specified window"));
            };
            return HttpResponse::Ok()
                .content_type(ContentType::jpeg())
                .body(img);
        } else {
            let Some(img) = backend.display_screenshot() else {
                return HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(format!("can not get access to desctop"));
            };
            return HttpResponse::Ok()
                .content_type(ContentType::jpeg())
                .body(img);
        }

        HttpResponse::Ok().body(format!("WATCH"))
    }

    //  //  //  //  //  //  //
    fn auth_failed() -> HttpResponse {
        HttpResponse::build(StatusCode::LOCKED)
            .content_type(ContentType::plaintext())
            .body("no or invalide user/pin pair")
    }

    #[get("/{target}")]
    async fn screenshot(
        _state: web::Data<AppState>,
        path: web::Path<String>,
        query: web::Query<CredentialsInfo>,
    ) -> HttpResponse {
        let status = AuthStatus::from_query(&query);
        println!("{:?} gets status {:?}", query.username, status);
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

    pub async fn not_found(req: HttpRequest) -> HttpResponse {
        println!("not found: <{}>", req.path());
        HttpResponse::build(StatusCode::NOT_FOUND)
            .body(format!("Error (404):\nPage not found <{}>", req.path()))
    }
}
/*
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
*/
#[derive(Debug, serde::Deserialize)]
struct MainQuery {
    hwnd: Option<isize>,
}

#[derive(Debug, serde::Deserialize)]
struct CredentialsInfo {
    username: String,
    password: String,
}

#[derive(Debug, PartialEq)]
enum AuthStatus {
    Unknown,
    Ok,
}
impl AuthStatus {
    fn from_query(query: &actix_web::web::Query<CredentialsInfo>) -> Self {
        Self::from_user(&query.username, &query.password)
    }

    fn from_user(user: &str, pass: &str) -> Self {
        match (user, pass) {
            ("tester", "42") => {
                return Self::Ok;
            }
            _ => {
                return Self::Unknown;
            }
        }
    }
}

static HTML_LOGIN: &str = r#"
    <html>
        <body>
            <h1>Login Page</h1>
                <form action="/auth" method="post">
                    <label for="username">Username:</label><br>
                    <input type="text" id="username" name="username"><br><br>
                    <label for="password">Password:</label><br>
                    <input type="password" id="password" name="password"><br><br>
                    <button type="submit">Login</button>
                </form>
        </body>
    </html>
"#;
