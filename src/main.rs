#[macro_use]
extern crate serde_gelf;
extern crate serde_json;
use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::prelude::*;
use clap::{App,Arg};

fn main() {
    let args = App::new("log-spit")
        .version("0.1")
        .about("Spit a syslog file into graylog")
        .arg(Arg::with_name("File")
        .help("Syslog file to read")
        .takes_value(true)
        .required(false))
        .get_matches();
    //load required vars from config file
    
    //read cli input
    let input = args.value_of("File").unwrap_or("-");
    //handle stdin
    if input == "-" {
        let stdin = io::stdin();
        let reader = stdin.lock();
        push_syslog(reader);
    }else{ //load from filesystem
        let f = File::open(input).unwrap();
        let reader = BufReader::new(f);
        push_syslog(reader);
    }
}
///take syslog file wrapped in a reader type and push to graylog 
fn push_syslog<T: BufRead + Sized>(reader: T){
    for line_ in reader.lines(){
        let line = line_.unwrap();
        //  parse line for highlighted fields
        //read timestamp hostname program[pid] message
        //full_message -> line
        //  build gelf packet 
        //println!("{} ({} bytes long)", line, line.len());
        let rec = gelf_record!("{}",line);
        println!("{}", serde_json::to_string_pretty(&rec).unwrap());
        //  push to graylog
    }
}