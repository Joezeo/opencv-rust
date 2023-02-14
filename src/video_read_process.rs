#![allow(unused_imports)]
#![allow(dead_code)]
use opencv::{
    highgui::{imshow, wait_key},
    imgproc::{cvt_color, COLOR_BGR2GRAY},
    prelude::*,
    videoio::{VideoCapture, CAP_ANY},
};
use crate::image_read_show::show_image;

pub fn video_read_process() {
    let mut vc = VideoCapture::default().unwrap();
    let open = vc.open_file("butterfly.mp4", CAP_ANY).unwrap();
    if open {
        let mut mat = Mat::default();
        // Read a frame.
        while let Ok(res) = vc.read(&mut mat) {
            if !res {
                break;
            }
            let mut dst_mat = Mat::default();
            cvt_color(&mut mat, &mut dst_mat, COLOR_BGR2GRAY, 0).unwrap();

            imshow("Video capture", &dst_mat).unwrap();
            if wait_key(20).unwrap() & 0xFF == 27 {
                break;
            }
        }
    } else {
        println!("Open video file `butterfly.mp4` failed.")
    }
}
