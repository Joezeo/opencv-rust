use opencv::{
    core::{Rect, Size, CV_8UC3},
    highgui::{destroy_all_windows, imshow, wait_key},
    imgproc::{resize, INTER_LINEAR},
    prelude::*,
};

pub fn imshow_many(title: &str, mats: &[&Mat]) {
    let mut display_mat = Mat::default();

    let n_img = mats.len();

    let size;
    let mut x;
    let mut y;
    let w;
    let h;

    match n_img {
        0 => return,
        13.. => return,
        1 => {
            w = 1;
            h = 1;
            size = 400;
        }
        2 => {
            w = 2;
            h = 1;
            size = 400;
        }
        3 | 4 => {
            w = 2;
            h = 2;
            size = 400;
        }
        5 | 6 => {
            w = 3;
            h = 2;
            size = 300;
        }
        7 | 8 => {
            w = 4;
            h = 2;
            size = 300;
        }
        _ => {
            w = 4;
            h = 3;
            size = 200;
        }
    }
    unsafe {
        display_mat
            .create_size(Size::new(100 + size * w, 30 + size * h), CV_8UC3)
            .unwrap();
    }

    let mut max;
    let mut scale;
    let mut m = 20;
    let mut n = 20;

    for i in 0..n_img {
        x = mats[i].cols();
        y = mats[i].rows();
        max = x.max(y);
        scale = max as f32 / size as f32;

        if i as i32 % w == 0 && m != 20 {
            m = 20;
            n = 20 + size;
        }

        let width = (x as f32 / scale) as i32;
        let height = (y as f32 / scale) as i32;
        // 在display_mat中划分出roi区域
        let mut img_roi = Mat::roi(&display_mat, Rect::new(m, n, width, height)).unwrap();
        resize(
            mats[i],
            &mut img_roi,
            Size::new(width, height),
            0.,
            0.,
            INTER_LINEAR,
        )
        .unwrap();
        m += 20 + size;
    }

    imshow(title, &display_mat).unwrap();
    wait_key(0).unwrap();
    destroy_all_windows().unwrap();
}
