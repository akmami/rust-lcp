use std::collections::HashMap;
use std::cmp;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use crate::statics::LABELS;
use crate::statics::CHARACTERS;
use crate::statics::DICT_BIT_SIZE;
use crate::statics::ENCODING_INIT;
use crate::statics::LOG_INIT;
use std::env;
use std::process;
use log::{info, error};


#[allow(dead_code)]
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


#[allow(dead_code)]
pub unsafe fn init_logging(verbose: bool) {

    if LOG_INIT == true {
        return;
    }

    if verbose == true {
        env::set_var("RUST_LOG", "main");
        env_logger::init();
    } else {
        env::remove_var("RUST_LOG");
    }
    LOG_INIT = true;
}


#[allow(dead_code)]
pub unsafe fn encoding_summary() {
    info!("# Alphabet encoding summary");
    info!("# Coefficients: {:?}", LABELS.into_iter().enumerate().filter(|&(_i, v)| v != -1).map(|(i, e)| ((i as u8) as char, e)).collect::<Vec<_>>());
    info!("# Dictionary bit size: {}", DICT_BIT_SIZE);
}


#[allow(dead_code)]
pub unsafe fn init_coefficients_default(verbose: bool) {

    init_logging(verbose);

    LABELS = [-1; 128];
    CHARACTERS = [126 as char; 128];

    // init coefficients A/a=0, T/t=3, G/g=2, C/c=1
    
    LABELS['A' as usize] = 0; LABELS['a' as usize] = 0;
    LABELS['T' as usize] = 3; LABELS['t' as usize] = 3;
    LABELS['G' as usize] = 2; LABELS['g' as usize] = 2;
    LABELS['C' as usize] = 1; LABELS['c' as usize] = 1;
    
    CHARACTERS[0] = 'A';
    CHARACTERS[1] = 'C';
    CHARACTERS[2] = 'G';
    CHARACTERS[3] = 'T';

    DICT_BIT_SIZE = 2;

    if DICT_BIT_SIZE > 6 {
        error!("Dictionary bit size is : {}. This cannot be greater than 6. Please provide labels with values less then 64.", DICT_BIT_SIZE);
        process::exit(1);
    }

    encoding_summary(); 

    ENCODING_INIT = true;
}


#[allow(dead_code)]
pub unsafe fn init_coefficients_map(map: HashMap<char, i32>, verbose: bool) {

    init_logging(verbose);

    LABELS = [-1; 128];
    CHARACTERS = [126 as char; 128];

    // init coefficients A/a=0, T/t=3, G/g=2, C/c=1

    let mut max_value = 0;
    
    for (key, value) in map.into_iter() {

        if value < 0 {
            error!("Invalid value ({}) provided for {}", value, key);
            process::exit(1);
        }
        LABELS[key as usize] = value;
        CHARACTERS[value as usize] = key;
        max_value = cmp::max(max_value, value);
    };
    
    let mut bit_count = 0;
    
    while max_value > 0 {
        bit_count += 1;
        max_value = max_value / 2;
    }

    DICT_BIT_SIZE = bit_count;

    if DICT_BIT_SIZE > 6 {
        error!("Dictionary bit size is : {}. This cannot be >6. Please provide labels with smaller values.", DICT_BIT_SIZE);
        process::exit(1);
    }

    encoding_summary(); 

    ENCODING_INIT = true;
}


#[allow(dead_code)]
pub unsafe fn init_coefficients_file(_encoding_file: &str, verbose: bool) {

    LABELS = [-1; 128];
    CHARACTERS = [126 as char; 128];

    let mut map: HashMap<char, i32> = HashMap::new();

    if let Ok(lines) = read_lines(_encoding_file) {
        for line in lines {
            if let Ok(ip) = line {
                let splitted: Vec<&str>= ip.split(" ").collect();
                assert_eq!(splitted.len(), 2);
                assert_eq!(splitted[0].len(), 1);
                map.insert(splitted[0].chars().next().expect("string is empty"), splitted[1].parse::<i32>().unwrap());
            }
        }
    }

    init_coefficients_map(map, verbose);
}