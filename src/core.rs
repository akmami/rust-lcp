use crate::statics::LABELS;
use crate::statics::DICT_BIT_SIZE;
use crate::statics::SIZE_PER_BLOCK;
use std::collections::VecDeque;
use std::mem;
use std::cmp;
use std::cmp::Ordering;

#[derive(Eq, Ord)]
pub struct Core {
	/// This pointer stores the addres of the blocks
	/// that is being used to define the substring with
	/// either character encodings or with sub-level cores.
	pub ptr: *mut u8,
	/// This variable is the number of blocks (8bits) being
	/// used to store bit representation of substring/sub-level cores.
	pub block_number: usize,
	/// This variable is the start index of this representation.
	/// This value is important as bits are aligned to right.
	/// If core is encoded with 14 bits, then, start_index is 1 as 
	/// encoding starts from 1st index in first block.
	pub start_index: usize,
	/// This variable is used to store the start index of given core.
	/// As level will increase, Core will covert larger substrings
	/// and thus, it is important to remember start position.
	pub start: usize,
	/// This variable is used to store the end index of given core.
	/// As level will increase, Core will covert larger substrings
	/// and thus, it is important to remember end position.
	pub end: usize,
}


impl Core {

	/// Constructor of Core from string.
	/// String is converted to binary representation by given encoding values of characters. This is basically
	/// numerical value for given string.
	///
	/// # Arguments
	/// 
	/// * `start` - start position of given substring within original string.
	/// * `str` - substring that will be processed with lcp algorithm.
	///
	#[allow(dead_code)]
	pub fn from_str(start: usize, string: &str) -> Self {

		unsafe {

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
				end: start+string.len()
			}
		}
	}


	/// Constructor of Core from character.
	/// Character is converted to binary representation by given encoding values of characters. This is basically
	/// numerical value for given character.
	///
	/// # Arguments
	/// 
	/// * `start` - index of given character within original string.
	/// * `ch` - character that will be used to create Core.
	///
	#[allow(dead_code)]
	pub fn from_char(start: usize, ch: char) -> Self {

		unsafe {

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
				end: start+1
			}
		}
	}

	/// Constructor of Core from Cores.
	/// Cores are concatinated into single Core. 
	/// All cores' blocks are assigned to new core one by one. Hence, it is efficient 
	/// since it depends on total block number. 
	/// Start index will be taken from first Core and end from the last Core given in VecDeque. 
	///
	/// # Arguments
	/// * `start` - start index of the cores in the VecDeque that will be concatinated.
	/// * `end` - end index of the cores in the VecDeque that will be concatinated.
	/// * `cores` - Cores that will be concatinated into single Core.
	///
	#[allow(dead_code)]
	pub fn from_cores(start: usize, end: usize, cores: &VecDeque<Core>) -> Self {

		unsafe {

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
					} else {
						*ptr.add( index / SIZE_PER_BLOCK ) |= block << ( SIZE_PER_BLOCK - index % SIZE_PER_BLOCK - 1);
					}
					if i == 0 {
						if index > SIZE_PER_BLOCK - core.start_index {
							index -= SIZE_PER_BLOCK - core.start_index;
						}
					}
					else {
						index -= SIZE_PER_BLOCK;
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


	/// Compression of core with respect to another Core.
	/// The compression is done accordinly:
	/// 	1 - Find index of first bit differs in another Core in same index starting from right.
	/// 	2 - Add the differing index to the right (index >> 1 | bit). 
	/// The Core will be modified according to the value found by compression.
	///
	/// # Arguments
	/// * `other` - The other Core that be used in compression. This Core will not be modified.
	/// * `cores` - Cores that will be concatinated into single Core.
	///
	pub fn compress(&mut self, other: &Core) {

		// declare variables
		let mut o_block_index = other.block_number - 1;
		let mut t_block_index = self.block_number - 1;
		let o_values = unsafe { std::slice::from_raw_parts(other.ptr, other.block_number as usize) };
		let t_values = unsafe { std::slice::from_raw_parts(self.ptr, self.block_number as usize) };
		let mut o: u8 = o_values[o_block_index as usize];
		let mut t: u8 = t_values[t_block_index as usize];
		let mut helper_var1: usize = 0;
		let mut helper_var2: usize;

		// skip block that are same
		// helper_var1 is used to track index from the right side of both Cores
		// which will be used in compressed value
		while o_block_index > 0 && t_block_index > 0 && o == t {
			o_block_index -= 1;
			t_block_index -= 1;
			o = o_values[o_block_index as usize];
			t = t_values[t_block_index as usize];
			helper_var1 += SIZE_PER_BLOCK;
		}

		// find exact index where bits differ.
		// helper_var2 is used to make sure that iteration for
		// finding difference ends within block (prevents out of boundary problem).
		if o_block_index > 0 {
			if t_block_index > 0 {
				helper_var2 = 0;
			} else {
				helper_var2 = self.start_index;
			}
		} else {
			if t_block_index > 0 {
				helper_var2 = other.start_index;
			} else {
				helper_var2 = cmp::max(other.start_index, self.start_index);
			}
		}

		
		while helper_var2 < SIZE_PER_BLOCK && o % 2 == t % 2 {
			o /= 2;
			t /= 2;
			helper_var1 += 1;
			helper_var2 += 1;
		}

		let index = 2 * helper_var1 + ( t as usize ) % 2;

		// helper_var1 and helper_var2 values are no longer required
		
		// find how many bits requered to store new value
		// helper_var1 will be used as a new bit count requred to create new Core
		// helper_var2 will be used as a helper value that will store index
		helper_var1 = 0;
		helper_var2 = index;
		while helper_var2 != 0 {
			helper_var1 += 1;
			helper_var2 /= 2;
		}

		if helper_var1 < 3 {
			helper_var1 = 2;
		}

		// Compressed value is: index

		// deallocate previous core
		unsafe { Vec::from_raw_parts(self.ptr, self.block_number as usize, self.block_number as usize); }

		// Change this object according to  the new values represents compressed version.
		self.block_number = (helper_var1 - 1) / SIZE_PER_BLOCK + 1;
		self.start_index = self.block_number * SIZE_PER_BLOCK - helper_var1;

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
		// helper_var1 is used as a helper to assign block to the new Core.
		helper_var1 = 0;

		// assign index block by block
		for i in index.to_le_bytes().iter().rev() {
			if *i == 0 {
				continue;
			}
			unsafe { *(self.ptr).add(helper_var1.try_into().unwrap()) = *i as u8; }
			helper_var1 += 1;
		}
	}

	/// This function returns bit count that is being used in bit representation of the Core.
	/// 
	/// # Arguments
	/// * `self` - The core itself is required to get block_number and start_index
	///
	#[inline(always)]
	#[allow(dead_code)]
	pub fn get_bit_count(&self) -> usize {
		self.block_number * SIZE_PER_BLOCK - self.start_index
	}

	/// This function the label of the Core (bit representation) in bits.
	/// 
	/// # Arguments
	/// * `self` - The core itself is required to ptr, get block_number and start_index
	///
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

	/// This function returns the blocks of the ptr as slice.
	/// 
	/// # Arguments
	/// * `self` - The core itself is required to ptr and get block_number.
	///
	#[inline(always)]
	#[allow(dead_code)]
	pub fn get_blocks(&self) -> &[u8] {
		let values = unsafe { std::slice::from_raw_parts(self.ptr, self.block_number as usize) };
		return values;
	}

	/// This function returns bit representation as u64.
	/// If the label has more than 64 bits, this function will return 64 bits from right.
	/// 
	/// # Arguments
	/// * `self` - The core itself is required to ptr and get block_number.
	///
	#[inline(always)]
	#[allow(dead_code)]
	pub fn encode(&self) -> u64 {
		let values = unsafe { std::slice::from_raw_parts(self.ptr, self.block_number as usize) };
		let mut encoding: u64 = 0;
		for (index, value) in values.iter().rev().enumerate().filter(|&(i, _x)| i < 4){
			encoding |= (*value as u64) << (index * 8);
		}
		return encoding;
	}
}


impl Drop for Core {
	/// Deconstructor of Core struct is needed as ptr is allocated in heap.
	/// 
	/// # Arguments
	/// * `self` - The core itself is required to ptr and get block_number.
	///
	fn drop(&mut self) {
		unsafe {
			Vec::from_raw_parts(self.ptr, self.block_number as usize, self.block_number as usize);
		}
	}
}


impl PartialEq for Core {
	/// Equal-to operator (==) is overloaded for Core comparison.
	/// 
	/// # Arguments
	/// * `self` - Core that will be used as lhs.
	/// * `other` - Core that will be used as rhs
	///
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

	/// Less-than operator (<) is overloaded for Core comparison.
	/// 
	/// # Arguments
	/// * `self` - Core that will be used as lhs.
	/// * `other` - Core that will be used as rhs
	///
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

	/// Less-than-or-equal-to operator (<=) is overloaded for Core comparison.
	/// 
	/// # Arguments
	/// * `self` - Core that will be used as lhs.
	/// * `other` - Core that will be used as rhs
	///
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

	/// Greater-than operator (<) is overloaded for Core comparison.
	/// 
	/// # Arguments
	/// * `self` - Core that will be used as lhs.
	/// * `other` - Core that will be used as rhs
	///
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

	/// Greater-than-or-equal-to operator (>=) is overloaded for Core comparison.
	/// 
	/// # Arguments
	/// * `self` - Core that will be used as lhs.
	/// * `other` - Core that will be used as rhs
	///
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