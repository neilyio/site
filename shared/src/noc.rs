use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;

#[derive(Deserialize, Serialize)]
pub struct ExerciseIds(pub Vec<String>);

impl Default for ExerciseIds {
    fn default() -> Self {
        ExerciseIds(
            vec!["0.1", "0.2", "0.3", "0.4", "0.5", "0.6", "1.1"]
                .into_iter()
                .map(|s| s.to_string())
                .collect(),
        )
    }
}

impl From<Vec<String>> for ExerciseIds {
    fn from(value: Vec<String>) -> Self {
        Self(value)
    }
}

impl TryFrom<JsValue> for ExerciseIds {
    type Error = serde_wasm_bindgen::Error;

    fn try_from(value: JsValue) -> Result<Self, Self::Error> {
        serde_wasm_bindgen::from_value(value).map(|filters| Self(filters))
    }
}

impl<'a> IntoIterator for &'a ExerciseIds {
    type Item = &'a String;
    type IntoIter = std::slice::Iter<'a, String>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl IntoIterator for ExerciseIds {
    type Item = String;
    type IntoIter = std::vec::IntoIter<String>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
