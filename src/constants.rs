pub const DEFAULT_COMPONENT_NAME: &str = "cookie-delete-filter";
pub const DEFAULT_COOKIE_NAMES: [&str; 2] = ["ignore1", "ignore2"];

pub fn default_component_name() -> String {
    String::from(DEFAULT_COMPONENT_NAME)
}

pub fn default_cookie_names() -> Vec<String> {
    DEFAULT_COOKIE_NAMES
        .iter()
        .map(|x| String::from(*x))
        .collect()
}
