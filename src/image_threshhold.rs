#![allow(unused_imports)]
#![allow(dead_code)]
use opencv::{
    highgui::{destroy_all_windows, imshow, wait_key},
    imgcodecs::{imread, IMREAD_COLOR, IMREAD_GRAYSCALE},
    imgproc::{
        threshold, THRESH_BINARY, THRESH_BINARY_INV, THRESH_TOZERO, THRESH_TOZERO_INV, THRESH_TRUNC,
    },
    prelude::*,
};

pub fn image_threshhold() {
    let mat = imread("rust.jpeg", IMREAD_GRAYSCALE).unwrap();

    let mut dst = Mat::default();
    // 各个typ的含义
    // THRESH_TOZERO: 大于阈值部分不改变，否则设为0
    // THRESH_TOZERO_INV: THRESH_TOZERO反转
    // THRESH_BINARY: 超过阈值取最大值，否则取0
    // THRESH_BINARY_INV: THRESH_BINARY反转
    // THRESH_TRUNC: 大于阈值部分设为阈值，否则不变
    // THRESH_TRUNC_INV: THRESH_TRUNC反转
    threshold(&mat, &mut dst, 127., 255., THRESH_TRUNC).unwrap();

    imshow("Image Threshhold", &dst).unwrap();

    wait_key(0).unwrap();

    destroy_all_windows().unwrap();
}
