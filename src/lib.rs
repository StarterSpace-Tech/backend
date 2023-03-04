use sqlx::postgres::PgRow;
use sqlx::Row;
use sqlx::FromRow;
use serde::{Serialize, Deserialize};
use chrono::{NaiveDate, NaiveDateTime, Utc};
use actix_web::cookie::time::Date;
use futures::join;
use sqlx::{Pool, Postgres, query_as};

#[derive(FromRow, Debug)]
pub struct RawID {
    pub id: i64,
}

#[derive(Serialize, Debug)]
pub struct Team {
    id: i64,
    rank: Option<i32>,
    score: i64,
    stage: i32,
    name: String,
    logo_url: Option<String>,
    banner_url: Option<String>,
    description: String,
    creation_date: String,
    location: String,
    labels: Vec<Label>,
    persons: Vec<Person>,
    badges: Vec<OwnedBadge>,
}

#[derive(FromRow, Debug)]
pub struct RawTeam {
    pub id: i64,
    pub rank: Option<i32>,
    pub score: i64,
    pub stage: i32,
    pub name: String,
    pub logo_url: Option<String>,
    pub banner_url: Option<String>,
    pub description: String,
    pub creation_date: actix_web::cookie::time::Date,
    pub location: String,
}

impl RawTeam {
    pub fn from(create_team: CreateTeam) -> RawTeam {
        let format = actix_web::cookie::time::format_description::parse("[year]-[month]-[day]").unwrap();
        RawTeam {
            id: 0,
            rank: None,
            score: 0,
            stage: 1,
            name: create_team.name,
            logo_url: None,
            banner_url: None,
            description: create_team.description,
            creation_date: actix_web::cookie::time::Date::parse(&create_team.creation_date, &format).unwrap(),
            location: create_team.location,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateTeam {
    name: String,
    description: String,
    location: String,
    creation_date: String,
}

impl Team {
    pub fn new() -> Team {
        Team {
            id: 0,
            rank: None,
            score: 0,
            stage: 0,
            name: String::new(),
            logo_url: None,
            banner_url: None,
            description: String::new(),
            creation_date: String::new(),
            location: String::new(),
            labels: vec![],
            persons: vec![],
            badges: vec![],
        }
    }
    pub async fn from(raw_team: RawTeam, pool: &sqlx::postgres::PgPool) -> Team {
        let mut team = Team::new();
        team.id = raw_team.id;
        team.rank = raw_team.rank;
        team.score = raw_team.score;
        team.stage = raw_team.stage;
        team.name = String::from(&raw_team.name);
        if let Some(logo_url) = &raw_team.logo_url {
            team.logo_url = Some(String::from(logo_url));
        }
        if let Some(banner_url) = &raw_team.banner_url {
            team.banner_url = Some(String::from(banner_url));
        }
        team.description = String::from(&raw_team.description);
        team.creation_date = raw_team.creation_date.to_string();
        team.location = String::from(&raw_team.location);

        let labels = team.load_labels(pool.clone());
        let badges = team.load_badges(pool.clone());
        let persons = team.load_persons(pool.clone());
        let (labels, badges, persons) = join!(labels, badges, persons);
        team.labels = labels;
        team.badges = badges;
        team.persons = persons;
        team
    }
    pub async fn load_labels(&self, pool: sqlx::postgres::PgPool) -> Vec<Label> {
        println!("labels");
        let team_id:i32 = self.id as i32;
        let label_ownerships = sqlx::query_as::<sqlx::postgres::Postgres, LabelOwnership>( "SELECT * FROM label_ownerships WHERE team_id = $1 ")
            .bind(self.id)
            .fetch_all(&pool)
            .await
            .unwrap();
        if label_ownerships.is_empty()  { return vec![] };
        let label_id_values:Vec<i64> = label_ownerships.iter().map(|l| l.label_id).collect();
        let labels = sqlx::query_as::<sqlx::postgres::Postgres, Label>("SELECT * FROM labels WHERE id = ANY($1)")
            .bind(&label_id_values[..])
            .fetch_all(&pool)
            .await
            .unwrap();
        println!("labels");
        labels
    }
    pub async fn load_badges(&self, pool: sqlx::postgres::PgPool) -> Vec<OwnedBadge> {
        println!("badges");
        let badge_ownerships = sqlx::query_as::<sqlx::postgres::Postgres, BadgeOwnership>("SELECT * FROM badge_ownerships WHERE team_id = $1")
            .bind(self.id)
            .fetch_all(&pool)
            .await
            .unwrap();
        if badge_ownerships.is_empty()  { return vec![] };
        let mut badges:Vec<OwnedBadge> = vec![];
        for badge_ownership in badge_ownerships {
            let raw_badge = sqlx::query_as::<sqlx::postgres::Postgres, RawBadge>("SELECT * FROM badges WHERE id = $1")
                .bind(badge_ownership.badge_id)
                .fetch_one(&pool)
                .await
                .unwrap();
            let category = sqlx::query_as::<sqlx::postgres::Postgres, Category>("SELECT * FROM badge_categories WHERE id = $1")
                .bind(raw_badge.category)
                .fetch_one(&pool)
                .await
                .unwrap();
            let owned_badge = OwnedBadge::from(raw_badge, category, &badge_ownership);
            badges.push(owned_badge);
        }
        println!("badges");
        badges
    }
    pub async fn load_persons(& self, pool: sqlx::postgres::PgPool) -> Vec<Person> {
        println!("persons");
        let raw_persons = sqlx::query_as::<sqlx::postgres::Postgres, RawPerson>("SELECT * FROM persons WHERE team_id = $1")
            .bind(self.id)
            .fetch_all(&pool)
            .await
            .unwrap();
        let persons:Vec<Person> = raw_persons.iter().map(|p| Person::from(&p)).collect();
        println!("persons");
        persons
    }
}

#[derive(Serialize, Debug)]
pub struct Person {
    id: i64,
    team_id: i64,
    name: String,
    career: String,
    graduation_date: String,
    picture_url: Option<String>,
    portafolio_url: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreatePerson {
    pub team_id: i64,
    pub name: String,
    pub career: String,
    pub graduation_date: String,
}

impl Person {
    pub fn new() -> Person {
        Person {
            id: 0,
            team_id: 0,
            name: String::new(),
            career: String::new(),
            graduation_date: String::new(),
            picture_url: None,
            portafolio_url: None,
        }
    }
    pub fn from(raw_person: &RawPerson) -> Person {
        let mut person = Person::new();
        person.id = raw_person.id;
        person.team_id = raw_person.team_id;
        person.name = raw_person.name.clone();
        person.career = raw_person.career.clone();
        person.graduation_date = raw_person.graduation_date.to_string();
        if let Some(picture_url) = &raw_person.picture_url {
            person.picture_url = Some(String::from(picture_url));
        }
        if let Some(portafolio_url) = &raw_person.portafolio_url {
            person.portafolio_url = Some(String::from(portafolio_url));
        }
        person
    }
}

#[derive(FromRow, Debug)]
pub struct RawPerson {
    pub id: i64,
    pub team_id: i64,
    pub name: String,
    pub career: String,
    pub graduation_date: actix_web::cookie::time::Date,
    pub picture_url: Option<String>,
    pub portafolio_url: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateCategory {
    pub name: String,
}

#[derive(FromRow, Serialize, Debug)]
pub struct Category {
    pub id: i64,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateBadge {
    pub name: String,
    pub description: String,
    pub points: i64,
    pub category: i64,
}

#[derive(Serialize, Debug)]
pub struct Badge {
    id: i64,
    name: String,
    description: String,
    points: i64,
    category: Category,
}

impl Badge {
    pub fn from(raw_badge: RawBadge, category: Category) -> Badge {
        Badge {
            id: raw_badge.id,
            name: raw_badge.name,
            description: raw_badge.description,
            points: raw_badge.points,
            category,
        }
    }
}

#[derive(FromRow, Debug, Serialize)]
pub struct RawBadge {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub points: i64,
    pub category: i64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateBadgeOwnership {
    pub team_id: i64,
    pub badge_id: i64,
    pub acquisition_date: String,
}

#[derive(FromRow, Debug)]
pub struct BadgeOwnership {
    pub id: i64,
    pub team_id: i64,
    pub badge_id: i64,
    pub acquisition_date: actix_web::cookie::time::Date,
}

#[derive(Serialize, Debug)]
struct OwnedBadge {
    id: i64,
    acquisition_date: String,
    badge: Badge,
}

impl OwnedBadge {
    pub fn from(raw_badge: RawBadge, category: Category, badge_ownership: &BadgeOwnership) -> OwnedBadge {
        let badge = Badge::from(raw_badge, category);
        OwnedBadge {
            id: badge_ownership.id,
            acquisition_date: badge_ownership.acquisition_date.to_string(),
            badge,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateLabelOwnership {
    pub team_id: i64,
    pub label_id: i64,
}

#[derive(FromRow, Debug)]
pub struct LabelOwnership {
    pub id: i64,
    pub team_id: i64,
    pub label_id: i64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateLabel {
    pub name: String,
}

#[derive(FromRow, Serialize, Debug)]
pub struct Label {
    pub id: i64,
    pub name: String,
}
