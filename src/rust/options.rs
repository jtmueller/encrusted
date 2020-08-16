#[derive(Debug)]
pub struct Options {
    pub save_dir: String,
    pub save_name: String,
    pub log_instructions: bool,
    pub rand_seed: [u8; 16],
}

impl Options {
    pub fn default() -> Options {
        Options {
            save_dir: String::new(),
            save_name: String::new(),
            log_instructions: false,
            rand_seed: [
                0xa8, 0x5b, 0xb3, 0x37, 0x1b, 0x36, 0xc8, 0x11, 0x72, 0x12, 0xfa, 0xd7, 0xea, 0xde,
                0xda, 0x8b,
            ],
        }
    }
}
