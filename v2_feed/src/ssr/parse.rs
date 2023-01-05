use crate::utils::base64_de;

pub struct Ssr {
    pub ip: String,
    pub port: String,
    pub protocol: String,
    pub method: String,
    pub obfs: String,
    pub password: String,
}

pub fn parse(link: &str) -> Ssr {
    // ssr://server:port:protocol:method:obfs:password_base64/?params_base64
    let mut link = link.replace("ssr://", "");
    link = link.replace("/?", ":");

    let link = link.split(':').collect::<Vec<&str>>();

    Ssr {
        ip: link[0].to_string(),
        port: link[1].to_string(),
        protocol: link[2].to_string(),
        method: link[4].to_string(),
        obfs: link[4].to_string(),
        password: base64_de(&link[5]),
    }
}

pub fn ssr_to_json(config: &str, body: Ssr) -> (String, String) {
    let json = config
        .replace("$port", body.port.as_str())
        .replace("$address", body.ip.as_str())
        .replace("$name", format!("\"{}\"", trojan.peer).as_str())
        .replace("$password", body.password.as_str());

    (host.to_string(), json)
}
