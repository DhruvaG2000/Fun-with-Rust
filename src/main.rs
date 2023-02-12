use clap::Parser;
use std::fs::File;
use std::io::{self, Read};
use std::io::BufReader;

#[derive(Parser)]
struct CmdLineArgs {
        pattern: String,
        path: std::path::PathBuf,
}

fn main() {
    let arguments = CmdLineArgs::parse();
    let _ret = calc_n_occurances(arguments);
}

fn calc_n_occurances (arg: CmdLineArgs) -> Result<String, io::Error> {
    let file = File::open(arg.path).expect("File not Found!!");
    let mut buf_read = BufReader::new(file);
    let mut contents = String::new();
    let mut i = 0;
    buf_read.read_to_string(&mut contents)?;
    let lines = contents.split("\n");
    for line in lines {
        if line.contains(&arg.pattern)    {
            eprintln!("{}",line);
            i+=1;
        } else {
            i+=0;
            continue
        }
    }
    if i == 0 {
        eprintln!("Pattern Not Found");
    }
    Ok(contents)
}
