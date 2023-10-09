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


/// This function creates buffer that will help to read file
/// 
/// # Arguments
/// 
/// * `filename` - filename that will be read.
///
#[allow(dead_code)]
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


/// This function initalizes logging. `log` dependancy is used for logging. Specifically info and error ones.
/// 
/// # Arguments
/// 
/// * `verbose` - this parameter is used to determine the logging functionality of the program to be used or not. If True is passed, then logging is on, and if False then logging off.
///
#[allow(dead_code)]
pub fn init_logging(verbose: bool) {

    unsafe {
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
}


/// This function prints summary of alphabet encoding. It will only print characters and values of those which were initialized.
///
#[allow(dead_code)]
pub fn encoding_summary() {
    info!("# Alphabet encoding summary");
    info!("# Coefficients: {:?}", unsafe { LABELS.into_iter().enumerate().filter(|&(_i, v)| v != -1).map(|(i, e)| ((i as u8) as char, e)).collect::<Vec<_>>() });
    info!("# Dictionary bit size: {}", unsafe { DICT_BIT_SIZE} );
}


/// This function initalizes alphabet encoding with default values. Default values are as follows:
/// 
/// A/a=0, T/t=3, G/g=2, C/c=1
/// 
/// Since this algorithm is designed to be used in bioinformatics, characters are selected to be base pairs.
/// 
/// Invalid value is set to -1.
/// 
/// # Arguments
/// 
/// * `verbose` - this parameter is used determine whether to use logging or not.
/// 
#[allow(dead_code)]
pub fn init_coefficients_default(verbose: bool) {

    unsafe {
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

}


/// This function initalizes alphabet encoding with given values in map. 
/// For the characters that value is not provided, it is set to -1. 
/// Also, negatıve values are not accepted.
/// 
/// # Arguments
/// 
/// * `map` - map that contains character and the value that is intended to be encoded.
/// * `verbose` - this parameter is used determine whether to use logging or not.
/// 
#[allow(dead_code)]
pub fn init_coefficients_map(map: HashMap<char, i32>, verbose: bool) {

    unsafe {
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
}


/// This function initalizes alphabet encoding with given filename. 
/// The function opens file and read line by line in order to extract character and 
/// its value. The format of the file should be as follows:
/// 
/// `char` `i32`
/// 
/// For the characters that value is not provided, it is set to -1. 
/// Also, negative values are not accepted.
/// 
/// # Arguments
/// 
/// * `_encoding_file` - filename where encodings are provided.
/// * `verbose` - this parameter is used determine whether to use logging or not.
/// 
#[allow(dead_code)]
pub fn init_coefficients_file(_encoding_file: &str, verbose: bool) {

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