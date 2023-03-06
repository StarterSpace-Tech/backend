use actix_web::{get, post, web, HttpResponse, Responder};
use star_tec_backend::*;
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

    HttpResponse::Ok().json(teams)
}

#[get("/labels")]
async fn labels(db: web::Data<AppState>) -> impl Responder {
    let labels = sqlx::query_as::<sqlx::postgres::Postgres, Label>("SELECT * FROM labels")
        .fetch_all(&db.pool)
        .await
        .unwrap();

    HttpResponse::Ok().json(labels)
}

#[get("/badges")]
async fn badges(db: web::Data<AppState>) -> impl Responder {
    let badges = sqlx::query_as::<sqlx::postgres::Postgres, RawBadge>("SELECT * FROM badges")
        .fetch_all(&db.pool)
        .await
        .unwrap();

    HttpResponse::Ok().json(badges)
}

#[get("/categories")]
async fn categories(db: web::Data<AppState>) -> impl Responder {
    let categories = sqlx::query_as::<sqlx::postgres::Postgres, Category> ("SELECT * FROM badge_categories")
        .fetch_all(&db.pool)
        .await
        .unwrap();

    HttpResponse::Ok().json(categories)
}

#[get("/team/{id}")]
async fn team_id(db: web::Data<AppState>, key: web::Path<i64>) -> impl Responder {
    let raw_team = sqlx::query_as::<sqlx::postgres::Postgres, RawTeam>("SELECT * FROM teams WHERE id = $1")
        .bind(key.into_inner())
        .fetch_one(&db.pool)
        .await;
    if let Ok(raw_team) = raw_team {
        let team =  Team::from(raw_team, &db.pool).await;
        return HttpResponse::Ok().json(team)
    };
    HttpResponse::NotFound().json("ID does not exist")
}

#[post("/create/team")]
async fn team_create(db: web::Data<AppState>, bytes: web::Bytes) -> impl Responder {
    let raw_team = String::from_utf8(bytes.to_vec()).unwrap();
    let raw_team = serde_json::from_str(&raw_team);
    let raw_team:CreateTeam = match raw_team {
        Ok(raw_team) => raw_team,
        Err(err) => return HttpResponse::BadRequest().json(format!("ERROR PARSING JSON: {}", err.to_string())),
    };
    let raw_team = RawTeam::from(raw_team);
    println!("{:?}", raw_team);

    if let Err(err) = sqlx::query("INSERT INTO teams (score, stage, name, description, creation_date, location) VALUES ($1, $2, $3, $4, $5, $6)")
    .bind(raw_team.score)
    .bind(raw_team.stage)
    .bind(&raw_team.name)
    .bind(&raw_team.description)
    .bind(&raw_team.creation_date)
    .bind(&raw_team.location)
    .execute(&db.pool.clone())
    .await
    { return HttpResponse::BadRequest().json(format!("ERROR ADDING TO DATABASE: {}", err.to_string())) }

    let id:i64 = sqlx::query_as::<sqlx::postgres::Postgres, RawID>("SELECT id FROM teams WHERE name = $1")
    .bind(&raw_team.name)
    .fetch_one(&db.pool)
    .await
    .unwrap()
    .id;

    HttpResponse::Ok().json(id)
}

#[post("/add/label")]
async fn add_label(db: web::Data<AppState>, bytes: web::Bytes) -> impl Responder {
    let label_ownership = String::from_utf8(bytes.to_vec()).unwrap();
    let label_ownership = serde_json::from_str(&label_ownership);
    let label_ownership:CreateLabelOwnership = match label_ownership {
        Ok(label_ownership) => label_ownership,
        Err(err) => return HttpResponse::BadRequest().json(format!("ERROR PARSING JSON: {}", err.to_string())),
    };

    if let Err(err) = sqlx::query("INSERT INTO label_ownerships (team_id, label_id) VALUES ($1, $2)")
    .bind(label_ownership.team_id)
    .bind(label_ownership.label_id)
    .execute(&db.pool.clone())
    .await
    { return HttpResponse::BadRequest().json(format!("ERROR ADDING TO DATABASE: {}", err.to_string())) }
    HttpResponse::Ok().into()
}

