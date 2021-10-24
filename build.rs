
use std::fs;

use codegen::Scope;

struct SkinFile {
    filename: String,
    regions: Vec<Vec<SkinFileRegion>> 
}

struct SkinFileRegion {
    top_left_x: u32,
    top_left_y: u32,
    bottom_right_x: u32,
    bottom_right_y: u32,
    enum_name: String
}

const IMAGE_ENUM_NAME: &str = "SkinImage";


fn regions(scope: &mut Scope) -> Vec<SkinFile>{
    let volume_slider_bar_names = numbered_enum_names("VolumeSliderBar", 28, scope);

    vec![
    SkinFile {
        filename: "CBUTTONS.BMP".to_string(),
        regions: vec![
            map_repeated(0, 0, 22, 18, 23, 0, vec![
                "ButtonPrev",
                "ButtonPlay",
                "ButtonPause",
                "ButtonStop",
                "ButtonNext",
                "ButtonEject" ]),
            map_repeated(0, 18, 22, 18, 23, 0, vec![
                "ButtonPrevPressed",
                "ButtonPlayPressed",
                "ButtonPausePressed",
                "ButtonStopPressed",
                "ButtonNextPressed",
                "ButtonEjectPressed" ])
        ]
    },
    SkinFile {
        filename: "MAIN.BMP".to_string(),
        regions: vec![vec![
            SkinFileRegion {
                top_left_x: 0,
                top_left_y: 0,
                bottom_right_x: 275,
                bottom_right_y: 116,
                enum_name: "MainWindow".to_string()
            }
        ]]
    },
    SkinFile {
        filename: "VOLUME.BMP".to_string(),
        regions: vec![
            map_repeated(0, 0, 68, 13, 0, 70, volume_slider_bar_names.iter().map(|s| s.as_str()).collect()),
            vec![
                SkinFileRegion {
                    top_left_x: 0,
                    top_left_y: 422,
                    bottom_right_x: 13,
                    bottom_right_y: 432,
                    enum_name: "VolumeSliderButton".to_string()
                },
                SkinFileRegion {
                    top_left_x: 15,
                    top_left_y: 422,
                    bottom_right_x: 28,
                    bottom_right_y: 432,
                    enum_name: "VolumeSliderButtonPressed".to_string()
                },
            ]
        ]
    },
    ]
}

fn main() {
    let mut scope = Scope::new();
    let regions = regions(&mut scope);
    {
        let mut skin_image_enum = scope.new_enum(IMAGE_ENUM_NAME);
        skin_image_enum.vis("pub");
        skin_image_enum.derive("Debug, Copy, Clone, fixed_map::Key, strum_macros::AsRefStr, strum_macros::Display");
        for file in &regions {
            for region in file.regions.iter().flatten() {
                skin_image_enum.new_variant(&region.enum_name);
            }
        }
    }

    {
        let mut load_function = scope.new_fn("get_skin_load_specs");
        load_function.vis("pub");
        load_function.ret("Vec<crate::skin::FileLoadSpec<'static>>");
        load_function.line("vec![");
        for file in &regions {
            load_function.line("crate::skin::FileLoadSpec {");
            load_function.line(format!("filename: \"{}\",", file.filename));
            load_function.line("regions: vec![");
            for region in file.regions.iter().flatten() {
                load_function.line("crate::skin::RectLoadSpec {");
                load_function.line(format!("top_left_x: {},", region.top_left_x));
                load_function.line(format!("top_left_y: {},", region.top_left_y));
                load_function.line(format!("bottom_right_x: {},", region.bottom_right_x));
                load_function.line(format!("bottom_right_y: {},", region.bottom_right_y));
                load_function.line(format!("image: SkinImage::{},", region.enum_name));
                load_function.line("},");
            }
            load_function.line("]},");
        }
        load_function.line("]");
    }

    fs::write("src/skin_generated.rs", scope.to_string()).unwrap();
}

fn map_repeated(top_left_x: u32, top_left_y: u32, width: u32, height: u32, stride_x: u32, stride_y: u32, names: Vec<&str>) -> Vec<SkinFileRegion> {
    let mut output = vec![];
    for i in 0..names.len() as u32 {
        // todo boundary check
        let this_top_left_x = top_left_x + (stride_x * i);
        let this_top_left_y = top_left_y + (stride_y * i);
        output.push(SkinFileRegion {
            top_left_x: this_top_left_x,
            top_left_y: this_top_left_y,
            bottom_right_x: this_top_left_x + width,
            bottom_right_y: this_top_left_y + height,
            enum_name: names[i as usize].to_string()
        });
    }
    output

}

/// Generates a set of numbered enum names, and a function which will iterate them
fn numbered_enum_names(prefix: &str, count: usize, scope: &mut Scope) -> Vec<String> {
    let names = (0..count).into_iter().map(|i| format!("{}{}", prefix, i)).collect();

    let mut iter_fn = scope.new_fn(format!("iter_{}", prefix).as_str());
    iter_fn.vis("pub");
    iter_fn.ret("std::vec::IntoIter<SkinImage>");
    iter_fn.line("vec![");
    for name in &names {
        iter_fn.line(format!("{}::{},", IMAGE_ENUM_NAME, name));
    }
    iter_fn.line("].into_iter()");

    names
}