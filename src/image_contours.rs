#![allow(unused_imports)]
#![allow(dead_code)]
use opencv::{
    core::{no_array, Point, Scalar, CV_8UC1, subtract},
    imgcodecs::{imread, IMREAD_GRAYSCALE, IMREAD_COLOR},
    imgproc::{
        draw_contours, find_contours, threshold, CHAIN_APPROX_NONE, INTER_MAX, LINE_8, RETR_TREE,
        THRESH_BINARY, cvt_color, COLOR_GRAY2BGR, contour_area, arc_length,
    },
    prelude::*,
    types::VectorOfVectorOfPoint, highgui::{imshow, wait_key, destroy_all_windows},
};

use crate::opencv_util::imshow_many;

pub fn image_contours() {
    let mat = imread("github_binary.jpeg", IMREAD_GRAYSCALE).unwrap();

    let mut thresh = Mat::default();
    threshold(&mat, &mut thresh, 127., 255., THRESH_BINARY).unwrap();

    // mode 轮廓检测模式
    // RETR_EXTERNAL: 只检测最外面的轮廓
    // RETR_LIST: 检测所有轮廓，并保存至list中
    // RETR_CCOMP: 检测所有轮廓，并将他么的组织分层，顶层是各部分的外部边界，第二层是空洞的边界
    // RETR_TREE: 检测所有轮廓，并重构嵌套轮廓的各个层次
    //
    // method 轮廓逼近方法
    // CHAIN_APPROX_NONE: 以Freeman链码的方式输出轮廓，所有其他方法输出多边形(顶点的序列)
    // CHAIN_APPROX_SIMPLE: 压缩水平的、垂直的和斜的部分，也就是，函数只保留他们的终点部分
    let mut contours = VectorOfVectorOfPoint::new();
    find_contours(
        &thresh,
        &mut contours,
        RETR_TREE,
        CHAIN_APPROX_NONE,
        Point::default(),
    )
    .unwrap();

    let mut ct = Mat::default();
    mat.copy_to(&mut ct).unwrap();

    let mut ctc = Mat::default();
    cvt_color(&ct, &mut ctc, COLOR_GRAY2BGR, 0).unwrap();

    draw_contours(
        &mut ctc,
        &contours,
        -1,
        Scalar::new(0., 0., 255., 255.),
        2,
        LINE_8,
        &no_array(),
        INTER_MAX,
        Point::default(),
    )
    .unwrap();

    let mut mat_color = Mat::default();
    cvt_color(&mat, &mut mat_color, COLOR_GRAY2BGR, 0).unwrap();

    let mut sub = Mat::default();
    subtract(&ctc, &mat_color, &mut sub, &no_array(), -1).unwrap();

    for (idx, cts) in contours.iter().enumerate() {
        let area = contour_area(&cts, false).unwrap();
        let length = arc_length(&cts, true).unwrap();
        println!("[{}] area: {}, length: {}", idx, area, length);
    }

    imshow_many("Contours", &[&ctc, &mat_color, &sub], false);
}
