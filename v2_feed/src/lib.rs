pub mod vmess {
    pub mod parse;
}

pub mod trojan {
    pub mod parse;
}

pub mod ssr {
    pub mod parse;
}

pub mod utils {
    use base64::{decode, encode};
    use std::{fs::File, io::Write, time::SystemTime};

    pub fn remove_first_and_last(value: &str) -> Vec<char> {
        let mut chars = value.chars();
        chars.next();
        chars.next_back();
        chars.collect::<Vec<char>>()
    }

    pub fn base64_de(s: &str) -> String {
        String::from_utf8(decode(s).expect("base64_de")).expect("base64_de -> utf8")
    }

    pub fn base64_en(s: &str) -> String {
        encode(s.as_bytes())
    }

    pub fn save_to_file(filepath: &str, filename: &str, text: &str) -> std::io::Result<()> {
        let filename = format!("{}_{}", &filename, get_time_ns()).replace('"', "");
        // remove double quote
        let mut file = File::create(format!("{}/{}.json", filepath, filename.as_str()))?;

        file.write_all(text.as_bytes())
    }

    // random filename
    pub fn get_time_ns() -> u128 {
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("get millis error");
        now.as_nanos()
    }
}
