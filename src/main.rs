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
    let mut teams = future::join_all(teams).await;
    teams.sort_by(|a, b| {
        if a.score == b.score { a.name.cmp(&b.name) }
        else { b.score.cmp(&a.score) }
    });
    HttpResponse::Ok().append_header(("Access-Control-Allow-Origin", "*")).json(teams)
}

#[get("/labels")]
async fn labels(db: web::Data<AppState>) -> impl Responder {
    let labels = sqlx::query_as::<sqlx::postgres::Postgres, Label>("SELECT * FROM labels")
        .fetch_all(&db.pool)
        .await
        .unwrap();

    HttpResponse::Ok().append_header(("Access-Control-Allow-Origin", "*")).json(labels)
}

#[get("/badges")]
async fn badges(db: web::Data<AppState>) -> impl Responder {
    let badges = sqlx::query_as::<sqlx::postgres::Postgres, RawBadge>("SELECT * FROM badges")
        .fetch_all(&db.pool)
        .await
        .unwrap();

    HttpResponse::Ok().append_header(("Access-Control-Allow-Origin", "*")).json(badges)
}

#[get("/categories")]
async fn categories(db: web::Data<AppState>) -> impl Responder {
    let categories = sqlx::query_as::<sqlx::postgres::Postgres, Category> ("SELECT * FROM badge_categories")
        .fetch_all(&db.pool)
        .await
        .unwrap();

    HttpResponse::Ok().append_header(("Access-Control-Allow-Origin", "*")).json(categories)
}

#[get("/team/{id}")]
async fn team_id(db: web::Data<AppState>, key: web::Path<i64>) -> impl Responder {
    let raw_team = sqlx::query_as::<sqlx::postgres::Postgres, RawTeam>("SELECT * FROM teams WHERE id = $1")
        .bind(key.into_inner())
        .fetch_one(&db.pool)
        .await;
    if let Ok(raw_team) = raw_team {
        let team =  Team::from(raw_team, &db.pool).await;
        return HttpResponse::Ok().append_header(("Access-Control-Allow-Origin", "*")).json(team)
    };
    HttpResponse::NotFound().append_header(("Access-Control-Allow-Origin", "*")).json("ID does not exist")
}

#[post("/create/team")]
async fn team_create(db: web::Data<AppState>, bytes: web::Bytes) -> impl Responder {
    let raw_team = String::from_utf8(bytes.to_vec()).unwrap();
    let raw_team = serde_json::from_str(&raw_team);
    let raw_team:CreateTeam = match raw_team {
        Ok(raw_team) => raw_team,
        Err(err) => return HttpResponse::BadRequest()
        .append_header(("Access-Control-Allow-Origin", "*"))
        .json(format!("ERROR PARSING JSON: {}", err)),
    };
    let raw_team = RawTeam::from(raw_team);

    if let Err(err) = sqlx::query("INSERT INTO teams (score, stage, name, description, creation_date, location, logo_url, banner_url) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)")
    .bind(raw_team.score)
    .bind(raw_team.stage)
    .bind(&raw_team.name)
    .bind(&raw_team.description)
    .bind(raw_team.creation_date)
    .bind(&raw_team.location)
    .bind(&raw_team.logo_url)
    .bind(&raw_team.banner_url)
    .execute(&db.pool.clone())
    .await
    { return HttpResponse::BadRequest()        
        .append_header(("Access-Control-Allow-Origin", "*"))
        .json(format!("ERROR ADDING TO DATABASE: {}", err)) 

    }

    let id:i64 = sqlx::query_as::<sqlx::postgres::Postgres, RawID>("SELECT id FROM teams WHERE name = $1")
    .bind(&raw_team.name)
    .fetch_one(&db.pool)
    .await
    .unwrap()
    .id;

    HttpResponse::Ok()
        .append_header(("Access-Control-Allow-Origin", "*"))
        .json(id)
}

#[post("/add/label")]
async fn add_label(db: web::Data<AppState>, bytes: web::Bytes) -> impl Responder {
    let label_ownership = String::from_utf8(bytes.to_vec()).unwrap();
    let label_ownership = serde_json::from_str(&label_ownership);
    let label_ownership:CreateLabelOwnership = match label_ownership {
        Ok(label_ownership) => label_ownership,
        Err(err) => return HttpResponse::BadRequest().append_header(("Access-Control-Allow-Origin", "*")).json(format!("ERROR PARSING JSON: {}", err)),
    };

    if let Err(err) = sqlx::query("INSERT INTO label_ownerships (team_id, label_id) VALUES ($1, $2)")
    .bind(label_ownership.team_id)
    .bind(label_ownership.label_id)
    .execute(&db.pool.clone())
    .await
    { return HttpResponse::BadRequest().append_header(("Access-Control-Allow-Origin", "*")).json(format!("ERROR ADDING TO DATABASE: {}", err)) }
    HttpResponse::Ok().append_header(("Access-Control-Allow-Origin", "*")).json("Success")
}

