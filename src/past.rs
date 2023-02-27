use serde::Serialize;

#[derive(Serialize, Debug)]
enum Label {
    Web, Edtec, Web3, Videogame, Fintech, Logistics, Marketplace, SAAS, Hardware, IoT,
}

#[derive(Serialize, Debug)]
pub enum BadgeCategory {
    Talk, Beginner, Advanced, NotAssigned
}

#[derive(Serialize, Debug)]
enum BadgeInfo {
    Not(Badge), Id(String), Error(String), NotAssigned,
}

#[derive(Serialize, Debug)]
struct EarnedBadge {
    badge: BadgeInfo,
    acquisition_date: String,
}

#[derive(Serialize, Debug)]
pub struct Badge {
    pub id: String,
    name: String,
    pub category: BadgeCategory,
    description: String,
    points: usize,
}

impl Badge {
    pub fn new() -> Badge {
        Badge {
            id: String::from("0x000"),
            name: String::from("Badge"),
            category: BadgeCategory::NotAssigned,
            description: String::from("Pending description"),
            points: 100,
        }
    }
}

impl EarnedBadge {
    fn new() -> EarnedBadge {
        EarnedBadge {
            badge: BadgeInfo::NotAssigned,
            acquisition_date: String::from("00-00-2000"),
        }
    }
    fn id(id: &str) -> EarnedBadge{
        EarnedBadge {
            badge: BadgeInfo::Id(String::from(id)),
            acquisition_date: String::from("00-00-2000"),
        }
    }
}

#[derive(Serialize, Debug)]
pub struct Team {
    pub id: String,
    pub rank:usize,
    score: usize,
    stage: usize,
    name: String,
    logo_url: String,
    team: Vec<Person>,
    description: String,
    creation_date: String,
    banner_url: String,
    labels: Vec<Label>,
    badges: Vec<EarnedBadge>,
    location: String,
}

#[derive(Serialize, Debug)]
struct Person {
    id: String,
    name: String,
    career: String,
    graduation_date: String,
    portafolio_url: String,
    picture_url: String,
}

impl Team {
    pub fn new() -> Team {
        Team {
            id: String::from("0x0000"),
            rank:0,
            score: 800,
            stage: 1,
            name: String::from("Startup"),
            logo_url: String::from("https://media.licdn.com/dms/image/C4E0BAQH7TefJMYuKCg/company-logo_200_200/0/1519894138847?e=2147483647&v=beta&t=kBCvEozdJQUO00EXcRe8UcDZr32wuItkdUXvM9B3UwQ"),
            team: vec![],
            description: String::from(" Lorem ipsum dolor sit amet, consectetur adipiscing elit. Aliquam in aliquet orci. Mauris vel aliquet quam. Nunc orci orci, fermentum a quam ut, laoreet egestas eros. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Phasellus vestibulum commodo eros at consequat. Nam quis cursus massa. Sed commodo euismod odio, vel egestas massa congue a. Aliquam erat volutpat.

Etiam tincidunt risus quis arcu ullamcorper feugiat. Nullam consequat imperdiet sapien, et accumsan elit tincidunt malesuada. Quisque interdum risus lectus, eget rhoncus ipsum feugiat at. Phasellus mi lacus, dapibus id nisl ac, tristique sodales diam. Praesent convallis condimentum lobortis. Aenean scelerisque scelerisque augue, ut bibendum eros posuere nec. Ut porta, nisi at ullamcorper tempus, erat purus ultrices est, in placerat nibh quam vitae neque. Ut tincidunt dolor quis pharetra imperdiet. Etiam euismod tristique libero non consectetur. Aenean vehicula est in urna dignissim scelerisque. Suspendisse at tincidunt tortor. Nunc dapibus, nibh vel maximus egestas, nibh lacus euismod sapien, tincidunt faucibus ipsum nisl vel massa. Orci varius natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus. Maecenas malesuada quam ac dapibus accumsan. Ut mattis pretium nisi, non molestie sapien placerat eget."),
            creation_date: String::from("21-02-2023"),
            banner_url: String::from("https://i.pinimg.com/originals/a9/97/51/a99751ac6e165b94030b86c62fa00294.jpg"),
            labels: vec![Label::Web3, Label::Videogame],
            badges: vec![EarnedBadge::new(), EarnedBadge::new()],
            location: String::from("Monterrey, Mexico"),
        }
    }
}
