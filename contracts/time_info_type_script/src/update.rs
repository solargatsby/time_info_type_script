use ckb_std::{ckb_constants::Source};

use crate::error::*;
use crate::helper::{
    cell_args_check,
    get_script_hash_cell_count,
    get_timestamp_from_cell_data,
    input_cell_since_check,
    TIME_INFO_CELL_DATA_LEN,
    timestamp_check,
};

pub fn update(script_hash: [u8; 32]) -> Result<(), Error>{
    //should only one time info cell in inout
    if get_script_hash_cell_count(script_hash, Source::Input) != 1{
        return Err(Error::InvalidTimeInfoInput);
    }
    //should only one time info cell in output
    if get_script_hash_cell_count(script_hash, Source::Output) != 1{
        return Err(Error::InvalidTImeInfoOutput)
    }
    //check whether args of script of input not empty and equal args of output's
    cell_args_check(script_hash)?;

    let input_cell_data = crate::helper::load_cell_data(script_hash, Source::Input)?;
    if input_cell_data.len() != TIME_INFO_CELL_DATA_LEN as usize{
        return Err(Error::InvalidCellData)
    }
    let output_cell_data = crate::helper::load_cell_data(script_hash, Source::Output)?;
    if output_cell_data.len() != TIME_INFO_CELL_DATA_LEN as usize{
        return Err(Error::InvalidCellData)
    }

    let last_timestamp = get_timestamp_from_cell_data(&input_cell_data);
    let current_timestamp = get_timestamp_from_cell_data(&output_cell_data);
    timestamp_check(last_timestamp, current_timestamp)?;

    //check since of input cell in case time info update to early
    input_cell_since_check(last_timestamp)?;

    //time index in output cell should equal time index in input cell
    if output_cell_data[0] != input_cell_data[0] {
        return Err(Error::InvalidTimeIndex)
    }
    Ok(())
}