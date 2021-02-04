use crate::error::Error;
use crate::create::*;
use alloc::{vec::Vec};
use ckb_std::ckb_types::{bytes::Bytes, prelude::*};
use ckb_std::{high_level::*, ckb_constants::Source};
use ckb_std::error::SysError;

pub fn cell_data_check(cell_data: &Vec<u8>)->Result<(), Error>{
    if cell_data.len() != TIME_INFO_CELL_DATA_LEN as usize{
        return Err(Error::InvalidCellData)
    }
    Ok(())
}

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

pub fn load_cell_data(script_hash:[u8; 32], source: Source) -> Result<Vec<u8>, SysError>{
    let cell_index = QueryIter::new(load_cell_type_hash, source).
        position(|type_script|type_script.unwrap() == script_hash).unwrap();

    ckb_std::high_level::load_cell_data(cell_index, source)
}

pub fn get_timestamp_from_data(cell_data: &Vec<u8>) -> u32{
    let mut buf = [0_u8; 4];
    buf.copy_from_slice(&(cell_data.as_slice()[1..]));
    u32::from_be_bytes(buf)
}

pub fn create_cell_args_check() -> Result<(), Error>{
    let script= load_script()?;
    if script.args().is_empty(){
        return Err(Error::InvalidArgument)
    }
    Ok(())
}

pub fn update_cell_args_check(script_hash: [u8; 32]) -> Result<(),Error>{
    let script= load_script()?;
    let script_args: Bytes = script.args().unpack();
    if script_args.is_empty(){
        return Err(Error::InvalidArgument)
    }

    let cell_index = QueryIter::new(load_cell_type_hash, Source::Input).
        position(|type_script|type_script.unwrap() == script_hash).unwrap();
    let input_cell_data = load_cell(cell_index, Source::Input)?;
    let input_script = input_cell_data.type_().to_opt().unwrap();
    let input_script_args: Bytes = input_script.args().unpack();

    if input_script_args[..] != script_args[..] {
        return Err(Error::InvalidArgument)
    }
    Ok(())
}

pub fn input_cell_check(script_hash: [u8; 32]) -> Result<(), Error>{
    if get_script_hash_cell_count(script_hash, Source::Input) != 1{
        return Err(Error::InvalidTimeInfoInput);
    }
    Ok(())
}

pub fn output_cell_check(script_hash: [u8; 32]) -> Result<(), Error>{
    //should only one time info cell in output
    if get_script_hash_cell_count(script_hash, Source::Output) != 1{
        return Err(Error::InvalidTImeInfoOutput)
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

pub fn create_time_index_check(time_index: u8) -> Result<(), Error>{
    if time_index >= TIME_INFO_CELL_DATA_N{
        return Err(Error::InvalidTimeIndex)
    }
    Ok(())
}

pub fn update_time_index_check(input_time_index: u8, output_time_index: u8) -> Result<(), Error>{
    if output_time_index != input_time_index {
        return Err(Error::InvalidTimeIndex)
    }
    Ok(())
}