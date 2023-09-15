use crate::encoding::COEFFICIENTS;
use crate::encoding::DICT_BIT_SIZE;
use crate::statics::SIZE_PER_BLOCK;
use crate::statics::COMPRESSION_ITERATION_COUNT;
use crate::statics::CORE_LENGTH;
use std::mem;

pub struct Core {
	// Represenation related variables
	ptr: *mut u8,
	block_number: u32,
	start_index: u32,

	// Core related variables
	start: u32,
	end: u32,
}

impl Core {

	pub fn new(start: u32, end:u32, string: &str) -> Self {
		unsafe {
			let block_number: u32 = ( ( string.len() as u32 ) * DICT_BIT_SIZE - 1) / SIZE_PER_BLOCK + 1;
	        let start_index: u32 = block_number * SIZE_PER_BLOCK - ( string.len() as u32 ) * DICT_BIT_SIZE;

			// create a new mutable buffer with capacity `block_number`
		    let mut buf = Vec::with_capacity(block_number.try_into().unwrap());
		    // take a mutable pointer to the buffer
		    let ptr: *mut u8 = buf.as_mut_ptr();
		    // prevent the buffer from being deallocated when it goes out of scope
		    mem::forget(buf);
		    
		    // clear dumps
			for _ in 0..block_number {
				let _ = *ptr.add(0);
			}

			// Encoding string to bits
	        let mut coefficient: i32 = 0;
	        let mut index: u32 = 0;

			for ch in string.chars() { 
				coefficient = COEFFICIENTS[ch as usize];
				for i in (0..DICT_BIT_SIZE).rev() {
					if coefficient % 2 == 1 {
						*(ptr.wrapping_add( ( (start_index + index + (i as u32) ) / SIZE_PER_BLOCK ) as usize ) ) |= 1 << ( SIZE_PER_BLOCK - ( (start_index + index + (i as u32) ) % SIZE_PER_BLOCK ) - 1 )  as u8;
					}
					coefficient = coefficient / 2;
				}
				index += DICT_BIT_SIZE;
			}

			Core {
				ptr: ptr,
				block_number: block_number,
				start_index: start_index,
				start: start,
				end: end
			}
		}
    }

    pub fn show(&self) {
    	let values = unsafe { std::slice::from_raw_parts(self.ptr, self.block_number as usize) };
    	for value in values {
    		print!("{:b}", value);
    	}
    }

    pub fn get_blocks(&self) -> &[u8] {
    	let values = unsafe { std::slice::from_raw_parts(self.ptr, self.block_number as usize) };
    	return values;
    }

    pub fn get_block_number(&self) -> u32 {
    	self.block_number
    }

    pub fn get_start_index(&self) -> u32 {
    	self.start_index
    }
}

impl Drop for Core {
	fn drop(&mut self) {
    	unsafe {
	        Vec::from_raw_parts(self.ptr, self.block_number as usize, self.block_number as usize);
	    }
    }
}