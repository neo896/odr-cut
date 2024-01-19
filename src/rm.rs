use std::collections::HashSet;
use std::ffi::CString;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::os::raw::{c_char, c_float, c_int};
use std::time::Instant;


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
    ) -> ();
    fn RM_GetPositionData(handle: c_int, data: *mut RM_PositionData) -> ();
}

pub fn cal_road_id(
    position_path: String,
    odr_path: &String,
) -> Result<HashSet<String>, Box<dyn std::error::Error>> {
    println!("###### Finding Road.... ######");
    let start = Instant::now();
    let f = File::open(position_path)?;
    let buffer = BufReader::new(f);
    let mut set = HashSet::new();

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
    let odr_string = CString::new(odr_path.to_string()).unwrap();
    let odr_ptr = odr_string.as_ptr();

    unsafe {
        // init road
        let odr_res = RM_Init(odr_ptr);
        if odr_res == -1 {
            panic!("No xodr selected");
        }

        let rm_pos = RM_CreatePosition();

        for line in buffer.lines() {
            let line = line?;
            let x_y: Vec<&str> = line.split_whitespace().collect();
            let x = x_y[0].parse::<std::ffi::c_float>()?;
            let y = x_y[1].parse::<std::ffi::c_float>()?;
            RM_SetWorldPosition(rm_pos, x, y, 0.0, 0.0, 0.0, 0.0);
            RM_GetPositionData(rm_pos, &mut rm_pos_data);
            set.insert(rm_pos_data.road_id.to_string());
        }
    }
    let duration = start.elapsed();
    println!("find road id: {:?}, using time: {:?}", set, duration);
    println!();
    Ok(set)
}
