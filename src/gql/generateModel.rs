use reqwest_graphql::Client;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt::format};

#[derive(Deserialize)]
pub struct Data {
    pub generateModel: String,
}

#[derive(Serialize)]
pub struct Vars {
    imagePath: String,
}

pub async fn generate_model(
    auth_token: String,
    imagePath: String,
) -> Result<Data, Box<dyn std::error::Error>> {
    let endpoint = "http://localhost:4000/graphql";
    let query = r#"
        mutation GenerateModel($imagePath: String!) {
            generateModel(imagePath: $imagePath)
        }
   "#;

    let auth_header = "Bearer ".to_owned() + &auth_token.clone();
    let auth_header_str: &str = &auth_header[..];

    let mut headers = HashMap::new();
    headers.insert("Authorization", auth_header_str);

    let client = Client::new_with_headers(endpoint, headers);

    let vars = Vars { imagePath };
    let data = client
        .query_with_vars::<Data, Vars>(query, vars)
        .await
        .unwrap();

    Ok(data)
}
