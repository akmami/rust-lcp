use crate::encoding::init_coefficients_default;
use crate::encoding::init_coefficients_map;
use crate::encoding::init_coefficients_file;
use crate::encoding::COEFFICIENTS;
use crate::encoding::CHARACTERS;
use crate::encoding::DICT_BIT_SIZE;
use std::collections::HashMap;
use crate::core::Core;
//use std::sync::Mutex;

//static mtx: Mutex<i32>= Mutex::new(0);


//------------------------------------------------------------------------------------
//------------------------------------------------------------------------------------
// TESTS FOR ENCODING
//------------------------------------------------------------------------------------
//------------------------------------------------------------------------------------
#[test]
fn test_encoding_default() {
    //let guard = mtx.lock().unwrap();
    unsafe {
        let verbose = true;
        init_coefficients_default(verbose);
        
        assert_eq!(COEFFICIENTS['A' as usize], 0); 
        assert_eq!(COEFFICIENTS['a' as usize], 0);
        assert_eq!(COEFFICIENTS['T' as usize], 3); 
        assert_eq!(COEFFICIENTS['t' as usize], 3);
        assert_eq!(COEFFICIENTS['G' as usize], 2); 
        assert_eq!(COEFFICIENTS['g' as usize], 2);
        assert_eq!(COEFFICIENTS['C' as usize], 1); 
        assert_eq!(COEFFICIENTS['c' as usize], 1);
        
        assert_eq!(CHARACTERS[0], 'A');
        assert_eq!(CHARACTERS[1], 'C');
        assert_eq!(CHARACTERS[2], 'G');
        assert_eq!(CHARACTERS[3], 'T');

        assert_eq!(DICT_BIT_SIZE, 2);
    }
    //drop(guard);
}


#[test]
fn test_encoding_map() {
    //let guard = mtx.lock().unwrap();
    unsafe {
        let verbose = true;
        let map = HashMap::from([
            ('a', 3),
            ('c', 0),
            ('t', 1),
            ('g', 2)
            ]);
        init_coefficients_map(map, verbose);
        
        assert_eq!(COEFFICIENTS['a' as usize], 3);
        assert_eq!(COEFFICIENTS['t' as usize], 1);
        assert_eq!(COEFFICIENTS['g' as usize], 2);
        assert_eq!(COEFFICIENTS['c' as usize], 0);
        
        assert_eq!(CHARACTERS[3], 'a');
        assert_eq!(CHARACTERS[0], 'c');
        assert_eq!(CHARACTERS[2], 'g');
        assert_eq!(CHARACTERS[1], 't');

        assert_eq!(DICT_BIT_SIZE, 2);
    }
    //drop(guard);
}


#[test]
fn test_encoding_file() {
    //let guard = mtx.lock().unwrap();
    unsafe {
        let verbose = true;
        let path = "src/encodings.txt";
        init_coefficients_file(path, verbose);
        
        assert_eq!(COEFFICIENTS['a' as usize], 3);
        assert_eq!(COEFFICIENTS['t' as usize], 1);
        assert_eq!(COEFFICIENTS['g' as usize], 2);
        assert_eq!(COEFFICIENTS['c' as usize], 0);
        
        assert_eq!(CHARACTERS[3], 'a');
        assert_eq!(CHARACTERS[0], 'c');
        assert_eq!(CHARACTERS[2], 'g');
        assert_eq!(CHARACTERS[1], 't');

        assert_eq!(DICT_BIT_SIZE, 2);
    }
    //drop(guard);
}


//------------------------------------------------------------------------------------
//------------------------------------------------------------------------------------
// TESTS FOR CORE
//------------------------------------------------------------------------------------
//------------------------------------------------------------------------------------
#[test]
fn test_core_encoding() {
    //let guard = mtx.lock().unwrap();

    // A/a=0, T/t=3, G/g=2, C/c=1
    unsafe {
        let verbose = true;
        init_coefficients_default(verbose);

        let core: Core = Core::new(1, 6, "ATGTC");

        assert_eq!(core.get_block_number(), 2);
        assert_eq!(core.get_start_index(), 6);
        assert_eq!(core.get_blocks(), [0b00, 0b11101101]);
    }
    //drop(guard);
}