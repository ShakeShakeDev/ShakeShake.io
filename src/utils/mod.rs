pub fn generate_link(key: &str, name: &str) -> (String, String) {
    return match key {
        "twitter" => {
            (name.to_string(), format!("https://twitter.com/{name}"))
        },
        "instagram" => {
            (name.to_string(), format!("https://www.instagram.com/{name}"))
        }
        _ => {
            (key.to_string(), name.to_string())
        },
    }
}