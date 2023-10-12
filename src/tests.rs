use crate::encoding::init_coefficients_default;
use crate::encoding::init_coefficients_map;
use crate::encoding::init_coefficients_file;
use crate::statics::LABELS;
use crate::statics::CHARACTERS;
use crate::statics::DICT_BIT_SIZE;
use std::collections::HashMap;
use std::collections::VecDeque;
use crate::core::Core;
use crate::String;


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
        
        assert_eq!(LABELS['A' as usize], 0); 
        assert_eq!(LABELS['a' as usize], 0);
        assert_eq!(LABELS['T' as usize], 3); 
        assert_eq!(LABELS['t' as usize], 3);
        assert_eq!(LABELS['G' as usize], 2); 
        assert_eq!(LABELS['g' as usize], 2);
        assert_eq!(LABELS['C' as usize], 1); 
        assert_eq!(LABELS['c' as usize], 1);
        
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
        
        assert_eq!(LABELS['a' as usize], 3);
        assert_eq!(LABELS['t' as usize], 1);
        assert_eq!(LABELS['g' as usize], 2);
        assert_eq!(LABELS['c' as usize], 0);
        
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
        
        assert_eq!(LABELS['a' as usize], 3);
        assert_eq!(LABELS['t' as usize], 1);
        assert_eq!(LABELS['g' as usize], 2);
        assert_eq!(LABELS['c' as usize], 0);
        
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
fn test_core_encoding_str() {
    //let guard = mtx.lock().unwrap();

    // A/a=0, T/t=3, G/g=2, C/c=1
    unsafe {
        let verbose = true;
        init_coefficients_default(verbose);

        let core: Core = Core::from_str(1, "ATGTC");

        assert_eq!(core.block_number, 2);
        assert_eq!(core.start_index, 6);
        assert_eq!(core.get_blocks(), [0b00, 0b11101101]);
        assert_eq!(core.start, 1);
        assert_eq!(core.end, 6);
    }
    //drop(guard);
}


#[test]
fn test_core_encoding_ch() {
    //let guard = mtx.lock().unwrap();

    // A/a=0, T/t=3, G/g=2, C/c=1
    unsafe {
        let verbose = true;
        init_coefficients_default(verbose);

        let core: Core = Core::from_char(1, 'C');

        assert_eq!(core.block_number, 1);
        assert_eq!(core.start_index, 6);
        assert_eq!(core.get_blocks(), [0b01]);
    }
    //drop(guard);
}


#[test]
fn test_core_concatination() {
    //let guard = mtx.lock().unwrap();

    // A/a=0, T/t=3, G/g=2, C/c=1
    unsafe {
        let verbose = true;
        init_coefficients_default(verbose);

        let core1: Core = Core::from_char(1, 'C');
        let core2: Core = Core::from_char(1, 'G');
        let core3: Core = Core::from_str(1, "ATC");
        let core4: Core = Core::from_str(1, "TTTCAG");
        let core5: Core = Core::from_char(1, 'A');
        println!("concatination started.");

        let core6 = Core::from_cores(0, 5, &&VecDeque::from(vec![core1, core2, core3, core4, core5]));
        println!("concatination is done.");
        println!("{}", core6.encode());
        assert_eq!(core6.encode(), 0b011000110111111101001000);
    }
    //drop(guard);
}


