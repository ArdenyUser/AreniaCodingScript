use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::prelude::rust_2021::String as String;


pub fn main() {
    let data = fs::read_to_string("config.set");
    let fsread = data;
    let mut stringset:String;
    let mut stringset = String::new();
    let mut auga:String;
    let mut augb:String;
    let mut setting;
    let mut stringsetaug:String;
    let mut stringsetaug = String::new();
    setting = "B";
    let f = BufReader::new(File::open("main.fs").expect("open failed"));
    for line in f.lines() {
        for c in line.expect("lines failed").chars() {
	    stringset.push(c);
            if c != ' ' {
				
            } else {
                setting = "A";
                if c != ';' {
			if setting != "A" {
				stringsetaug.push(c);
		        }
                }
            }
        }
    }
}
