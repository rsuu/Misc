use fs_extra;
use std::{
    fs::{self, File},
    io::{Read, Write},
    path::{Path, PathBuf},
};
use walkdir;

const NULL: char = '\u{0000}';

pub struct Args<'s> {
    depth: usize,
    path: &'s str,
    force: bool,
}

fn main() {
    let args = Args {
        depth: 100,
        path: "./",
        force: true,
    };

    let mut walk: Vec<(usize, PathBuf)> = Vec::new(); // Vec<(id, path)>
    let mut n = 0;

    for _path in walkdir::WalkDir::new(args.path)
        .max_depth(args.depth)
        .into_iter()
    {
        // Only support for rename filename
        let path = _path.as_ref().expect("").path();
        walk.push((n, path.to_path_buf()));
        n += 1;
    }

    //eprintln!("{:?}", walk);

    let mut buffer = String::new();

    for (id, path) in walk.iter() {
        buffer.push_str(format!("{}{}{}", id, NULL, path.display()).as_str());
        buffer.push('\n');
    }

    //eprintln!("{}", buffer);

    // TODO: tempfile
    let mut file = File::create("./now").expect("");
    file.write_all(buffer.as_bytes()).expect("");

    let editor: String = if let Ok(v) = std::env::var("VISUAL") {
        v
    } else {
        std::env::var("EDITOR").unwrap()
    };

    std::process::Command::new(editor)
        .arg("./now")
        .status()
        .expect("Something went wrong");

    let mut walk_new: Vec<(usize, &str)> = Vec::new(); // Vec<(path, id)>
    let mut buffer = String::new();
    let mut file = File::open("now").unwrap();
    file.read_to_string(&mut buffer).unwrap();

    for _line in buffer.split('\n').into_iter() {
        let line = _line.split(NULL).into_iter().collect::<Vec<&str>>();

        if line[0].is_empty() {
        } else {
            walk_new.push((line[0].parse::<usize>().expect(""), line[1]));
        }

        //eprintln!("{:?}", line);
    }

    //eprintln!("{:?}", walk_new);

    let fs_options = fs_extra::dir::CopyOptions::new();

    let mut n: usize = 0;

    for (_id, _path) in walk.iter() {
        let from = _path.as_path();

        if *_id == walk_new[n].0 {
            let to = Path::new(walk_new[n].1);

            if from != to {
                println!("Rename: {} -> {}", from.display(), to.display());

                if from.is_dir() && to.is_dir() {
                    // move
                    fs_extra::move_items(&vec![from], to, &fs_options).expect("");
                } else {
                    // rename
                    fs::rename(from, to).expect("");
                }
            } else {
                // do nothing
            }

            n += 1;
        } else {
            println!("    Delete: {} ", from.display());

            if from.is_file() {
                fs::remove_file(from).expect("");
            } else if from.is_dir() {
                if args.force {
                    fs::remove_dir_all(from).expect("");
                } else {
                    fs::remove_dir(from).expect("");
                }
            }
        }
    }
}
