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

const ERROR_TIME_INFO_ARGS: i8 = 54;
const ERROR_TIME_INFO_CELL_DATA: i8 = 55;
const ERROR_TIME_INFO_INPUT: i8 = 56;
const ERROR_TIME_INFO_OUTPUT: i8 = 57;
const ERROR_TIME_INFO_SINCE: i8 = 58;
const ERROR_TIME_INFO_TIMESTAMP: i8 = 59;
const ERROR_TIME_INFO_TIME_INDEX: i8 = 60;

const MAX_CYCLES: u64 = 10_000_000;
const TIME_INFO_CELL_DATA_N: u8 = 12;
const TIME_INFO_CELL_DATA_LEN: usize = 5;
const TIME_INFO_UPDATE_INTERVAL: u32 = 60; //s
const TIME_COST_A_ROUND: u32 = TIME_INFO_CELL_DATA_N as u32 * TIME_INFO_UPDATE_INTERVAL;

fn build_time_info_cell_data(index: u8, timestamp: u32) -> Bytes{
    let mut time_buf = BytesMut::with_capacity(TIME_INFO_CELL_DATA_LEN);
    time_buf.put_u8(index);
    time_buf.put_u32( timestamp);
    Bytes::from(time_buf.to_vec())
}

#[test]
fn test_success_update(){
    // deploy contract
    let mut context = Context::default();
    let contract_bin: Bytes = Loader::default().load_binary("time_info_type_script");
    let out_point = context.deploy_cell(contract_bin);
    let type_script = context.
        build_script(&out_point, out_point.as_bytes()).
        expect("script");
    let type_script_dep = CellDep::new_builder().
        out_point(out_point).
        build();

    // deploy always_success script
    let always_success_out_point = context.deploy_cell(ALWAYS_SUCCESS.clone());
    // prepare lock scripts
    let lock_script = context
        .build_script(&always_success_out_point, Default::default())
        .expect("script");
    let lock_script_dep = CellDep::new_builder()
        .out_point(always_success_out_point)
        .build();

    let now = Utc::now().timestamp() as u32;
    let time_index = 0;

    // prepare cells
    let input_out_point = context.create_cell(
        CellOutput::new_builder()
            .capacity(1000u64.pack())
            .lock(lock_script.clone())
            .type_(Some(type_script.clone()).pack())
            .build(),
        build_time_info_cell_data(time_index, now - TIME_COST_A_ROUND),
    );

    let since: u64 = 1 << 62;
    let input = CellInput::new_builder()
        .previous_output(input_out_point)
        .since((since + now as u64).pack())
        .build();
    let outputs = vec![
        CellOutput::new_builder()
            .capacity(500u64.pack())
            .lock(lock_script.clone())
            .type_(Some(type_script.clone()).pack())
            .build(),
    ];

    let outputs_data = vec![build_time_info_cell_data(time_index, now)];
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
fn test_error_invalid_input(){
    // deploy contract
    let mut context = Context::default();
    let contract_bin: Bytes = Loader::default().load_binary("time_info_type_script");
    let out_point = context.deploy_cell(contract_bin);
    let type_script = context.
        build_script(&out_point, Default::default()).
        expect("script");
    let type_script_dep = CellDep::new_builder().
        out_point(out_point.clone()).
        build();

    // deploy always_success script
    let always_success_out_point = context.deploy_cell(ALWAYS_SUCCESS.clone());
    // prepare lock scripts
    let lock_script = context
        .build_script(&always_success_out_point, out_point.as_bytes())
        .expect("script");
    let lock_script_dep = CellDep::new_builder()
        .out_point(always_success_out_point)
        .build();

    let now = Utc::now().timestamp() as u32;
    let time_index = 0;

    // prepare cells
    let input_out_point = context.create_cell(
        CellOutput::new_builder()
            .capacity(1000u64.pack())
            .lock(lock_script.clone())
            .type_(Some(type_script.clone()).pack())
            .build(),
        build_time_info_cell_data(time_index, now - TIME_COST_A_ROUND),
    );

    let since: u64 = 1 << 62;
    let intpus = vec![
        CellInput::new_builder()
            .previous_output(input_out_point.clone())
            .since((since + now as u64).pack())
            .build(),
        CellInput::new_builder()
            .previous_output(input_out_point.clone())
            .since((since + now as u64).pack())
            .build()
    ];
    let outputs = vec![
        CellOutput::new_builder()
            .capacity(500u64.pack())
            .lock(lock_script.clone())
            .type_(Some(type_script.clone()).pack())
            .build(),
    ];

    let outputs_data = vec![build_time_info_cell_data(time_index, now)];
    // build transaction
    let tx = TransactionBuilder::default()
        .inputs(intpus)
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
        ScriptError::ValidationFailure(ERROR_TIME_INFO_INPUT).input_type_script(0)
    );
}

