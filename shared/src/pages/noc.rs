use askama::Template;
use serde::{Deserialize, Serialize};

pub const EXERCISE_IDS: &[&str] = &["0.1", "0.2", "0.3", "0.4", "0.5", "0.6", "1.1"];

#[derive(Deserialize, Serialize, Template)]
#[template(path = "noc.html")]
pub struct Data {
    pub filter_ids: Vec<String>,
}
