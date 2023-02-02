pub fn generate_link(key: &str, name: &str) -> String {
    return match key {
        "twitter" => {
            format!("https://twitter.com/{name}")
        },
        "instagram" => {
            format!("https://www.instagram.com/{name}")
        }
        _ => {
            name.to_string()
        },
    }
}