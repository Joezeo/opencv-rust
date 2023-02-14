use opencv::{imgcodecs::{imread, IMREAD_COLOR}, highgui::{imshow, wait_key, destroy_all_windows}};

fn main() {
    let mat = imread("image.jpg", IMREAD_COLOR).unwrap();
    println!("{:?}", mat);

    // 图像的显示
    imshow("Bibop", &mat).unwrap();

    // 等待时间，毫秒级，0表示任意时间终止
    wait_key(0).unwrap();

    destroy_all_windows().unwrap();
}
