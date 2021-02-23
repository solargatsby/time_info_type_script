use ckb_std::{ckb_constants::Source};
use ckb_std::high_level::load_script;

use crate::error::*;
use crate::helper::{get_script_hash_cell_count, TIME_INFO_CELL_DATA_LEN, TIME_INFO_CELL_DATA_N};

pub fn create(script_hash: [u8; 32]) -> Result<(), Error> {
    //should only one time info cell in output
    if get_script_hash_cell_count(script_hash, Source::Output) != 1 {
        return Err(Error::InvalidTImeInfoOutput);
    }

    let script = load_script()?;
    //scrip args cannot empty
    if script.args().is_empty() {
        return Err(Error::InvalidArgument);
    }

    let output_cell_data = crate::helper::load_cell_data(script_hash, Source::Output)?;
    if output_cell_data.len() != TIME_INFO_CELL_DATA_LEN as usize {
        return Err(Error::InvalidCellData);
    }

    //time index cannot large then TIME_INFO_CELL_DATA_N
    let time_index = output_cell_data[0];
    if time_index >= TIME_INFO_CELL_DATA_N {
        return Err(Error::InvalidTimeIndex);
    }
    Ok(())
}