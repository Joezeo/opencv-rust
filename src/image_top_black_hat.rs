#![allow(unused_imports)]
#![allow(dead_code)]
use opencv::{
    core::{Point, Size, BORDER_CONSTANT, add_weighted},
    imgcodecs::{imread, IMREAD_COLOR},
    imgproc::{
        get_structuring_element, morphology_default_border_value, morphology_ex, MORPH_RECT,
        MORPH_TOPHAT, MORPH_BLACKHAT,
    },
    prelude::*,
};

use crate::opencv_util::imshow_many;

pub fn image_top_black_hat() {
    let mat = imread("opencv_bi.jpeg", IMREAD_COLOR).unwrap();

    let kernel = get_structuring_element(MORPH_RECT, Size::new(3, 3), Point::new(-1, -1)).unwrap();

    // Top hat 顶帽： 原始输入-开运算
    let mut top_hat = Mat::default();
    morphology_ex(
        &mat,
        &mut top_hat,
        MORPH_TOPHAT,
        &kernel,
        Point::new(-1, -1),
        1,
        BORDER_CONSTANT,
        morphology_default_border_value().unwrap(),
    ).unwrap();

    // Black hat 黑帽： 闭运算-原始输入
    let mut black_hat = Mat::default();
    morphology_ex(
        &mat,
        &mut black_hat,
        MORPH_BLACKHAT,
        &kernel,
        Point::new(-1, -1),
        1,
        BORDER_CONSTANT,
        morphology_default_border_value().unwrap(),
    ).unwrap();

    imshow_many("Top Black hat", &[&mat, &top_hat, &black_hat], false)
}
