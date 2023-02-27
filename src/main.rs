use actix_web::{http, error, Result, get, post, web, App, HttpResponse, HttpServer, Responder};
use sqlx::{mysql::*, ConnectOptions};
use star_tec_backend::*;
use derive_more::{Display, Error};

#[get("/teams")]
async fn teams(db: web::Data<AppState>) -> impl Responder {
    let raw_teams = sqlx::query_as::<sqlx::mysql::MySql, RawTeam>("SELECT * FROM teams")
        .fetch_all(&db.pool)
        .await
        .unwrap();

    let mut teams:Vec<Team> = vec![];
    for raw_team in raw_teams {
        let team =  Team::from(&raw_team, &db.pool).await;
        teams.push(team);
    }

    ( HttpResponse::Ok().json(teams), http::StatusCode::ACCEPTED )
}

#[get("/team/{id}")]
async fn team_id(db: web::Data<AppState>, key: web::Path<String>) -> impl Responder {
    let raw_team = sqlx::query_as::<sqlx::mysql::MySql, RawTeam>(&format!("SELECT * FROM teams WHERE `id`={}", key.into_inner()))
        .fetch_one(&db.pool)
        .await;
    if let Ok(raw_team) = &raw_team {
        let team =  Team::from(&raw_team, &db.pool).await;
        println!("{:?}", team);
        return ( HttpResponse::Ok().json(team), http::StatusCode::ACCEPTED )
    };
    ( HttpResponse::Ok().json("ID does not exist"), http::StatusCode::NOT_FOUND )
}

#[get("/create/team")]
async fn team_create(db: web::Data<AppState>, bytes: web::Bytes) -> impl Responder {
    let raw_team = String::from_utf8(bytes.to_vec()).unwrap();
    let raw_team:CreateTeam = serde_json::from_str(&raw_team).unwrap();
    let raw_team = RawTeam::from(raw_team);
    sqlx::query!(
        r#"INSERT INTO teams (`score`, `stage`, `name`, `description`, `creation_date`, `location`) VALUES (?, ?, ?, ?, ?, ?)"#,
        raw_team.score,
        raw_team.stage,
        &raw_team.name,
        &raw_team.description,
        raw_team.creation_date.to_string(),
        &raw_team.location,
    )
    .execute(&db.pool)
    .await;
    HttpResponse::Ok()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Get host and port where application will run on
    let host:String = std::env::var("HOST").expect("No HOST found in enviroment variables");
    let port:u16 = std::env::var("PORT").expect("No PORT found in enviroment variables").parse().expect("PORT is not a number");

    // Check if auth key exists
    // let _auth_key: String = std::env::var("AUTH_KEY").expect("No AUTH_KEY has been set");

    // Connect to database
    let database_url = std::env::var("DATABASE_URL").unwrap();
    let pool = MySqlPool::connect(&database_url).await.expect("Unable to connect to database");

    // Set debugger
    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();

    let app_state = AppState { pool };
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .service(teams)
            .service(team_id)
            .service(team_create)
    })
    .bind((host, port))?
    .run()
    .await
}

#[derive(Clone)]
struct AppState {
    pool: sqlx::mysql::MySqlPool,
}
