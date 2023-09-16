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


#[test]
fn test_core_compression() {
    //let guard = mtx.lock().unwrap();

    // A/a=0, T/t=3, G/g=2, C/c=1
    unsafe {
        let verbose = true;
        init_coefficients_default(verbose);

        let core1: Core = Core::new(1, 6, "ATGTC");
        let mut core2: Core = Core::new(2, 8, "TTGTC");

        core2.compress(&core1);
        print!("core1 :");
        core1.show();
        print!("core2 :");
        core2.show();
        // assertions for core1
        assert_eq!(core1.get_block_number(), 2);
        assert_eq!(core1.get_start_index(), 6);
        assert_eq!(core1.get_blocks(), [0b00, 0b11101101]);
        // assertions for core2
        assert_eq!(core2.get_block_number(), 1);
        assert_eq!(core2.get_start_index(), 3);
        assert_eq!(core2.get_blocks(), [0b10001]);

        println!("Compression btw core1 and core2 completed successfully.")

        let core3: Core = Core::new(1, 6, "A");
        let mut core4: Core = Core::new(2, 8, "TAAAA");

        core4.compress(&core3);
        print!("core3 :");
        core3.show();
        print!("core4 :");
        core4.show();
        // assertions for core3
        assert_eq!(core3.get_block_number(), 1);
        assert_eq!(core3.get_start_index(), 6);
        assert_eq!(core3.get_blocks(), [0b00]);
        // assertions for core4
        assert_eq!(core4.get_block_number(), 1);
        assert_eq!(core4.get_start_index(), 5);
        assert_eq!(core4.get_blocks(), [0b100]);

        println!("Compression btw core3 and core4 completed successfully.")

        let core5: Core = Core::new(1, 6, "T");
        let mut core6: Core = Core::new(2, 8, "TAAAA");

        core6.compress(&core5);
        print!("core5 :");
        core5.show();
        print!("core5 :");
        core6.show();
        // assertions for core5
        assert_eq!(core5.get_block_number(), 1);
        assert_eq!(core5.get_start_index(), 6);
        assert_eq!(core5.get_blocks(), [0b11]);
        // assertions for core6
        assert_eq!(core6.get_block_number(), 1);
        assert_eq!(core6.get_start_index(), 6);
        assert_eq!(core6.get_blocks(), [0b00]);

        println!("Compression btw core5 and core6 completed successfully.")

        let core7: Core = Core::new(1, 6, "C");
        let mut core8: Core = Core::new(2, 8, "T");

        core8.compress(&core7);
        print!("core7 :");
        core7.show();
        print!("core8 :");
        core8.show();
        // assertions for core7
        assert_eq!(core7.get_block_number(), 1);
        assert_eq!(core7.get_start_index(), 6);
        assert_eq!(core7.get_blocks(), [0b01]);
        // assertions for core8
        assert_eq!(core8.get_block_number(), 1);
        assert_eq!(core8.get_start_index(), 6);
        assert_eq!(core8.get_blocks(), [0b11]);

        println!("Compression btw core7 and core8 completed successfully.")
        
    }
    //drop(guard);
}