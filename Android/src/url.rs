#[derive(Debug, Clone)]
pub struct Url {
    pub host: String,
    pub path_and_query: String,
}

impl Url {
    // ud url parser
    pub fn parse_url(url: &str) -> Self {
        if url.is_empty() {
            return Url { host: String::new(), path_and_query: String::new() };
        }

        if let Some(proto_end) = url.find("://") {
            let proto = &url[..proto_end + 3]; // http://
            let remainder = &url[proto_end + 3..];
            let path_start = remainder.find('/').unwrap_or(remainder.len());
            let host = format!("{}{}", proto, &remainder[..path_start]); // http://host:port
            let path_and_query = if path_start < remainder.len() {
                remainder[path_start..].to_string() // starts with /
            } else {
                "/".to_string() // fallback if no path
            };

            Url { host, path_and_query }
        } else { // no protocol???????
            let path_start = url.find('/').unwrap_or(url.len());
            let host = url[..path_start].to_string();
            let path_and_query = if path_start < url.len() {
                url[path_start..].to_string()
            } else {
                "/".to_string()
            };

            Url { host, path_and_query }
        }
    }

    // why i love rust frfr
    pub fn should_redirect(host: &str) -> bool {
        let host = host
            .strip_prefix("http://")
            .or_else(|| host.strip_prefix("https://"))
            .unwrap_or(host);

        let host = host.split_once(':').map(|(h, _)| h).unwrap_or(host);
        const EPIC_DOMAINS: [&str; 6] = [
            "game-social.epicgames.com",
            "ol.epicgames.com",
            "ol.epicgames.net",
            "on.epicgames.com",
            "ak.epicgames.com",
            "epicgames.dev",
        ];

        EPIC_DOMAINS.iter().any(|d| host.ends_with(d))
    }

    pub fn create_url(host: &str, path_and_query: &str) -> String {
        format!("{}{}", host, path_and_query)
    }
}