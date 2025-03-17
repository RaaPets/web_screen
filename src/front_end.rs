//  //  //  //  //  //  //  //
use std::sync::RwLock;

pub struct AppState {
    user_name: String,
    password: String,
    backend: RwLock<crate::back_end::Backend>,
    session_id: RwLock<String>,
}

impl AppState {
    pub fn new(user_name: &str, password: &str) -> Self {
        let back = crate::back_end::Backend::default();
        println!("\n[+]AppState\n");
        Self {
            user_name: user_name.to_owned(),
            password: password.to_owned(),
            backend: RwLock::new(back),
            session_id: RwLock::new(String::new()),
        }
    }

    pub fn login(&self, user: &str, pass: &str) -> Option<String> {
        if user != self.user_name || pass != self.password {
            return None;
        }
        let t = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH).unwrap()
            .as_nanos();
        let new_session_id = format!("{:x}",t);
        println!("new session id: {}", new_session_id);
        let mut session = self.session_id.write().unwrap();
        *session = new_session_id.clone();
        Some(new_session_id)
    }

    pub fn check_session(&self, session: &str) -> bool {
        let valid_session = self.session_id.read().unwrap();
        if valid_session.is_empty() {
            println!("there is no registered session");
            return false;
        };
        if session == *valid_session {
            return true;
        }

        println!("invalid session");
        false
    }
}

//  //  //  //  //  //  //  //
//  //  //  //  //  //  //  //
pub mod methods {
    use super::*;
    use dyn_fmt::AsStrFormatExt;
    use actix_web::http::header::LOCATION;
    use actix_web::http::{header::ContentType, StatusCode};
    use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};

    //  //  //  //  //  //  //
    #[get("/login")]
    async fn get_login_page() -> HttpResponse {
        HttpResponse::Ok()
            .content_type(ContentType::html())
            .body(HTML_LOGIN)
    }

    //  //  //  //  //  //  //
    #[post("/auth")]
    async fn post_auth(
        state: web::Data<AppState>,
        form: web::Form<CredentialsInfo>,
    ) -> impl Responder {
        println!("auth: {:?}", form);
        let Some(session) = state.login(&form.username, &form.password) else {
            return HttpResponse::Forbidden().body("invalid credentials");
        };
        HttpResponse::Found()
            .insert_header((LOCATION, format!("/session/{}", session)))
            .finish()
    }

    //  //  //  //  //  //  //
    #[get("/session/{session}")]
    async fn get_welcome(state: web::Data<AppState>, session: web::Path<String>) -> HttpResponse {
        println!("welcome: {:?}", session);
        if !state.check_session(&session) {
            return HttpResponse::Found()
                .insert_header((LOCATION, "/login"))
                .finish();
        }

        let backend = state.backend.read().unwrap();
        match backend.try_list() {
            Ok(list) => {
                let mut list_block = String::new();
                list_block += &format!("\n<a href=\"/session/{session}/watch\">DESKTOP</a><br>");
                for (hwnd, win_name) in &list {
                    list_block += &format!(
                        "\n<a href=\"/session/{session}/watch?hwnd={hwnd}\">[{:x}] {win_name}</a><br>",
                        hwnd
                        );
                }
                return HttpResponse::Ok()
                    .content_type(ContentType::html())
                    .body(HTML_LIST.format([&list_block]));
            }
            Err(e) => {
                return HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(format!("Internal Server Error:\n{}", e));
            }
        }
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
    }

    pub async fn not_found(req: HttpRequest) -> HttpResponse {
        println!("not found: <{}>", req.path());
        HttpResponse::build(StatusCode::NOT_FOUND)
            .body(format!("Error (404):\nPage not found <{}>", req.path()))
    }
}

//  //  //  //  //  //  //  //
//  //  //  //  //  //  //  //
#[derive(Debug, serde::Deserialize)]
struct MainQuery {
    hwnd: Option<isize>,
}

#[derive(Debug, serde::Deserialize)]
struct CredentialsInfo {
    username: String,
    password: String,
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

static HTML_LIST: &str = r#"
    <html>
        <body>
            {}
        </body>
    </html>
"#;
