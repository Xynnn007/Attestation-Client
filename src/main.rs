//! Attestation-Client for Guanfu 

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, value_parser)]
    rvps_addr: String,

    #[clap(short, long, value_parser)]
    as_addr:String,
}

fn main() {
    let args = Args::parse();

    println!("rvps addr: {}", args.rvps_addr);
    println!("as addr: {}", args.as_addr);
    
    // get evidences
    
    // get reference values

    // compare
    
}

