use byteorder::*;
use std::io::Seek;
use std::io::{Cursor, Read};
use std::mem::transmute;

type ByteSlice<'a> = Cursor<&'a [u8]>;
type OggPacket = Vec<u8>;
type OggPackets = Vec<OggPacket>;

const MAX_FRAME_SIZE: usize = 1275; // 255 * 4 + 255

const MAGICK_OGGS: u32 = 1399285583; // "Oggs"
const MAGICK_OPUS_HEAD: u64 = 7233173838382854223; // "OpusHead"
const MAGICK_OPUS_TAGS: u64 = 8315722252305133647; // "OpusTags"

fn main() {
    ogg_parser();
}

pub fn ogg_parser() {
    let mut f = std::fs::File::open("0.opus").unwrap();
    let mut buf = Vec::new();
    f.read_to_end(&mut buf).unwrap();
    let mut rdr = Cursor::new(buf.as_slice());

    let mut pages: Vec<Page> = Vec::new();
    let mut ogg_packets: OggPackets = Vec::new();

    let mut is_last = false;
    while !is_last {
        let page = Page::parse(&mut rdr, &mut ogg_packets);
        is_last = page.is_eos;

        pages.push(page);
    }

    let mut opus_head = &ogg_packets[0];
    let opus_head = OpusHead::parse(&opus_head);
    dbg!(&opus_head);

    let mut comment = &ogg_packets[1];
    let comment = OpusComment::parse(&comment);
    dbg!(&comment);

    let mut opus_stream = Vec::new();

    for f in 2..10 {
        let packet = &ogg_packets[f];

        let opus_packet = OpusPacket::parse(&packet);
        opus_stream.push(opus_packet[0]);
        println!("{:?}", &opus_packet[0..10]);
        dbg!(packet.len());
    }
}

#[derive(Debug, Clone)]
pub struct Page {
    pub magick: u32,
    pub version: u8,
    pub header_type: u8,
    pub granule_position: u64,
    pub serial_number: u32,
    pub page_sequence_number: u32,
    pub crc_checksum: u32,
    pub page_segments: u8,
    pub segment_table: Vec<u8>,
    pub packets: Vec<OggPacket>,

    pub is_pad: bool, //
    pub is_bos: bool, // is first page
    pub is_eos: bool, // is last page
}

#[derive(Debug)]
pub struct OpusHead {
    pub magick: u64,              // 8
    pub version: u8,              // 1
    pub channel_count: u8,        // 1
    pub pre_skip: u16,            // 2
    pub input_smaple_rate: u32,   // 4
    pub ouput_gain: u16,          // 2
    pub mapping_family: u8,       // 1
    pub stream_count: u8,         // 1
    pub coupled_count: u8,        // 1
    pub channel_mapping: Vec<u8>, // 8 * channel_count
}

#[derive(Debug)]
pub struct OpusComment {
    pub magick: u64, // 8
    pub vendor_string_length: u32,
    pub vendor_string: String,
    pub user_comment_list_length: u32,
    pub tags: Vec<String>,
}

impl Page {
    fn parse(rdr: &mut ByteSlice, packets: &mut OggPackets) -> Self {
        let magick = rdr.read_u32::<LittleEndian>().unwrap(); // 4

        if magick == MAGICK_OGGS {
        } else {
            panic!()
        }

        let version = rdr.read_u8().unwrap(); // 1
        let header_type = rdr.read_u8().unwrap(); // 1
        let granule_position = rdr.read_u64::<LittleEndian>().unwrap(); // 8
        let serial_number = rdr.read_u32::<LittleEndian>().unwrap(); // 4
        let page_sequence_number = rdr.read_u32::<LittleEndian>().unwrap(); // 4
        let crc_checksum = rdr.read_u32::<LittleEndian>().unwrap(); // 4
        let page_segments = rdr.read_u8().unwrap(); // 1

        let mut segment_table = vec![0_u8; page_segments as usize];
        rdr.read_exact(&mut segment_table).unwrap();

        let mut sum = 0_u32;
        for segment in segment_table.iter() {
            sum += *segment as u32;

            if *segment < 255 {
                let mut buffer = vec![0_u8; sum as usize];
                rdr.read_exact(&mut buffer);
                packets.push(buffer);

                sum = 0;
            } else {
            }
        }

        let is_pad = header_type & 0b0000_0001_u8 != 0;
        let is_bos = header_type & 0b0000_0010_u8 != 0;
        let is_eos = header_type & 0b0000_0100_u8 != 0;

        Self {
            magick,
            version,
            header_type,
            granule_position,
            serial_number,
            page_sequence_number,
            crc_checksum,
            page_segments,
            segment_table,
            packets: vec![],

            is_pad,
            is_bos,
            is_eos,
        }
    }
}

