use eframe::egui::{self, Color32, Rect, Response, Sense, TextureId, Ui, Vec2, Widget};

use crate::app::LoadedTexture;


/// A clickable image within a frame.
#[must_use = "You should put this widget in an ui with `ui.add(widget);`"]
#[derive(Clone, Debug)]
pub struct LinearBar {
    bar_images: Vec<egui::widgets::Image>,
    handle_image: egui::widgets::Image,
    handle_image_clicked: egui::widgets::Image,
    sense: Sense,
    selected: bool,
    orientation: BarOrientation,
    handle_margin: f32,
    value: f32,
}

#[derive(Clone, Debug)]
pub enum BarOrientation {
    Vertical,
    Horizontal
}

impl LinearBar {
    pub fn new(orientation: BarOrientation, handle_margin: f32, bar_textures: Vec<&LoadedTexture>, hover_texture: &LoadedTexture, click_texture: &LoadedTexture, default_value: f32) -> Self {
        Self {
            bar_images: bar_textures.iter().map(|tex| egui::widgets::Image::new(tex.texture, tex.size.clone())).collect(),
            handle_image: egui::widgets::Image::new(hover_texture.texture, hover_texture.size.clone()),
            handle_image_clicked: egui::widgets::Image::new(click_texture.texture, hover_texture.size.clone()),
            sense: Sense::click_and_drag(),
            selected: false,
            orientation,
            handle_margin,
            value: default_value,
        }
    }

    /// If `true`, mark this button as "selected".
    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    /// By default, buttons senses clicks.
    /// Change this to a drag-button with `Sense::drag()`.
    pub fn sense(mut self, sense: Sense) -> Self {
        self.sense = sense;
        self
    }
}

impl Widget for LinearBar {
    fn ui(self, ui: &mut Ui) -> Response {
        let Self {
            bar_images,
            handle_image,
            handle_image_clicked,
            sense,
            selected,
            orientation,
            handle_margin,
            value,
        } = self;

        // pick the right bar image based on the value
        let bar_image_index = (value * (bar_images.len() as f32 - 1.0)).round() as usize;
        let bar_image = bar_images.get(bar_image_index).unwrap(); // double check the math here.

        let bar_size = bar_image.size();
        let (rect, response) = ui.allocate_exact_size(bar_size, sense);
        response.widget_info(|| egui::WidgetInfo::new(egui::WidgetType::DragValue));

        if ui.clip_rect().intersects(rect) {
            // let image_to_show = {
            //     if response.clicked() || response.is_pointer_button_down_on() {
            //         click_image
            //     }
            //     else if selected {
            //         click_image
            //     }
            //     else if response.hovered() {
            //         hover_image
            //     }
            //     else {
            //         image
            //     }
            // };

            // image_to_show.paint_at(ui, rect);
        }

        response
    }
}
