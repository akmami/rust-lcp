use std::collections::HashMap;
use std::cmp;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub static mut COEFFICIENTS: [i32; 128] = [-1; 128];
pub static mut CHARACTERS: [char; 128] = [126 as char; 128];
pub static mut DICT_BIT_SIZE: u32 = 0;
pub static mut ENCODING_INITIALIZED: bool = false;


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub unsafe fn encoding_summary() {
    println!("# Alphabet encoding summary");
    println!("# Coefficients: ");
    for i in 0..128 {
        if COEFFICIENTS[i] != -1 {
            print!("{}: {}, ", (i as u8) as char,  COEFFICIENTS[i]);
        }
    }
    println!();
    println!("# Dictionary bit size: {}", DICT_BIT_SIZE);
}

pub unsafe fn init_coefficients_default(verbose: bool) {
    
    COEFFICIENTS = [-1; 128];
    CHARACTERS = [126 as char; 128];

    // init coefficients A/a=0, T/t=3, G/g=2, C/c=1
    
    COEFFICIENTS['A' as usize] = 0; COEFFICIENTS['a' as usize] = 0;
    COEFFICIENTS['T' as usize] = 3; COEFFICIENTS['t' as usize] = 3;
    COEFFICIENTS['G' as usize] = 2; COEFFICIENTS['g' as usize] = 2;
    COEFFICIENTS['C' as usize] = 1; COEFFICIENTS['c' as usize] = 1;
    
    CHARACTERS[0] = 'A';
    CHARACTERS[1] = 'C';
    CHARACTERS[2] = 'G';
    CHARACTERS[3] = 'T';

    DICT_BIT_SIZE = 2;

    if verbose { 
        encoding_summary(); 
    }

    ENCODING_INITIALIZED = true;
}

pub unsafe fn init_coefficients_map(map: HashMap<char, i32>, verbose: bool) {

    COEFFICIENTS = [-1; 128];
    CHARACTERS = [126 as char; 128];

    // init coefficients A/a=0, T/t=3, G/g=2, C/c=1

    let mut max_value = -1;
    
    for (key, value) in map.into_iter() {
        if value < 0 {
            println!("Invalid value given. key: {}, value: {}", key, value);   
        }
        COEFFICIENTS[key as usize] = value;
        CHARACTERS[value as usize] = key;
        max_value = cmp::max(max_value, value);
    };
    
    let mut bit_count = 0;
    
    while max_value > 0 {
        bit_count += 1;
        max_value = max_value / 2;
    }

    DICT_BIT_SIZE = bit_count;

    if verbose { 
        encoding_summary(); 
    }

    ENCODING_INITIALIZED = true;
}

pub unsafe fn init_coefficients_file(_encoding_file: &str, verbose: bool) {

    COEFFICIENTS = [-1; 128];
    CHARACTERS = [126 as char; 128];

    let mut map: HashMap<char, i32> = HashMap::new();

    if let Ok(lines) = read_lines(_encoding_file) {
        for line in lines {
            if let Ok(ip) = line {
                let splitted: Vec<&str>= ip.split(" ").collect();
                assert_eq!(splitted.len(), 2);
                assert_eq!(splitted[0].len(), 1);
                map.insert(splitted[0].chars().next().expect("string is empty"), splitted[1].parse().unwrap());
            }
        }
    }

    init_coefficients_map(map, verbose);
}