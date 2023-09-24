mod statics;
mod encoding;
mod core;
use crate::statics::COMPRESSION_ITERATION_COUNT;
use crate::statics::CORE_LENGTH;
use crate::statics::LABELS;
use crate::core::Core;
use std::cmp;
use std::collections::VecDeque;
use log::{info, error};


pub struct String {
    level: u32,
    cores: VecDeque<Core>
}


impl String {

    pub fn new(string: &str) -> Self {
        unsafe {
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
            let mut min_value: i32;
            let mut max_value: i32;

            while index1 < end - 1 && read[index1] == read[index1+1] {
                index1 += 1;
            }

            while index1 + 2 < end {

                if read[index1] == read[index1+1] { index1 += 1; continue; }
                
                // if there are same characters in subsequenct order such as xyyz, xyyyz, .... where x!=y and y!=z
                if LABELS[read[index1+1] as usize] == LABELS[read[index1+2] as usize] {

                    index2 = index1 + 3;

                    while index2 < end && read[index2-1] == read[index2] {
                        index2 += 1;
                    }

                    if index2 == end { break; }

                    index2 += 1;
                    cores.push_back( Core::new(index1 as u32, index2 as u32, std::str::from_utf8(&read[index1..index2]).unwrap() ) );
                    println!("found {} at index {}", std::str::from_utf8(&read[index1..index2]).unwrap(), index1);
                    index1 = index2 - 3;
                    
                    continue;
                }
                index2 = index1 + 5;
                // if there is no subsequent characters such as xyzuv where z!=y and y!=z and z!=u and u!=v
                println!("Processing {}", std::str::from_utf8(&read[index1..index2]).unwrap());
                min_value = LABELS[read[index1] as usize];
                max_value = min_value;

                index2 = index1 + 1;

                if index1 + CORE_LENGTH >= end { index1 += 1; continue; }

                while index2 < index1 + CORE_LENGTH {
                    if read[index2-1] == read[index2] { break; }

                    if min_value > LABELS[read[index2] as usize] { min_value = LABELS[read[index2] as usize]; }
                    
                    if max_value < LABELS[read[index2] as usize] { max_value = LABELS[read[index2] as usize]; }

                    index2 += 1;
                }

                if index2 == index1 + CORE_LENGTH && 
                (
                    min_value == LABELS[read[index1 + CORE_LENGTH / 2] as usize] ||               // local minima
                    (
                        max_value == LABELS[read[index1 + CORE_LENGTH / 2] as usize] &&           // local maxima without immediate local minima neighbours
                        min_value != LABELS[read[index1 + CORE_LENGTH / 2 - 1] as usize] && 
                        min_value != LABELS[read[index1 + CORE_LENGTH / 2 + 1] as usize] 
                        ) 
                    ) 
                {

                    if min_value == -1 { index1 += 1; continue; }

                    cores.push_back( Core::new(index1 as u32, index2 as u32, std::str::from_utf8(&read[index1..index2]).unwrap() ) ); 
                    println!("found {} at index {}", std::str::from_utf8(&read[index1..index2]).unwrap(), index1);
                    index1 = index1 + CORE_LENGTH / 2 - 1;
                }

                index1 += 1;
            }

            String {
                level: 1,
                cores: cores
            }
        }
    }


    pub fn deepen(&mut self) {

        // Compress cores

        for _ in 0..COMPRESSION_ITERATION_COUNT {

            let mut max_bit_length: usize = 0;

            if self.cores.len() < 2 { return; }

            let end = self.cores.len();

            let mut mut_iter = self.cores.iter_mut().rev();
            let mut lhs = mut_iter.next().unwrap();
            
            let mut index = 1;

            while index < end {
                let rhs = mut_iter.next().unwrap();

                index += 1;
                
                rhs.compress(lhs);

                max_bit_length = cmp::max(max_bit_length, rhs.get_bit_count());

                lhs = rhs;
            }

            self.cores.pop_front();

            info!("Compressed. Max length is: {}", max_bit_length);

            info!("Finding new cores."); 
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
            let mut min_value = &self.cores[index1];
            let mut max_value = min_value;

            index2 = index1 + 1;

            if index1 + CORE_LENGTH >= end { index1 += 1; continue; }

            while index2 < index1 + CORE_LENGTH {
                if self.cores[index2-1] == self.cores[index2] { break; }

                if min_value > &self.cores[index2] { min_value = &self.cores[index2]; }
                
                if max_value < &self.cores[index2] { max_value = &self.cores[index2]; }

                index2 += 1;
            }

            if index2 == index1 + CORE_LENGTH && 
            (
                min_value == &self.cores[index1 + CORE_LENGTH / 2] ||               // local minima
                (
                    max_value == &self.cores[index1 + CORE_LENGTH / 2] &&           // local maxima without immediate local minima neighbours
                    min_value != &self.cores[index1 + CORE_LENGTH / 2 - 1] && 
                    min_value != &self.cores[index1 + CORE_LENGTH / 2 + 1] 
                    ) 
                ) 
            {
                // cores.push_back( Core::new(index1 as u32, index2 as u32, std::str::from_utf8(&read[index1..index2]).unwrap() ) ); 
                index1 = index1 + CORE_LENGTH / 2 - 1;
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
