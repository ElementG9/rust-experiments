extern crate reqwest;
extern crate serde_json;
// use tokio::prelude::*;

#[tokio::main]
async fn main() {
    mojang_api::username_to_uuid("ElementG9").await;
    // let uuid = mojang_api::username_to_uuid("12315198579873t564bfsdbvuigasb").await.unwrap();
    // let uuid = uuid.as_array().unwrap();
    // println!("{}", uuid.len())
}
mod mojang_api {
    async fn username_to_uuid_helper(name: &str) -> Result<serde_json::Value, reqwest::Error> {
        let client = reqwest::Client::new();
        let body = vec![name];
        let res = client.post("https://api.mojang.com/profiles/minecraft")
            // .body("the exact body that is sent")
            .json(&body)
            .send()
            .await?
            .json()
            .await?;
        Ok(res)
    }
    pub async fn username_to_uuid(name: &str) -> Result<&str, &str> {
        // let client = reqwest::Client::new();
        // let body = vec![name];
        // let res = client.post("https://api.mojang.com/profiles/minecraft").json(&body).send().await;
        let res = match username_to_uuid_helper(&name).await {
            Ok(val) => val,
            Err(_e) => return Err("request error"),
        };
        let names = match res.as_array() {
            Some(val) => val,
            None => return Err("response is not an array"),
        };
        let first = match names.get(0) {
            Some(val) => val,
            None => return Err("username not found"),
        };
        let out =  match first.get("id") {
            Some(val) => val,
            None => return Err("response does not have id property"),
        };
        Ok(out.as_str().unwrap())
        // let s = s.json().await;
        // let response: serde_json::Value = match s {
        //     Ok(a) => a,
        //     Err(_e) => return Err("request error"),
        // };
        // let response: &Vec<serde_json::Value> = match response.as_array() {
        //     Some(arr) => arr,
        //     None => return Err("response is not an array"),
        // };
        // match response.get(0) {
        //     Some(val) => Ok(*val),
        //     None => Err("username not found"),
        // }
    }
}
// async fn get(url: &str) -> Result<String, reqwest::Error> {
//     let body = reqwest::get(url)
//     .await?
//     .text()
//     .await?;
//     Ok(body)
// }
// async fn post(url: &str) -> Result<String, reqwest::Error> {
//     let client = reqwest::Client::new();
//     let body = vec!["ElementG9"];
//     let res = client.post("https://api.mojang.com/profiles/minecraft")
//         // .body("the exact body that is sent")
//         .json(&body)
//         .send()
//         .await?
//         .text()
//         .await?;
//     Ok(res)
// }
