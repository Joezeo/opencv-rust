#![allow(unused_imports)]
#![allow(dead_code)]
use opencv::{
    highgui::{destroy_all_windows, imshow, wait_key},
    imgcodecs::{imread, IMREAD_COLOR, IMREAD_GRAYSCALE},
    prelude::*, core::{Size_, Point_, Rect, split, merge}, types::VectorOfMat,
};
use std::slice;

pub fn image_read_show() {
    let mut mat = imread("rust.jpeg", IMREAD_COLOR).unwrap();

    println!("{:?}", mat);
    let _data = unsafe { slice::from_raw_parts(mat.data(), mat.total()) };

    // ROI: 区域裁剪
    let _roi_mat = Mat::roi(&mat, Rect::new(0, 0, 200, 200)).unwrap();

    // RGB通道的分离与合并
    let mut splits = VectorOfMat::new();
    split(&mat, &mut splits).unwrap();
    println!("{:?}", splits);
    merge(&splits, &mut mat).unwrap();
    // let s = splits.get(0).unwrap();

    // 图像的显示
    // imshow("Rust", &s).unwrap();
    imshow("Rust", &mat).unwrap();

    // 等待时间，毫秒级，0表示任意时间终止
    wait_key(0).unwrap();

    destroy_all_windows().unwrap();
}

pub fn show_image(mat: &Mat) {
    imshow("Image Show", &mat).unwrap();

    // 等待时间，毫秒级，0表示任意时间终止
    wait_key(0).unwrap();

    destroy_all_windows().unwrap();
}
