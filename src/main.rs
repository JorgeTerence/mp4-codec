use core::str;
use std::{
    fs,
    io::{Read, Seek, SeekFrom},
};

fn main() {
    let mut input_file = fs::File::open("./sample/01.mp4").unwrap();
    input_file.read_exact(&mut [0; 4]).unwrap();
    read_atom(&mut input_file);
    input_file.read_exact(&mut [0; 4]).unwrap();
    let buf: &mut [u8; 10] = &mut [0; 10];
    input_file.read_exact(buf).unwrap();
    println!("{}", buf.iter().map(|n| *n as char).collect::<String>());
    // input_file.seek(pos)
    read_atom(&mut input_file);
}

fn read_atom(file: &mut fs::File) {
    let box_label = &mut [0; 4];
    file.read_exact(box_label).unwrap();

    match str::from_utf8(box_label).unwrap() {
        "ftyp" => {
            let buf: &mut [u8; 4] = &mut [0; 4];

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
        unknown => {
            println!("{}", unknown)
        }
    };
}

fn skip_padding(file: &mut fs::File) {
    let mut pos = file
        .seek(SeekFrom::Current(0))
        .expect("Failed to locate cursor position");

    loop {
        let byte = &mut [0 as u8];
        file.read_exact(byte).expect("Failed to read from file");

        if byte[0] != 0 {
            let pos = file
                .seek(SeekFrom::Current(0))
                .expect("Failed to locate cursor position");

            file.seek(SeekFrom::Current(pos as i64 - 1))
                .expect("Failed to set cursor position");
        }
        pos += 1;
    }
}
