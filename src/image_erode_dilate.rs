#![allow(unused_imports)]
#![allow(dead_code)]
use opencv::{
    core::{Point, Size, BORDER_CONSTANT},
    highgui::{destroy_all_windows, imshow, wait_key},
    imgcodecs::{imread, IMREAD_COLOR},
    imgproc::{
        dilate, erode, get_structuring_element, morphology_default_border_value, MORPH_RECT,
    },
    prelude::*,
};

/// 图像的腐蚀和膨胀
pub fn image_erode_dilate() {
    let mat = imread("opencv_bi.jpeg", IMREAD_COLOR).unwrap();

    // 获取卷积核
    let kenerl = get_structuring_element(MORPH_RECT, Size::new(3, 3), Point::new(-1, -1)).unwrap();
    println!("{:?}", kenerl.data_bytes().unwrap());

    // 进行腐蚀操作
    let mut erode_mat = Mat::default();
    erode(
        &mat,
        &mut erode_mat,
        &kenerl,
        Point::new(-1, -1),
        1,
        BORDER_CONSTANT,
        morphology_default_border_value().unwrap(),
    )
    .unwrap();

    // 进行膨胀操作
    let mut dilate_mat = Mat::default();
    dilate(
        &erode_mat,
        &mut dilate_mat,
        &kenerl,
        Point::new(-1, -1),
        1,
        BORDER_CONSTANT,
        morphology_default_border_value().unwrap(),
    )
    .unwrap();

    imshow("Image Origin", &mat).unwrap();
    imshow("Image Erode", &erode_mat).unwrap();
    imshow("Image Dilate", &dilate_mat).unwrap();
    wait_key(0).unwrap();
    destroy_all_windows().unwrap();
}
