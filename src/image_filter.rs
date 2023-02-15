#![allow(unused_imports)]
#![allow(dead_code)]
use opencv::{
    core::{Point, Size, BORDER_DEFAULT},
    highgui::{destroy_all_windows, imshow, wait_key},
    imgcodecs::{imread, IMREAD_COLOR},
    imgproc::{
        blur, box_filter, cvt_color, gaussian_blur, threshold, COLOR_BGR2GRAY, THRESH_BINARY, median_blur,
    },
    prelude::*,
};

pub fn image_filter() {
    let mat = imread("nois.jpeg", IMREAD_COLOR).unwrap();

    // let filtered = image_blur_filter(&mat);
    // let filtered = image_box_filter(&mat);
    // let filtered = image_gaussian_filter(&mat);
    let filtered = image_median_filter(&mat);

    show_in_binary(&filtered, &mat);

    imshow("Image Origin", &mat).unwrap();
    imshow("Image Filter", &filtered).unwrap();
    wait_key(0).unwrap();
    destroy_all_windows().unwrap();
}

fn image_blur_filter(src: &Mat) -> Mat {
    // 均值滤波
    // 简单的平均卷积操作
    let mut blured = Mat::default();
    blur(
        &src,
        &mut blured,
        Size::new(3, 3),
        Point::new(-1, -1),
        BORDER_DEFAULT,
    )
    .unwrap();
    blured
}

fn image_box_filter(src: &Mat) -> Mat {
    // 方框滤波
    // 基本和均值滤波相似，可以选择归一化，容易越界(normalize=false时，超过255的部分会设为255)
    let mut filtered = Mat::default();
    box_filter(
        src,
        &mut filtered,
        -1,
        Size::new(77, 77),
        Point::new(-1, -1),
        true,
        BORDER_DEFAULT,
    )
    .unwrap();
    filtered
}

fn image_gaussian_filter(src: &Mat) -> Mat {
    // 高斯(正态分布)滤波
    // 卷积核里的数值是满足正态分布的，相当于更加重视中间点
    let mut filtered = Mat::default();
    gaussian_blur(src, &mut filtered, Size::new(5, 5), 1., 0., BORDER_DEFAULT).unwrap();
    filtered
}

fn image_median_filter(src: &Mat) -> Mat {
    // 中值滤波
    let mut filtered = Mat::default();
    median_blur(src, &mut filtered, 3).unwrap();
    filtered
}

fn show_in_binary(filtered: &Mat, origin: &Mat) {
    let mut dst_mat_blured = Mat::default();
    cvt_color(&filtered, &mut dst_mat_blured, COLOR_BGR2GRAY, 0).unwrap();

    let mut dst_mat_origin = Mat::default();
    cvt_color(&origin, &mut dst_mat_origin, COLOR_BGR2GRAY, 0).unwrap();

    let mut dst_filtered = Mat::default();
    threshold(&dst_mat_blured, &mut dst_filtered, 127., 255., THRESH_BINARY).unwrap();

    let mut dst_origin = Mat::default();
    threshold(&dst_mat_origin, &mut dst_origin, 127., 255., THRESH_BINARY).unwrap();

    imshow("Image Origin Binary", &dst_origin).unwrap();
    imshow("Image Fitered Binary", &dst_filtered).unwrap();
    wait_key(0).unwrap();
}
