use ckb_std::{ckb_constants::Source};

use crate::error::*;
use crate::helper::*;

pub fn update(script_hash: [u8; 32]) -> Result<(), Error>{
    input_cell_check(script_hash)?;
    output_cell_check(script_hash)?;
    update_cell_args_check(script_hash)?;

    let input_cell_data = crate::helper::load_cell_data(script_hash, Source::Input)?;
    cell_data_check(&input_cell_data)?;
    let output_cell_data = crate::helper::load_cell_data(script_hash, Source::Output)?;
    cell_data_check(&output_cell_data)?;

    let last_timestamp = get_timestamp_from_data(&input_cell_data);
    let current_timestamp = get_timestamp_from_data(&output_cell_data);
    timestamp_check(last_timestamp, current_timestamp)?;

    input_cell_since_check(last_timestamp)?;
    update_time_index_check(input_cell_data[0], output_cell_data[0])?;
    Ok(())
}