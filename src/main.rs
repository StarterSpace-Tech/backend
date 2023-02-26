use actix_web::{Result, get, post, web, App, HttpResponse, HttpServer, Responder};
use star_tec_backend::*;



#[get("/startups")]
async fn hello() -> Result<impl Responder> {

    let mut vec_obj:Vec<Team> = vec![];
    for i in 1..30 {
        let mut team = Team::new();
        team.rank = i;
        team.id = format!("0x{:0>4}", i);
        vec_obj.push(team);
    }
    Ok(web::Json(vec_obj))
}

#[get("/echo")]
async fn echo(bytes: web::Bytes) -> Result<String, actix_web::error::JsonPayloadError> {
    match String::from_utf8(bytes.to_vec()) {
        Ok(text) => Ok(format!("Hello, {}!\n", text)),
        Err(_) => Err(actix_web::error::JsonPayloadError::ContentType)
    }
}

#[get("/startup/{id}")]
async fn pipe(id: web::Path<String>) -> Result<impl Responder> {
    let mut team = Team::new();
    team.id = id.into_inner();
    Ok(web::Json(team))
}

#[get("/objectives")]
async fn obj() -> Result<impl Responder> {
    let mut badges:Vec<Badge> = vec![];
    for i in 1..30 {
        let mut  b = Badge::new();
        if i > 15 {
            b.category = BadgeCategory::Advanced;
        }
        b.id = format!("0x{:0>4}", i);
        badges.push(b);
    }
    Ok(web::Json(badges))
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(pipe)
            .service(obj)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
