use chrono::*;
use ckb_testtool::{builtin::ALWAYS_SUCCESS, context::Context};
use ckb_tool::ckb_error::assert_error_eq;
use ckb_tool::ckb_script::ScriptError;
use ckb_tool::ckb_types::{
    bytes::{Bytes, BytesMut},
    core::TransactionBuilder,
    packed::*,
    prelude::*,
};
use ckb_tool::ckb_types::bytes::BufMut;

use super::*;

const MAX_CYCLES: u64 = 10_000_000;
const TIME_INFO_CELL_DATA_N: u8 = 12;
const TIME_INFO_CELL_DATA_LEN: usize = 5;

// error numbers
const ERROR_TIME_INFO_ARGS: i8 = 54;
const ERROR_TIME_INFO_CELL_DATA: i8 = 55;
const ERROR_TIME_INFO_OUTPUT: i8 = 57;
const ERROR_TIME_INFO_INDEX: i8 = 60;

fn build_time_info_cell_data(index: u8, timestamp: u32) -> Bytes{
    let mut time_buf = BytesMut::with_capacity(TIME_INFO_CELL_DATA_LEN);
    time_buf.put_u8(index);
    time_buf.put_u32( timestamp);
    Bytes::from(time_buf.to_vec())
}

#[test]
fn test_success_create() {
    let mut context = Context::default();
    // deploy always_success script
    let always_success_out_point = context.deploy_cell(ALWAYS_SUCCESS.clone());
    // prepare lock scripts
    let lock_script = context
        .build_script(&always_success_out_point, Default::default())
        .expect("script");
    let lock_script_dep = CellDep::new_builder()
        .out_point(always_success_out_point)
        .build();

    // prepare cells
    let input_out_point = context.create_cell(
        CellOutput::new_builder()
            .capacity(1000u64.pack())
            .lock(lock_script.clone())
            .build(),
        Bytes::new(),
    );

    // deploy contract
    let contract_bin: Bytes = Loader::default().load_binary("time_info_type_script");
    let out_point = context.deploy_cell(contract_bin);
    let type_script = context.
        build_script(&out_point, input_out_point.as_bytes()).
        expect("script");
    let type_script_dep = CellDep::new_builder().
        out_point(out_point).
        build();

    let input = CellInput::new_builder()
        .previous_output(input_out_point)
        .build();
    let outputs = vec![
        CellOutput::new_builder()
            .capacity(500u64.pack())
            .lock(lock_script.clone())
            .type_(Some(type_script.clone()).pack())
            .build(),
    ];

    let time_index = 1;
    let timestamp = Utc::now().timestamp() as u32;
    let outputs_data = vec![build_time_info_cell_data(time_index, timestamp)];

    // build transaction
    let tx = TransactionBuilder::default()
        .input(input)
        .outputs(outputs)
        .outputs_data(outputs_data.pack())
        .cell_dep(lock_script_dep)
        .cell_dep(type_script_dep)
        .build();
    let tx = context.complete_tx(tx);

    // run
    let cycles = context
        .verify_tx(&tx, MAX_CYCLES)
        .expect("pass verification");
    println!("consume cycles: {}", cycles);
}

#[test]
fn test_error_with_invalid_output() {
    // deploy contract
    let mut context = Context::default();
    // deploy always_success script
    let always_success_out_point = context.deploy_cell(ALWAYS_SUCCESS.clone());
    // prepare lock scripts
    let lock_script = context
        .build_script(&always_success_out_point, Default::default())
        .expect("script");
    let lock_script_dep = CellDep::new_builder()
        .out_point(always_success_out_point)
        .build();
    // prepare cells
    let input_out_point = context.create_cell(
        CellOutput::new_builder()
            .capacity(1000u64.pack())
            .lock(lock_script.clone())
            .build(),
        Bytes::new(),
    );

    let contract_bin: Bytes = Loader::default().load_binary("time_info_type_script");
    let out_point = context.deploy_cell(contract_bin);
    let type_script = context.
        build_script(&out_point, input_out_point.as_bytes()).
        expect("script");
    let type_script_dep = CellDep::new_builder().
        out_point(out_point).
        build();

    let input = CellInput::new_builder()
        .previous_output(input_out_point)
        .build();
    let outputs = vec![
        CellOutput::new_builder()
            .capacity(250u64.pack())
            .lock(lock_script.clone())
            .type_(Some(type_script.clone()).pack())
            .build(),
        CellOutput::new_builder()
            .capacity(250u64.pack())
            .lock(lock_script.clone())
            .type_(Some(type_script.clone()).pack())
            .build(),
    ];

    let mut buf = BytesMut::with_capacity(5);
    buf.put_u8(0);
    buf.put_u32(Utc::now().timestamp() as u32);
    let outputs_data = vec![Bytes::from(buf.to_vec()), Bytes::new()];

    // build transaction
    let tx = TransactionBuilder::default()
        .input(input)
        .outputs(outputs)
        .outputs_data(outputs_data.pack())
        .cell_dep(lock_script_dep)
        .cell_dep(type_script_dep)
        .build();
    let tx = context.complete_tx(tx);

    // run
    let err = context.verify_tx(&tx, MAX_CYCLES).unwrap_err();
    assert_error_eq!(
        err,
        ScriptError::ValidationFailure(ERROR_TIME_INFO_OUTPUT).output_type_script(0)
    );
}

