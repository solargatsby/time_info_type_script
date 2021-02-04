use core::result::Result;
use ckb_std::high_level::{load_cell, load_script_hash, QueryIter};
use ckb_std::ckb_constants::Source;
use ckb_std::{ckb_types::{prelude::*}};
use crate::error::Error;
use crate::hash::blake2b_256;
use crate::create::create;
use crate::update::update;

pub fn main() -> Result<(), Error> {
    let script_hash = load_script_hash()?;
    if QueryIter::new(load_cell, Source::Input).
        any(|cell|blake2b_256(cell.type_().as_slice()) == script_hash){
        update(script_hash)
    }else{
        create(script_hash)
    }
}

