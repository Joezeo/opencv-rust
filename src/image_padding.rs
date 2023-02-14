#![allow(dead_code)]
#![allow(unused_imports)]
use opencv::{
    core::{
        add, add_weighted, copy_make_border, no_array, Rect, Scalar, Size_, BORDER_CONSTANT,
        BORDER_REFLECT, BORDER_REFLECT_101, BORDER_REPLICATE,
    },
    highgui::{destroy_all_windows, imshow, wait_key},
    imgcodecs::{imread, IMREAD_COLOR},
    imgproc::{resize, INTER_LINEAR},
    prelude::*,
};

pub fn image_padding() {
    let mat = imread("rust.jpeg", IMREAD_COLOR).unwrap();
    // let roi_mat = Mat::roi(&mat, Rect::new(10, 10, 200, 200)).unwrap();

    let (top, bottom, left, right) = (50, 50, 50, 50);
    let mut dst_mat = Mat::default();
    copy_make_border(
        &mat,
        &mut dst_mat,
        top,
        bottom,
        left,
        right,
        BORDER_REPLICATE,
        Scalar::new(0., 0., 0., 0.),
    )
    .unwrap();

    let mut dst = Mat::default();
    add(&dst_mat, &dst_mat.clone(), &mut dst, &no_array(), -1).unwrap();

    let mut resized_mat1 = Mat::default();
    resize(
        &dst,
        &mut resized_mat1,
        Size_::new(300, 300),
        0.,
        0.,
        INTER_LINEAR,
    )
    .unwrap();
    let mut resized_mat2 = Mat::default();
    resize(
        &mat,
        &mut resized_mat2,
        Size_::new(300, 300),
        0.,
        0.,
        INTER_LINEAR,
    )
    .unwrap();

    let mut final_mat = Mat::default();
    add_weighted(
        &resized_mat1,
        0.4,
        &resized_mat2,
        0.7,
        0.,
        &mut final_mat,
        -1,
    ).unwrap();

    imshow("Image Padding", &final_mat).unwrap();

    wait_key(0).unwrap();

    destroy_all_windows().unwrap();
}