#[post("/add/badge")]
async fn add_badge(db: web::Data<AppState>, bytes: web::Bytes) -> impl Responder {
    let badge_ownership = String::from_utf8(bytes.to_vec()).unwrap();
    let badge_ownership = serde_json::from_str(&badge_ownership);
    let badge_ownership:CreateBadgeOwnership = match badge_ownership {
        Ok(badge_ownership) => badge_ownership,
        Err(err) => return HttpResponse::BadRequest().append_header(("Access-Control-Allow-Origin", "*")).json(format!("ERROR PARSING JSON: {}", err)),
    };
    
    let format = actix_web::cookie::time::format_description::parse("[year]-[month]-[day]").unwrap();
    if let Err(err) = sqlx::query("INSERT INTO badge_ownerships (team_id, badge_id, acquisition_date) VALUES ($1, $2, $3)")
    .bind(badge_ownership.team_id)
    .bind(badge_ownership.badge_id)
    .bind(actix_web::cookie::time::Date::parse(&badge_ownership.acquisition_date, &format).unwrap())
    .execute(&db.pool)
    .await
    { return HttpResponse::BadRequest().append_header(("Access-Control-Allow-Origin", "*")).json(format!("ERROR ADDING TO DATABASE: {}", err)) }
    update_score(badge_ownership.team_id, db.pool.clone()).await;
    update_ranking(db.pool.clone()).await;
    HttpResponse::Ok().append_header(("Access-Control-Allow-Origin", "*")).json("Success")
}

