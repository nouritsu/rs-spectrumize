use rand::{distributions::Alphanumeric, thread_rng, Rng};
use serde::{Deserialize, Serialize};

pub struct ConfigHandler;

#[derive(Serialize, Deserialize)]
pub struct Config {
    username: String,
    public: bool,
    input_mode: InputMode,
}

#[derive(Serialize, Deserialize)]
pub enum InputMode {
    RGB,
    Hex,
}

impl Default for Config {
    fn default() -> Self {
        let rand_str: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(5)
            .map(char::from)
            .collect();

        Self {
            username: "guest".to_string() + &rand_str,
            public: true,
            input_mode: InputMode::RGB,
        }
    }
}
