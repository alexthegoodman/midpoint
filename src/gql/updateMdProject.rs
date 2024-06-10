use reqwest_graphql::Client;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt::format};

use crate::contexts::local::LocalState;

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
    projectId: String,
    title: String,
    context: String,
}

pub async fn update_md_project(
    auth_token: String,
    projectId: String,
    title: String,
    context: &LocalState,
) -> Result<Data, Box<dyn std::error::Error>> {
    let endpoint = "http://localhost:4000/graphql";
    let query = r#"
        mutation UpdateMdProject($projectId: String!, $title: String, $context: String) {
            updateMdProject(projectId: $projectId, title: $title, context: $context) {
                id
            }
        }
   "#;

    let auth_header = "Bearer ".to_owned() + &auth_token.clone();
    let auth_header_str: &str = &auth_header[..];

    let mut headers = HashMap::new();
    headers.insert("Authorization", auth_header_str);

    let client = Client::new_with_headers(endpoint, headers);

    let context = serde_json::to_string(&*context).expect("Failed to serialize");

    let vars = Vars {
        projectId,
        title,
        context,
    };
    let data = client
        .query_with_vars::<Data, Vars>(query, vars)
        .await
        .unwrap();

    // println!("Id: {}, Name: {}", data.user.id, data.user.name);

    Ok(data)
}
