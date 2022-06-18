#![feature(is_sorted)]
use std::path::PathBuf;

use serde::Deserialize;



#[derive(Default, Deserialize, Debug, PartialEq, Eq)]
pub enum LoadingStyle {
    #[default]
    Unknown,
    LIOS,
    FIOS,
    Merge,
    Duplicate
    // add "special snowflakes"
}
const FOLDER_BEHAVIOR: [(&str, LoadingStyle); 1] = { 
    use LoadingStyle::*;
    let arr = [
        ("achievements", Unknown)
    ];
    assert!(arr.is_sorted_by_key(|w| w.0));
    arr
};

struct Mod {
    path: PathBuf,
    remoate_file_id: u64,
    name: String // should maybe be just bytes, 
                 // since it's in the ACP I think
}

struct ModSet<'a> {
    mod_indices: Vec<&'a Mod>
}