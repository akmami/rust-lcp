mod statics;
mod encoding;
mod core;
use crate::statics::COMPRESSION_ITERATION_COUNT;
use crate::statics::CORE_LENGTH;
use crate::encoding::COEFFICIENTS;
use crate::core::Core;
use std::cmp;
use std::collections::VecDeque;

pub struct String {
	level: u32,
	cores: VecDeque<Core>
}


impl String {

    pub fn new(string: &str, core_len: Option<u32>) -> Self {
        unsafe {
            if string.len() < 3 { panic!("Given string is too small!"); }

            let mut index1: usize = 0;
            let mut index2: usize;
            let end = string.len();
            let core_length = core_len.unwrap_or(CORE_LENGTH) as usize;
            let read = string.as_bytes();
            let mut cores: VecDeque<Core> = VecDeque::new();
            let mut min_value: i32;
            let mut max_value: i32;

            while index1 < end - 1 && read[index1] == read[index1+1] {
                index1 += 1;
            }

            while index1 + 2 < end {

                if read[index1] == read[index1+1] { index1 += 1; continue; }
                
                // if there are same characters in subsequenct order such as xyyz, xyyyz, .... where x!=y and y!=z
                if COEFFICIENTS[read[index1+1] as usize] == COEFFICIENTS[read[index1+2] as usize] {

                    index2 = index1 + 3;

                    while index2 < end && read[index2-1] == read[index2] {
                        index2 += 1;
                    }

                    if index2 == end { break; }

                    index2 += 1;
                    cores.push_back( Core::new(index1 as u32, index2 as u32, std::str::from_utf8(&read[index1..index2]).unwrap() ) );
                    index1 = index2 - 3;

                    continue;
                }

                // if there is no subsequent characters such as xyzuv where z!=y and y!=z and z!=u and u!=v
                min_value = COEFFICIENTS[read[index1] as usize];
                max_value = min_value;

                index2 = index1 + 1;

                if index1 + core_length >= end { index1 += 1; continue; }

                while index2 < index1 + core_length {
                    if read[index2-1] == read[index2] { break; }

                    if min_value > COEFFICIENTS[read[index2] as usize] { min_value = COEFFICIENTS[read[index2] as usize]; }
                    
                    if max_value < COEFFICIENTS[read[index2] as usize] { max_value = COEFFICIENTS[read[index2] as usize]; }

                    index2 += 1;
                }

                if index2 == index1 + core_length && 
                (
                    min_value == COEFFICIENTS[read[index1 + core_length / 2] as usize] ||               // local minima
                    (
                        max_value == COEFFICIENTS[read[index1 + core_length / 2] as usize] &&           // local maxima without immediate local minima neighbours
                        min_value != COEFFICIENTS[read[index1 + core_length / 2 - 1] as usize] && 
                        min_value != COEFFICIENTS[read[index1 + core_length / 2 + 1] as usize] 
                        ) 
                    ) 
                {
                    if min_value == -1 { index1 += 1; continue; }

                    cores.push_back( Core::new(index1 as u32, index2 as u32, std::str::from_utf8(&read[index1..index2]).unwrap() ) ); 
                    index1 = index1 + core_length / 2 - 1;
                }

                index1 += 1;
            }

            String {
                level: 1,
                cores: cores
            }
        }
    }


    pub fn deepen(&mut self, core_len: Option<u32>, vb: Option<bool>) {
        let core_length = core_len.unwrap_or(CORE_LENGTH) as usize;
        let verbose = vb.unwrap_or(false);

        // Compress cores

        for _ in 0..COMPRESSION_ITERATION_COUNT {

            let mut max_bit_length: u32 = 0;

            if self.cores.len() < 2 { return; }

            let mut next_iter = self.cores.iter().rev();
            next_iter.next();

            for rhs in self.cores.iter().rev() {
                let lhs = next_iter.next();
                
                if lhs == None { break; }

                rhs.compress(lhs.unwrap());

                max_bit_length = cmp::max(max_bit_length, rhs.get_bit_count());
            }

            self.cores.pop_front();

            if verbose { 
                println!("Compressed. Max length is: {}", max_bit_length);

                println!("Finding new cores."); 
            }
        }

              
        // Find cores from compressed cores.
        let end = self.cores.len();
        let mut index1: usize = 0;
        let mut index2: usize;

        while index1 < end -1 && self.cores[index1] == self.cores[index1+1] {
            index1 += 1;
        }


        while index1 + 2 < end {

            if self.cores[index1] == self.cores[index1+1] { index1 += 1; continue; }
                
            // if there are same characters in subsequenct order such as xyyz, xyyyz, .... where x!=y and y!=z
            if self.cores[index1+1] == self.cores[index1+2] {

                index2 = index1 + 3;

                while index2 < end && self.cores[index2-1] == self.cores[index2] {
                    index2 += 1;
                }

                if index2 == end { break; }

                index2 += 1;
                //cores.push_back( Core::new(index1 as u32, index2 as u32, std::str::from_utf8(&read[index1..index2]).unwrap() ) );
                index1 = index2 - 3;

                continue;
            }

            // if there is no subsequent characters such as xyzuv where z!=y and y!=z and z!=u and u!=v
            let mut min_value = self.cores[index1];
            let mut max_value = min_value;

            index2 = index1 + 1;

            if index1 + core_length >= end { index1 += 1; continue; }

            while index2 < index1 + core_length {
                if self.cores[index2-1] == self.cores[index2] { break; }

                if min_value > self.cores[index2] { min_value = self.cores[index2]; }
                
                if max_value < self.cores[index2] { max_value = self.cores[index2]; }

                index2 += 1;
            }

            if index2 == index1 + core_length && 
            (
                min_value == self.cores[index1 + core_length / 2] ||               // local minima
                (
                    max_value == self.cores[index1 + core_length / 2] &&           // local maxima without immediate local minima neighbours
                    min_value != self.cores[index1 + core_length / 2 - 1] && 
                    min_value != self.cores[index1 + core_length / 2 + 1] 
                    ) 
                ) 
            {
                // cores.push_back( Core::new(index1 as u32, index2 as u32, std::str::from_utf8(&read[index1..index2]).unwrap() ) ); 
                index1 = index1 + core_length / 2 - 1;
            }

            index1 += 1;
        }

        //this->cores.insert(this->cores.end(), temp_cores.begin(), temp_cores.end());
        //temp_cores.erase(temp_cores.begin(), temp_cores.end());

        self.level += 1;
    }


    pub fn get_small_cores(&self) -> Vec<u32> {
        let mut cores: Vec<u32> = vec![];
        for core in &self.cores {
            let mut value: u32 = 0;
            for block in core.get_blocks() {
                value = value << 8;
                value |= *block as u32;
            }
            cores.push(value);
        }
        cores
    }
}


#[cfg(test)]
mod tests;
