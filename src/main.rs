#[macro_use]
extern crate serde_gelf;
extern crate serde_json;

fn main() {
    println!("Hello, world!");
    //read environment variables
    //  to find syslog folder?
    //  authentication vars
    //load required vars from config file
    //find list of syslog files
    // foreach syslog file
    //  decompress from gzip format
    //  pushSyslog(file)

    //pushSyslog(file) 
    //  read file line by line
    //  parse line for highlighted fields
    //  build gelf packet 
    //  push to graylog
    let rec = gelf_record!("hello");
    println!("{}", serde_json::to_string_pretty(&rec).unwrap());
}