#[test]
fn test_error_invalid_output(){
    // deploy contract
    let mut context = Context::default();
    let contract_bin: Bytes = Loader::default().load_binary("time_info_type_script");
    let out_point = context.deploy_cell(contract_bin);
    let type_script = context.
        build_script(&out_point, out_point.as_bytes()).
        expect("script");
    let type_script_dep = CellDep::new_builder().
        out_point(out_point.clone()).
        build();

    // deploy always_success script
    let always_success_out_point = context.deploy_cell(ALWAYS_SUCCESS.clone());
    // prepare lock scripts
    let lock_script = context
        .build_script(&always_success_out_point, Default::default())
        .expect("script");
    let lock_script_dep = CellDep::new_builder()
        .out_point(always_success_out_point)
        .build();

    let now = Utc::now().timestamp() as u32;
    let time_index = 0;

    // prepare cells
    let input_out_point = context.create_cell(
        CellOutput::new_builder()
            .capacity(1000u64.pack())
            .lock(lock_script.clone())
            .type_(Some(type_script.clone()).pack())
            .build(),
        build_time_info_cell_data(time_index, now - TIME_COST_A_ROUND),
    );

    let since: u64 = 1 << 62;
    let intpus = vec![
        CellInput::new_builder()
            .previous_output(input_out_point.clone())
            .since((since + now as u64).pack())
            .build(),
    ];
    let outputs = vec![
        CellOutput::new_builder()
            .capacity(500u64.pack())
            .lock(lock_script.clone())
            .type_(Some(type_script.clone()).pack())
            .build(),
        CellOutput::new_builder()
            .capacity(500u64.pack())
            .lock(lock_script.clone())
            .type_(Some(type_script.clone()).pack())
            .build(),
    ];

    let outputs_data = vec![
        build_time_info_cell_data(time_index, now),
        build_time_info_cell_data(time_index, now)];
    // build transaction
    let tx = TransactionBuilder::default()
        .inputs(intpus)
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
        ScriptError::ValidationFailure(ERROR_TIME_INFO_OUTPUT).input_type_script(0)
    );
}

