extern crate github_rs;
extern crate serde_json;
use github_rs::client::{Executor, Github};
use serde_json::Value;
use std::env;

pub struct GithubClient {
    client: Github,
}

impl GithubClient {
    pub fn new() -> Self {
        Self {
            client: Github::new("API TOKEN").unwrap(),
        }
    }
    pub fn search(&self) {
        let me = self.client.get().user().execute::<Value>();
        match me {
            Ok((headers, status, json)) => {
                println!("{}", headers);
                println!("{}", status);
                if let Some(json) = json {
                    println!("{}", json);
                }
            }
            Err(e) => println!("{}", e),
        }
        println!("{}", "hello world");
    }
}
