#[derive(Debug, Copy, Clone, fixed_map::Key, strum_macros::AsRefStr, strum_macros::Display)]
pub enum SkinImage {
    ButtonPrev,
    ButtonPlay,
    ButtonPause,
    ButtonStop,
    ButtonNext,
    ButtonEject,
    ButtonPrevPressed,
    ButtonPlayPressed,
    ButtonPausePressed,
    ButtonStopPressed,
    ButtonNextPressed,
    ButtonEjectPressed,
    MainWindow,
}

pub fn get_skin_load_specs() -> Vec<crate::skin::FileLoadSpec<'static>> {
    vec![
    crate::skin::FileLoadSpec {
    filename: "CBUTTONS.BMP",
    regions: vec![
    crate::skin::RectLoadSpec {
    top_left_x: 0,
    top_left_y: 0,
    bottom_right_x: 22,
    bottom_right_y: 18,
    image: SkinImage::ButtonPrev,
    },
    crate::skin::RectLoadSpec {
    top_left_x: 23,
    top_left_y: 0,
    bottom_right_x: 45,
    bottom_right_y: 18,
    image: SkinImage::ButtonPlay,
    },
    crate::skin::RectLoadSpec {
    top_left_x: 46,
    top_left_y: 0,
    bottom_right_x: 68,
    bottom_right_y: 18,
    image: SkinImage::ButtonPause,
    },
    crate::skin::RectLoadSpec {
    top_left_x: 69,
    top_left_y: 0,
    bottom_right_x: 91,
    bottom_right_y: 18,
    image: SkinImage::ButtonStop,
    },
    crate::skin::RectLoadSpec {
    top_left_x: 92,
    top_left_y: 0,
    bottom_right_x: 114,
    bottom_right_y: 18,
    image: SkinImage::ButtonNext,
    },
    crate::skin::RectLoadSpec {
    top_left_x: 115,
    top_left_y: 0,
    bottom_right_x: 137,
    bottom_right_y: 18,
    image: SkinImage::ButtonEject,
    },
    crate::skin::RectLoadSpec {
    top_left_x: 0,
    top_left_y: 18,
    bottom_right_x: 22,
    bottom_right_y: 36,
    image: SkinImage::ButtonPrevPressed,
    },
    crate::skin::RectLoadSpec {
    top_left_x: 23,
    top_left_y: 18,
    bottom_right_x: 45,
    bottom_right_y: 36,
    image: SkinImage::ButtonPlayPressed,
    },
    crate::skin::RectLoadSpec {
    top_left_x: 46,
    top_left_y: 18,
    bottom_right_x: 68,
    bottom_right_y: 36,
    image: SkinImage::ButtonPausePressed,
    },
    crate::skin::RectLoadSpec {
    top_left_x: 69,
    top_left_y: 18,
    bottom_right_x: 91,
    bottom_right_y: 36,
    image: SkinImage::ButtonStopPressed,
    },
    crate::skin::RectLoadSpec {
    top_left_x: 92,
    top_left_y: 18,
    bottom_right_x: 114,
    bottom_right_y: 36,
    image: SkinImage::ButtonNextPressed,
    },
    crate::skin::RectLoadSpec {
    top_left_x: 115,
    top_left_y: 18,
    bottom_right_x: 137,
    bottom_right_y: 36,
    image: SkinImage::ButtonEjectPressed,
    },
    ]},
    crate::skin::FileLoadSpec {
    filename: "MAIN.BMP",
    regions: vec![
    crate::skin::RectLoadSpec {
    top_left_x: 0,
    top_left_y: 0,
    bottom_right_x: 275,
    bottom_right_y: 116,
    image: SkinImage::MainWindow,
    },
    ]},
    ]
}