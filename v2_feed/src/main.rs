// curl xxx | vmess_parse -
// curl http | base64 -d | cut vemss.* | base64 -d | jq | json_parse

use std::io::{self, prelude::*, Write};
use v2_feed::{trojan, utils::*, vmess};

fn main() {
    let args: Vec<_> = std::env::args().collect();

    let mut link = String::new();
    if &args[1] == "-" {
        // 0. from stdin
        io::stdin().read_to_string(&mut link).unwrap();
    } else {
        link = args[1].clone();
    }

    let output_path = args[2].clone();

    let (mut vmess_config, mut trojan_config, mut ssr_config) =
        (String::new(), String::new(), String::new());

    let mut f = std::fs::File::open(args[3].as_str()).unwrap();
    f.read_to_string(&mut vmess_config).unwrap();

    let mut f = std::fs::File::open(args[4].as_str()).unwrap();
    f.read_to_string(&mut trojan_config).unwrap();

    let mut f = std::fs::File::open(args[5].as_str()).unwrap();
    f.read_to_string(&mut ssr_config).unwrap();

    // 1. base64 -d
    base64_de(&link);
    let link = base64_de(&link);

    // 2. split '\n' and return ["link1","link2",...]
    let links = link.split('\n').collect::<Vec<&str>>();

    for f in links.iter() {
        //eprintln!("f: {}", f);

        let parse: Option<(String, String)> = if f.starts_with("vmess://") {
            let body = vmess::parse::parse_vmess(f);
            Some(vmess::parse::vmess_to_json(&vmess_config, &body))
        } else if f.starts_with("trojan://") {
            let body = trojan::parse::parse_trojan(f);
            Some(trojan::parse::trojan_to_json(&trojan_config, &body))
        } else if f.is_empty() {
            None
        } else {
            None
        };

        if parse.is_some() {
            let (filename, output) = parse.unwrap();

            if &output_path == "-" {
                io::stdout().write_all(output.as_bytes()).expect("stdout");
            } else {
                save_to_file(&output_path, &filename, &output).unwrap()
            }
        }
    }
}