#[test]
fn test_core_compression() {
    //let guard = mtx.lock().unwrap();

    // A/a=0, T/t=3, G/g=2, C/c=1
    unsafe {
        let verbose = true;
        init_coefficients_default(verbose);

        let core1: Core = Core::from_str(1, "ATGTC");
        let mut core2: Core = Core::from_str(2, "TTGTC");

        core2.compress(&core1);
        print!("core1 :");
        core1.show();
        print!("core2 :");
        core2.show();
        // assertions for core1
        assert_eq!(core1.block_number, 2);
        assert_eq!(core1.start_index, 6);
        assert_eq!(core1.get_blocks(), [0b00, 0b11101101]);
        // assertions for core2
        assert_eq!(core2.block_number, 1);
        assert_eq!(core2.start_index, 3);
        assert_eq!(core2.get_blocks(), [0b10001]);

        println!("Compression btw core1 and core2 completed successfully.");

        let core3: Core = Core::from_str(1, "A");
        let mut core4: Core = Core::from_str(2, "TAAAA");

        core4.compress(&core3);
        print!("core3 :");
        core3.show();
        print!("core4 :");
        core4.show();
        // assertions for core3
        assert_eq!(core3.block_number, 1);
        assert_eq!(core3.start_index, 6);
        assert_eq!(core3.get_blocks(), [0b00]);
        // assertions for core4
        assert_eq!(core4.block_number, 1);
        assert_eq!(core4.start_index, 5);
        assert_eq!(core4.get_blocks(), [0b100]);

        println!("Compression btw core3 and core4 completed successfully.");

        let core5: Core = Core::from_str(1, "T");
        let mut core6: Core = Core::from_str(2, "TAAAA");

        core6.compress(&core5);
        print!("core5 :");
        core5.show();
        print!("core5 :");
        core6.show();
        // assertions for core5
        assert_eq!(core5.block_number, 1);
        assert_eq!(core5.start_index, 6);
        assert_eq!(core5.get_blocks(), [0b11]);
        // assertions for core6
        assert_eq!(core6.block_number, 1);
        assert_eq!(core6.start_index, 6);
        assert_eq!(core6.get_blocks(), [0b00]);

        println!("Compression btw core5 and core6 completed successfully.");

        let core7: Core = Core::from_str(1, "C");
        let mut core8: Core = Core::from_str(2, "T");

        core8.compress(&core7);
        print!("core7 :");
        core7.show();
        print!("core8 :");
        core8.show();
        // assertions for core7
        assert_eq!(core7.block_number, 1);
        assert_eq!(core7.start_index, 6);
        assert_eq!(core7.get_blocks(), [0b01]);
        // assertions for core8
        assert_eq!(core8.block_number, 1);
        assert_eq!(core8.start_index, 6);
        assert_eq!(core8.get_blocks(), [0b11]);

        println!("Compression btw core7 and core8 completed successfully.");
        
    }
    //drop(guard);
}


#[test]
fn test_core_comparison_eq() {
    //let guard = mtx.lock().unwrap();

    // A/a=0, T/t=3, G/g=2, C/c=1
    unsafe {
        let verbose = true;
        init_coefficients_default(verbose);

        let core1: Core = Core::from_str(1, "ATGTGCT");
        let core2: Core = Core::from_str(2, "ATGTGCT");
        
        assert_eq!(core1 == core2, true);

        let core3: Core = Core::from_str(1, "T");
        let core4: Core = Core::from_str(2, "TTTT");

        assert_eq!(core3 == core4, false);

        let core5: Core = Core::from_str(1, "A");
        let core6: Core = Core::from_str(2, "AAAA");

        assert_eq!(core5 == core6, false);

        let core7: Core = Core::from_str(1, "CC");
        let core8: Core = Core::from_str(2, "CC");

        assert_eq!(core7 == core8, true);

        let core9: Core = Core::from_str(1, "TT");
        let core10: Core = Core::from_str(2, "TT");

        assert_eq!(core9 != core10, false);

        let core11: Core = Core::from_str(1, "ATGGCT");
        let core12: Core = Core::from_str(2, "ATGTGCT");
        
        assert_eq!(core11 != core12, true);
    }
    //drop(guard);
}


#[test]
fn test_core_comparison_cmp() {
    //let guard = mtx.lock().unwrap();

    // A/a=0, T/t=3, G/g=2, C/c=1
    unsafe {
        let verbose = true;
        init_coefficients_default(verbose);

        let core1: Core = Core::from_str(1, "ATGTGCT");
        let core2: Core = Core::from_str(2, "ATGTGCT");
        
        assert_eq!(core1 < core2, false);

        let core3: Core = Core::from_str(1, "ATGTGCT");
        let core4: Core = Core::from_str(2, "ATGTGCT");

        assert_eq!(core3 <= core4, true);

        let core5: Core = Core::from_str(1, "A");
        let core6: Core = Core::from_str(2, "AAAAA");

        assert_eq!(core5 < core6, true);

        let core7: Core = Core::from_str(1, "TC");
        let core8: Core = Core::from_str(2, "CC");

        assert_eq!(core7 > core8, true);

        let core9: Core = Core::from_str(1, "TC");
        let core10: Core = Core::from_str(2, "CC");

        assert_eq!(core9 >= core10, true);

        let core11: Core = Core::from_str(1, "AGTGCT");
        let core12: Core = Core::from_str(2, "ATGTGCT");
        
        assert_eq!(core11 > core12, false);
    }
    //drop(guard);
}