#[post("/add/badge")]
async fn add_badge(db: web::Data<AppState>, bytes: web::Bytes) -> impl Responder {
    let badge_ownership = String::from_utf8(bytes.to_vec()).unwrap();
    let badge_ownership = serde_json::from_str(&badge_ownership);
    let badge_ownership:CreateBadgeOwnership = match badge_ownership {
        Ok(badge_ownership) => badge_ownership,
        Err(err) => return HttpResponse::BadRequest().json(format!("ERROR PARSING JSON: {}", err.to_string())),
    };
    
    let format = actix_web::cookie::time::format_description::parse("[year]-[month]-[day]").unwrap();
    if let Err(err) = sqlx::query("INSERT INTO badge_ownerships (team_id, badge_id, acquisition_date) VALUES ($1, $2, $3)")
    .bind(badge_ownership.team_id)
    .bind(badge_ownership.badge_id)
    .bind(actix_web::cookie::time::Date::parse(&badge_ownership.acquisition_date, &format).unwrap())
    .execute(&db.pool)
    .await
    { return HttpResponse::BadRequest().json(format!("ERROR ADDING TO DATABASE: {}", err.to_string())) }
    HttpResponse::Ok().into()
}

#[post("/add/person")]
async fn add_person(db: web::Data<AppState>, bytes: web::Bytes) -> impl Responder {
    let person = String::from_utf8(bytes.to_vec()).unwrap();
    let person = serde_json::from_str(&person);
    let person:CreatePerson = match person {
        Ok(person) => person,
        Err(err) => return HttpResponse::BadRequest().json(format!("ERROR PARSING JSON: {}", err.to_string())),
    };

    let format = actix_web::cookie::time::format_description::parse("[year]-[month]-[day]").unwrap();
    if let Err(err) = sqlx::query("INSERT INTO persons (team_id, name, career, graduation_date, picture_url, portafolio_url) VALUES ($1, $2, $3, $4, $5, $6)")
    .bind(person.team_id)
    .bind(&person.name)
    .bind(&person.career)
    .bind(actix_web::cookie::time::Date::parse(&person.graduation_date, &format).unwrap())
    .bind(&person.picture_url)
    .bind(&person.portafolio_url)
    .execute(&db.pool)
    .await
    { return HttpResponse::BadRequest().json(format!("ERROR ADDING TO DATABASE: {}", err.to_string())) }
    HttpResponse::Ok().into()
}

#[post("/create/badge")]
async fn create_badge(db: web::Data<AppState>, bytes: web::Bytes) -> impl Responder {
    let badge = String::from_utf8(bytes.to_vec()).unwrap();
    let badge = serde_json::from_str(&badge);
    let badge:CreateBadge = match badge {
        Ok(badge) => badge,
        Err(err) => return HttpResponse::BadRequest().json(format!("ERROR PARSING JSON: {}", err.to_string())),
    };

    if let Err(err) = sqlx::query("INSERT INTO badges (name, description, points, category) VALUES ($1, $2, $3, $4)")
    .bind(&badge.name)
    .bind(&badge.description)
    .bind(badge.points)
    .bind(badge.category)
    .execute(&db.pool.clone())
    .await
    { return HttpResponse::BadRequest().json(format!("ERROR ADDING TO DATABASE: {}", err.to_string())) }

    let id:i64 = sqlx::query_as::<sqlx::postgres::Postgres, RawID>("SELECT id FROM badges WHERE name = $1")
    .bind(&badge.name)
    .fetch_one(&db.pool)
    .await
    .unwrap()
    .id;

    HttpResponse::Ok().json(id)
}

#[post("/create/label")]
async fn create_label(db: web::Data<AppState>, bytes: web::Bytes) -> impl Responder {
    let label = String::from_utf8(bytes.to_vec()).unwrap();
    let label = serde_json::from_str(&label);
    let label:CreateLabel = match label {
        Ok(label) => label,
        Err(err) => return HttpResponse::BadRequest().json(format!("ERROR PARSING JSON: {}", err.to_string())),
    };

    if let Err(err) = sqlx::query("INSERT INTO labels (name) VALUES ($1)")
    .bind(&label.name)
    .execute(&db.pool.clone())
    .await
    { return HttpResponse::BadRequest().json(format!("ERROR ADDING TO DATABASE: {}", err.to_string())) }

    let id:i64 = sqlx::query_as::<sqlx::postgres::Postgres, RawID>("SELECT id FROM labels WHERE name = $1")
    .bind(&label.name)
    .fetch_one(&db.pool)
    .await
    .unwrap()
    .id;

    HttpResponse::Ok().json(id)
}

