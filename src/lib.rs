use sqlx::FromRow;
use serde::{Serialize, Deserialize};
use futures::join;

#[derive(FromRow, Debug, Serialize)]
pub struct RawID {
    pub id: i64,
}

#[derive(Serialize, Debug)]
pub struct Team {
    pub id: i64,
    pub rank: Option<i32>,
    pub score: i64,
    pub stage: i32,
    pub name: String,
    pub logo_url: Option<String>,
    pub banner_url: Option<String>,
    pub description: String,
    pub creation_date: String,
    pub location: String,
    pub labels: Vec<Label>,
    pub persons: Vec<Person>,
    pub badges: Vec<OwnedBadge>,
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
            stage: 0,
            name: create_team.name,
            logo_url: create_team.logo_url,
            banner_url: create_team.banner_url,
            description: create_team.description,
            creation_date: actix_web::cookie::time::Date::parse(&create_team.creation_date, &format).unwrap(),
            location: create_team.location,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateTeam {
    pub name: String,
    pub description: String,
    pub location: String,
    pub creation_date: String,
    pub banner_url: Option<String>,
    pub logo_url: Option<String>,
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
        let label_ownerships = sqlx::query_as::<sqlx::postgres::Postgres, LabelOwnership>( "SELECT * FROM label_ownerships WHERE team_id = $1")
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
        labels
    }
    pub async fn load_badges(&self, pool: sqlx::postgres::PgPool) -> Vec<OwnedBadge> {
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
        badges
    }
    pub async fn load_persons(& self, pool: sqlx::postgres::PgPool) -> Vec<Person> {
        let raw_persons = sqlx::query_as::<sqlx::postgres::Postgres, RawPerson>("SELECT * FROM persons WHERE team_id = $1")
            .bind(self.id)
            .fetch_all(&pool)
            .await
            .unwrap();
        let persons:Vec<Person> = raw_persons.iter().map(|p| Person::from(&p)).collect();
        persons
    }
}

#[derive(Serialize, Debug)]
pub struct Person {
    pub id: i64,
    pub team_id: i64,
    pub name: String,
    pub career: String,
    pub graduation_date: String,
    pub picture_url: Option<String>,
    pub portafolio_url: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreatePerson {
    pub team_id: i64,
    pub name: String,
    pub career: String,
    pub graduation_date: String,
    pub picture_url: Option<String>,
    pub portafolio_url: Option<String>,
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
    pub id: i64,
    pub name: String,
    pub description: String,
    pub points: i64,
    pub category: Category,
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
pub struct OwnedBadge {
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

#[derive(Deserialize, Debug)]
pub struct EditTeam {
    pub name: Option<String>,
    pub description: Option<String>,
    pub stage: Option<i32>,
    pub creation_date: Option<String>,
    pub logo_url: Option<String>,
    pub banner_url: Option<String>,
    pub location: Option<String>,
}

impl EditTeam {
    pub fn query(self) -> String {
        let mut query = String::from("UPDATE teams SET ");
        if let Some(name) = &self.name { query.push_str(&format!("name = '{}', ", name)); }
        if let Some(description) = &self.description { query.push_str(&format!("description = '{}', ", description)); }
        if let Some(stage) = &self.stage { query.push_str(&format!("stage = {}, ", stage)); }
        if let Some(creation_date) = &self.creation_date { query.push_str(&format!("creation_date = '{}', ", creation_date)); }
        if let Some(location) = &self.location { query.push_str(&format!("location = '{}', ", location)); }
        let null = String::from("null");
        match self.logo_url {
            Some(logo_url) if logo_url == null =>  query.push_str("logo_url = null, "),
            Some(logo_url) =>  query.push_str(&format!("logo_url = '{}', ", logo_url)),
            None => {},
        }
        match self.banner_url {
            Some(banner_url) if banner_url == null =>  query.push_str("banner_url = null, "),
            Some(banner_url) =>  query.push_str(&format!("banner_url = '{}', ", banner_url)),
            None => {},
        }
        query.truncate(query.len() - 2);
        query.push_str(" WHERE id = $1");
        query
    }
}

#[derive(Deserialize, Debug)]
pub struct EditLabel {
    pub name: Option<String>,
}

impl EditLabel {
    pub fn query(self) -> String {
        let mut query = String::from("UPDATE labels SET ");
        if let Some(name) = &self.name { query.push_str(&format!("name = '{}', ", name)); }
        query.truncate(query.len() - 2);
        query.push_str(" WHERE id = $1");
        query
    }
}

#[derive(Deserialize, Debug)]
pub struct EditBadge {
    pub name: Option<String>,
    pub description: Option<String>,
    pub points: Option<i64>,
    pub category: Option<i64>,
}

impl EditBadge {
    pub fn query(self) -> String {
        let mut query = String::from("UPDATE badges SET ");
        if let Some(name) = &self.name { query.push_str(&format!("name = '{}', ", name)); }
        if let Some(description) = &self.description { query.push_str(&format!("description = '{}', ", description)); }
        if let Some(points) = &self.points { query.push_str(&format!("points = {}, ", points)); }
        if let Some(category) = &self.category { query.push_str(&format!("category = {}, ", category)); }
        query.truncate(query.len() - 2);
        query.push_str(" WHERE id = $1");
        query
    }
}

#[derive(Deserialize, Debug)]
pub struct EditCategory {
    pub name: Option<String>,
}

impl EditCategory {
    pub fn query(self) -> String {
        let mut query = String::from("UPDATE badge_categories SET ");
        if let Some(name) = &self.name { query.push_str(&format!("name = '{}', ", name)); }
        query.truncate(query.len() - 2);
        query.push_str(" WHERE id = $1");
        query
    }
}

#[derive(Deserialize, Debug)]
pub struct EditPerson {
    pub name: Option<String>,
    pub team_id: Option<i64>,
    pub career: Option<String>,
    pub graduation_date: Option<String>,
    pub picture_url: Option<String>,
    pub portafolio_url: Option<String>,
}

impl EditPerson {
    pub fn query(self) -> String {
        let mut query = String::from("UPDATE persons SET ");
        if let Some(name) = &self.name { query.push_str(&format!("name = '{}', ", name)); }
        if let Some(team_id) = &self.team_id { query.push_str(&format!("team_id = {}, ", team_id)); }
        if let Some(career) = &self.career { query.push_str(&format!("career = '{}', ", career)); }
        if let Some(graduation_date) = &self.graduation_date { query.push_str(&format!("graduation_date = '{}', ", graduation_date)); }
        let null = String::from("null");
        match self.picture_url {
            Some(picture_url) if picture_url == null =>  query.push_str("picture_url = null, "),
            Some(picture_url) =>  query.push_str(&format!("picture_url = '{}', ", picture_url)),
            None => {},
        }
        match self.portafolio_url {
            Some(portafolio_url) if portafolio_url == null =>  query.push_str("portafolio_url = null, "),
            Some(portafolio_url) =>  query.push_str(&format!("portafolio_url = '{}', ", portafolio_url)),
            None => {},
        }
        query.truncate(query.len() - 2);
        query.push_str(" WHERE id = $1");
        query
    }
}
