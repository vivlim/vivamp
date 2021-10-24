
use std::fs;

use codegen::{Scope, Type};

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
    let small_font_names = char_mapped_enum_names("SmallFont", "ABCDEFGHIJKLMNOPQRSTUVWXYZ\"@0123456789….:()-'!_+\\/[]^&%,=$#".to_string(), scope);

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
            map_repeated(0, 0, 68, 13, 0, 15, volume_slider_bar_names.iter().map(|s| s.as_str()).collect()),
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
    SkinFile {
        filename: "TEXT.BMP".to_string(),
        regions: vec![
            map_repeated(0, 0, 4, 6, 5, 0, small_font_names.iter().take(28).map(|s| s.as_str()).collect()),
            map_repeated(0, 6, 4, 6, 5, 0, small_font_names.iter().skip(28).take(31).map(|s| s.as_str()).collect()),
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

/// Generates a set of numbered enum names, and a function which will iterate them
fn char_mapped_enum_names(prefix: &str, char_string: String, scope: &mut Scope) -> Vec<String> {
    let names: Vec<String> = char_string.chars().into_iter().map(|c| format!("{}{}", prefix, map_special_chars_to_names(c))).collect();

    let mut iter_fn = scope.new_fn(format!("char_{}", prefix).as_str());
    iter_fn.arg("c", Type::new("char"));
    iter_fn.vis("pub");
    iter_fn.ret("Option<SkinImage>");
    iter_fn.line("match c {");
    let chars: Vec<char> = char_string.chars().into_iter().collect();
    for i in 0..names.len() {
        iter_fn.line(format!("'{}' => Some({}::{}),", escape_char_for_string(chars[i]), IMAGE_ENUM_NAME, names[i]));
    }
    iter_fn.line("_ => None");
    iter_fn.line("}");

    names
}

fn map_special_chars_to_names(c: char) -> String {
    match c {
        '"' => "DoubleQuote".to_string(),
        '@' => "At".to_string(),
        '.' => "PeriodMaybe".to_string(),
        '-' => "Hyphen".to_string(),
        ':' => "Colon".to_string(),
        '(' => "LeftParen".to_string(),
        ')' => "RightParen".to_string(),
        '\'' => "SingleQuote".to_string(),
        '!' => "Exclamation".to_string(),
        '_' => "Underscore".to_string(),
        '+' => "Plus".to_string(),
        '\\' => "Backslash".to_string(),
        '/' => "Slash".to_string(),
        '[' => "LeftBracket".to_string(),
        ']' => "RightBracket".to_string(),
        '^' => "Caret".to_string(),
        '&' => "Ampersand".to_string(),
        '%' => "Percent".to_string(),
        ',' => "Comma".to_string(),
        '=' => "Equals".to_string(),
        '$' => "Dollar".to_string(),
        '#' => "Hash".to_string(),
        '…' => "Ellipsis".to_string(),
        _ => c.to_string()

    }
}

fn escape_char_for_string(c: char) -> String {
    match c {
        '\'' => "\\'".to_string(),
        '\\' => "\\\\".to_string(),
        _ => c.to_string()
    }

}