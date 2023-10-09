pub mod statics;
pub mod encoding;
pub mod core;
use crate::statics::COMPRESSION_ITERATION_COUNT;
use crate::statics::CORE_LENGTH;
use crate::statics::LABELS;
use crate::statics::ENCODING_INIT;
use crate::encoding::init_coefficients_default;
use crate::core::Core;
use std::cmp;
use std::collections::VecDeque;
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
    pub cores: VecDeque<Core>
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
                    cores: VecDeque::new()
                };
            }

            let mut index1: usize = 0;
            let mut index2: usize;
            let end = string.len();
            let read = string.as_bytes();
            let mut cores: VecDeque<Core> = VecDeque::new();

            while index1 + 1 < end && read[index1] == read[index1+1] { index1 += 1; }

            while index1 + 2 < end {
                if read[index1] == read[index1+1] { index1 += 1; continue; }
                
                // if there are same characters in subsequenct order such as xyyz, xyyyz, .... where x!=y and y!=z
                if LABELS[read[index1+1] as usize] == LABELS[read[index1+2] as usize] {
                    index2 = index1 + 3;

                    while index2 < end && read[index2-1] == read[index2] { index2 += 1; }

                    if index2 == end { break; }

                    index2 += 1;
                    cores.push_back( Core::from_str(index1, std::str::from_utf8(&read[index1..index2]).unwrap() ) );
                    index1 = index2 - 3;
                    continue;
                }

                // if there is no subsequent characters such as xyz where z!=y and y!=z

                if end < index1 + 3 { break; }

                if LABELS[read[index1 + 1] as usize] == LABELS[read[index1 + 2 ] as usize] {
                    index1 += 1;
                    continue;
                }

                if LABELS[read[index1 + 1] as usize] < LABELS[read[index1] as usize] && LABELS[read[index1 + 1] as usize] < LABELS[read[index1 + 2] as usize] ||                // local minima
                    (
                        LABELS[read[index1 + 1] as usize] > LABELS[read[index1] as usize] && LABELS[read[index1 + 1] as usize] > LABELS[read[index1 + 2] as usize] &&       // local maxima without immediate local minima neighbours
                        !(LABELS[read[index1] as usize] < LABELS[read[index1 + 1] as usize] && LABELS[read[index1] as usize] < LABELS[read[index1 - 1] as usize]) &&
                        !(LABELS[read[index1 + 2] as usize] < LABELS[read[index1 + 3] as usize] && LABELS[read[index1 + 2] as usize] < LABELS[read[index1 + 1] as usize]) 
                    ) 
                {
                    cores.push_back( Core::from_str(index1, std::str::from_utf8(&read[index1..(index1+3)]).unwrap() ) ); 
                }

                index1 += 1;
            }

            // this is done to make VecDeque elements stored contiguous. It can be removed.
            cores.make_contiguous();
            
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
    ///     1 - middle core's label is local minimum compared to its neighbors
    /// 
    ///     2 - middle core's label is local maximum compared to its neighbors while none if its neighbors are local minima
    /// 
    ///     3 - core contains multiple cores that has same label in the middle.
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
        let mut index1: usize = 0;
        let mut index2: usize;

        while index1 < end -1 && self.cores[index1] == self.cores[index1+1] { index1 += 1; }

        while index1 + 2 < end {

            if self.cores[index1] == self.cores[index1+1] { index1 += 1; continue; }
                
            // if there are same characters in subsequenct order such as xyyz, xyyyz, .... where x!=y and y!=z
            if self.cores[index1+1] == self.cores[index1+2] {

                index2 = index1 + 3;

                while index2 < end && self.cores[index2-1] == self.cores[index2] {
                    index2 += 1;
                }

                if index2 == end { break; }

                self.cores.push_back( Core::from_cores(index1, index2, &self.cores) ); 
                index1 += 1;

                continue;
            }

            // if there is no subsequent characters such as xyzuv where z!=y and y!=z and z!=u and u!=v
            index2 = index1 + 1;

            if index1 + CORE_LENGTH >= end { index1 += 1; continue; }

            while index2 < index1 + CORE_LENGTH {
                if self.cores[index2-1] == self.cores[index2] { break; }
                index2 += 1;
            }

            if index2 == index1 + CORE_LENGTH && self.cores[index2-1] != self.cores[index2] &&
            (
                self.cores[index1 + CORE_LENGTH - 2 ] < self.cores[index1 + CORE_LENGTH - 1 ] && self.cores[index1 + CORE_LENGTH - 2 ] < self.cores[index1 + CORE_LENGTH - 3 ] ||           // local minima
                (
                    self.cores[index1 + CORE_LENGTH - 2 ] > self.cores[index1 + CORE_LENGTH - 1 ] && self.cores[index1 + CORE_LENGTH - 2 ] > self.cores[index1 + CORE_LENGTH - 3 ] &&       // local maxima without immediate local minima neighbours
                    !(self.cores[index1 + CORE_LENGTH - 3 ] < self.cores[index1 + CORE_LENGTH - 2 ] && self.cores[index1 + CORE_LENGTH - 3 ] < self.cores[index1 + CORE_LENGTH - 4 ]) &&
                    !(self.cores[index1 + CORE_LENGTH - 1 ] < self.cores[index1 + CORE_LENGTH ] && self.cores[index1 + CORE_LENGTH - 1 ] < self.cores[index1 + CORE_LENGTH - 2 ]) 
                ) 
            ) 
            {
                self.cores.push_back( Core::from_cores(index1, index2, &self.cores) ); 
            }

            index1 += 1;
        }

        self.cores.drain(0..end);
        self.cores.make_contiguous();
        self.level += 1;
    }

    /// This fuction compresses each core with respect to its left neigbor. The total number of dct
    /// should be defined as static variable.
	/// # Arguments
	/// 
	/// * `self` - this function needs to access core.
	///
    pub fn dct(&mut self) {

        // deterministic cion tossing

        for _ in 0..COMPRESSION_ITERATION_COUNT {

            if self.cores.len() < 2 { return; }

            let mut max_bit_length: usize = 0;
            let end = self.cores.len();
            let mut mut_iter = self.cores.iter_mut().rev();
            let mut rhs = mut_iter.next().unwrap();
            let mut index = 1;

            while index < end {
                let lhs = mut_iter.next().unwrap();
                index += 1;

                
                rhs.compress(lhs);
                max_bit_length = cmp::max(max_bit_length, rhs.get_bit_count());
                rhs = lhs;
            }

            self.cores.pop_front();
            // println!("Compression iteration index {}. Max length is: {}", i, max_bit_length);
            // println!("Finding new cores.");
        }
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