#[test]
fn test_error_invalid_cell_data(){
    // deploy contract
    let mut context = Context::default();
    let contract_bin: Bytes = Loader::default().load_binary("time_info_type_script");
    let out_point = context.deploy_cell(contract_bin);
    let type_script = context.
        build_script(&out_point, out_point.as_bytes()).
        expect("script");
    let type_script_dep = CellDep::new_builder().
        out_point(out_point.clone()).
        build();

    // deploy always_success script
    let always_success_out_point = context.deploy_cell(ALWAYS_SUCCESS.clone());
    // prepare lock scripts
    let lock_script = context
        .build_script(&always_success_out_point, Default::default())
        .expect("script");
    let lock_script_dep = CellDep::new_builder()
        .out_point(always_success_out_point)
        .build();

    let now = Utc::now().timestamp() as u32;
    let time_index = 0;

    // prepare cells
    let input_out_point = context.create_cell(
        CellOutput::new_builder()
            .capacity(1000u64.pack())
            .lock(lock_script.clone())
            .type_(Some(type_script.clone()).pack())
            .build(),
        build_time_info_cell_data(time_index, now - TIME_COST_A_ROUND),
    );

    let since: u64 = 1 << 62;
    let intpus = vec![
        CellInput::new_builder()
            .previous_output(input_out_point.clone())
            .since((since + now as u64).pack())
            .build(),
    ];
    let outputs = vec![
        CellOutput::new_builder()
            .capacity(500u64.pack())
            .lock(lock_script.clone())
            .type_(Some(type_script.clone()).pack())
            .build(),
    ];

    let outputs_data = vec![Bytes::new()];
    // build transaction
    let tx = TransactionBuilder::default()
        .inputs(intpus)
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
        ScriptError::ValidationFailure(ERROR_TIME_INFO_CELL_DATA).input_type_script(0)
    );
}

#[test]
fn test_error_invalid_since(){
    // deploy contract
    let mut context = Context::default();
    let contract_bin: Bytes = Loader::default().load_binary("time_info_type_script");
    let out_point = context.deploy_cell(contract_bin);
    let type_script = context.
        build_script(&out_point, out_point.as_bytes()).
        expect("script");
    let type_script_dep = CellDep::new_builder().
        out_point(out_point.clone()).
        build();

    // deploy always_success script
    let always_success_out_point = context.deploy_cell(ALWAYS_SUCCESS.clone());
    // prepare lock scripts
    let lock_script = context
        .build_script(&always_success_out_point, Default::default())
        .expect("script");
    let lock_script_dep = CellDep::new_builder()
        .out_point(always_success_out_point)
        .build();

    let now = Utc::now().timestamp() as u32;
    let time_index = 0;

    // prepare cells
    let input_out_point = context.create_cell(
        CellOutput::new_builder()
            .capacity(1000u64.pack())
            .lock(lock_script.clone())
            .type_(Some(type_script.clone()).pack())
            .build(),
        build_time_info_cell_data(time_index, now - TIME_COST_A_ROUND),
    );

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

    let outputs_data = vec![build_time_info_cell_data(time_index, now)];
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
        ScriptError::ValidationFailure(ERROR_TIME_INFO_SINCE).input_type_script(0)
    );
}

#[test]
fn test_error_invalid_timestamp(){
    // deploy contract
    let mut context = Context::default();
    let contract_bin: Bytes = Loader::default().load_binary("time_info_type_script");
    let out_point = context.deploy_cell(contract_bin);
    let type_script = context.
        build_script(&out_point, out_point.as_bytes()).
        expect("script");
    let type_script_dep = CellDep::new_builder().
        out_point(out_point.clone()).
        build();

    // deploy always_success script
    let always_success_out_point = context.deploy_cell(ALWAYS_SUCCESS.clone());
    // prepare lock scripts
    let lock_script = context
        .build_script(&always_success_out_point, Default::default())
        .expect("script");
    let lock_script_dep = CellDep::new_builder()
        .out_point(always_success_out_point)
        .build();

    let now = Utc::now().timestamp() as u32;
    let time_index = 0;

    // prepare cells
    let input_out_point = context.create_cell(
        CellOutput::new_builder()
            .capacity(1000u64.pack())
            .lock(lock_script.clone())
            .type_(Some(type_script.clone()).pack())
            .build(),
        build_time_info_cell_data(time_index, now - TIME_COST_A_ROUND),
    );

    let since: u64 = 1 << 62;
    let input = CellInput::new_builder()
        .previous_output(input_out_point)
        .since((since + now as u64).pack())
        .build();
    let outputs = vec![
        CellOutput::new_builder()
            .capacity(500u64.pack())
            .lock(lock_script.clone())
            .type_(Some(type_script.clone()).pack())
            .build(),
    ];

    let outputs_data = vec![build_time_info_cell_data(time_index, now - TIME_INFO_UPDATE_INTERVAL)];
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
        ScriptError::ValidationFailure(ERROR_TIME_INFO_TIMESTAMP).input_type_script(0)
    );
}

