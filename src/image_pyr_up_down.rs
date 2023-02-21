#![allow(unused_imports)]
#![allow(dead_code)]
use opencv::{
    core::{no_array, subtract, Size, BORDER_DEFAULT},
    imgcodecs::{imread, IMREAD_COLOR},
    imgproc::{pyr_down, pyr_up},
    prelude::*,
};

use crate::opencv_util::imshow_many;

// 图像金字塔，上下采样
pub fn image_pyr_up_down() {
    let mat = imread("young.jpg", IMREAD_COLOR).unwrap();
    let size = mat.size().unwrap();

    let mut down = Mat::default();
    pyr_down(&mat, &mut down, Size::default(), BORDER_DEFAULT).unwrap();

    let mut up = Mat::default();
    pyr_up(&down, &mut up, size, BORDER_DEFAULT).unwrap();

    let mut sub = Mat::default();
    subtract(&mat, &up, &mut sub, &no_array(), -1).unwrap();

    imshow_many("Pyr up down", &[&mat, &up, &down, &sub], false);
}
