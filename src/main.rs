use std::{fs, io::Read, os::unix::fs::FileExt};

fn main() {
    let mut input_file = fs::File::open("./sample/01.mp4").unwrap();

    let buf: &mut [u8; 40] = &mut [0; 40];
    match input_file.read_exact(buf) {
        Ok(_) => println!("Read first label"),
        Err(err) => println!("{}", err),
    };
    let out_str: String = buf.iter().map(|n| *n as char).collect();
    println!("bytes = {:?}", buf);
    println!("label = {}", out_str);
}

fn read_box(file: fs::File, offset: u64) -> [u8; 4] {
    let box_label = &mut [0; 4];
    match file.read_exact_at(box_label, offset) {
        Ok(_) => (),
        Err(err) => panic!("Error reading bytes: {}", err),
    }

    let box_label: String = box_label.map(|n| n as char).iter().collect();
    match box_label.as_str() {
        "ftyp" => {
            let size: i32;
            let atom_type: u32;
            let brand: u32;
            let minor_version: [u8; 4];
            // let compatible: [u32; size - 128];
        },
        _ => todo!()
    };
}
