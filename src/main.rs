use core::str;
use std::{
    fs::{self, File},
    io::{Read, Seek, SeekFrom},
};
use uuid::Uuid;

fn main() {
    let mut input_file = fs::File::open("./sample/01.mp4").unwrap();

    skip_four_bytes(&mut input_file);

    read_atom(&mut input_file);

    skip_four_bytes(&mut input_file);

    read_atom(&mut input_file);

    skip_four_bytes(&mut input_file);

    read_atom(&mut input_file);

    // let buf = &mut [0; 500];
    // input_file.read_exact(buf).unwrap();
    // println!("{}", buf.iter().map(|n| *n as char).collect::<String>());
    input_file.seek(SeekFrom::Start(0)).unwrap();

    let mut full: Vec<u8> = vec![];
    match input_file.read_to_end(&mut full) {
        Ok(_) => (),
        Err(e) => panic!("Failed to read the entire file: {}", e.to_string()),
    }

    let full_string = full.iter().map(|b| *b as char).collect::<String>();
    let index = full_string.find("moov").unwrap();

    let piece = full[index..index + 1]
        .iter()
        .map(|c| *c as char)
        .collect::<String>();
    println!("{}", piece);
}

fn read_atom(file: &mut fs::File) {
    let box_label = &mut [0; 4];
    file.read_exact(box_label).unwrap();

    match str::from_utf8(box_label).unwrap() {
        "ftyp" => {
            let buf = &mut [0; 4];

            file.read_exact(buf).unwrap();
            let major = String::from_utf8(buf.to_vec()).unwrap();

            file.read_exact(buf).unwrap();
            let minor = String::from_utf8(buf.to_vec()).unwrap();

            file.read_exact(buf).unwrap();
            let compatibility1 = String::from_utf8(buf.to_vec()).unwrap();
            file.read_exact(buf).unwrap();
            let compatibility2 = String::from_utf8(buf.to_vec()).unwrap();

            println!(
                "[fytp]\nmajor = {}\nminor = {}\ncompatible with = {}, {}",
                major, minor, compatibility1, compatibility2
            );
        }
        "uuid" => {
            let buf = &mut [42; 16];

            file.read_exact(buf).unwrap();
            let id = Uuid::from_bytes(*buf).to_string();

            file.read_exact(buf).unwrap();
            let gpac_version = str::from_utf8(&buf[4..]).unwrap();

            println!("[uuid]\n{}\n{}", id, gpac_version);
        }
        "mdat" => {
            let buf = &mut [0; 4];

            skip_four_bytes(file);

            file.read_exact(buf).unwrap();
            let size = i32::from_le_bytes(*buf);
            let size_be = i32::from_be_bytes(*buf);

            println!("[mdat]\nsize = {}(le) or {}(be)", size, size_be)
        }
        unknown => {
            println!("{}", unknown)
        }
    };
}

fn skip_four_bytes(file: &mut File) {
    file.read_exact(&mut [0; 4]).unwrap();
}

fn _skip_padding(file: &mut fs::File) {
    loop {
        let byte = &mut [0 as u8];
        file.read_exact(byte).expect("Failed to read from file");

        if byte[0] != 0 {
            let pos = file
                .seek(SeekFrom::Current(0))
                .expect("Failed to locate cursor position");

            file.seek(SeekFrom::Current(pos as i64 - 1))
                .expect("Failed to set cursor position");

            return;
        }
    }
}