#[test]
fn test_error_invalid_time_index(){
    // deploy contract
    let mut context = Context::default();
    let contract_bin: Bytes = Loader::default().load_binary("time_info_type_script");
    let out_point = context.deploy_cell(contract_bin);
    let type_script = context.
        build_script(&out_point, out_point.as_bytes()).
        expect("script");
    let type_script_dep = CellDep::new_builder().
        out_point(out_point.clone()).
        build();

    // deploy always_success script
    let always_success_out_point = context.deploy_cell(ALWAYS_SUCCESS.clone());
    // prepare lock scripts
    let lock_script = context
        .build_script(&always_success_out_point, Default::default())
        .expect("script");
    let lock_script_dep = CellDep::new_builder()
        .out_point(always_success_out_point)
        .build();

    let now = Utc::now().timestamp() as u32;
    let time_index = 0;

    // prepare cells
    let input_out_point = context.create_cell(
        CellOutput::new_builder()
            .capacity(1000u64.pack())
            .lock(lock_script.clone())
            .type_(Some(type_script.clone()).pack())
            .build(),
        build_time_info_cell_data(time_index+1, now - TIME_COST_A_ROUND),
    );

    let since: u64 = 1 << 62;
    let input = CellInput::new_builder()
        .previous_output(input_out_point)
        .since((since + now as u64).pack())
        .build();
    let outputs = vec![
        CellOutput::new_builder()
            .capacity(500u64.pack())
            .lock(lock_script.clone())
            .type_(Some(type_script.clone()).pack())
            .build(),
    ];

    let outputs_data = vec![build_time_info_cell_data(time_index, now)];
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
        ScriptError::ValidationFailure(ERROR_TIME_INFO_TIME_INDEX).input_type_script(0)
    );
}

#[test]
fn test_error_empty_args(){
    // deploy contract
    let mut context = Context::default();
    let contract_bin: Bytes = Loader::default().load_binary("time_info_type_script");
    let out_point = context.deploy_cell(contract_bin);
    let type_script = context.
        build_script(&out_point, Bytes::default()).
        expect("script");
    let type_script_dep = CellDep::new_builder().
        out_point(out_point.clone()).
        build();

    let type_script_out = context.build_script(&out_point, Bytes::default());

    // deploy always_success script
    let always_success_out_point = context.deploy_cell(ALWAYS_SUCCESS.clone());
    // prepare lock scripts
    let lock_script = context
        .build_script(&always_success_out_point, Default::default())
        .expect("script");
    let lock_script_dep = CellDep::new_builder()
        .out_point(always_success_out_point)
        .build();

    let now = Utc::now().timestamp() as u32;
    let time_index = 0;

    // prepare cells
    let input_out_point = context.create_cell(
        CellOutput::new_builder()
            .capacity(1000u64.pack())
            .lock(lock_script.clone())
            .type_(Some(type_script.clone()).pack())
            .build(),
        build_time_info_cell_data(time_index, now - TIME_COST_A_ROUND),
    );

    let since: u64 = 1 << 62;
    let input = CellInput::new_builder()
        .previous_output(input_out_point)
        .since((since + now as u64).pack())
        .build();
    let outputs = vec![
        CellOutput::new_builder()
            .capacity(500u64.pack())
            .lock(lock_script.clone())
            .type_(type_script_out.pack())
            .build(),
    ];

    let outputs_data = vec![build_time_info_cell_data(time_index, now)];
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
        ScriptError::ValidationFailure(ERROR_TIME_INFO_ARGS).input_type_script(0)
    );
}