#[post("/create/category")]
async fn create_category(db: web::Data<AppState>, bytes: web::Bytes) -> impl Responder {
    let category = String::from_utf8(bytes.to_vec()).unwrap();
    let category = serde_json::from_str(&category);
    let category:CreateCategory = match category {
        Ok(category) => category,
        Err(err) => return HttpResponse::BadRequest().json(format!("ERROR PARSING JSON: {}", err.to_string())),
    };

    if let Err(err) = sqlx::query("INSERT INTO badge_categories (name) VALUES ($1)")
    .bind(&category.name)
    .execute(&db.pool.clone())
    .await
    { return HttpResponse::BadRequest().json(format!("ERROR ADDING TO DATABASE: {}", err.to_string())) }

    let id:i64 = sqlx::query_as::<sqlx::postgres::Postgres, RawID>("SELECT id FROM badge_categories WHERE name = $1")
    .bind(&category.name)
    .fetch_one(&db.pool)
    .await
    .unwrap()
    .id;

    HttpResponse::Ok().json(id)
}

async fn delete_secondaries(table: &str, field: &str, id: i64, pool: sqlx::postgres::PgPool) -> Result<sqlx::postgres::PgQueryResult, sqlx::Error> {
    let query = format!("DELETE FROM {} WHERE {} = $1", table, field);

    sqlx::query(&query)
    .bind(id)
    .execute(&pool)
    .await
}

#[post("/delete")]
async fn delete(db: web::Data<AppState>, req: actix_web::HttpRequest) -> impl Responder {
    let headers = req.headers();
    let force = match headers.get("force"){
        Some(value) if value.as_bytes() == "true".as_bytes() => true,
        Some(value) if value.as_bytes() == "false".as_bytes() => false,
        Some(_) => return HttpResponse::BadRequest().json("UNEXPECTED VALUE FOR force"),
        None => false,
    };
    let id:i64 = match headers.get("id") {
        Some(id) => match String::from_utf8(id.as_bytes().to_vec()).unwrap().parse() {
            Ok(id) => id,
            Err(err) => return HttpResponse::BadRequest().json(format!("ERROR WITH id: {}", err.to_string())),
        }
        None => return HttpResponse::BadRequest().json("NO id FOUND"),
    };
    let taip = match headers.get("type") {
        Some(taip) => String::from_utf8(taip.as_bytes().to_vec()).unwrap(),
        None => return HttpResponse::BadRequest().json("NO type FOUND"),
    };
    let query = format!("DELETE FROM {} WHERE id = $1", match &taip[..] {
        // Delete label ownerships
        "label" => {
            if force { delete_secondaries("label_ownerships", "label_id", id, db.pool.clone()).await.unwrap(); }        
            "labels"
        },

        // Delete badge ownerships
        "badge" => {
            if force { delete_secondaries("badge_ownerships", "badge_id", id, db.pool.clone()).await.unwrap(); }        
            "badges"
        },

        // delete badges with category and badge ownerhips
        "category" => {
            if force {
                let badge_ids = sqlx::query_as::<sqlx::postgres::Postgres, RawID>("SELECT id FROM badges WHERE category = $1")
                    .bind(id)
                    .fetch_all(&db.pool.clone())
                    .await
                    .unwrap();
                let badge_ids:Vec<i64> = badge_ids.iter().map(|b| b.id).collect();
                sqlx::query("DELETE FROM badge_ownerships WHERE badge_id = ANY($1)")
                    .bind(&badge_ids[..])
                    .execute(&db.pool.clone())
                    .await
                    .unwrap();
                sqlx::query("DELETE FROM badges WHERE id = $1")
                    .bind(id)
                    .execute(&db.pool.clone())
                    .await
                    .unwrap();
            }
            "badge_categories"
        },

        "person" => "persons",

        // Delete badge and label ownerhips and people
        "team" => {
            if force {
                let links = vec![
                    delete_secondaries("label_ownerships", "team_id", id, db.pool.clone()),
                    delete_secondaries("badge_ownerships", "team_id", id, db.pool.clone()),
                    delete_secondaries("persons", "team_id", id, db.pool.clone()),
                ];
                let links = future::join_all(links).await;
                for res in  links {
                    res.unwrap();
                }
            }        
            "teams"
        },
        _ => return HttpResponse::BadRequest().json("TYPE IS NOT AVAILABLE FOR DELETION"),
    });
    if let Err(err) = sqlx::query(&query)
    .bind(id)
    .execute(&db.pool)
    .await
    { return HttpResponse::BadRequest().json(format!("ERROR ADDING TO DATABASE: {}", err.to_string())) }
    HttpResponse::Ok().into()
}

