use reqwest_graphql::Client;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt::format};

use crate::contexts::{local::MdProject, saved::SavedState};

// #[derive(Deserialize)]
// pub struct MdProject {
//     pub id: String,
//     pub title: String,
//     // pub context: SavedState,
//     pub createdAt: String,
//     pub updatedAt: String,
// }

#[derive(Deserialize)]
pub struct Data {
    pub getMdProjects: Vec<MdProject>,
}

#[derive(Serialize)]
pub struct Vars {}

pub async fn get_md_projects(auth_token: String) -> Result<Data, Box<dyn std::error::Error>> {
    let endpoint = "http://localhost:4000/graphql";
    let query = r#"
        query GetMdProjects {
            getMdProjects {
                id
                title
                updatedAt
                createdAt
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
