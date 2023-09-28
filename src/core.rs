use crate::statics::LABELS;
use crate::statics::DICT_BIT_SIZE;
use crate::statics::ENCODING_INIT;
use crate::statics::SIZE_PER_BLOCK;
use crate::encoding::init_coefficients_default;
use std::collections::VecDeque;
use std::mem;
use std::cmp;
use std::cmp::Ordering;


#[derive(Eq, Ord)]
pub struct Core {
	// Represenation related variables
	ptr: *mut u8,
	block_number: usize,
	start_index: usize,

	// Core related variables
	start: usize,
	end: usize,
}


impl Core {

	#[allow(dead_code)]
	pub fn new(start: usize, end:usize, string: &str) -> Self {

		unsafe {

			if !ENCODING_INIT {
				init_coefficients_default(false);
			}

			let block_number: usize = ( string.len() * DICT_BIT_SIZE - 1) / SIZE_PER_BLOCK + 1;
			let start_index: usize = block_number * SIZE_PER_BLOCK - string.len() * DICT_BIT_SIZE;

			// create a new mutable buffer with capacity `block_number`
			let mut buf = Vec::with_capacity(block_number.try_into().unwrap());
			// take a mutable pointer to the buffer
			let ptr: *mut u8 = buf.as_mut_ptr();
			// prevent the buffer from being deallocated when it goes out of scope
			mem::forget(buf);

			// clear dumps
			for i in 0..block_number {
				*ptr.add(i.try_into().unwrap()) &= 0;
			}

			// Encoding string to bits
			let mut index: usize = 0;

			for ch in string.chars() { 
				if SIZE_PER_BLOCK - ( start_index + index ) % SIZE_PER_BLOCK >= DICT_BIT_SIZE {
					*ptr.add( ( start_index + index) / SIZE_PER_BLOCK ) |= ( ( LABELS[ch as usize] as usize ) << ( SIZE_PER_BLOCK - ( start_index + index + DICT_BIT_SIZE ) % SIZE_PER_BLOCK ) % SIZE_PER_BLOCK ) as u8;	
				} else {
					*ptr.add( ( start_index + index ) / SIZE_PER_BLOCK ) |= ( ( LABELS[ch as usize] as usize ) >> ( start_index + index + DICT_BIT_SIZE ) % SIZE_PER_BLOCK ) as u8;
					*ptr.add( ( start_index + index) / SIZE_PER_BLOCK  + 1 ) |= ( ( LABELS[ch as usize] as usize ) << ( SIZE_PER_BLOCK - ( start_index + index + DICT_BIT_SIZE ) % SIZE_PER_BLOCK ) % SIZE_PER_BLOCK ) as u8;
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


	#[allow(dead_code)]
	pub fn new2(start: usize, end:usize, ch: char) -> Self {

		unsafe {

			if !ENCODING_INIT {
				init_coefficients_default(false);
			}

			// create a new mutable buffer with capacity `block_number`
			let mut buf = Vec::with_capacity(1);
			// take a mutable pointer to the buffer
			let ptr: *mut u8 = buf.as_mut_ptr();
			// prevent the buffer from being deallocated when it goes out of scope
			mem::forget(buf);

			*ptr.add(0) &= 0;
			*ptr.add(0) |= LABELS[ch as usize] as u8;

			Core {
				ptr: ptr,
				block_number: 1,
				start_index: SIZE_PER_BLOCK - DICT_BIT_SIZE,
				start: start,
				end: end
			}
		}
	}


	#[allow(dead_code)]
	pub fn new3(start: usize, end:usize, cores: &VecDeque<Core>) -> Self {

		unsafe {

			if !ENCODING_INIT {
				init_coefficients_default(false);
			}
			let new_cores = cores.into_iter().enumerate().filter(|&(_i, _v)| start <= _i && _i < end).map(|(_, v)| v).collect::<Vec<_>>();
			let bit_count: usize = new_cores.iter().map(|s| s.get_bit_count()).sum();
			let block_number = ( bit_count - 1 ) / SIZE_PER_BLOCK + 1;
			let start_index = block_number * SIZE_PER_BLOCK - bit_count;

			// create a new mutable buffer with capacity `block_number`
			let mut buf = Vec::with_capacity(block_number);
			// take a mutable pointer to the buffer
			let ptr: *mut u8 = buf.as_mut_ptr();
			// prevent the buffer from being deallocated when it goes out of scope
			mem::forget(buf);

			// clear dumps
			for i in 0..block_number {
				*ptr.add(i.try_into().unwrap()) &= 0;
			}

			let mut index: usize = block_number * SIZE_PER_BLOCK - 1;

			for core in new_cores.iter().rev() {
				for (i, block) in core.get_blocks().iter().enumerate().rev() {
					if index >= SIZE_PER_BLOCK {
						*ptr.add( index / SIZE_PER_BLOCK ) |= block << ( SIZE_PER_BLOCK - index % SIZE_PER_BLOCK - 1 );
						if index % SIZE_PER_BLOCK != SIZE_PER_BLOCK - 1 {
							*ptr.add( index / SIZE_PER_BLOCK - 1) |= block >> ( index % SIZE_PER_BLOCK  + 1 );
						}
						if i == 0 {
							index -= SIZE_PER_BLOCK - core.get_start_index();
						} else {
							index -= SIZE_PER_BLOCK;
						}
					} else {
						*ptr.add( index / SIZE_PER_BLOCK ) |= block << ( SIZE_PER_BLOCK - index % SIZE_PER_BLOCK - 1);
						index -= SIZE_PER_BLOCK - core.get_start_index();
					}
				}
			}

			Core {
				ptr: ptr,
				block_number: block_number,
				start_index: start_index,
				start: cores[start].start,
				end: cores[end-1].end
			}
		}
	}

	pub fn compress(&mut self, other: &Core) {
		let mut o_block_index = other.block_number - 1;
		let mut t_block_index = self.block_number - 1;
		let o_values = unsafe { std::slice::from_raw_parts(other.ptr, other.block_number as usize) };
		let t_values = unsafe { std::slice::from_raw_parts(self.ptr, self.block_number as usize) };
		let mut o: u8 = o_values[o_block_index as usize];
		let mut t: u8 = t_values[t_block_index as usize];

		let mut current_index;
		let mut new_bit_size;
		let mut temp = 0;

		while o_block_index > 0 && t_block_index > 0 && o == t {
			o_block_index -= 1;
			t_block_index -= 1;
			o = o_values[o_block_index as usize];
			t = t_values[t_block_index as usize];
		}

		if o_block_index > 0 {
			if t_block_index > 0 {
				current_index = 0;
			} else {
				current_index = self.start_index;
			}
		} else {
			if t_block_index > 0 {
				current_index = other.start_index;
			} else {
				current_index = cmp::max(other.start_index, self.start_index);
			}
		}

		while current_index < SIZE_PER_BLOCK && o % 2 == t % 2 {
			o /= 2;
			t /= 2;
			current_index += 1;
			temp += 1;
		}

		let index = 2 * ( (self.block_number - t_block_index - 1) * SIZE_PER_BLOCK + temp ) + ( t as usize ) % 2;

		new_bit_size = 0;
		temp = index;
		while temp != 0 {
			new_bit_size += 1;
			temp /= 2;
		}

		if new_bit_size < 3 {
			new_bit_size = 2;
		}

		// Compressed value is: index

		// deallocate previous core
		unsafe { Vec::from_raw_parts(self.ptr, self.block_number as usize, self.block_number as usize); }

		// Change this object according to  the new values represents compressed version.
		self.block_number = (new_bit_size - 1) / SIZE_PER_BLOCK + 1;
		self.start_index = self.block_number * SIZE_PER_BLOCK - new_bit_size;

		// create a new mutable buffer with capacity `block_number`
		let mut buf = Vec::with_capacity(self.block_number.try_into().unwrap());
		// take a mutable pointer to the buffer
		self.ptr = buf.as_mut_ptr();
		// prevent the buffer from being deallocated when it goes out of scope
		mem::forget(buf);
		
		// clear dumps
		for i in 0..self.block_number {
			unsafe { *(self.ptr).add(i.try_into().unwrap()) &= 0; }
		}

		// Set bits block by block and avoid unnecesary assignments
		temp = 0;

		for i in index.to_le_bytes().iter().rev() {
			if *i == 0 {
				continue;
			}
			unsafe { *(self.ptr).add(temp.try_into().unwrap()) = *i as u8; }
			temp += 1;
		}
	}

	#[inline(always)]
	#[allow(dead_code)]
	pub fn get_bit_count(&self) -> usize {
		self.block_number * SIZE_PER_BLOCK - self.start_index
	}

	#[inline(always)]
	#[allow(dead_code)]
	pub fn show(&self) {
		let values = unsafe { std::slice::from_raw_parts(self.ptr, self.block_number as usize) };
		print!("0b");
		for index in self.start_index..SIZE_PER_BLOCK {
			print!("{}", ( values[0] >> ( SIZE_PER_BLOCK - index - 1 ) ) % 2  );
		}
		for value in values[1..].iter() {
			print!("{:08b}", value);
		}
		print!(" ");
	}

	#[inline(always)]
	#[allow(dead_code)]
	pub fn get_blocks(&self) -> &[u8] {
		let values = unsafe { std::slice::from_raw_parts(self.ptr, self.block_number as usize) };
		return values;
	}

	#[inline(always)]
	#[allow(dead_code)]
	pub fn get_block_number(&self) -> usize {
		self.block_number
	}

	#[inline(always)]
	#[allow(dead_code)]
	pub fn get_start_index(&self) -> usize {
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


impl PartialEq for Core {
    fn eq(&self, other: &Self) -> bool {

    	if self.block_number != other.block_number {
    		return false;
    	}

    	if self.start_index != other.start_index {
    		return false;
    	}

    	for (self_block, other_block) in self.get_blocks().iter().zip(other.get_blocks()) {
    		if self_block != other_block {
    			return false;
    		}
    	}
    	return true;
    }
}


impl PartialOrd for Core {

	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }

	fn lt(&self, other: &Self) -> bool { 
		if self.block_number < other.block_number {
    		return true;
    	} else if self.block_number > other.block_number {
    		return false;
    	}

    	if self.start_index > other.start_index {
    		return true;
    	} else if self.start_index < other.start_index {
    		return false;
    	}

    	for (self_block, other_block) in self.get_blocks().iter().zip(other.get_blocks()) {
    		if self_block < other_block {
    			return true;
    		} else if self_block > other_block {
    			return false;
    		}
    	}

    	return false;
	}

    fn le(&self, other: &Self) -> bool {
    	if self.block_number < other.block_number {
    		return true;
    	} else if self.block_number > other.block_number {
    		return false;
    	}

    	if self.start_index > other.start_index {
    		return true;
    	} else if self.start_index < other.start_index {
    		return false;
    	}

    	for (self_block, other_block) in self.get_blocks().iter().zip(other.get_blocks()) {
    		if self_block < other_block {
    			return true;
    		} else if self_block > other_block {
    			return false;
    		}
    	}

    	return true;
    }
    
    fn gt(&self, other: &Self) -> bool {
    	if self.block_number > other.block_number {
    		return true;
    	} else if self.block_number < other.block_number {
    		return false;
    	}

    	if self.start_index < other.start_index {
    		return true;
    	} else if self.start_index > other.start_index {
    		return false;
    	}

    	for (self_block, other_block) in self.get_blocks().iter().zip(other.get_blocks()) {
    		if self_block > other_block {
    			return true;
    		} else if self_block < other_block {
    			return false;
    		}
    	}

    	return false;
    }
    
    fn ge(&self, other: &Self) -> bool {
    	if self.block_number > other.block_number {
    		return true;
    	} else if self.block_number < other.block_number {
    		return false;
    	}

    	if self.start_index < other.start_index {
    		return true;
    	} else if self.start_index > other.start_index {
    		return false;
    	}

    	for (self_block, other_block) in self.get_blocks().iter().zip(other.get_blocks()) {
    		if self_block > other_block {
    			return true;
    		} else if self_block < other_block {
    			return false;
    		}
    	}

    	return true;
    }
}