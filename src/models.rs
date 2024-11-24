use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct Response {
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    id: u32,
    url: String,
    name: String,
    email: String,
}

pub fn users_list() -> Vec<User> {
    vec![
        User {
            id: 1,
            url: "/users/1".to_string(),
            name: "Thor".to_string(),
            email: "thor@asguard.realm".to_string(),
        },
        User {
            id: 2,
            url: "/users/2".to_string(),
            name: "Loki".to_string(),
            email: "loki@asguard.realm".to_string(),
        },
        User {
            id: 3,
            url: "/users/3".to_string(),
            name: "Iron Man".to_string(),
            email: "iron.man@stark.org".to_string(),
        },
        User {
            id: 4,
            url: "/users/4".to_string(),
            name: "Wanda".to_string(),
            email: "wanda@multi.verse".to_string(),
        },
        User {
            id: 5,
            url: "/users/5".to_string(),
            name: "Dr. Strange".to_string(),
            email: "strange@multiverse.madness".to_string(),
        },
    ]
}
