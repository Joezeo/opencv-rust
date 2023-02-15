#![allow(unused_imports)]
#![allow(dead_code)]
use crate::opencv_util::imshow_many;
use opencv::{
    core::{
        add_weighted, convert_scale_abs, BORDER_DEFAULT, CV_16S, CV_16U, CV_32F, CV_64F, CV_8U,
        CV_8UC3,
    },
    highgui::{destroy_all_windows, imshow, wait_key},
    imgcodecs::{imread, IMREAD_COLOR},
    imgproc::sobel,
    prelude::*,
};

pub fn image_sobel() {
    let mat = imread("github_binary.jpeg", IMREAD_COLOR).unwrap();

    // Sobel x
    let mut mat_sobelx = Mat::default();
    sobel(
        &mat,
        &mut mat_sobelx,
        CV_64F,
        1,
        0,
        3,
        1.,
        0.,
        BORDER_DEFAULT,
    )
    .unwrap();

    let mut mat_sobelx_abs = Mat::default();
    convert_scale_abs(&mat_sobelx, &mut mat_sobelx_abs, 1., 0.).unwrap();

    // Sobel y
    let mut mat_sobely = Mat::default();
    sobel(
        &mat,
        &mut mat_sobely,
        CV_64F,
        0,
        1,
        3,
        1.,
        0.,
        BORDER_DEFAULT,
    )
    .unwrap();

    let mut mat_sobely_abs = Mat::default();
    convert_scale_abs(&mat_sobely, &mut mat_sobely_abs, 1., 0.).unwrap();

    // Gx\Gy 相加
    let mut sobeled = Mat::default();
    add_weighted(
        &mat_sobelx_abs,
        0.5,
        &mat_sobely_abs,
        0.5,
        0.,
        &mut sobeled,
        -1,
    )
    .unwrap();

    imshow_many("Sobel", &[&mat_sobelx_abs, &mat_sobely_abs, &sobeled, &mat]);
}
