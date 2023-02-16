#![allow(unused_imports)]
#![allow(dead_code)]
use opencv::{
    core::{add_weighted, convert_scale_abs, BORDER_DEFAULT, CV_64F},
    highgui::{destroy_all_windows, imshow, wait_key},
    imgcodecs::{imread, IMREAD_COLOR, IMREAD_GRAYSCALE},
    imgproc::{laplacian, scharr, sobel},
    prelude::*,
};

use crate::opencv_util::imshow_many;

pub fn image_scharr_laplacian() {
    // let mat = imread("woman.jpg", IMREAD_GRAYSCALE).unwrap();
    let mat = imread("github_binary.jpeg", IMREAD_COLOR).unwrap();

    // Sobel
    let mut sobel_x = Mat::default();
    sobel(&mat, &mut sobel_x, CV_64F, 1, 0, 3, 1., 0., BORDER_DEFAULT).unwrap();
    let mut sobel_y = Mat::default();
    sobel(&mat, &mut sobel_y, CV_64F, 1, 0, 3, 1., 0., BORDER_DEFAULT).unwrap();

    let mut sobel_x_abs = Mat::default();
    convert_scale_abs(&sobel_x, &mut sobel_x_abs, 1., 0.).unwrap();
    let mut sobel_y_abs = Mat::default();
    convert_scale_abs(&sobel_y, &mut sobel_y_abs, 1., 0.).unwrap();

    let mut sobel = Mat::default();
    add_weighted(&sobel_x_abs, 0.5, &sobel_y_abs, 0.5, 0., &mut sobel, -1).unwrap();

    // Scharr
    let mut scharr_x = Mat::default();
    scharr(&mat, &mut scharr_x, CV_64F, 1, 0, 1., 0., BORDER_DEFAULT).unwrap();
    let mut scharr_y = Mat::default();
    scharr(&mat, &mut scharr_y, CV_64F, 0, 1, 1., 0., BORDER_DEFAULT).unwrap();

    let mut scharr_x_abs = Mat::default();
    convert_scale_abs(&scharr_x, &mut scharr_x_abs, 1., 0.).unwrap();
    let mut scharr_y_abs = Mat::default();
    convert_scale_abs(&scharr_x, &mut scharr_y_abs, 1., 0.).unwrap();

    let mut scharr = Mat::default();
    add_weighted(&scharr_x_abs, 0.5, &scharr_y_abs, 0.5, 0., &mut scharr, -1).unwrap();

    // Laplacian
    let mut laplacian_mat = Mat::default();
    laplacian(&mat, &mut laplacian_mat, CV_64F, 1, 1., 0., BORDER_DEFAULT).unwrap();
    let mut laplacian = Mat::default();
    convert_scale_abs(&laplacian_mat, &mut laplacian, 1., 0.).unwrap();

    imshow_many("Scharr", &[&mat, &sobel, &scharr, &laplacian], false);
}
