use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};

// What the values do exactly is explained in README.md.

#[derive(Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "colored_board_default")]
    pub colored_board: bool,

    #[serde(default = "modern_piece_rng_default")]
    pub modern_piece_rng: bool,

    #[serde(default = "bag_amount_default")]
    pub bag_amount: u8,

    #[serde(default = "first_piece_no_overhang_default")]
    pub first_piece_no_overhang: bool,

    #[serde(default = "holding_enabled_default")]
    pub holding_enabled: bool,
}

fn colored_board_default() -> bool {
    true
}
fn modern_piece_rng_default() -> bool {
    true
}
fn bag_amount_default() -> u8 {
    5
}
fn first_piece_no_overhang_default() -> bool {
    true
}
fn holding_enabled_default() -> bool {
    true
}

impl Default for Config {
    fn default() -> Self {
        Self {
            colored_board: colored_board_default(),
            modern_piece_rng: modern_piece_rng_default(),
            bag_amount: bag_amount_default(),
            first_piece_no_overhang: first_piece_no_overhang_default(),
            holding_enabled: holding_enabled_default(),
        }
    }
}

pub fn load_config() -> Config {
    if !Path::new("./config.json").exists() {
        println!(
            "Create a config.json file to configure this game. Using default settings for now..."
        );

        return Config::default();
    }

    let json_file = fs::read_to_string("./config.json").unwrap();

    let c: Config = serde_json::from_str(&json_file).unwrap();

    Config {
        colored_board: c.colored_board,
        modern_piece_rng: c.modern_piece_rng,
        bag_amount: c.bag_amount,
        first_piece_no_overhang: c.first_piece_no_overhang,
        holding_enabled: c.holding_enabled,
    }
}
