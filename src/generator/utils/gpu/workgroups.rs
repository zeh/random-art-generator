// TODO: make this dynamic
pub fn get_workgroup_width() -> u32 {
	16
}

// TODO: make this dynamic
pub fn get_workgroup_height() -> u32 {
	16
}

// TODO: make this dynamic
pub fn get_workgroup_depth() -> u32 {
	1
}

pub fn get_workgroup_count_width(dimension: u32) -> u32 {
	(dimension as f64 / get_workgroup_width() as f64).ceil() as u32
}

pub fn get_workgroup_count_height(dimension: u32) -> u32 {
	(dimension as f64 / get_workgroup_height() as f64).ceil() as u32
}

pub fn get_workgroup_count_depth(dimension: u32) -> u32 {
	(dimension as f64 / get_workgroup_depth() as f64).ceil() as u32
}
