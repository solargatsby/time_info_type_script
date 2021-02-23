use core::result::Result;

use ckb_std::ckb_constants::Source;
use ckb_std::high_level::{load_cell_type_hash, load_script_hash, QueryIter};

use crate::create::create;
use crate::error::Error;
use crate::update::update;

pub fn main() -> Result<(), Error> {
    let script_hash = load_script_hash()?;
    if QueryIter::new(load_cell_type_hash, Source::Input).
        any(|cell_hash| {
            match cell_hash {
                Some(type_hash) => type_hash == script_hash,
                None => return false
            }
        }) {
        update(script_hash)
    } else {
        create(script_hash)
    }
}

