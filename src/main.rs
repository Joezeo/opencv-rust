mod image_canny;
mod image_edge_extraction;
mod image_erode_dilate;
mod image_filter;
mod image_gradient_operation;
mod image_padding;
mod image_read_show;
mod image_open_close_operation;
mod image_scharr_laplacian;
mod image_sobel;
mod image_threshhold;
mod image_top_black_hat;
mod opencv_util;
mod video_read_process;

fn main() {
    // video_read_process::video_read_process();
    // image_read_show::image_read_show();
    // image_padding::image_padding();
    // image_threshhold::image_threshhold();
    // image_filter::image_filter();
    // image_erode_dilate::image_erode_dilate();
    // image_open_close_operation::image_open_close_operation();
    // image_gradient_operation::image_gradient_operation();
    // image_top_black_hat::image_top_black_hat();
    // image_edge_extraction::image_edge_extraction();
    // image_sobel::image_sobel();
    // image_scharr_laplacian::image_scharr_laplacian();
    image_canny::image_canny();
}