//------------------------------------------------------------------------------------
//------------------------------------------------------------------------------------
// TESTS FOR STRING
//------------------------------------------------------------------------------------
//------------------------------------------------------------------------------------
#[test]
fn test_string_init() {
    //let guard = mtx.lock().unwrap();

    // A/a=0, T/t=3, G/g=2, C/c=1
    unsafe {
        let verbose = true;
        init_coefficients_default(verbose);

        let string: String = String::new("GGGACCTGGTGACCCCAGCCCACGACAGCCAAGCGCCAGCTGAGCTCAGGTGTGAGGAGATCACAGTCCTCTGTAATAGGCTGTCCG");

        assert_eq!(string.get_small_cores(), [0b100001, 0b10111, 0b11110, 0b11101011, 0b101110, 0b100001, 0b000101010100, 0b10010, 0b1001010100,
        0b10001, 0b100001, 0b10010, 0b10010100, 0b1000010, 0b100110, 0b10010100, 0b10010, 0b100111, 0b100010, 0b100111, 0b10010, 0b101011,
        0b111011, 0b100010, 0b101000, 0b100010, 0b100011, 0b10001, 0b10010, 0b101101, 0b11010111, 0b110111, 0b111011, 0b11000011, 0b110010,
        0b101001, 0b100111, 0b111011, 0b11010110]);
    }
    //drop(guard);
}


//------------------------------------------------------------------------------------
//------------------------------------------------------------------------------------
// TESTS FOR STRING
//------------------------------------------------------------------------------------
//------------------------------------------------------------------------------------
#[test]
fn test_string_init_from_u8() {
    //let guard = mtx.lock().unwrap();

    // A/a=0, T/t=3, G/g=2, C/c=1
    unsafe {
        let verbose = true;
        init_coefficients_default(verbose);
        let sequence = "GGGACCTGGTGACCCCAGCCCACGACAGCCAAGCGCCAGCTGAGCTCAGGTGTGAGGAGATCACAGTCCTCTGTAATAGGCTGTCCG";
        let string: String = String::from_u8(sequence.as_bytes());

        assert_eq!(string.get_small_cores(), [0b100001, 0b10111, 0b11110, 0b11101011, 0b101110, 0b100001, 0b000101010100, 0b10010, 0b1001010100,
        0b10001, 0b100001, 0b10010, 0b10010100, 0b1000010, 0b100110, 0b10010100, 0b10010, 0b100111, 0b100010, 0b100111, 0b10010, 0b101011,
        0b111011, 0b100010, 0b101000, 0b100010, 0b100011, 0b10001, 0b10010, 0b101101, 0b11010111, 0b110111, 0b111011, 0b11000011, 0b110010,
        0b101001, 0b100111, 0b111011, 0b11010110]);
    }
    //drop(guard);
}


#[test]
fn test_string_compress() {
    //let guard = mtx.lock().unwrap();

    // A/a=0, T/t=3, G/g=2, C/c=1
    unsafe {
        let verbose = true;
        init_coefficients_default(verbose);

        let mut string: String = String::new("GGGACCTGGTGACCCCAGCCCACGACAGCCAAGCGCCAGCTGAGCTCAGGTGTGAGGAGATCACAGTCCTCTGTAATAGGCTGTCCG");

        assert_eq!(string.get_small_cores(), [0b100001, 0b10111, 0b11110, 0b11101011, 0b101110, 0b100001, 0b000101010100, 0b10010, 0b1001010100,
        0b10001, 0b100001, 0b10010, 0b10010100, 0b1000010, 0b100110, 0b10010100, 0b10010, 0b100111, 0b100010, 0b100111, 0b10010, 0b101011,
        0b111011, 0b100010, 0b101000, 0b100010, 0b100011, 0b10001, 0b10010, 0b101101, 0b11010111, 0b110111, 0b111011, 0b11000011, 0b110010,
        0b101001, 0b100111, 0b111011, 0b11010110]);

        string.deepen();

        assert_eq!(string.get_small_cores(), [0b0100010001, 0b0100010001, 0b01000100100, 0b00100110110, 0b1101100001, 0b0001100001, 0b1000010001,
        0b00011000011, 0b10000110110, 0b1101100010, 0b1000100111, 0b01111000011, 0b0011100111]);
    }
    //drop(guard);
}