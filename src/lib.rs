extern crate console_error_panic_hook;

use std::io::Cursor;

use icns::{IconFamily, IconType};
use ico::{IconDir, IconImage};
use image::{imageops, DynamicImage, ImageBuffer, ImageFormat, Rgba};
use imageproc::drawing::{draw_filled_ellipse_mut, draw_filled_rect_mut};
use js_sys::Uint8Array;
use wasm_bindgen::prelude::*;

const SIZES_PNG: [u32; 5] = [32, 64, 128, 256, 512];
const SIZES_ICON: [u32; 6] = [16, 32, 64, 128, 256, 512];
const FILTER: image::imageops::FilterType = image::imageops::FilterType::Lanczos3;

#[wasm_bindgen]
pub fn set_panic_hook() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn generate(bytes: &[u8]) -> js_sys::Array {
    let src = image::load_from_memory(bytes).unwrap_or_else(|e| {
        panic!("Failed to load image: {}", e);
    }).into_rgba8();

    let png_files = generate_png_files(&src);
    let ico_file = generate_ico_file(&src);
    let icns_file = generate_icns_file(&src);

    let out = js_sys::Array::new();

    out.push(&ico_file);
    out.push(&icns_file);
    png_files.iter().for_each(|png| {
        out.push(png);
    });

    out
}

fn apply_rounded(src: &ImageBuffer<Rgba<u8>, Vec<u8>>, s: u32) -> DynamicImage {
    let resized = imageops::resize(src, s, s, FILTER);
    let img = DynamicImage::ImageRgba8(resized);
    let scale = 4;
    let mut mask = 
        ImageBuffer::<Rgba<u8>, Vec<u8>>::from_pixel(s * scale, s * scale, Rgba([0, 0, 0, 0]));

    let r = (s * scale) / 4;
    let side = s * scale;

    draw_filled_rect_mut(
        &mut mask,
        imageproc::rect::Rect::at(r as i32, 0).of_size((side - 2 * r) as u32, side as u32),
        Rgba([0, 0, 0, 255])
    );
    draw_filled_rect_mut(
        &mut mask,
        imageproc::rect::Rect::at(0, r as i32).of_size(side as u32, (side - 2 * r) as u32),
        Rgba([0, 0, 0, 255])
    );

    draw_filled_ellipse_mut(
        &mut mask,
        (r as i32, r as i32),
        r as i32,
        r as i32,
        Rgba([0, 0, 0, 255])
    );
    draw_filled_ellipse_mut(
        &mut mask,
        ((side - r) as i32, r as i32),
        r as i32,
        r as i32,
        Rgba([0, 0, 0, 255])
    );
    draw_filled_ellipse_mut(
        &mut mask,
        (r as i32, (side - r) as i32),
        r as i32,
        r as i32,
        Rgba([0, 0, 0, 255])
    );
    draw_filled_ellipse_mut(
        &mut mask,
        ((side - r) as i32, (side - r) as i32),
        r as i32,
        r as i32,
        Rgba([0, 0, 0, 255])
    );

    let mask = 
        image::imageops::resize(&mask, s, s, FILTER);
    let mut out = img.to_rgba8();

    for (p_dst, p_mask) in out.pixels_mut().zip(mask.pixels()) {
        p_dst.0[3] = p_mask.0[3];
    }

    DynamicImage::ImageRgba8(out)
}

fn generate_png_files(src: &ImageBuffer<Rgba<u8>, Vec<u8>>) -> Vec<Uint8Array> {
    let mut result = Vec::new();

    for s in SIZES_PNG {
        let rounded = apply_rounded(&src, s);
        
        let mut data = Cursor::new(Vec::new());
        
        rounded.write_to(&mut data, ImageFormat::Png).unwrap_or_else(|e| {
            panic!("Failed to write image to PNG format: {}", e);
        });

        let bytes = Uint8Array::from(&data.get_ref()[..]);
        
        result.push(bytes);
    }

    result
}

fn generate_ico_file(src: &ImageBuffer<Rgba<u8>, Vec<u8>>) -> Uint8Array {
    let mut dir = IconDir::new(ico::ResourceType::Icon);
    
    for s in SIZES_ICON {
        let rounded = apply_rounded(&src, s);
        let rgba_data = rounded.to_rgba8().into_vec();
        let icon = IconImage::from_rgba_data(s, s, rgba_data);

        let entry = ico::IconDirEntry::encode(&icon).unwrap_or_else(|e| {
            panic!("Failed to encode icon entry: {}", e);
        });
        dir.add_entry(entry);
    }

    let mut data = Vec::new();
    dir.write(&mut data).unwrap_or_else(|e| {
        panic!("Failed to write ICO file: {}", e);
    });

    Uint8Array::from(&data[..])
}

fn generate_icns_file(src: &ImageBuffer<Rgba<u8>, Vec<u8>>) -> Uint8Array {
    let mut fam = IconFamily::new();

    for s in SIZES_ICON {
        let rounded = apply_rounded(&src, s);
        let rgba_data = rounded.to_rgba8().into_vec();
        let image = icns::Image::from_data(icns::PixelFormat::RGBA, s, s, rgba_data)
            .unwrap_or_else(|e| {
                panic!("Failed to create ICNS image: {}", e);
            });

        let icon_type: IconType = match s {
            16 => IconType::RGBA32_16x16,
            32 => IconType::RGBA32_32x32,
            64 => IconType::RGBA32_64x64,
            128 => IconType::RGBA32_128x128,
            256 => IconType::RGBA32_256x256,
            512 => IconType::RGBA32_512x512,
            _ => continue,
        };
        
        fam.add_icon_with_type(&image, icon_type).unwrap_or_else(|e| {
            panic!("Failed to add icon to ICNS family: {}", e);
        });
    }

    let mut data = Vec::new();
    fam.write(&mut data).unwrap_or_else(|e| {
        panic!("Failed to write ICNS file: {}", e);
    });

    Uint8Array::from(&data[..])
}