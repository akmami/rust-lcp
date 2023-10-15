pub mod statics;
pub mod encoding;
pub mod core;
use crate::statics::COMPRESSION_ITERATION_COUNT;
use crate::statics::LABELS;
use crate::statics::ENCODING_INIT;
use crate::encoding::init_coefficients_default;
use crate::core::Core;
use std::cmp;
use log::error;


pub struct String {
	/// Level of LCP algorithm being called. This parameter
	/// is autamatically updated as deepen() func called.
	pub level: u32,
	/// This vector is used to store cores of given string.
	/// Each core belongs to same level and order is as increasing
	/// order in terms of start indexes. I have used VecDeque as I
	/// needed to insert to tail and remove from head efficiently while 
	/// increasing the level.
	pub cores: Vec<Core>
}


impl String {

	/// Constructor of String with given str. This String does not stores the actual character array
	/// but the cores and level that defınes the lcp.
	/// 
	/// # Arguments
	/// 
	/// * `str` - string that will be processed with lcp algorithm.
	///
	pub fn new(string: &str) -> Self {
		// str can be converted to &[u8] with as_bytes() func.
		// hence, there is no need to duplicate code.
		// return Self::from_u8(&string.as_bytes());
		unsafe {
			// make sure that encodings are initialized
			if !ENCODING_INIT {
				init_coefficients_default(false);
			}

			// if minimum lenght is 3 as 0-level cores have length of 3 as there is no compression yet.
			if string.len() < 3 { 
				error!("Given string ({}) is too small!", string); 
				return String {
					level: 1,
					cores: Vec::new()
				};
			}

			let mut index2: usize;
			let end = string.len();
			let read = string.as_bytes();
			let mut cores: Vec<Core> = Vec::new();
			
			// index here should be taken as index + 1 in read as window does not starts with core but with left neighour
			// window consists of 5 characters and the core that is being processed is middle 3 character.
			for (index, window) in read.windows(5).enumerate() {
				
				if window[1] == window[2] { continue; }

				// if there are same characters in subsequenct order such as xyyz, xyyyz, .... where x!=y and y!=z
				if window[2] == window[3] { 
					index2 = index + 3;
					
					let mut prev: u8 = window[3];
					for ch in &read[index2..] {
						if prev != *ch  { break; }
						
						prev = *ch;
						index2 += 1;
					}

					if index2 == end { break; }

					index2 += 1;
					
					cores.push( Core::from_str(index + 1, std::str::from_utf8(&read[index+1..index2]).unwrap() ) );
					continue;
				}

				// if there is no subsequent characters such as xyz where z!=y and y!=z

				// if window[0] == window[1] || window[3] == window[4] { continue; } // should we add this?

				if LABELS[window[2] as usize] < LABELS[window[1] as usize] && LABELS[window[2] as usize] < LABELS[window[3] as usize] ||	// local minima
					(
						LABELS[window[2] as usize] > LABELS[window[1] as usize] && LABELS[window[2] as usize] > LABELS[window[3] as usize] &&       // local maxima without immediate local minima neighbours
						!(LABELS[window[1] as usize] < LABELS[window[2] as usize] && LABELS[window[1] as usize] < LABELS[window[0] as usize]) &&
						!(LABELS[window[3] as usize] < LABELS[window[4] as usize] && LABELS[window[3] as usize] < LABELS[window[2] as usize]) 
					) 
				{
					cores.push( Core::from_str(index + 1, std::str::from_utf8(&window[1..4]).unwrap() ) ); 
				}
			}
			
			String {
				level: 1,
				cores: cores
			}
		}
	}

	/// Constructor of String with given [u8]. This String does not stores the actual character array
	/// but the cores and level that defines the lcp.
	/// 
	/// # Arguments
	/// 
	/// * `string` - string given in [u8] format that will be processed with lcp algorithm.
	///
	pub fn from_u8(string: &[u8]) -> Self {
		
		unsafe {
			// make sure that encodings are initialized
			if !ENCODING_INIT {
				init_coefficients_default(false);
			}

			// if minimum lenght is 3 as 0-level cores have length of 3 as there is no compression yet.
			if string.len() < 3 { 
				error!("Given string ({}) is too small!", std::str::from_utf8(string).unwrap()); 
				return String {
					level: 1,
					cores: Vec::new()
				};
			}

			let mut index2: usize;
			let end = string.len();
			let mut cores: Vec<Core> = Vec::new();
			
			// index here should be taken as index + 1 in read as window does not starts with core but with left neighour
			// window consists of 5 characters and the core that is being processed is middle 3 character.
			for (index, window) in string.windows(5).enumerate() {

				if window[1] == window[2] { continue; }
				
				// if there are same characters in subsequenct order such as xyyz, xyyyz, .... where x!=y and y!=z

				if window[2] == window[3] {

					index2 = index + 3;

					let mut prev: u8 = window[3];
					for ch in &string[index2..] {
						if prev != *ch  { break; }
						
						prev = *ch;
						index2 += 1;
					}

					if index2 == end { break; }

					index2 += 1;
					cores.push( Core::from_u8(index + 1, &string[index+1..index2]) );
					continue;
				}

				// if there is no subsequent characters such as xyz where z!=y and y!=z

				// if window[0] == window[1] || window[3] == window[4] { continue; } // should we add this?

				if LABELS[window[2] as usize] < LABELS[window[1] as usize] && LABELS[window[2] as usize] < LABELS[window[3] as usize] ||	// local minima
					(
						LABELS[window[2] as usize] > LABELS[window[1] as usize] && LABELS[window[2] as usize] > LABELS[window[3] as usize] &&       // local maxima without immediate local minima neighbours
						!(LABELS[window[1] as usize] < LABELS[window[2] as usize] && LABELS[window[1] as usize] < LABELS[window[0] as usize]) &&
						!(LABELS[window[3] as usize] < LABELS[window[4] as usize] && LABELS[window[3] as usize] < LABELS[window[2] as usize]) 
					) 
				{
					cores.push( Core::from_u8(index + 1, &window[1..4]) ); 
				}
			}

			String {
				level: 1,
				cores: cores
			}
		}
	}
	

