use askama_axum::Template;
use axum::extract::{Query, State};
use serde::{Deserialize, Serialize};
use shared::noc::ExerciseIds;
use shared::Config;

use crate::state::AppState;

#[derive(Deserialize)]
pub struct Params {
    f: Option<String>,
}

#[derive(Serialize, Deserialize, Template)]
#[template(path = "noc.html")]
pub struct Page {
    pub exercise_ids: ExerciseIds,
    pub js_path: String,
}

impl Page {
    fn new(config: Config, params: Params) -> Self {
        Page {
            exercise_ids: Self::parse_exercise_ids(params),
            js_path: config.wasm_js_endpoint.into(),
        }
    }

    fn parse_exercise_ids(params: Params) -> ExerciseIds {
        let ex_str = params.f.unwrap_or_default();
        let filter_ids: Vec<String> = ex_str
            .trim_matches(',')
            .split(",")
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect();

        if filter_ids.is_empty() {
            ExerciseIds::default()
        } else {
            ExerciseIds::from(filter_ids)
        }
    }
}

pub async fn response(State(state): State<AppState>, Query(params): Query<Params>) -> Page {
    Page::new(state.config, params)
}
