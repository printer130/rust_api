use std::fmt::Display;

use actix_web::{get,post, web, delete, put, App, HttpServer, Responder, HttpResponse, http::{header::ContentType, StatusCode}, body::BoxBody, HttpRequest, ResponseError};
use serde::{Serialize, Deserialize};

static mut THINGS: Vec<User> = vec![];

#[derive(Serialize, Deserialize,Debug)]
struct User {
    id: u32,
    name: String,
}

#[derive(Serialize, Deserialize,Debug)]
struct ErrNoId {
    id: u32,
    err: String
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

#[get("/api/things")]
async fn get_things() -> impl Responder {
    unsafe {
        let response = serde_json::to_string(&(*THINGS)).unwrap();
        println!("GET: {:?}", response);
        HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(response)
    }
}

#[post("/api/new")]
async fn new_thing(req: web::Json<User>) -> impl Responder {
    let new_user = User {
        id: req.id,
        name: String::from(&req.name),
    };
    println!("new_user: {:?}", new_user);
    let response = serde_json::to_string(&new_user).unwrap();
    unsafe {
        THINGS.push(new_user);
    }
    HttpResponse::Created()
        .content_type(ContentType::json())
        .body(response)
}

impl ResponseError for ErrNoId {
    fn status_code(&self) -> StatusCode {
        StatusCode::NOT_FOUND
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        let body = serde_json::to_string(&self).unwrap();
        let res = HttpResponse::new(self.status_code());
        res.set_body(BoxBody::new(body))
    }

}

impl Display for ErrNoId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
 }

#[delete("/api/delete/{id}")]
async fn delete_thing(id: web::Path<u32>) -> Result<User, ErrNoId> {
    let user_id: u32 = *id;
    println!("user_id: {:?}", user_id);

    unsafe {
        let id_idx = THINGS.iter().position(
            |x| x.id == user_id
        );
        println!("{:?}", id_idx);

    match id_idx {
            Some(id) => {
                let deleted_user = THINGS.remove(id);
                Ok(deleted_user)
            },
            None => {
                let response = ErrNoId {
                    id: user_id,
                    err: String::from("Ticked not found")
                };
                Err(response)
            }
        }
    }
}

#[put("/api/update/{id}")]
async fn update_thing(id: web::Path<u32>, req: web::Json<User>) -> Result<HttpResponse, ErrNoId>{
    let user_id = *id;

    let updated_user = User {
        id: req.id,
        name: String::from(&req.name),
    };
    unsafe {
        let id_idx = THINGS.iter().position(|x| x.id == user_id);
        println!("{:#?}",id_idx);

        match id_idx {
            Some(id) => {
                let response = serde_json::to_string(&updated_user).unwrap();

                THINGS[id] = updated_user;

                Ok(HttpResponse::Ok()
                .content_type(ContentType::json())
                .body(response))
            },
            None => {
                let response = ErrNoId {
                    id: user_id,
                    err: String::from("Ticket Not Found"),
                };
                Err(response)
            }
        }
    }

}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .service(new_thing)
        .service(get_things)
        .service(delete_thing)
        .service(update_thing)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
