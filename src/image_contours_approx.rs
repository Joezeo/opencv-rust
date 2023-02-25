#![allow(unused_imports)]
#![allow(dead_code)]
use crate::opencv_util::imshow_many;
use opencv::{
    core::{no_array, Point, Point_, Scalar, Vector},
    imgcodecs::{imread, IMREAD_GRAYSCALE},
    imgproc::{
        approx_poly_dp, arc_length, bounding_rect, cvt_color, draw_contours, find_contours,
        rectangle, threshold, CHAIN_APPROX_NONE, COLOR_GRAY2BGR, INTER_MAX, LINE_8, RETR_TREE,
        THRESH_BINARY,
    },
    prelude::*,
    types::{VectorOfPoint, VectorOfVectorOfPoint},
};

pub fn image_contours_approx() {
    let mat = imread("github_binary.jpeg", IMREAD_GRAYSCALE).unwrap();

    let mut thresh = Mat::default();
    threshold(&mat, &mut thresh, 127., 255., THRESH_BINARY).unwrap();

    let mut contours = VectorOfVectorOfPoint::new();
    find_contours(
        &thresh,
        &mut contours,
        RETR_TREE,
        CHAIN_APPROX_NONE,
        Point::default(),
    )
    .unwrap();

    let mut holds = vec![];
    let mut curs = vec![];
    for (idx, cnt) in contours.iter().enumerate() {
        if idx == 12 {
            break;
        }
        let epsilon = arc_length(&cnt, true).unwrap() * 0.01;
        let mut approx: VectorOfPoint = Vector::default();
        approx_poly_dp(&cnt, &mut approx, epsilon, true).unwrap();

        let mut ctc = Mat::default();
        cvt_color(&mat, &mut ctc, COLOR_GRAY2BGR, 0).unwrap();

        let mut vec: VectorOfVectorOfPoint = Vector::new();
        vec.push(approx);
        draw_contours(
            &mut ctc,
            &vec,
            -1,
            Scalar::new(0., 0., 255., 255.),
            3,
            LINE_8,
            &no_array(),
            INTER_MAX,
            Point::default(),
        )
        .unwrap();

        // 轮廓的外接矩形
        let rect = bounding_rect(&cnt).unwrap();
        rectangle(
            &mut ctc,
            rect,
            Scalar::new(0., 255., 0., 255.),
            2,
            LINE_8,
            0,
        )
        .unwrap();

        holds.push(ctc);
    }
    for r in holds.iter() {
        curs.push(r);
    }

    imshow_many("Contours", &curs, false);
}