#[post("/edit")]
async fn edit(db: web::Data<AppState>, req: actix_web::HttpRequest, bytes: web::Bytes) -> impl Responder {
    let raw_json = String::from_utf8(bytes.to_vec()).unwrap();
    let raw_json = serde_json::from_str(&raw_json);
    let raw_json:Edit = match raw_json {
        Ok(raw_json) => raw_json,
        Err(err) => return HttpResponse::BadRequest().json(format!("ERROR PARSING JSON: {}", err.to_string())),
    };

    let headers = req.headers();
    let id:i64 = match headers.get("id") {
        Some(id) => match String::from_utf8(id.as_bytes().to_vec()).unwrap().parse() {
            Ok(id) => id,
            Err(err) => return HttpResponse::BadRequest().json(format!("ERROR WITH id: {}", err.to_string())),
        }
        None => return HttpResponse::BadRequest().json("NO id FOUND"),
    };
    let taip = match headers.get("type") {
        Some(taip) => String::from_utf8(taip.as_bytes().to_vec()).unwrap(),
        None => return HttpResponse::BadRequest().json("NO type FOUND"),
    };

    let format = actix_web::cookie::time::format_description::parse("[year]-[month]-[day]").unwrap();
    let mut str_values = std::collections::HashMap::new();
    let mut url_values = std::collections::HashMap::new();
    let mut i64_values = std::collections::HashMap::new();
    let mut i32_values = std::collections::HashMap::new();
    let mut date_values = std::collections::HashMap::new();

    if taip == "label" || taip == "badge" || taip == "person" || taip == "team" { if let Some(name) = raw_json.name { str_values.insert("name", name); }; };
    if taip == "badge" || taip == "team" { if let Some(description) = raw_json.description { str_values.insert("description", description); }; };
    if taip == "team" { if let Some(career) = raw_json.career { str_values.insert("career", career); }; };
    if taip == "team" { if let Some(location) = raw_json.location { str_values.insert("location", location); }; };

    if taip == "team" { if let Some(logo_url) = raw_json.logo_url { url_values.insert("logo_url", match &logo_url[..] { "null" => None, _ => Some(logo_url), }); }; };
    if taip == "team" { if let Some(banner_url) = raw_json.banner_url { url_values.insert("banner_url", match &banner_url[..] { "null" => None, _ => Some(banner_url), }); }; };
    if taip == "person" { if let Some(picture_url) = raw_json.picture_url { url_values.insert("picture_url", match &picture_url[..] { "null" => None, _ => Some(picture_url), }); }; };
    if taip == "person" { if let Some(portafolio_url) = raw_json.portafolio_url { url_values.insert("portafolio_url", match &portafolio_url[..] { "null" => None, _ => Some(portafolio_url), }); }; };

    if taip == "team" { if let Some(stage) = raw_json.stage { i32_values.insert("stage", stage); }; }

    if taip == "person" { if let Some(teamid) = raw_json.team_id { i64_values.insert("team_id", teamid); }; };
    if taip == "badge" { if let Some(points) = raw_json.points { i64_values.insert("points", points); }; };
    if taip == "badge" { if let Some(category) = raw_json.category { i64_values.insert("category", category); }; };


    if taip == "team" { 
        if let Some(creation_date) = raw_json.creation_date { 
            let creation_date = match actix_web::cookie::time::Date::parse(&creation_date, &format) {
                Ok(date) => date,
                Err(err) => return HttpResponse::BadRequest().json(format!("ERROR PARSING creation_date: {}", err)),
            };
            date_values.insert("creation_date", creation_date); 
        }; 
    };
    if taip == "person" { 
        if let Some(graduation_date) = raw_json.graduation_date { 
            let graduation_date = match actix_web::cookie::time::Date::parse(&graduation_date, &format) {
                Ok(date) => date,
                Err(err) => return HttpResponse::BadRequest().json(format!("ERROR PARSING graduation_date: {}", err)),
            };
            date_values.insert("graduation_date", graduation_date); 
        }; 
    };

    let taip = match &taip[..] {
        "label" => "labels",
        "badge" => "badges",
        "person" => "persons",
        "team" => "teams",
        _ => return HttpResponse::BadRequest().json("TYPE IS NOT AVAILABLE FOR DELETION"),
    };

    let mut results = vec![];
    for val in str_values {
        let query = format!("UPDATE {} SET {} = $1 WHERE id = $2", taip, val.0);
        let res = sqlx::query(&query)
            .bind(&val.1)
            .bind(id)
            .execute(&db.pool.clone())
            .await;
        results.push(match res {
            Ok(_) => format!("SUCCESFULLY ADDED {}", val.0),
            Err(err) => format!("ERROR ADDING {}: {}", val.0, err),
        });
    }
    for val in i32_values {
        let query = format!("UPDATE {} SET {} = $1 WHERE id = $2", taip, val.0);
        let res = sqlx::query(&query)
            .bind(&val.1)
            .bind(id)
            .execute(&db.pool.clone())
            .await;
        results.push(match res {
            Ok(_) => format!("SUCCESFULLY ADDED {}", val.0),
            Err(err) => format!("ERROR ADDING {}: {}", val.0, err),
        });
    }
    for val in i64_values {
        let query = format!("UPDATE {} SET {} = $1 WHERE id = $2", taip, val.0);
        let res = sqlx::query(&query)
            .bind(&val.1)
            .bind(id)
            .execute(&db.pool.clone())
            .await;
        results.push(match res {
            Ok(_) => format!("SUCCESFULLY ADDED {}", val.0),
            Err(err) => format!("ERROR ADDING {}: {}", val.0, err),
        });
    }
    for val in date_values {
        let query = format!("UPDATE {} SET {} = $1 WHERE id = $2", taip, val.0);
        let res = sqlx::query(&query)
            .bind(&val.1)
            .bind(id)
            .execute(&db.pool.clone())
            .await;
        results.push(match res {
            Ok(_) => format!("SUCCESFULLY ADDED {}", val.0),
            Err(err) => format!("ERROR ADDING {}: {}", val.0, err),
        });
    }
    for val in url_values {
        let query = format!("UPDATE {} SET {} = $1 WHERE id = $2", taip, val.0);
        let res = sqlx::query(&query)
            .bind(&val.1)
            .bind(id)
            .execute(&db.pool.clone())
            .await;
        results.push(match res {
            Ok(_) => format!("SUCCESFULLY ADDED {}", val.0),
            Err(err) => format!("ERROR ADDING {}: {}", val.0, err),
        });
    }

    HttpResponse::Ok().json(results)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Get host and port where application will run on
    let host:String = std::env::var("HOST").expect("No HOST found in enviroment variables");
    let port:u16 = std::env::var("PORT").expect("No PORT found in enviroment variables").parse().expect("PORT is not a number");

    // Connect to database
    let database_url = std::env::var("DATABASE_URL").unwrap();
    let pool = sqlx::postgres::PgPool::connect(&database_url).await.expect("Unable to connect to database");

    // Set debugger
    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();

    let app_state = AppState { pool };
    actix_web::HttpServer::new(move || {
        actix_web::App::new()
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
            .service(delete)
            .service(edit)
    })
    .bind((host, port))?
    .run()
    .await
}

#[derive(Clone)]
struct AppState {
    pool: sqlx::postgres::PgPool,
}
