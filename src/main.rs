use actix_web::{get,post, web, App, HttpServer, Responder, HttpResponse, http::header::ContentType, body::BoxBody, HttpRequest};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct User {
  id: u8,
  name: String,
}

impl Responder for User {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) ->
    HttpResponse<Self::Body> {
        let res_body = serde_json::to_string(&self).unwrap();

        HttpResponse::Ok()
           .content_type(ContentType::json())
           .body(res_body)
    }
}

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

#[post("/api/new")]
async fn new_thing(req: web::Json<User>) -> impl Responder {
    let new_user = User {
        id: req.id,
        name: String::from(&req.name),
    };
    let response = serde_json::to_string(&new_user).unwrap();

    println!("{}", response);
    HttpResponse::Created()
        .content_type(ContentType::json())
        .body(response)
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .service(greet)
        .service(new_thing)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
