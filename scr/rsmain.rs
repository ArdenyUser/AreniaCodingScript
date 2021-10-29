use std::fs;
use std::File;
use std::io::BufReader;

fn main() {
    let data = fs::read_to_string("config.set").expect("Unable to read file!");
    let fsread = data;
    let mut f = BufReader::new(File::open(fsread).expect("open failed"));
    let mut stringset;
    let mut cmda;
    let mut auga;
    let mut augb;
    let mut augset;
    for line in f.lines() {
        for c in line.expect("lines failed").chars() {
            if c != "." {
                stringset = concat!(stringset, c).expect(".CONCAT of file failed...");
                cmda = stringset;
            } else {
                if c != "." {
                    augset = concat!(stringset, c).expect(".CONCAT of aug failed");
                }
            }
        }
    }
}
