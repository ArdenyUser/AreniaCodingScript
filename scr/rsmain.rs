use std::fs;
use std::File;
use std::io::BufReader;
use std::io::BufRead;

fn main() {
    let data = fs::read_to_string("config.set").expect("Unable to read file!");
    let fsread = data;
    let mut f = BufReader::new(File::open(fsread).expect("open failed"));
    let mut stringset;
    let mut cmda;
    let mut auga;
    let mut augb;
    let mut augset;
    let mut setting;
    for line in f.lines() {
        for c in line.expect("lines failed").chars() {
            if c != " " && setting != "A" {
                stringset = [stringset, c].join().expect(".CONCAT of file failed...");
                cmda = stringset;
            } else {
                setting = "A";
                if c != ";" {
                    stringset
                    augset = [stringset, c].join().expect(".CONCAT of file failed...");
                }
            }
        }
    }
}
