use reqwest_graphql::Client;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt::format};

#[derive(Deserialize)]
pub struct MdProject {
    pub id: String,
    // pub title: String,
    // pub createdAt: String,
    // pub updatedAt: String,
}

#[derive(Deserialize)]
pub struct Data {
    pub createMdProject: MdProject,
}

// #[derive(Deserialize)]
// pub struct User {
//     id: String,
//     name: String,
// }

#[derive(Serialize)]
pub struct Vars {
    // id: u32,
}

pub async fn create_md_project(auth_token: String) -> Result<Data, Box<dyn std::error::Error>> {
    let endpoint = "http://localhost:4000/graphql";
    let query = r#"
        mutation CreateMdProject {
            createMdProject {
                id
            }
        }
   "#;

    let auth_header = "Bearer ".to_owned() + &auth_token.clone();
    let auth_header_str: &str = &auth_header[..];

    let mut headers = HashMap::new();
    headers.insert("Authorization", auth_header_str);

    let client = Client::new_with_headers(endpoint, headers);

    let vars = Vars {};
    let data = client
        .query_with_vars::<Data, Vars>(query, vars)
        .await
        .unwrap();

    // println!("Id: {}, Name: {}", data.user.id, data.user.name);

    Ok(data)
}
