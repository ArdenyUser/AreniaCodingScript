use std::fs;
use std::File;

fn main() {
    let data = fs::read_to_string("config.set").expect("Unable to read file!");
    let fsread = data;
    let filedat = fs::read_to_string(fsread).expect("Unable to read file!");
    let mut stringset;
    let mut cmda;
    let mut auga;
    let mut augb;
    let mut augset;
    let mut setting;
    setting = "B";
    for line in filedat.lines() {
        for c in line.expect("lines failed").chars() {
            if c != " " && setting != "A" {
                stringset = [stringset, c].expect(".CONCAT of file failed...");
                cmda = stringset;
            } else {
                setting = "A";
                if c != ";" && setting == "A" {
                    stringset = " ";
                    augset = [stringset, c].expect(".CONCAT of file failed...");
                }
            }
        }
    }
}
