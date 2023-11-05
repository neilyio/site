#[derive(Clone)]
pub struct Config {
    pub wasm_target_dir_path: String,
    pub wasm_dir_endpoint: String,
    pub wasm_js_endpoint: String,
    pub resume_toml_path: String,
}

impl<'a> Default for Config {
    fn default() -> Self {
        Self {
            wasm_target_dir_path: "target/wasm-build".into(),
            wasm_dir_endpoint: "/wasm".into(),
            wasm_js_endpoint: "/wasm/wasm.js".into(),

            resume_toml_path: "server/data/data.toml".into(),
        }
    }
}
