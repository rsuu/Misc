#[derive(Debug)]
pub struct Trojan<'a> {
    pub password: &'a str,
    pub host: &'a str,
    pub port: &'a str,
    pub allow_insecure: &'a str,
    pub peer: &'a str,
    pub sni: &'a str,
    pub remark: &'a str,
}

pub fn parse_trojan(s: &str) -> String {
    s.split("trojan://").collect::<Vec<_>>()[1].to_string()
}

pub fn trojan_to_json(config: &str, body: &str) -> (String, String) {
    // trojan://[password]@[host]:[port]?wss=[bool]&mux=[bool]&peer=[server_name]#[remark (url encoded)]

    let body = body.split('@').collect::<Vec<&str>>();
    let password = body[0];
    let mut body: &str = body[1];

    let host = body.split(':').collect::<Vec<&str>>()[0];
    body = body.split(':').collect::<Vec<&str>>()[1];

    let port = body.split('?').collect::<Vec<&str>>()[0];
    body = body.split('?').collect::<Vec<&str>>()[1];

    let allow_insecure = body.split('&').collect::<Vec<&str>>()[0]
        .split('=')
        .collect::<Vec<&str>>()[1];
    let peer = body.split("&peer=").collect::<Vec<&str>>()[1]
        .split('&')
        .collect::<Vec<&str>>()[0];
    let sni = body.split("&sni=").collect::<Vec<&str>>()[1]
        .split('#')
        .collect::<Vec<&str>>()[0];
    let remark = body.split('&').collect::<Vec<&str>>()[2]
        .split('#')
        .collect::<Vec<&str>>()[1]
        .trim_end();

    let trojan = Trojan {
        password,
        host,
        port,
        allow_insecure,
        peer,
        sni,
        remark,
    };

    let json = config
        .replace("$port", trojan.port.to_string().as_str())
        .replace("$address", format!("\"{}\"", trojan.host).as_str())
        .replace("$name", format!("\"{}\"", trojan.peer).as_str())
        .replace("$password", format!("\"{}\"", trojan.password).as_str());

    (host.to_string(), json)
}
