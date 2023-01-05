//
// Copyright (c) 2018 est31 <MTest31@outlook.com>
// and contributors. All rights reserved.
// Licensed under MIT license, or Apache 2 license,
// at your option. Please see the LICENSE file
// attached to this source distribution for details.

extern crate ogg;

use ogg::writing::PacketWriteEndInfo;
use ogg::{PacketReader, PacketWriter};
use std::env;
use std::fs::File;

fn main() {
    match run() {
        Ok(_) => (),
        Err(err) => println!("Error: {}", err),
    }
}

fn run() -> Result<(), std::io::Error> {
    let input_path = env::args()
        .nth(1)
        .expect("No arg for input path found. Please specify a file to open.");
    println!("Opening file: {}", input_path);
    let mut f_i = File::open(input_path)?;
    let mut pck_rdr = PacketReader::new(&mut f_i);

    pck_rdr.delete_unread_packets();

    for f in 0..10 {
        let r = pck_rdr.read_packet().unwrap();

        match r {
            Some(pck) => {
                let inf = if pck.last_in_stream() {
                    PacketWriteEndInfo::EndStream
                } else if pck.last_in_page() {
                    PacketWriteEndInfo::EndPage
                } else {
                    PacketWriteEndInfo::NormalPacket
                };

                println!("{:?}", &pck.data.len());
                println!("{:?}", &pck.data[0..10]);
            }
            // End of stream
            None => break,
        }
    }
    Ok(())
}