	/// This fuction calls lcp algorithm to increase level multiple time. 
	/// Instead of calling lcp multiple times as if the number is greater than 2,
	/// this funcion can be called.
	/// 
	/// # Arguments
	/// 
	/// * `self` - this function needs to access core.
	/// * `level` - number of times that lcp needs to be called.
	///
	pub fn deepen_multiple(&mut self, level: u32) {
		for _ in 0..level {
			self.deepen();
		}
	}

	/// This fuction calls lcp algorithm to increase level once. 
	/// The lcp algorithm uses 2 rules when increasing the level using the available cores
	/// at that instance:
	/// 
	///  - middle core's label is local minimum compared to its neighbors
	/// 
	///  - middle core's label is local maximum compared to its neighbors while none if its neighbors are local minima
	/// 
	///  - core contains multiple cores that has same label in the middle.
	/// 
	/// While increasing the level, compression (deterministic coin tossing) is done. the compression algoritm is implemented
	/// and described under Core struct. Since dct is used to compress values with its left neighor, they will have affect on
	/// the labels in core that meets one of the rules. So, cores from the left hand side besides the original core will be taken
	/// to compose new core. The number of cores from left hand side to be taken will be determined from the number of iteration
	/// of compression is done.
	/// 
	/// (3 + COMPRESSION_ITERATION_COUNT == NEW CORE LENGTH at new level)
	/// 
	/// # Arguments
	/// 
	/// * `self` - this function needs to access core.
	///
	pub fn deepen(&mut self) {

		// Compress cores
		self.dct();
				
		// Find cores from compressed cores.
		let end = self.cores.len();
		let mut index2: usize;
		let mut cores: Vec<Core> = Vec::new();

		for (index, window) in self.cores.windows(5).enumerate().skip(2*COMPRESSION_ITERATION_COUNT-1) {
			
			if window[4].end - window[0].start >= 10000 { continue; }

			if window[1] == window[2] { continue; }
				
			// if there are same characters in subsequenct order such as xyyz, xyyyz, .... where x!=y and y!=z
			if window[2] == window[3] { 
				index2 = index + 3;
				
				let mut prev: &Core = &window[3];
				for ch in &self.cores[index2..] {
					if prev != ch  { break; }
					
					prev = ch;
					index2 += 1;
				}

				if index2 == end { break; }

				index2 += 1;
				
				cores.push( Core::from_cores(&self.cores[index+1..index2]) );
				continue;
			}

			// if there is no subsequent characters such as xyzuv where z!=y and y!=z and z!=u and u!=v
			
			// if window[0] == window[1] || window[3] == window[4] { continue; } // should we add this?

			if window[2] < window[1] && window[2] < window[3] ||           // local minima
				(
					window[2] > window[1] && window[2] > window[3] &&       // local maxima without immediate local minima neighbours
					!(window[1] < window[2] && window[1] < window[0]) &&
					!(window[3] < window[4] && window[3] < window[2]) 
				)
			{
				cores.push( Core::from_cores(&self.cores[index+1-COMPRESSION_ITERATION_COUNT..index+4]) ); 
			}
		}

		//self.cores.drain(0..end);
		self.cores = cores;
		self.level += 1;
	}

	/// This fuction compresses each core with respect to its left neigbor. The total number of dct
	/// should be defined as static variable.
	/// # Arguments
	/// 
	/// * `self` - this function needs to access core.
	///
	pub fn dct(&mut self) {

		// deterministic coin tossing

		for iter_index in 0..COMPRESSION_ITERATION_COUNT {

			if self.cores.len() < 2 { return; }

			let mut max_bit_length: usize = 0;
			let end = self.cores.len();
			let mut mut_iter = self.cores.iter_mut().rev();
			let mut rhs = mut_iter.next().unwrap();
			let mut index = 1;

			while index < end - iter_index {
				let lhs = mut_iter.next().unwrap();
				index += 1;

				
				rhs.compress(lhs);
				max_bit_length = cmp::max(max_bit_length, rhs.get_bit_count());
				rhs = lhs;
			}

			// println!("Compression iteration index {}. Max length is: {}", i, max_bit_length);
			// println!("Finding new cores.");
		}

		// be aware that the first N elements where N=COMPRESSION_ITERATION_COUNT is not removed from the Vec.
		// this is because drain/remove function compies all items in the right to left which makes the operation O(n).
	}

	/// This fuction returns labels of cores as u64 type.
	/// For some of the cores, u64 might not be enough to represent core,
	/// as the cores that has repetıtıve character can have great length.
	/// In that case, only 64 bits from right hand side will be used.
	/// # Arguments
	/// 
	/// * `self` - this function needs to access core.
	///
	pub fn get_small_cores(&self) -> Vec<u64> {
		let mut cores: Vec<u64> = vec![];
		for core in &self.cores {
			cores.push(core.encode());
		}
		cores
	}
}


#[cfg(test)]
mod tests;
