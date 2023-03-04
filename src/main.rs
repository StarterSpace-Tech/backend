use actix_web::{http, error, Result, get, post, web, App, HttpResponse, HttpServer, Responder};
use sqlx::{mysql::*, ConnectOptions};
use star_tec_backend::*;
use derive_more::{Display, Error};
use futures::*;

#[get("/teams")]
async fn teams(db: web::Data<AppState>) -> impl Responder {
    let raw_teams:Vec<RawTeam> = sqlx::query_as("SELECT * FROM teams")
        .fetch_all(&db.pool)
        .await
        .unwrap();


    let mut teams = vec![];
    for raw_team in raw_teams {
        let team =  Team::from(raw_team, &db.pool);
        teams.push(team);
    }
    let teams = future::join_all(teams).await;

    ( HttpResponse::Ok().json(teams), http::StatusCode::ACCEPTED )
}

#[get("/labels")]
async fn labels(db: web::Data<AppState>) -> impl Responder {
    let labels = sqlx::query_as::<sqlx::postgres::Postgres, Label>("SELECT * FROM labels")
        .fetch_all(&db.pool)
        .await
        .unwrap();

    ( HttpResponse::Ok().json(labels), http::StatusCode::ACCEPTED )
}

#[get("/badges")]
async fn badges(db: web::Data<AppState>) -> impl Responder {
    let badges = sqlx::query_as::<sqlx::postgres::Postgres, RawBadge>("SELECT * FROM badges")
        .fetch_all(&db.pool)
        .await
        .unwrap();

    ( HttpResponse::Ok().json(badges), http::StatusCode::ACCEPTED )
}

#[get("/categories")]
async fn categories(db: web::Data<AppState>) -> impl Responder {
    let categories = sqlx::query_as::<sqlx::postgres::Postgres, Category> ("SELECT * FROM badge_categories")
        .fetch_all(&db.pool)
        .await
        .unwrap();

    ( HttpResponse::Ok().json(categories), http::StatusCode::ACCEPTED )
}


#[get("/team/{id}")]
async fn team_id(db: web::Data<AppState>, key: web::Path<i64>) -> impl Responder {
    let raw_team = sqlx::query_as::<sqlx::postgres::Postgres, RawTeam>("SELECT * FROM teams WHERE id = $1")
        .bind(key.into_inner())
        .fetch_one(&db.pool)
        .await;
    if let Ok(raw_team) = raw_team {
        let team =  Team::from(raw_team, &db.pool).await;
        return ( HttpResponse::Ok().json(team), http::StatusCode::ACCEPTED )
    };
    ( HttpResponse::Ok().json("ID does not exist"), http::StatusCode::NOT_FOUND )
}

#[post("/create/team")]
async fn team_create(db: web::Data<AppState>, bytes: web::Bytes) -> impl Responder {
    let mut conn = db.pool.clone().acquire().await.unwrap();
    let raw_team = String::from_utf8(bytes.to_vec()).unwrap();
    let raw_team:CreateTeam = serde_json::from_str(&raw_team).unwrap();
    let raw_team = RawTeam::from(raw_team);

    sqlx::query("INSERT INTO teams (`score`, `stage`, `name`, `description`, `creation_date`, `location`) VALUES (?, ?, ?, ?, ?, ?)")
    .bind(raw_team.score)
    .bind(raw_team.stage)
    .bind(&raw_team.name)
    .bind(&raw_team.description)
    .bind(raw_team.creation_date.to_string())
    .bind(&raw_team.location)
    .execute(&mut conn)
    .await
    .unwrap();

    let raw_id = sqlx::query_as::<sqlx::postgres::Postgres, RawID>("SELECT id FROM teams WHERE name = $1")
    .bind(&raw_team.name)
    .fetch_one(&db.pool)
    .await
    .unwrap();

    HttpResponse::SeeOther()
    .header(http::header::LOCATION, format!("/team/{}", raw_id.id))
    .finish()
}

