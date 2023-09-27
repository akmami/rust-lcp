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

            while index1 + 1 < end && read[index1] == read[index1+1] { index1 += 1; }

            while index1 + 2 < end {
                if read[index1] == read[index1+1] { index1 += 1; continue; }
                
                // if there are same characters in subsequenct order such as xyyz, xyyyz, .... where x!=y and y!=z
                if LABELS[read[index1+1] as usize] == LABELS[read[index1+2] as usize] {
                    index2 = index1 + 3;

                    while index2 < end && read[index2-1] == read[index2] { index2 += 1; }

                    if index2 == end { break; }

                    index2 += 1;
                    cores.push_back( Core::new(index1, index2, std::str::from_utf8(&read[index1..index2]).unwrap() ) );
                    index1 = index2 - 3;
                    continue;
                }

                // if there is no subsequent characters such as xyz where z!=y and y!=z

                if end < index1 + 3 { break; }

                if LABELS[read[index1 + 1] as usize] == LABELS[read[index1 + 2 ] as usize] {
                    index1 += 1;
                    continue;
                }

                if LABELS[read[index1 + 1] as usize] < LABELS[read[index1] as usize] && LABELS[read[index1 + 1] as usize] < LABELS[read[index1 + 2] as usize] ||           // local minima
                    (
                        LABELS[read[index1 + 1 ] as usize] > LABELS[read[index1 ] as usize] && LABELS[read[index1 + 1 ] as usize] > LABELS[read[index1 + 2 ] as usize] &&       // local maxima without immediate local minima neighbours
                        !(LABELS[read[index1] as usize] < LABELS[read[index1 + 1 ] as usize] && LABELS[read[index1] as usize] < LABELS[read[index1 - 1 ] as usize]) &&
                        !(LABELS[read[index1 + 2 ] as usize] < LABELS[read[index1 + 3 ] as usize] && LABELS[read[index1 + 2 ] as usize] < LABELS[read[index1 + 1 ] as usize]) 
                        ) 
                {
                    cores.push_back( Core::new(index1, index1+3, std::str::from_utf8(&read[index1..(index1+3)]).unwrap() ) ); 
                }

                index1 += 1;
            }

            cores.make_contiguous();

            String {
                level: 1,
                cores: cores
            }
        }
    }

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

                self.cores.push_back( Core::new3(index1, index2, &self.cores) ); 
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
                self.cores.push_back( Core::new3(index1, index2, &self.cores) ); 
            }

            index1 += 1;
        }

        self.cores.drain(0..end);
        self.cores.make_contiguous();
        self.level += 1;
    }

    pub fn dct(&mut self) {
        
        // deterministic cion tossing

        for _ in 0..COMPRESSION_ITERATION_COUNT {

            if self.cores.len() < 2 { return; }

            let mut max_bit_length: usize = 0;
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
    }

    pub fn extract_cores() {

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
