use serde::{Serialize, Deserialize};
use sqlx::mysql::MySqlRow;
use sqlx::Row;
use sqlx::FromRow;

#[derive(Serialize, Debug)]
pub struct Team {
    id: u64,
    rank: u32,
    score: u64,
    stage: u32,
    name: String,
    logo_url: String,
    banner_url: String,
    description: String,
    creation_date: String,
    location: String,
    labels: Vec<Label>,
    persons: Vec<Person>,
    badges: Vec<OwnedBadge>,
}

#[derive(FromRow, Debug)]
pub struct RawTeam {
    pub id: u64,
    pub rank: Option<u32>,
    pub score: u64,
    pub stage: u32,
    pub name: String,
    pub logo_url: Option<String>,
    pub banner_url: Option<String>,
    pub description: String,
    pub creation_date: actix_web::cookie::time::Date,
    pub location: String,
}

impl RawTeam {
    pub fn from(create_team: CreateTeam) -> RawTeam {
        RawTeam {
            id: 0,
            rank: None,
            score: 0,
            stage: 1,
            name: create_team.name,
            logo_url: Some(String::from("https://www.pngitem.com/pimgs/m/150-1503945_transparent-user-png-default-user-image-png-png.png")),
            banner_url: Some(String::from("https://thumbs.dreamstime.com/b/user-profile-icon-crystal-blue-banner-background-isolated-169986843.jpg")),
            description: create_team.description,
            creation_date: actix_web::cookie::time::Date::from_calendar_date(2020, actix_web::cookie::time::Month::January, 1).unwrap(),
            location: create_team.location,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateTeam {
    name: String,
    description: String,
    location: String,
}

impl Team {
    pub fn new() -> Team {
        Team {
            id: 0,
            rank: 0,
            score: 0,
            stage: 0,
            name: String::new(),
            logo_url: String::from("https://www.pngitem.com/pimgs/m/150-1503945_transparent-user-png-default-user-image-png-png.png"),
            banner_url: String::from("https://thumbs.dreamstime.com/b/user-profile-icon-crystal-blue-banner-background-isolated-169986843.jpg"),
            description: String::new(),
            creation_date: String::new(),
            location: String::new(),
            labels: vec![],
            persons: vec![],
            badges: vec![],
        }
    }
    pub async fn from(row: &RawTeam, pool: &sqlx::mysql::MySqlPool) -> Team {
        let mut team = Team::new();
        team.id = row.id;
        if let Some(rank) = row.rank {
            team.rank = rank;
        }
        team.score = row.score;
        team.stage = row.stage;
        team.name = String::from(&row.name);
        if let Some(logo_url) = &row.logo_url {
            team.logo_url = String::from(logo_url);
        }
        if let Some(banner_url) = &row.banner_url {
            team.banner_url = String::from(banner_url);
        }
        team.description = String::from(&row.description);
        team.creation_date = row.creation_date.to_string();
        team.location = String::from(&row.location);
        team.load_labels(&pool).await;
        team.load_badges(&pool).await;
        team.load_persons(&pool).await;
        team
    }
    pub async fn load_labels(&mut self, pool: &sqlx::mysql::MySqlPool) {
        let label_ownerships = sqlx::query_as::<sqlx::mysql::MySql, LabelOwnership>(&format!("SELECT * FROM label_ownerships WHERE `team_id`={}", self.id)[..])
            .fetch_all(pool)
            .await
            .unwrap();
        if label_ownerships.is_empty()  { return };
        let label_id_values:Vec<String> = label_ownerships.iter().map(|l| l.label_id.to_string()).collect();
        let label_id_values = label_id_values.join(", ");
        let labels = sqlx::query_as::<sqlx::mysql::MySql, Label>(&format!("SELECT * FROM labels WHERE `id` IN ({})", label_id_values))
            .fetch_all(pool)
            .await
            .unwrap();
        self.labels = labels
    }
    pub async fn load_badges(&mut self, pool: &sqlx::mysql::MySqlPool) {
        let badge_ownerships = sqlx::query_as::<sqlx::mysql::MySql, BadgeOwnership>(&format!("SELECT * FROM badge_ownerships WHERE `team_id`={}", self.id)[..])
            .fetch_all(pool)
            .await
            .unwrap();
        if badge_ownerships.is_empty()  { return };
        let mut badges:Vec<OwnedBadge> = vec![];
        for b in badge_ownerships {
            let raw_badge = sqlx::query_as::<sqlx::mysql::MySql, RawBadge>(&format!("SELECT * FROM badges WHERE `id`={}", b.badge_id))
                .fetch_one(pool)
                .await
                .unwrap();
            let category = sqlx::query_as::<sqlx::mysql::MySql, Category>(&format!("SELECT * FROM badge_categories WHERE `id`={}", raw_badge.category))
                .fetch_one(pool)
                .await
                .unwrap();
            let owned_badge = OwnedBadge::from(raw_badge, category, &b);
            badges.push(owned_badge);
        }
        self.badges = badges;
    }
    pub async fn load_persons(&mut self, pool: &sqlx::mysql::MySqlPool) {
        let raw_persons = sqlx::query_as::<sqlx::mysql::MySql, RawPerson>(&format!("SELECT * FROM persons WHERE `team`={}", self.id)[..])
            .fetch_all(pool)
            .await
            .unwrap();
        let persons:Vec<Person> = raw_persons.iter().map(|p| Person::from(&p)).collect();
        self.persons = persons;
    }
}

#[derive(Serialize, Debug)]
struct Person {
    id: u64,
    team: u64,
    name: String,
    career: String,
    graduation_date: String,
    picture_url: String,
    portafolio_url: String,
}

impl Person {
    pub fn new() -> Person {
        Person {
            id: 0,
            team: 0,
            name: String::new(),
            career: String::new(),
            graduation_date: String::new(),
            picture_url: String::from("Error"),
            portafolio_url: String::from("Error"),
        }
    }
    pub fn from(raw_person: &RawPerson) -> Person {
        let mut person = Person::new();
        person.id = raw_person.id;
        person.team = raw_person.team;
        person.name = raw_person.name.clone();
        person.career = raw_person.career.clone();
        person.graduation_date = raw_person.graduation_date.to_string();
        if let Some(picture_url) = &raw_person.picture_url {
            person.picture_url = String::from(picture_url);
        }
        if let Some(portafolio_url) = &raw_person.portafolio_url {
            person.portafolio_url = String::from(portafolio_url);
        }
        person
    }
}

#[derive(FromRow, Debug)]
struct RawPerson {
    pub id: u64,
    pub team: u64,
    pub name: String,
    pub career: String,
    pub graduation_date: actix_web::cookie::time::Date,
    pub picture_url: Option<String>,
    pub portafolio_url: Option<String>,
}

#[derive(FromRow, Serialize, Debug)]
struct Category {
    id: u64,
    name: String,
}

#[derive(Serialize, Debug)]
struct Badge {
    id: u64,
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

#[derive(FromRow, Debug)]
struct RawBadge {
    id: u64,
    name: String,
    description: String,
    points: i64,
    category: u64,
}

#[derive(FromRow, Debug)]
pub struct BadgeOwnership {
    pub id: u64,
    pub team_id: u64,
    pub badge_id: u64,
    pub acquisition_date: actix_web::cookie::time::Date,
}

#[derive(Serialize, Debug)]
struct OwnedBadge {
    id: u64,
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

#[derive(FromRow, Debug)]
pub struct LabelOwnership {
    pub id: u64,
    pub team_id: u64,
    pub label_id: u64,
}

#[derive(FromRow, Serialize, Debug)]
struct Label {
    id: u64,
    name: String,
}
