use std::{collections::HashMap, fmt::Debug, fs::{self, File}, io::{self, Read}};
use eframe::egui::{Color32, Rect};
use image::{DynamicImage, GenericImageView};
use thiserror::Error;
use zip::ZipArchive;


#[derive(Error, Debug)]
pub enum SkinError {
    #[error("IO error")]
    Io(#[from] io::Error),
    #[error("Zip")]
    Zip(#[from] zip::result::ZipError),
    #[error("Image")]
    Image(#[from] image::ImageError)

}

pub struct WinampSkin {
    pub images: HashMap<String, LoadedImage>
}

pub struct LoadedImage {
    pub pixels: Vec<Color32>,
    pub size: (usize, usize),
    pub name: String,
}

#[derive(Debug)]
struct SliceMapping {
    region: Rect,
    name: String
}

    //Texture((eframe::egui::Vec2, eframe::egui::TextureId))

pub fn open_skin() -> Result <WinampSkin, SkinError>{
    let zipfile = fs::File::open("/Users/vivlim/winamp/base-2.91.wsz.zip")?;
    let mut zip = zip::ZipArchive::new(zipfile)?;

    let mut slice_map: HashMap<String, Vec<SliceMapping>> = HashMap::new();
    slice_map.insert("CBUTTONS.BMP".to_string(), 
        map_repeated(0, 0, 22, 18, 23, 0, vec![
            "button-prev",
            "button-play",
            "button-pause",
            "button-stop",
            "button-next",
            "button-eject"
            ]));

    let mut map: HashMap<String, LoadedImage> = HashMap::new();
    for i in 0..zip.len() {
        let mut file = zip.by_index(i)?;

        if !file.name().ends_with(".BMP") {
            // only load bmps
            continue;
        }

        let mut data: Vec<u8> = Default::default();
        file.read_to_end(&mut data)?;
        let image_data = image::load_from_memory(&data)?;

        let default_slice_map = vec![map_def(0, 0, image_data.width(), image_data.height(), file.name())];

        println!("loading {}", file.name());
        let slice_maps = match slice_map.get(file.name()) {
            Some(map) => {
                println!("using slice map {:?}", map);
                map
            },
            None => &default_slice_map,
        };

        let loaded_images = load_image_slices_from_data(image_data, slice_maps)?;
        for image in loaded_images {
            map.insert(image.name.clone(), image);
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


fn load_image_slices_from_data(data: DynamicImage, slice_maps: &Vec<SliceMapping>) -> Result <Vec<LoadedImage>, SkinError>{
    let mut result: Vec<LoadedImage> = vec![];

    for slice_map in slice_maps {
        println!("doing slice map {:?}", slice_map);
        let top_left = slice_map.region.left_top();
        let data_cropped = data.crop_imm(top_left.x as u32, top_left.y as u32, slice_map.region.width() as u32, slice_map.region.height() as u32);
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
            name: slice_map.name.clone(),
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
