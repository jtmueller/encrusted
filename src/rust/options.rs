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
            rand_seed: [0; 16],
        }
    }
}
