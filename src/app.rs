use std::collections::{BTreeMap, HashMap};

use eframe::egui::{Pos2, Rect};
use eframe::{egui, epi};

use crate::skin::{self, LoadedImage, WinampSkin};
use crate::skin_generated::{SkinImage, iter_VolumeSliderBar};
use crate::widgets::button::MultiImageButton;
use crate::widgets::slider::{SliderGraphics, WinampSlider};
use fixed_map::{Key, Map};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    label: String,

    // this how you opt-out of serialization of a member
    #[cfg_attr(feature = "persistence", serde(skip))]
    value: f32,

    #[cfg_attr(feature = "persistence", serde(skip))]
    skin_images: Option<WinampSkin>,
    #[cfg_attr(feature = "persistence", serde(skip))]
    textures_loaded: bool,
    #[cfg_attr(feature = "persistence", serde(skip))]
    skin_textures: Map<SkinImage, LoadedTexture>,

    volume: f32,
}

pub struct LoadedTexture {
    pub size: eframe::egui::Vec2,
    pub texture: eframe::egui::TextureId,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            value: 2.7,
            skin_images: None,
            textures_loaded: false,
            skin_textures: Default::default(),
            volume: 0.5
        }
    }
}

impl epi::App for TemplateApp {
    fn name(&self) -> &str {
        "eframe template"
    }

    /// Called once before the first frame.
    fn setup(
        &mut self,
        _ctx: &egui::CtxRef,
        _frame: &mut epi::Frame<'_>,
        _storage: Option<&dyn epi::Storage>,
    ) {
        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        #[cfg(feature = "persistence")]
        if let Some(storage) = _storage {
            *self = epi::get_value(storage, epi::APP_KEY).unwrap_or_default()
        }
        let skin = skin::open_skin();
        self.skin_images = skin.ok();
    }

    /// Called by the frame work to save state before shutdown.
    /// Note that you must enable the `persistence` feature for this to work.
    #[cfg(feature = "persistence")]
    fn save(&mut self, storage: &mut dyn epi::Storage) {
        epi::set_value(storage, epi::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) {
        let Self { label, value, skin_images, textures_loaded, skin_textures, volume} = self;

        if !*textures_loaded {
            match skin_images {
                Some(skin) => {
                    for (name, image) in skin.images.iter() {
                        println!("loading texture for {}", name);
                        let texture = frame.tex_allocator().alloc_srgba_premultiplied(image.size, &image.pixels);
                        let size = egui::Vec2::new(image.size.0 as f32, image.size.1 as f32);
                        skin_textures.insert(name.clone(), LoadedTexture {
                            size,
                            texture,
                        });

                    }
                   *textures_loaded = true;
                },
                None => ()
            }
        }


        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::CentralPanel::default().show(ctx, |ui| {
            if *textures_loaded {
                let tex = skin_textures.get(SkinImage::MainWindow).unwrap();
                ui.put(egui::Rect::from_min_size(Pos2::new(0.0, 0.0), tex.size), create_image_widget(tex));


                use crate::skin_generated::SkinImage::*;
                struct ImageButtonSpec {
                    x: u32,
                    y: u32,
                    texture: SkinImage,
                    hover_texture: SkinImage,
                    click_texture: SkinImage,
                }
                for i in &[
                    ImageButtonSpec { x: 16, y: 88, texture: ButtonPrev, hover_texture: ButtonPrev, click_texture: ButtonPrevPressed },
                    ImageButtonSpec { x: 39, y: 88, texture: ButtonPlay, hover_texture: ButtonPlay, click_texture: ButtonPlayPressed },
                    ImageButtonSpec { x: 62, y: 88, texture: ButtonPause, hover_texture: ButtonPause, click_texture: ButtonPausePressed },
                    ImageButtonSpec { x: 85, y: 88, texture: ButtonStop, hover_texture: ButtonStop, click_texture: ButtonStopPressed },
                    ImageButtonSpec { x: 108, y: 88, texture: ButtonNext, hover_texture: ButtonNext, click_texture: ButtonNextPressed },
                    ImageButtonSpec { x: 136, y: 88, texture: ButtonEject, hover_texture: ButtonEject, click_texture: ButtonEjectPressed },
                ] {
                    let neutral_texture = skin_textures.get(i.texture).unwrap();
                    ui.put(get_abs_image_rect(neutral_texture, i.x as f32, i.y as f32),
                        MultiImageButton::new(
                        neutral_texture,
                        skin_textures.get(i.hover_texture).unwrap(),
                        skin_textures.get(i.click_texture).unwrap()));
                }
                let slider_textures = SliderGraphics::<&LoadedTexture> {
                    bar: iter_VolumeSliderBar().map(|i| skin_textures.get(i).unwrap()).collect(),
                    handle: skin_textures.get(SkinImage::VolumeSliderButton).unwrap(),
                    handle_clicked: skin_textures.get(SkinImage::VolumeSliderButtonPressed).unwrap(),
                }; 
                ui.put(egui::Rect::from_min_size(Pos2::new(106.0, 0.0), tex.size), WinampSlider::new(volume, 0.0..=1.0, slider_textures));
            }
        });


        /*
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                egui::menu::menu(ui, "File", |ui| {
                    if ui.button("Quit").clicked() {
                        frame.quit();
                    }
                });
            });
        });

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Side Panel");

            ui.horizontal(|ui| {
                ui.label("Write something: ");
                ui.text_edit_singleline(label);
            });

            ui.add(egui::Slider::new(value, 0.0..=10.0).text("value"));
            if ui.button("Increment").clicked() {
                *value += 1.0;
            }

            if *textures_loaded {
                ui.add(MultiImageButton::new(
                    skin_textures.get("button-play").unwrap(),
                    skin_textures.get("button-play").unwrap(),
                    skin_textures.get("button-play-pressed").unwrap()));
                ui.add(MultiImageButton::new(
                    skin_textures.get("button-prev").unwrap(),
                    skin_textures.get("button-prev").unwrap(),
                    skin_textures.get("button-prev-pressed").unwrap()));
            }

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.label("powered by ");
                    ui.hyperlink_to("egui", "https://github.com/emilk/egui");
                    ui.label(" and ");
                    ui.hyperlink_to("eframe", "https://github.com/emilk/egui/tree/master/eframe");
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's

            ui.heading("eframe template");
            ui.hyperlink("https://github.com/emilk/eframe_template");
            ui.add(egui::github_link_file!(
                "https://github.com/emilk/eframe_template/blob/master/",
                "Source code."
            ));
            egui::warn_if_debug_build(ui);

            for (name, texture) in skin_textures {
                ui.heading(name);
                ui.add(egui::Image::new(texture.texture, texture.size));
            }
        });
        */


        if false {
            egui::Window::new("Window").show(ctx, |ui| {
                ui.label("Windows can be moved by dragging them.");
                ui.label("They are automatically sized based on contents.");
                ui.label("You can turn on resizing and scrolling if you like.");
                ui.label("You would normally chose either panels OR windows.");
            });
        }
    }
}

fn create_image_widget(texture: &LoadedTexture) -> egui::Image {
    egui::Image::new(texture.texture, texture.size)
}

fn get_abs_image_rect(texture: &LoadedTexture, x: f32, y: f32) -> Rect {
    egui::Rect::from_min_size(Pos2::new(x, y), texture.size)
}