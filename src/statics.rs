// Encoding related static variables
pub static mut LABELS: [i32; 128] = [-1; 128];
pub static mut CHARACTERS: [char; 128] = [126 as char; 128];
pub static mut DICT_BIT_SIZE: usize = 0;
pub static mut ENCODING_INIT: bool = false;

// LCP algorithm related static variables
pub const COMPRESSION_ITERATION_COUNT: usize = 2;
pub const CORE_LENGTH: usize = 5; // 3 + 2 (COMPRESSION_ITERATION_COUNT)
pub const SIZE_PER_BLOCK: usize = 8;

// Other
pub static mut LOG_INIT: bool = false;