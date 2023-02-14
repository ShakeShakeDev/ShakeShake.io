pub fn generate_link(key: &str, name: &str) -> (String, String) {
    let (mut name, link) = match key {
        "twitter" => {
            (name.to_string(), format!("https://twitter.com/{name}"))
        },
        "instagram" => {
            (name.to_string(), format!("https://www.instagram.com/{name}"))
        }
        "email" => {
            (name.to_string(), format!("mailto:{name}"))
        }
        "tiktok" => {
            (name.to_string(), format!("https://www.tiktok.com/@{name}"))
        }
        "snapchat" => {
            (name.to_string(), format!("https://www.snapchat.com/add/{name}"))
        }
        _ => {
            (key.to_string(), name.to_string())
        },
    };

    if key != "email" {
        name = format!("@{name}");
    }

    return (name, link);
}