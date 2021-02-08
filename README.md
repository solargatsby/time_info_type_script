# time_info_type_script

[![License](https://img.shields.io/badge/license-MIT-green)](https://github.com/solargatsby/time_info_type_script/blob/main/COPYING)
[![Github Actions CI](https://github.com/solargatsby/time_info_type_script/workflows/CI/badge.svg)](https://github.com/solargatsby/time_info_type_script/actions)

The type script of timestamp info cell on Nervos CKB using [Capsule](https://github.com/nervosnetwork/capsule).

In order to resolve the problem how get current timestamp in script, we design two timestamp scripts.
They are time_info_type_script and [time_index_state_type_script](https://github.com/solargatsby/time_index_state_type_script).

They're 12 time info cell, each cell has a index, from 0 to 11. Every time info cell record the timestamp at update.
The time info cell will be update by index, and the update interval is one minute.

the cell data of time info cell is `time_info_cell_data = index as u8 | timestamp as u64`.

If when you want to get the current timestamp in script, you should first the current index of time info cell by time index cell,
the time index cell also update with time info cell.

the cell data of time index cell is `time_index_state_cell_data = index as u8 | N as u8`.

then, get the time info cell by index in time index cell.

At last, attach the time info cell as cell deps in custom script, for example:

```
// transaction structure
{
    cell_deps[
        {
            out_point: time_info_cell_out_point,
            dep_type: code
        }
    ]
    inputs
    outputs
    ...
}

// get the timestamp from time info cell data
let data: Bytes = load_cell_data(0, Source::CellDep)?
let timestamp = data[1:]
```

### Pre-requirement

- [capsule](https://github.com/nervosnetwork/capsule) >= 0.4.3
- [ckb-cli](https://github.com/nervosnetwork/ckb-cli) >= 0.35.0

Build contracts:

``` sh
capsule build
```

Run tests:

``` sh
capsule test
```