#[test]
fn test_error_with_invalid_cell_data(){
// deploy contract
    let mut context = Context::default();
    // deploy always_success script
    let always_success_out_point = context.deploy_cell(ALWAYS_SUCCESS.clone());
    // prepare lock scripts
    let lock_script = context
        .build_script(&always_success_out_point, Default::default())
        .expect("script");
    let lock_script_dep = CellDep::new_builder()
        .out_point(always_success_out_point)
        .build();
    // prepare cells
    let input_out_point = context.create_cell(
        CellOutput::new_builder()
            .capacity(1000u64.pack())
            .lock(lock_script.clone())
            .build(),
        Bytes::new(),
    );

    let contract_bin: Bytes = Loader::default().load_binary("time_info_type_script");
    let out_point = context.deploy_cell(contract_bin);
    let type_script = context.
        build_script(&out_point, input_out_point.as_bytes()).
        expect("script");
    let type_script_dep = CellDep::new_builder().
        out_point(out_point).
        build();

    let input = CellInput::new_builder()
        .previous_output(input_out_point)
        .build();
    let outputs = vec![
        CellOutput::new_builder()
            .capacity(500u64.pack())
            .lock(lock_script.clone())
            .type_(Some(type_script).pack())
            .build(),
    ];

    let mut buf = BytesMut::with_capacity(5);
    buf.put_u8(0);
    let outputs_data = vec![Bytes::from(buf.to_vec())];

    // build transaction
    let tx = TransactionBuilder::default()
        .input(input)
        .outputs(outputs)
        .outputs_data(outputs_data.pack())
        .cell_dep(lock_script_dep)
        .cell_dep(type_script_dep)
        .build();
    let tx = context.complete_tx(tx);

    let err = context.verify_tx(&tx, MAX_CYCLES).unwrap_err();
    assert_error_eq!(
        err,
        ScriptError::ValidationFailure(ERROR_TIME_INFO_CELL_DATA).output_type_script(0)
    );
}

#[test]
fn test_error_with_invalid_time_index(){
    // deploy contract
    let mut context = Context::default();
    // deploy always_success script
    let always_success_out_point = context.deploy_cell(ALWAYS_SUCCESS.clone());
    // prepare lock scripts
    let lock_script = context
        .build_script(&always_success_out_point, Default::default())
        .expect("script");
    let lock_script_dep = CellDep::new_builder()
        .out_point(always_success_out_point)
        .build();

    // prepare cells
    let input_out_point = context.create_cell(
        CellOutput::new_builder()
            .capacity(1000u64.pack())
            .lock(lock_script.clone())
            .build(),
        Bytes::new(),
    );

    let contract_bin: Bytes = Loader::default().load_binary("time_info_type_script");
    let out_point = context.deploy_cell(contract_bin);
    let type_script = context.
        build_script(&out_point, input_out_point.as_bytes()).
        expect("script");
    let type_script_dep = CellDep::new_builder().
        out_point(out_point).
        build();

    let input = CellInput::new_builder()
        .previous_output(input_out_point)
        .build();
    let outputs = vec![
        CellOutput::new_builder()
            .capacity(500u64.pack())
            .lock(lock_script.clone())
            .type_(Some(type_script).pack())
            .build(),
    ];

    let mut buf = BytesMut::with_capacity(5);
    buf.put_u8(TIME_INFO_CELL_DATA_N * 2);
    buf.put_u32(Utc::now().timestamp() as u32);
    let outputs_data = vec![Bytes::from(buf.to_vec())];

    // build transaction
    let tx = TransactionBuilder::default()
        .input(input)
        .outputs(outputs)
        .outputs_data(outputs_data.pack())
        .cell_dep(lock_script_dep)
        .cell_dep(type_script_dep)
        .build();
    let tx = context.complete_tx(tx);

    // run
    let err = context.verify_tx(&tx, MAX_CYCLES).unwrap_err();
    assert_error_eq!(
        err,
        ScriptError::ValidationFailure(ERROR_TIME_INFO_INDEX).output_type_script(0)
    );
}

#[test]
fn test_error_with_invalid_args(){
    let mut context = Context::default();
    // deploy always_success script
    let always_success_out_point = context.deploy_cell(ALWAYS_SUCCESS.clone());
    // prepare lock scripts
    let lock_script = context
        .build_script(&always_success_out_point, Default::default())
        .expect("script");
    let lock_script_dep = CellDep::new_builder()
        .out_point(always_success_out_point)
        .build();

    // prepare cells
    let input_out_point = context.create_cell(
        CellOutput::new_builder()
            .capacity(1000u64.pack())
            .lock(lock_script.clone())
            .build(),
        Bytes::new(),
    );

    // deploy contract
    let contract_bin: Bytes = Loader::default().load_binary("time_info_type_script");
    let out_point = context.deploy_cell(contract_bin);
    let type_script = context.
        build_script(&out_point, Bytes::default()).
        expect("script");
    let type_script_dep = CellDep::new_builder().
        out_point(out_point).
        build();

    let input = CellInput::new_builder()
        .previous_output(input_out_point)
        .build();
    let outputs = vec![
        CellOutput::new_builder()
            .capacity(500u64.pack())
            .lock(lock_script.clone())
            .type_(Some(type_script).pack())
            .build(),
    ];

    let mut buf = BytesMut::with_capacity(5);
    buf.put_u8(0);
    buf.put_u32(Utc::now().timestamp() as u32);
    let outputs_data = vec![Bytes::from(buf.to_vec())];

    // build transaction
    let tx = TransactionBuilder::default()
        .input(input)
        .outputs(outputs)
        .outputs_data(outputs_data.pack())
        .cell_dep(lock_script_dep)
        .cell_dep(type_script_dep)
        .build();
    let tx = context.complete_tx(tx);

    // run
    let err = context.verify_tx(&tx, MAX_CYCLES).unwrap_err();
    assert_error_eq!(
        err,
        ScriptError::ValidationFailure(ERROR_TIME_INFO_ARGS).output_type_script(0)
    );
}