impl OpusHead {
    fn parse(data: &[u8]) -> Self {
        let mut rdr = Cursor::new(data);

        let magick = rdr.read_u64::<LittleEndian>().unwrap();

        let version = rdr.read_u8().unwrap(); // 1
        let channel_count = rdr.read_u8().unwrap(); // 1
        let pre_skip = rdr.read_u16::<LittleEndian>().unwrap(); // 2
        let input_smaple_rate = rdr.read_u32::<LittleEndian>().unwrap(); // 4
        let ouput_gain = rdr.read_u16::<LittleEndian>().unwrap(); // 2
        let mapping_family = rdr.read_u8().unwrap(); // 1

        // 19
        let stream_count: u8;
        let coupled_count: u8;
        let mut channel_mapping: Vec<u8> = Vec::new();

        if mapping_family == 0 {
            //eprintln!("mapping_family: {}", mapping_family);

            stream_count = 1;
            coupled_count = stream_count - 1;

            if channel_count == 2 {
                channel_mapping = vec![0, 1];
            } else {
                channel_mapping = vec![0, 0];
            }
        } else {
            stream_count = rdr.read_u8().unwrap();
            coupled_count = rdr.read_u8().unwrap();

            channel_mapping = vec![0_u8; 8 * channel_count as usize];
            rdr.read_exact(&mut channel_mapping).unwrap();
        }

        Self {
            magick,
            version,
            channel_count,
            pre_skip,
            input_smaple_rate,
            ouput_gain,
            mapping_family,
            stream_count,
            coupled_count,
            channel_mapping,
        }
    }
}

impl OpusComment {
    fn parse(data: &[u8]) -> Self {
        let mut rdr = Cursor::new(data);

        let magick = rdr.read_u64::<LittleEndian>().unwrap();
        println!("{}", magick);

        //
        let vendor_string_length = rdr.read_u32::<LittleEndian>().unwrap();
        dbg!(vendor_string_length);

        let mut buf: Vec<u8> = vec![0_u8; vendor_string_length as usize];
        rdr.read_exact(&mut buf).unwrap();
        let vendor_string = String::from_utf8(buf).unwrap();

        //
        let user_comment_list_length = rdr.read_u32::<LittleEndian>().unwrap();
        let mut tags = Vec::new();

        // https://www.xiph.org/vorbis/doc/v-comment.html
        for _ in 0..user_comment_list_length {
            let len = rdr.read_u32::<LittleEndian>().unwrap();
            let mut buf: Vec<u8> = vec![0_u8; len as usize];

            rdr.read_exact(buf.as_mut()).unwrap();
            tags.push(String::from_utf8(buf).unwrap());
        }

        Self {
            magick,
            vendor_string_length,
            vendor_string,
            user_comment_list_length,
            tags,
        }
    }
}

#[derive(Debug)]
enum OpusTocConfig {
    CeltOnly = 31,
}

#[derive(Debug)]
enum OpusTocConfigW {
    Fb,
}

#[derive(Debug)]
enum OpusTocConfigF {
    A, // 2.5
    B, // 5
    C, // 10
    D, // 20
}

#[derive(Debug)]
enum OpusTocC {
    Zero,
    One,
    Two,
    Three,
}

#[derive(Debug)]
struct OpusPacket {
    toc: (OpusTocConfig, bool, u8),
    frame: Vec<Vec<u8>>,
}

impl OpusPacket {
    fn parse(data: &[u8]) -> Vec<u8> {
        let mut rdr = Cursor::new(data);
        Self::parse_toc(&mut rdr)
    }

    fn parse_toc(rdr: &mut ByteSlice) -> Vec<u8> {
        //  0
        //  0 1 2 3 4 5 6 7
        // +-+-+-+-+-+-+-+-+
        // | config  |s| c |
        // +-+-+-+-+-+-+-+-+
        let toc = rdr.read_u8().unwrap();
        let config = (toc >> 3) & 31;
        let is_stereo = ((toc >> 2) & 1) == 1;
        let code = toc & 3;
        println!("{:08b}", toc);

        let (config, w, frame_len) = match config {
            28 => (
                OpusTocConfig::CeltOnly,
                OpusTocConfigW::Fb,
                OpusTocConfigF::A,
            ),
            29 => (
                OpusTocConfig::CeltOnly,
                OpusTocConfigW::Fb,
                OpusTocConfigF::B,
            ),
            30 => (
                OpusTocConfig::CeltOnly,
                OpusTocConfigW::Fb,
                OpusTocConfigF::C,
            ),
            31 => (
                OpusTocConfig::CeltOnly,
                OpusTocConfigW::Fb,
                OpusTocConfigF::D,
            ),
            _ => {
                unreachable!()
            }
        };

        // ? VBR
        //  let frame1_len = match rdr.read_u8().unwrap() {
        //      0 => 0,
        //      n @ 1..=251 => n,
        //      n @ 252..=255 => {
        //          let len = rdr.read_u8().unwrap();
        //          (len * 4) + n
        //      }
        //      _ => {
        //          unreachable!()
        //      }
        //  };

        let mut data = vec![toc];
        rdr.read_to_end(&mut data).unwrap();

        // if data.len() > MAX_FRAME_SIZE {
        //     panic!()
        // }

        dbg!(data.len());

        data
    }
}
