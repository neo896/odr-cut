use std::collections::HashSet;
use std::ffi::CString;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::os::raw::{c_char, c_float, c_int};

#[repr(C)]
struct RM_PositionData {
    x: c_float,
    y: c_float,
    z: c_float,
    h: c_float,
    p: c_float,
    r: c_float,
    h_relative: c_float,
    road_id: c_int,
    junction_id: c_int,
    lane_id: c_int,
    lane_offset: c_float,
    s: c_float,
}

#[link(name = "esminiRMLib")]
extern "C" {
    fn RM_CreatePosition() -> c_int;

    fn RM_Init(odr_file: *const c_char) -> c_int;

    fn RM_SetWorldPosition(
        handle: c_int,
        x: c_float,
        y: c_float,
        z: c_float,
        h: c_float,
        p: c_float,
        r: c_float,
    );
    fn RM_GetPositionData(handle: c_int, data: *mut RM_PositionData);
}

fn calculate_road_id(odr_path: &String, x: c_float, y: c_float) -> i32 {
    let mut rm_pos_data = RM_PositionData {
        x: 0.0,
        y: 0.0,
        z: 0.0,
        h: 0.0,
        p: 0.0,
        r: 0.0,
        h_relative: 0.0,
        road_id: 0,
        junction_id: 0,
        lane_id: 0,
        lane_offset: 0.0,
        s: 0.0,
    };
    unsafe {
        let odr_res = RM_Init(CString::new(odr_path.to_string()).unwrap().as_ptr());
        if odr_res == -1 {
            panic!("No xodr selected");
        }
        let rm_pos = RM_CreatePosition();
        RM_SetWorldPosition(rm_pos, x, y, 0.0, 0.0, 0.0, 0.0);
        RM_GetPositionData(rm_pos, &mut rm_pos_data);
        rm_pos_data.road_id
    }
}

pub fn cal_road_id(position_path: String, odr_path: String) -> Result<HashSet<String>, Error> {
    let f = File::open(position_path)?;
    let buffer = BufReader::new(f);
    let mut set = HashSet::new();
    for line in buffer.lines() {
        let line = line?;
        let x_y: Vec<&str> = line.split_whitespace().collect();
        let road_id = calculate_road_id(
            &odr_path,
            x_y[0].parse::<std::ffi::c_float>().unwrap(),
            x_y[1].parse::<std::ffi::c_float>().unwrap(),
        );
        set.insert(road_id.to_string());
    }
    Ok(set)
}
