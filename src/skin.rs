use std::{collections::{BTreeMap, HashMap}, fmt::Debug, fs::{self, File}, io::{self, Read}, path::PathBuf};
use eframe::egui::{Color32, Rect};
use image::{DynamicImage, GenericImageView};
use thiserror::Error;
use zip::ZipArchive;
use fixed_map::{Key, Map};

use crate::skin_generated::get_skin_load_specs;


#[derive(Error, Debug)]
pub enum SkinError {
    #[error("IO error")]
    Io(#[from] io::Error),
    #[error("Zip")]
    Zip(#[from] zip::result::ZipError),
    #[error("Image")]
    Image(#[from] image::ImageError),
    #[error("Expected file missing {0}")]
    ExpectedFileMissing(String)

}

pub struct WinampSkin {
    pub images: Map<crate::skin_generated::SkinImage, LoadedImage>
}

pub struct LoadedImage {
    pub pixels: Vec<Color32>,
    pub size: (usize, usize),
    pub image: crate::skin_generated::SkinImage,
}

#[derive(Debug)]
struct SliceMapping {
    region: Rect,
    name: String
}

pub struct FileLoadSpec<'a> {
    pub filename: &'a str,
    pub regions: Vec<RectLoadSpec>
}

pub struct RectLoadSpec {
    pub top_left_x: u32,
    pub top_left_y: u32,
    pub bottom_right_x: u32,
    pub bottom_right_y: u32,
    pub image: crate::skin_generated::SkinImage
}

    //Texture((eframe::egui::Vec2, eframe::egui::TextureId))

pub fn open_skin(path: &PathBuf) -> Result <WinampSkin, SkinError>{
    let zipfile = fs::File::open(path)?;
    let mut zip = zip::ZipArchive::new(zipfile)?;

    let mut zip_filename_case_map = HashMap::new();
    for filename in zip.file_names() {
        zip_filename_case_map.insert(filename.to_ascii_lowercase(), filename.to_string());
    }

    let load_specs = get_skin_load_specs();

    let mut map = Map::new();
    for file_spec in load_specs {
        let mut filename_recased = zip_filename_case_map.get(&file_spec.filename.to_lowercase()).ok_or(SkinError::ExpectedFileMissing(file_spec.filename.to_string()))?;
        let mut file = zip.by_name(&filename_recased)?;

        let mut data: Vec<u8> = Default::default();
        file.read_to_end(&mut data)?;
        let image_data = image::load_from_memory(&data)?;


        let loaded_images = load_image_slices_from_data(image_data, &file_spec.regions)?;
        for image in loaded_images {
            map.insert(image.image, image);
        }
    }

    Ok(WinampSkin {
        images: map
    })
}

/// Generate named slice mappings for a 1d pattern - as many as there are names.
/// Mappings are generated from left to right and top to bottom. stride is added for each mapping, width and height are not included in stride
fn map_repeated(top_left_x: u32, top_left_y: u32, width: u32, height: u32, stride_x: u32, stride_y: u32, names: Vec<&str>) -> Vec<SliceMapping> {
    let mut output = vec![];
    for i in 0..names.len() as u32 {
        // todo boundary check
        let this_top_left_x = top_left_x + (stride_x * i);
        let this_top_left_y = top_left_y + (stride_y * i);
        output.push(map_def(this_top_left_x, this_top_left_y, this_top_left_x + width, this_top_left_y + height, names[i as usize]));
    }
    output

}

fn map_def(x0: u32, y0: u32, x1: u32, y1: u32, name: &str) -> SliceMapping {
    SliceMapping {
        region: Rect::from_x_y_ranges(x0 as f32..=x1 as f32, y0 as f32..=y1 as f32),
        name: name.to_string(),
    }
}


fn load_image_slices_from_data(data: DynamicImage, rect_specs: &Vec<RectLoadSpec>) -> Result <Vec<LoadedImage>, SkinError>{
    let mut result: Vec<LoadedImage> = vec![];

    for rect_spec in rect_specs {
        let data_cropped = data.crop_imm(rect_spec.top_left_x, rect_spec.top_left_y, rect_spec.bottom_right_x - rect_spec.top_left_x, rect_spec.bottom_right_y - rect_spec.top_left_y);
        // double image size
        //let data_cropped = data_cropped.resize(data_cropped.width()*2, data_cropped.height()*2, image::imageops::FilterType::Nearest);
        let image_buffer = data_cropped.to_rgba8();
        let size = (data_cropped.width() as usize, data_cropped.height() as usize);
        let pixels = image_buffer.into_vec();
        assert_eq!(size.0 * size.1 * 4, pixels.len());
        let pixels: Vec<_> = pixels
            .chunks_exact(4)
            .map(|p| eframe::egui::Color32::from_rgba_unmultiplied(p[0], p[1], p[2], p[3]))
            .collect();
        
        result.push(LoadedImage {
            pixels,
            size,
            image: rect_spec.image,
        });
    }
    Ok(result)
}

// fn read_file(zip: &mut ZipArchive<File>, filename: &str) -> Result<Vec<u8>, SkinError> {
//     let mut file = zip.by_name(filename)?;
//     let mut data: Vec<u8> = Default::default();
//     data.resize(file.size() as usize, 0);
//     file.read_exact(&mut data);
//     Ok(data)

// }

// fn read_image(zip: &mut ZipArchive<File>, filename: &str) -> Result<LoadedImage, SkinError> {
//     let mut file = read_file(zip, filename)?;
//     let image = image::load_from_memory(&file)?;
//     let image_buffer = image.to_rgba8();
//     let size = (image.width() as usize, image.height() as usize);
//     let pixels = image_buffer.into_vec();
//     assert_eq!(size.0 * size.1 * 4, pixels.len());
//     let pixels: Vec<_> = pixels
//         .chunks_exact(4)
//         .map(|p| eframe::egui::Color32::from_rgba_unmultiplied(p[0], p[1], p[2], p[3]))
//         .collect();

//     Ok(LoadedImage::Pixels(pixels, size))
// }
