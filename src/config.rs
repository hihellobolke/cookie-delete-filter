use crate::constants;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct FilterConfig {
    #[serde(default = "constants::default_component_name")]
    pub component_name: String,
    #[serde(default = "constants::default_cookie_names")]
    pub cookie_names: Vec<String>,
}

impl FilterConfig {
    pub fn default() -> FilterConfig {
        FilterConfig {
            component_name: "".to_string(),
            cookie_names: vec![],
        }
    }
}
