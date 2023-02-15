#![allow(unused_imports)]
#![allow(dead_code)]
use opencv::{
    core::{Point, Size, BORDER_CONSTANT},
    highgui::{destroy_all_windows, imshow, wait_key},
    imgcodecs::{imread, IMREAD_COLOR},
    imgproc::{
        get_structuring_element, morphology_default_border_value, morphology_ex, MORPH_RECT, MORPH_OPEN, MORPH_CLOSE,
    },
    prelude::*,
};

pub fn image_open_close_operation() {
    let mat = imread("opencv_bi.jpeg", IMREAD_COLOR).unwrap();

    let kernel = get_structuring_element(MORPH_RECT, Size::new(3, 3), Point::new(-1, -1)).unwrap();

    // 开运算： 先腐蚀 再膨胀
    let mut mat_open = Mat::default();
    morphology_ex(
        &mat,
        &mut mat_open,
        MORPH_OPEN,
        &kernel,
        Point::new(-1, -1),
        1,
        BORDER_CONSTANT,
        morphology_default_border_value().unwrap(),
    )
    .unwrap();

    // 闭运算： 先膨胀 再腐蚀
    let mut mat_close = Mat::default();
    morphology_ex(
        &mat,
        &mut mat_close,
        MORPH_CLOSE,
        &kernel,
        Point::new(-1, -1),
        1,
        BORDER_CONSTANT,
        morphology_default_border_value().unwrap(),
    )
    .unwrap();

    imshow("Image Origin", &mat).unwrap();
    imshow("Image Open", &mat_open).unwrap();
    imshow("Image Close", &mat_close).unwrap();
    wait_key(0).unwrap();
    destroy_all_windows().unwrap();
}
