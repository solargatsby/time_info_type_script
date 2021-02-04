use ckb_std::{ckb_constants::Source};
use crate::error::*;
use crate::helper::*;

pub const TIME_INFO_CELL_DATA_LEN : u8 = 5;
pub const TIME_INFO_CELL_DATA_N: u8 = 12;
pub const TIME_INFO_UPDATE_INTERVAL: u32 = 60; //s

pub fn create(script_hash: [u8; 32]) -> Result<(), Error>{
    output_cell_check(script_hash)?;
    create_cell_args_check()?;

    let output_cell_data = crate::helper::load_cell_data(script_hash, Source::Output)?;
    cell_data_check(&output_cell_data)?;

    let time_index = output_cell_data[0];
    create_time_index_check(time_index)?;
    Ok(())
}