//! YUV (I420 or NV12) to RGB, SIMD accelerated (NEON on the Pi), straight into the display's buffer.
use crate::Display;
use yuvutils_rs::{
    YuvBiPlanarImage, YuvConversionMode, YuvPlanarImage, YuvRange, YuvStandardMatrix,
};

/// Planes with their strides.
pub enum Yuv<'a> {
    I420([(&'a [u8], usize); 3]),
    Nv12([(&'a [u8], usize); 2]),
}

pub fn show(display: &dyn Display, width: u32, height: u32, yuv: Yuv) {
    let (range, matrix) = (YuvRange::Limited, YuvStandardMatrix::Bt601); // what phones send
    display.frame(width, height, &mut |out| {
        let r = match &yuv {
            Yuv::I420([y, u, v]) => {
                let img = YuvPlanarImage {
                    y_plane: y.0,
                    y_stride: y.1 as u32,
                    u_plane: u.0,
                    u_stride: u.1 as u32,
                    v_plane: v.0,
                    v_stride: v.1 as u32,
                    width,
                    height,
                };
                yuvutils_rs::yuv420_to_rgb(&img, out, width * 3, range, matrix)
            }
            Yuv::Nv12([y, uv]) => {
                let img = YuvBiPlanarImage {
                    y_plane: y.0,
                    y_stride: y.1 as u32,
                    uv_plane: uv.0,
                    uv_stride: uv.1 as u32,
                    width,
                    height,
                };
                yuvutils_rs::yuv_nv12_to_rgb(
                    &img,
                    out,
                    width * 3,
                    range,
                    matrix,
                    YuvConversionMode::Balanced,
                )
            }
        };
        if let Err(e) = r {
            eprintln!("video: convert: {e}");
        }
    });
}
