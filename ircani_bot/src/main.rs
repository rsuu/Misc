use std::{
    io::{BufRead, BufReader, BufWriter, Read, Write},
    net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, TcpStream, ToSocketAddrs},
};

use regex::Regex;

fn main() {
    let irc = "irc.libera.chat:6667";
    let stream = TcpStream::connect(irc).unwrap();
    println!("IRC: {}", irc);

    stream.set_read_timeout(None);
    stream.set_write_timeout(None);

    let mut reader = BufReader::new(stream.try_clone().unwrap());
    let mut writer = BufWriter::new(stream.try_clone().unwrap());

    init_bot(&mut writer);

    let mut is_connected = false;
    let mut is_exit = false;
    let mut response = String::new();

    // match nickname
    // :nick!~nick@user/nick PRIVMSG rustbot :send to #channel
    let re_nickname: Regex = Regex::new(r"^:(?P<nickname>.*)!~.*$").unwrap();

    while !is_exit {
        reader.read_line(&mut response).unwrap();

        if let Some(nickname) = match_nickname(&re_nickname, response.clone().as_str()) {
            response = response.to_lowercase();
            let s = response.as_str();

            if s.is_empty() {
            } else if s.contains("ping") {
                send_msg(&mut writer, nickname, "PING");
            } else if s.contains("hello") {
                send_msg(&mut writer, nickname, "HELLO");
            } else if s.contains("bye") {
                send_msg(&mut writer, nickname, "BYE");

                is_exit = true;

                println!("End of Stream!");
            } else {
                println!("wait for input");
            }
        } else {
        }

        response.clear();

        std::thread::sleep(std::time::Duration::from_millis(200));
    }

    println!("Connection timed out!");
}

// create a new nickname
fn init_bot(wtr: &mut BufWriter<TcpStream>) {
    wtr.write("NICK rustbot\r\n".as_bytes()).unwrap();
    wtr.write("USER rustbot 8 * :rustbot\r\n".as_bytes())
        .unwrap();
    wtr.flush().unwrap();
}

fn match_nickname<'a>(re: &'a Regex, text: &'a str) -> Option<&'a str> {
    re.captures(text.trim_end())
        .and_then(|cap| cap.name("nickname").map(|nickname| nickname.as_str()))
}

fn send_msg<T>(wtr: &mut BufWriter<TcpStream>, user: T, msg: T)
where
    T: AsRef<str> + std::fmt::Display,
{
    let send = format!("PRIVMSG {} :{}\r\n", user, msg);

    wtr.write(send.as_bytes()).unwrap();
    wtr.flush().unwrap();

    // println!("DONE: {}",msg);
}

// REF
// https://github.com/kzzch/rustbot/blob/master/rustdrop.rs
// https://www.reddit.com/r/irc/comments/1gavie/are_there_any_security_risks_to_using_irc/

// Default  | irc.libera.chat
// Europe  | irc.eu.libera.chat
// US & Canada  | irc.us.libera.chat
// Australia and New Zealand  | irc.au.libera.chat
// East Asia  | irc.ea.libera.chat
// IPv4 only  | irc.ipv4.libera.chat
// IPv6 only  | irc.ipv6.libera.chat
//
// Additional ports are available:
// Plain-text  | 6665-6667, 8000-8002
// TLS  | 6697, 7000, 7070
//