#[post("/add/label")]
async fn add_label(db: web::Data<AppState>, bytes: web::Bytes) -> impl Responder {
    let label_ownership = String::from_utf8(bytes.to_vec()).unwrap();
    let label_ownership:CreateLabelOwnership = serde_json::from_str(&label_ownership).unwrap();

    sqlx::query("INSERT INTO label_ownerships (`team_id`, `label_id`) VALUES (?, ?)")
    .bind(label_ownership.team_id)
    .bind(label_ownership.label_id)
    .execute(&db.pool)
    .await
    .unwrap();

    ( HttpResponse::Ok(), http::StatusCode::ACCEPTED )
}

#[post("/add/badge")]
async fn add_badge(db: web::Data<AppState>, bytes: web::Bytes) -> impl Responder {
    let badge_ownership = String::from_utf8(bytes.to_vec()).unwrap();
    let badge_ownership:CreateBadgeOwnership = serde_json::from_str(&badge_ownership).unwrap();

    sqlx::query("INSERT INTO badge_ownerships (`team_id`, `badge_id`, `acquisition_date`) VALUES (?, ?, ?)")
    .bind(badge_ownership.team_id)
    .bind(badge_ownership.badge_id)
    .bind(badge_ownership.acquisition_date.to_string())
    .execute(&db.pool)
    .await
    .unwrap();

    ( HttpResponse::Ok(), http::StatusCode::ACCEPTED )
}

#[post("/add/person")]
async fn add_person(db: web::Data<AppState>, bytes: web::Bytes) -> impl Responder {
    let person = String::from_utf8(bytes.to_vec()).unwrap();
    let person:CreatePerson = serde_json::from_str(&person).unwrap();

    sqlx::query("INSERT INTO persons (`team_id`, `name`, `career`, `graduation_date`) VALUES (?, ?, ?, ?)")
    .bind(person.team_id)
    .bind(&person.name)
    .bind(&person.career)
    .bind(&person.graduation_date)
    .execute(&db.pool)
    .await
    .unwrap();

    ( HttpResponse::Ok(), http::StatusCode::ACCEPTED )
}

#[post("/create/badge")]
async fn create_badge(db: web::Data<AppState>, bytes: web::Bytes) -> impl Responder {
    let badge = String::from_utf8(bytes.to_vec()).unwrap();
    let badge:CreateBadge = serde_json::from_str(&badge).unwrap();

    sqlx::query("INSERT INTO badges (`name`, `description`, `points`, `category`) VALUES (?, ?, ?, ?)")
    .bind(&badge.name)
    .bind(&badge.description)
    .bind(badge.points)
    .bind(badge.category)
    .execute(&db.pool)
    .await
    .unwrap();

    ( HttpResponse::Ok(), http::StatusCode::ACCEPTED )
}

#[post("/create/label")]
async fn create_label(db: web::Data<AppState>, bytes: web::Bytes) -> impl Responder {
    let label = String::from_utf8(bytes.to_vec()).unwrap();
    let label:CreateLabel = serde_json::from_str(&label).unwrap();

    sqlx::query("INSERT INTO labels (`name`) VALUES (?)")
    .bind(&label.name)
    .execute(&db.pool)
    .await
    .unwrap();

    ( HttpResponse::Ok(), http::StatusCode::ACCEPTED )
}

#[post("/create/category")]
async fn create_category(db: web::Data<AppState>, bytes: web::Bytes) -> impl Responder {
    let category = String::from_utf8(bytes.to_vec()).unwrap();
    let category:CreateCategory = serde_json::from_str(&category).unwrap();

    sqlx::query("INSERT INTO badge_categories (`name`) VALUES (?)")
    .bind(&category.name)
    .execute(&db.pool)
    .await
    .unwrap();

    ( HttpResponse::Ok(), http::StatusCode::ACCEPTED )
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
    let pool = sqlx::postgres::PgPool::connect(&database_url).await.expect("Unable to connect to database");

    // Set debugger
    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();

    let app_state = AppState { pool };
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .service(teams)
            .service(labels)
            .service(badges)
            .service(categories)
            .service(team_id)
            .service(team_create)
            .service(add_label)
            .service(add_badge)
            .service(add_person)
            .service(create_badge)
            .service(create_label)
            .service(create_category)
    })
    .bind((host, port))?
    .run()
    .await
}

#[derive(Clone)]
struct AppState {
    pool: sqlx::postgres::PgPool,
}