#[post("/add/person")]
async fn add_person(db: web::Data<AppState>, bytes: web::Bytes) -> impl Responder {
    let person = String::from_utf8(bytes.to_vec()).unwrap();
    let person = serde_json::from_str(&person);
    let person:CreatePerson = match person {
        Ok(person) => person,
        Err(err) => return HttpResponse::BadRequest().append_header(("Access-Control-Allow-Origin", "*")).json(format!("ERROR PARSING JSON: {}", err)),
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
    { return HttpResponse::BadRequest().append_header(("Access-Control-Allow-Origin", "*")).json(format!("ERROR ADDING TO DATABASE: {}", err)) }
    HttpResponse::Ok().append_header(("Access-Control-Allow-Origin", "*")).json("Success")
}

#[post("/create/badge")]
async fn create_badge(db: web::Data<AppState>, bytes: web::Bytes) -> impl Responder {
    let badge = String::from_utf8(bytes.to_vec()).unwrap();
    let badge = serde_json::from_str(&badge);
    let badge:CreateBadge = match badge {
        Ok(badge) => badge,
        Err(err) => return HttpResponse::BadRequest().append_header(("Access-Control-Allow-Origin", "*")).json(format!("ERROR PARSING JSON: {}", err)),
    };

    if let Err(err) = sqlx::query("INSERT INTO badges (name, description, points, category) VALUES ($1, $2, $3, $4)")
    .bind(&badge.name)
    .bind(&badge.description)
    .bind(badge.points)
    .bind(badge.category)
    .execute(&db.pool.clone())
    .await
    { return HttpResponse::BadRequest().append_header(("Access-Control-Allow-Origin", "*")).json(format!("ERROR ADDING TO DATABASE: {}", err)) }

    let id:i64 = sqlx::query_as::<sqlx::postgres::Postgres, RawID>("SELECT id FROM badges WHERE name = $1")
    .bind(&badge.name)
    .fetch_one(&db.pool)
    .await
    .unwrap()
    .id;

    HttpResponse::Ok().append_header(("Access-Control-Allow-Origin", "*")).json(id)
}

#[post("/create/label")]
async fn create_label(db: web::Data<AppState>, bytes: web::Bytes) -> impl Responder {
    let label = String::from_utf8(bytes.to_vec()).unwrap();
    let label = serde_json::from_str(&label);
    let label:CreateLabel = match label {
        Ok(label) => label,
        Err(err) => return HttpResponse::BadRequest().append_header(("Access-Control-Allow-Origin", "*")).json(format!("ERROR PARSING JSON: {}", err)),
    };

    if let Err(err) = sqlx::query("INSERT INTO labels (name) VALUES ($1)")
    .bind(&label.name)
    .execute(&db.pool.clone())
    .await
    { return HttpResponse::BadRequest().append_header(("Access-Control-Allow-Origin", "*")).json(format!("ERROR ADDING TO DATABASE: {}", err)) }

    let id:i64 = sqlx::query_as::<sqlx::postgres::Postgres, RawID>("SELECT id FROM labels WHERE name = $1")
    .bind(&label.name)
    .fetch_one(&db.pool)
    .await
    .unwrap()
    .id;

    HttpResponse::Ok().append_header(("Access-Control-Allow-Origin", "*")).json(id)
}

#[post("/create/category")]
async fn create_category(db: web::Data<AppState>, bytes: web::Bytes) -> impl Responder {
    let category = String::from_utf8(bytes.to_vec()).unwrap();
    let category = serde_json::from_str(&category);
    let category:CreateCategory = match category {
        Ok(category) => category,
        Err(err) => return HttpResponse::BadRequest().append_header(("Access-Control-Allow-Origin", "*")).json(format!("ERROR PARSING JSON: {}", err)),
    };

    if let Err(err) = sqlx::query("INSERT INTO badge_categories (name) VALUES ($1)")
    .bind(&category.name)
    .execute(&db.pool.clone())
    .await
    { return HttpResponse::BadRequest().append_header(("Access-Control-Allow-Origin", "*")).json(format!("ERROR ADDING TO DATABASE: {}", err)) }

    let id:i64 = sqlx::query_as::<sqlx::postgres::Postgres, RawID>("SELECT id FROM badge_categories WHERE name = $1")
    .bind(&category.name)
    .fetch_one(&db.pool)
    .await
    .unwrap()
    .id;

    HttpResponse::Ok().append_header(("Access-Control-Allow-Origin", "*")).json(id)
}

#[post("/delete_ownership")]
async fn delete_ownership(db: web::Data<AppState>, bytes: web::Bytes,  info: web::Query<DeleteOwnershipQuery>) -> impl Responder {
    let (tid, id, table, column) = match &info.kind[..] {
        "label" => {
            let label_ownership = String::from_utf8(bytes.to_vec()).unwrap();
            let label_ownership = serde_json::from_str(&label_ownership);
            let label_ownership:DeleteOwnedLabel = match label_ownership {
                Ok(label_ownership) => label_ownership,
                Err(err) => return HttpResponse::BadRequest().append_header(("Access-Control-Allow-Origin", "*")).json(format!("ERROR PARSING JSON: {}", err)),
            };
            (label_ownership.team_id, label_ownership.label_id, "label_ownerships", "label_id")
        },
        "badge"=> {
            let badge_ownership = String::from_utf8(bytes.to_vec()).unwrap();
            let badge_ownership = serde_json::from_str(&badge_ownership);
            let badge_ownership:DeleteOwnedBadge = match badge_ownership {
                Ok(badge_ownership) => badge_ownership,
                Err(err) => return HttpResponse::BadRequest().append_header(("Access-Control-Allow-Origin", "*")).json(format!("ERROR PARSING JSON: {}", err)),
            };
            (badge_ownership.team_id, badge_ownership.badge_id, "badge_ownerships", "badge_id")
        },
        _ => return HttpResponse::BadRequest().append_header(("Access-Control-Allow-Origin", "*")).json("NO type FOUND")
    };
    let query = format!("DELETE FROM {} WHERE team_id = $1 AND {} = $2", table, column);
    sqlx::query(&query)
        .bind(tid)
        .bind(id)
        .execute(&db.pool.clone())
        .await
        .unwrap();
    if let "badge_id" = column {
        update_score(id, db.pool.clone()).await;
        update_ranking(db.pool.clone()).await;
    }

    HttpResponse::Ok().append_header(("Access-Control-Allow-Origin", "*")).json("OK")
}

async fn delete_secondaries(table: &str, field: &str, id: i64, pool: sqlx::postgres::PgPool) -> Result<sqlx::postgres::PgQueryResult, sqlx::Error> {
    let query = format!("DELETE FROM {} WHERE {} = $1", table, field);

    sqlx::query(&query)
    .bind(id)
    .execute(&pool)
    .await
}

#[post("/delete")]
async fn delete(db: web::Data<AppState>, info: web::Query<DeleteQuery>) -> impl Responder {
    let force = info.force.unwrap_or(false);
    let query = format!("DELETE FROM {} WHERE id = $1", match &info.kind[..] {
        // Delete label ownerships
        "label" => {
            if force { delete_secondaries("label_ownerships", "label_id", info.id, db.pool.clone()).await.unwrap(); }        
            "labels"
        },

        // Delete badge ownerships
        "badge" => {
            if force { delete_secondaries("badge_ownerships", "badge_id", info.id, db.pool.clone()).await.unwrap(); }        
            update_scores(db.pool.clone()).await;
            "badges"
        },

        // delete badges with category and badge ownerhips
        "category" => {
            if force {
                let badge_ids = sqlx::query_as::<sqlx::postgres::Postgres, RawID>("SELECT id FROM badges WHERE category = $1")
                    .bind(info.id)
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
                    .bind(info.id)
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
                    delete_secondaries("label_ownerships", "team_id", info.id, db.pool.clone()),
                    delete_secondaries("badge_ownerships", "team_id", info.id, db.pool.clone()),
                    delete_secondaries("persons", "team_id", info.id, db.pool.clone()),
                ];
                let links = future::join_all(links).await;
                for res in  links {
                    res.unwrap();
                }
            }        
            "teams"
        },
        _ => return HttpResponse::BadRequest().append_header(("Access-Control-Allow-Origin", "*")).json("TYPE IS NOT AVAILABLE FOR DELETION"),
    });
    if let Err(err) = sqlx::query(&query)
    .bind(info.id)
    .execute(&db.pool)
    .await
    { return HttpResponse::BadRequest().append_header(("Access-Control-Allow-Origin", "*")).json(format!("ERROR ADDING TO DATABASE: {}", err)) }
    HttpResponse::Ok().append_header(("Access-Control-Allow-Origin", "*")).json("Success")
}

