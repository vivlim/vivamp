use eframe::egui::{self, Color32, Rect, Response, Sense, TextureId, Ui, Vec2, Widget};

use crate::app::LoadedTexture;


/// A clickable image within a frame.
#[must_use = "You should put this widget in an ui with `ui.add(widget);`"]
#[derive(Clone, Debug)]
pub struct MultiImageButton {
    image: egui::widgets::Image,
    hover_image: egui::widgets::Image,
    click_image: egui::widgets::Image,
    sense: Sense,
    frame: bool,
    selected: bool,
}

impl MultiImageButton {
    pub fn new(texture: &LoadedTexture, hover_texture: &LoadedTexture, click_texture: &LoadedTexture) -> Self {
        Self {
            image: egui::widgets::Image::new(texture.texture, texture.size.clone()),
            hover_image: egui::widgets::Image::new(hover_texture.texture, hover_texture.size.clone()),
            click_image: egui::widgets::Image::new(click_texture.texture, hover_texture.size.clone()),
            sense: Sense::click(),
            frame: false,
            selected: false,
        }
    }

    /// Select UV range. Default is (0,0) in top-left, (1,1) bottom right.
    pub fn uv(mut self, uv: impl Into<Rect>) -> Self {
        self.image = self.image.uv(uv);
        self
    }

    /// Multiply image color with this. Default is WHITE (no tint).
    pub fn tint(mut self, tint: impl Into<Color32>) -> Self {
        self.image = self.image.tint(tint);
        self
    }

    /// If `true`, mark this button as "selected".
    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    /// Turn off the frame
    pub fn frame(mut self, frame: bool) -> Self {
        self.frame = frame;
        self
    }

    /// By default, buttons senses clicks.
    /// Change this to a drag-button with `Sense::drag()`.
    pub fn sense(mut self, sense: Sense) -> Self {
        self.sense = sense;
        self
    }
}

impl Widget for MultiImageButton {
    fn ui(self, ui: &mut Ui) -> Response {
        let Self {
            image,
            hover_image,
            click_image,
            sense,
            frame,
            selected,
        } = self;

        let size = image.size();
        let (rect, response) = ui.allocate_exact_size(size, sense);
        response.widget_info(|| egui::WidgetInfo::new(egui::WidgetType::ImageButton));

        if ui.clip_rect().intersects(rect) {
            let image_to_show = {
                if response.clicked() || response.is_pointer_button_down_on() {
                    click_image
                }
                else if selected {
                    click_image
                }
                else if response.hovered() {
                    hover_image
                }
                else {
                    image
                }
            };

            image_to_show.paint_at(ui, rect);
        }

        response
    }
}
