use std::{fs, time::SystemTime};

use crate::draw::canvas::Canvas;

pub fn write_to_file(c: &Canvas, filename_prefix: &str) {
    let ppm_data = c.ppm();
    let filename = format!(
        "{}-{}.ppm",
        filename_prefix,
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("time went backwards")
            .as_secs()
    );
    fs::write(&filename, ppm_data).expect("unable to write file")
}
