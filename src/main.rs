use std::fs::File;
use std::io::{BufRead, Write, BufReader};
use gzp::{deflate::Gzip, ZBuilder};
use flate2::{read::GzDecoder};
use clap::{App, Arg};
use std::collections::HashMap;

fn main() {
    let matches = App::new("Simplify Quality Scores for FASTQ")
        .version("0.1.0")
        .author("Jiguang Peng")
        .arg(Arg::with_name("input")
             .short('i')
             .long("input")
             .value_name("FILE")
             .required(true)
             .help("Input FASTQ file (gzip compressed)")
             .takes_value(true))
        .arg(Arg::with_name("output")
             .short('o')
             .long("output")
             .value_name("FILE")
             .required(true)
             .help("Output FASTQ file (gzip compressed)")
             .takes_value(true))
        .get_matches();

    let input = matches.value_of("input").unwrap();
    let output = matches.value_of("output").unwrap();
    let input_file = File::open(input).expect("Could not open input file");
    let output_file = File::create(output).expect("Could not create output file");
    let input_reader = BufReader::new(GzDecoder::new(input_file));
    let mut parz = ZBuilder::<Gzip, _>::new().num_threads(0).from_writer(output_file);

    let mut score_map = HashMap::new();
    let offset = 33u8;
    for i in 0..3 {score_map.insert((i + offset) as char, (0 + offset) as char);}
    for i in 3..20 {score_map.insert((i + offset) as char, (11 + offset) as char);}
    for i in 20..30 {score_map.insert((i + offset) as char, (23 + offset) as char);}
    for i in 30..50 {score_map.insert((i + offset) as char, (37 + offset) as char);}

    for (num,line) in input_reader.lines().enumerate() {
        let mut line = line.unwrap();
        if (num+1) % 4 == 0 {
            line = line.chars().map(|c| score_map.get(&c).unwrap()).collect();
        }
        parz.write_all(line.as_bytes()).unwrap();
        parz.write_all(b"\n").unwrap();
    }
}