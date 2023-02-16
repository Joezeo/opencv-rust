#![allow(unused_imports)]
#![allow(dead_code)]
use opencv::{
    core::{Point, Size, BORDER_CONSTANT, BORDER_DEFAULT},
    highgui::{destroy_all_windows, imshow, wait_key},
    imgcodecs::{imread, IMREAD_COLOR},
    imgproc::{
        dilate, erode, get_structuring_element, morphology_default_border_value, morphology_ex,
        MORPH_GRADIENT, MORPH_RECT, gaussian_blur,
    },
    prelude::*,
};
use crate::opencv_util::imshow_many;

pub fn image_gradient_operation() {
    let mat = imread("pie.jpg", IMREAD_COLOR).unwrap();

    let kernel = get_structuring_element(MORPH_RECT, Size::new(7, 7), Point::new(-1, -1)).unwrap();

    let mut mat_erode = Mat::default();
    erode(
        &mat,
        &mut mat_erode,
        &kernel,
        Point::new(-1, -1),
        3,
        BORDER_CONSTANT,
        morphology_default_border_value().unwrap(),
    )
    .unwrap();

    let mut mat_dilate = Mat::default();
    dilate(
        &mat,
        &mut mat_dilate,
        &kernel,
        Point::new(-1, -1),
        3,
        BORDER_CONSTANT,
        morphology_default_border_value().unwrap(),
    )
    .unwrap();

    // 对腐蚀后的图像进行梯度运算
    let mut mat_erode_gradient = Mat::default();
    morphology_ex(
        &mat_erode,
        &mut mat_erode_gradient,
        MORPH_GRADIENT,
        &kernel,
        Point::new(-1, -1),
        1,
        BORDER_CONSTANT,
        morphology_default_border_value().unwrap(),
    )
    .unwrap();

    // 对膨胀后的图像进行梯度运算
    let mut mat_dilate_gradient = Mat::default();
    morphology_ex(
        &mat_dilate,
        &mut mat_dilate_gradient,
        MORPH_GRADIENT,
        &kernel,
        Point::new(-1, -1),
        1,
        BORDER_CONSTANT,
        morphology_default_border_value().unwrap(),
    )
    .unwrap();

    // imshow("Image Erode", &mat_erode).unwrap();
    // imshow("Image Erode Gradient", &mat_erode_gradient).unwrap();
    // imshow("Image Dilate", &mat_dilate).unwrap();
    // imshow("Image Dilate Gradient", &mat_dilate_gradient).unwrap();
    imshow_many(
        "Image Gradient",
        &[
            &mat_erode,
            &mat_erode_gradient,
            &mat_dilate,
            &mat_dilate_gradient,
        ],
        false
    );
    wait_key(0).unwrap();
    destroy_all_windows().unwrap();
}