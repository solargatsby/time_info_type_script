use alloc::{vec::Vec};

use ckb_std::{ckb_constants::Source, high_level::*};
use ckb_std::ckb_types::{bytes::Bytes, prelude::*};

use crate::error::Error;

pub const TIME_INFO_CELL_DATA_LEN : u8 = 5;
pub const TIME_INFO_CELL_DATA_N: u8 = 12;
pub const TIME_INFO_UPDATE_INTERVAL: u32 = 60; //s

pub fn get_script_hash_cell_count(script_hash: [u8; 32], source: Source) -> usize{
    QueryIter::new(load_cell_type_hash, source).
        filter(|type_hash| {
            match type_hash{
                Some(type_script_hash) => *type_script_hash == script_hash,
                None => false
            }
        }).
        count()
}

pub fn get_position_of_cell_with_type_script(
    script_hash: [u8; 32],
    source: Source,
) -> Option<usize> {
    QueryIter::new(load_cell_type_hash, source).position(|type_script_op| match type_script_op {
        Some(type_script) => type_script == script_hash,
        None => false,
    })
}

pub fn load_cell_data(script_hash: [u8; 32], source: Source) -> Result<Vec<u8>, Error> {
    let cell_index = match get_position_of_cell_with_type_script(script_hash, source) {
        Some(position) => position,
        None => return match source {
            Source::Input | Source::GroupInput => Err(Error::InvalidTimeInfoInput),
            Source::Output | Source::GroupOutput => Err(Error::InvalidTImeInfoOutput),
            _ => Err(Error::ItemMissing),
        },
    };
    match ckb_std::high_level::load_cell_data(cell_index, source) {
        Ok(cell_data) => Ok(cell_data),
        Err(sys_err) => Err(Error::from(sys_err)),
    }
}

pub fn get_timestamp_from_cell_data(cell_data: &Vec<u8>) -> u32{
    let mut buf = [0_u8; 4];
    buf.copy_from_slice(&(cell_data.as_slice()[1..]));
    u32::from_be_bytes(buf)
}

pub fn cell_args_check(script_hash: [u8; 32]) -> Result<(),Error>{
    let script= load_script()?;
    let script_args: Bytes = script.args().unpack();
    if script_args.is_empty(){
        return Err(Error::InvalidArgument)
    }

    let cell_index = match get_position_of_cell_with_type_script(script_hash, Source::Input){
        Some(position) => position,
        None => return Err(Error::InvalidTimeInfoInput),
    };
    let input_cell_data = load_cell(cell_index, Source::Input)?;
    let input_script = match input_cell_data.type_().to_opt() {
        Some(type_script) => type_script,
        None => return Err(Error::InvalidTimeInfoInput),
    };
    let input_script_args: Bytes = input_script.args().unpack();

    if input_script_args[..] != script_args[..] {
        return Err(Error::InvalidArgument)
    }
    Ok(())
}

pub fn timestamp_check(last_timestamp: u32, current_timestamp: u32) -> Result<(), Error>{
    let time_cost_of_a_round = TIME_INFO_CELL_DATA_N as u32 * TIME_INFO_UPDATE_INTERVAL;
    if current_timestamp <= last_timestamp + time_cost_of_a_round || current_timestamp > last_timestamp + time_cost_of_a_round * 2{
        return Err(Error::InvalidTimestamp)
    }
    Ok(())
}

pub fn input_cell_since_check(last_timestamp: u32) -> Result<(), Error>{
    let time_cost_of_a_round = TIME_INFO_CELL_DATA_N as u32 * TIME_INFO_UPDATE_INTERVAL;
    let since_base: u64 = 1 << 62;
    if QueryIter::new(load_input_since, Source::GroupInput).
        any(|since| since != since_base  + (last_timestamp + time_cost_of_a_round + TIME_INFO_UPDATE_INTERVAL) as u64){
        return Err(Error::InvalidTimeSince)
    }
    Ok(())
}