#[post("/edit")]
async fn edit(db: web::Data<AppState>, info: web::Query<DeleteQuery>, bytes: web::Bytes) -> impl Responder {
    let raw_json = String::from_utf8(bytes.to_vec()).unwrap();
    let query = match &info.kind[..] {
        "category" => {
            let raw_json:EditCategory = match serde_json::from_str(&raw_json) {
                Ok(raw_json) => raw_json,
                Err(err) => return HttpResponse::BadRequest().append_header(("Access-Control-Allow-Origin", "*")).json(format!("ERROR PARSING JSON: {}", err)),
            };
            raw_json.query()
        }
        "label" => {
            let raw_json:EditLabel = match serde_json::from_str(&raw_json) {
                Ok(raw_json) => raw_json,
                Err(err) => return HttpResponse::BadRequest().append_header(("Access-Control-Allow-Origin", "*")).json(format!("ERROR PARSING JSON: {}", err)),
            };
            raw_json.query()
        },
        "badge" => {
            let raw_json:EditBadge = match serde_json::from_str(&raw_json) {
                Ok(raw_json) => raw_json,
                Err(err) => return HttpResponse::BadRequest().append_header(("Access-Control-Allow-Origin", "*")).json(format!("ERROR PARSING JSON: {}", err)),
            };
            raw_json.query()
        },
        "person" => {
            let raw_json:EditPerson = match serde_json::from_str(&raw_json) {
                Ok(raw_json) => raw_json,
                Err(err) => return HttpResponse::BadRequest().append_header(("Access-Control-Allow-Origin", "*")).json(format!("ERROR PARSING JSON: {}", err)),
            };
            raw_json.query()
        },
        "team" => {
            let raw_json:EditTeam = match serde_json::from_str(&raw_json) {
                Ok(raw_json) => raw_json,
                Err(err) => return HttpResponse::BadRequest().append_header(("Access-Control-Allow-Origin", "*")).json(format!("ERROR PARSING JSON: {}", err)),
            };
            raw_json.query()
        },
        _ => return HttpResponse::BadRequest().append_header(("Access-Control-Allow-Origin", "*")).json("TYPE IS NOT AVAILABLE FOR DELETION"),
    };

    if let Err(err) = sqlx::query(&query)
        .bind(info.id)
        .execute(&db.pool)
        .await
    { return HttpResponse::BadRequest().append_header(("Access-Control-Allow-Origin", "*")).json(format!("ERROR ADDING TO DATABASE: {}", err)) }

    HttpResponse::Ok().append_header(("Access-Control-Allow-Origin", "*")).json("Success")
}

async fn update_scores(pool: sqlx::postgres::PgPool) {
    let ids = sqlx::query_as::<sqlx::postgres::Postgres, RawID>("SELECT id FROM teams")
        .fetch_all(&pool.clone())
        .await
        .unwrap();
    let mut updates = vec![];
    for id in ids {
        updates.push(update_score(id.id, pool.clone()));
    }
    update_ranking(pool.clone()).await;
    future::join_all(updates).await;

}
#[post("update/rankings")]
async fn update_rankings(db: web::Data<AppState>) -> impl Responder {
    update_scores(db.pool.clone()).await;
    HttpResponse::Ok().append_header(("Access-Control-Allow-Origin", "*")).json("OK")
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

    // let conn = pool.clone();
    // std::thread::spawn( move  || {
    //     loop {
    //         update_ranking(conn.clone()).await;
    //         std::thread::sleep(std::time::Duration::new(60 * 60, 0));
    //     }
    // });

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
            .service(delete_ownership)
            .service(delete)
            .service(edit)
            .service(update_rankings)
    })
    .bind((host, port))?
    .run()
    .await
}

#[derive(Clone)]
struct AppState {
    pool: sqlx::postgres::PgPool,
}
