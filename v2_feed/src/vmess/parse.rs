use crate::utils::*;
use tinyjson::JsonValue;

#[derive(Debug)]
pub struct Vmess<'a> {
    pub add: &'a str,
    pub host: &'a str,
    pub id: &'a str,
    pub net: &'a str,
    pub port: u32,
    pub aid: u32,
    pub protocol: &'a str,
    pub path: &'a str,
}

pub fn parse_vmess(s: &str) -> String {
    // e.g. vmess://xxxyyyzzz
    //   -> xxxyyyzzz
    base64_de(s.split("vmess://").collect::<Vec<_>>()[1])
}

pub fn vmess_to_json(config: &str, body: &str) -> (String, String) {
    let json: JsonValue = body.parse().expect("1"); // parse to json

    let mut port: u32 = 443;
    if let Some(p) = json["port"].get::<f64>() {
        port = *p as u32;
    } else if let Some(p) = json["port"].get::<String>() {
        // convert String to u32
        // remove "
        // e.g.
        // port: "443" -> port: 443
        port = p.replace('"', "").parse::<u32>().expect("");
    }

    let mut aid: u32 = 0;
    if let Some(p) = json["aid"].get::<f64>() {
        aid = *p as u32;
    } else {
    }

    let mut protocol = "vmess";
    if body.contains("protocol") {
        if let Some(p) = json["protocol"].get::<String>() {
            protocol = p.as_str();
        } else {
        }
    } else {
    }

    let mut path = "/";
    if body.contains("path") {
        if let Some(p) = json["path"].get::<String>() {
            path = p.as_str();
        } else {
        }
    } else {
    }

    let vmess = Vmess {
        add: (json["add"].get::<String>().expect("")),
        host: (json["host"].get::<String>().expect("")),
        id: (json["id"].get::<String>().expect("")),
        net: (json["net"].get::<String>().expect("")),
        protocol,
        port,
        aid,
        path,
    };

    let json = config
        .replace("$ip", format!("\"{}\"", vmess.host).as_str())
        .replace("$port", format!("{}", vmess.port).as_str())
        .replace("$aid", format!("{}", vmess.aid).as_str())
        .replace("$address", format!("\"{}\"", vmess.add).as_str())
        .replace("$users_id", format!("\"{}\"", vmess.id).as_str())
        .replace("$host", format!("\"{}\"", vmess.host).as_str())
        .replace("$network", format!("\"{}\"", vmess.net).as_str())
        .replace("$protocol", format!("\"{}\"", vmess.protocol).as_str())
        .replace("$path", format!("\"{}\"", vmess.path).as_str());

    (vmess.add.to_string(), json)
}
