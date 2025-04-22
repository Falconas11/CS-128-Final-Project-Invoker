#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod invoker;

use std::collections::HashMap;
use invoker::Invoker;
use std::collections::VecDeque;
use std::fs;
use rand::seq::SliceRandom;
use rand::thread_rng;
use eframe::{egui, App, CreationContext, NativeOptions};
use egui::ColorImage;

const INVOKE_QUOTES: &[&str] = &[
    "One of my favorites.",
    "Spherical sorcery!",
    "Behold!",
    "A rich tradition.",
    "Drawn from deep within.",
    "From the great mystery.",
    "A spell I well remember.",
    "True Arcanery thrives!",
    "Plucked from the Arcanery.",
    "My mind is my Arcanery.",
    "An incantation long remembered.",
    "A charming hex.",
    "Enlightenment stands ready!",
    "Words of power.",
    "Witness true sorcery!",
    "Augury abounds.",
    "Arcana known only to me!",
];

const FAILED_QUOTES: &[&str] = &[
    "This spell works nowise!",
    "This magic disappoints.",
    "My memory fails me.",
    "Did I miscast?",
    "My concentration--shattered!",
    "Did I mix my magics?",
    "Words fail me?",
    "Ruinous, ill-fated spell.",
    "Perturbations!",
    "Lamentable!",
    "Infelicitous!",
    "The errant cosmos works against me!",
    "No matter.",
];

pub struct InvokerApp {
    buffer: VecDeque<char>,
    invoked_spells: VecDeque<String>,
    info: Option<String>,
    invoker: Invoker,
    textures: HashMap<String, egui::TextureHandle>,
}

impl InvokerApp {
    pub fn new(cc: &CreationContext<'_>) -> Self {
        let mut app = Self {
            buffer: VecDeque::with_capacity(3),
            invoked_spells: VecDeque::with_capacity(2),
            info: None,
            invoker: Invoker::new(),
            textures: HashMap::new(),
        };
        app.load_textures(&cc.egui_ctx);
        app
    }

    fn load_textures(&mut self, ctx: &egui::Context) {
        let mut load = |key: &str, path: String| {
            if let Ok(bytes) = fs::read(&path) {
                if let Ok(image) = image::load_from_memory(&bytes) {
                    let image = image.to_rgba8();
                    let size = [image.width() as usize, image.height() as usize];
                    let pixels = image.as_flat_samples();
                    let ci = ColorImage::from_rgba_unmultiplied(size, pixels.as_slice());
                    let tex = ctx.load_texture(key, ci, Default::default());
                    self.textures.insert(key.to_string(), tex);
                }
            }
        };

        for key in &["q", "w", "e", "r"] {
            load(key, format!("assets/{}.png", key));
        }

        let mut seen = std::collections::HashSet::new();
        for &spell in self.invoker.combos.values() {
            let key = spell.to_lowercase().replace(' ', "_");
            if seen.insert(key.clone()) {
                load(&key, format!("assets/{}.png", key));
            }
        }
    }

    fn draw_input(&self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            for i in 0..3 {
                if let Some(&ch) = self.buffer.get(i) {
                    let key = ch.to_ascii_lowercase().to_string();
                    if let Some(tex) = self.textures.get(&key) {
                        ui.image(tex);
                    } else {
                        ui.label(&key);
                    }
                } else {
                    ui.label("[ ]");
                }
            }
        });

        ui.separator();
        ui.label("Skill Slots:");
        ui.horizontal(|ui| {
            for i in 0..2 {
                if let Some(spell) = self.invoked_spells.get(i) {
                    let key = spell.to_lowercase().replace(' ', "_");
                    if let Some(tex) = self.textures.get(&key) {
                        ui.image(tex);
                    } else {
                        ui.label(spell);
                    }
                } else {
                    ui.label("[       ]");
                }
            }
        });

        ui.separator();
        if let Some(msg) = &self.info {
            ui.label(msg);
        } else {
            ui.label("Press Q, W, E for Ice, Thunder and Fire...");
        }
    }

    fn on_key(&mut self, key: char) {
        let mut rng = thread_rng();

        match key {
            'q' | 'w' | 'e' => {
                if self.buffer.len() == 3 {
                    self.buffer.pop_front();
                }
                self.buffer.push_back(key.to_ascii_uppercase());
            }
            'r' => {
                if self.buffer.len() == 3 {
                    let combo: String = self.buffer.iter().collect();
                    if let Some(&spell_name) = self.invoker.get_spell(&combo) {
                        if self.invoked_spells.len() == 2 && self.invoked_spells[1] == spell_name {
                            self.invoked_spells.swap(0, 1);
                            self.info = INVOKE_QUOTES
                                .choose(&mut rng)
                                .copied()
                                .map(String::from);
                        
                        } else if self.invoked_spells.front().map_or(false, |s| s == spell_name) {
                            self.info = FAILED_QUOTES
                                .choose(&mut rng)
                                .copied()
                                .map(String::from);
                        } else {
                            if self.invoked_spells.len() == 2 {
                                self.invoked_spells.pop_back();
                            }
                            self.invoked_spells.push_front(spell_name.to_string());
                            self.info = INVOKE_QUOTES
                                .choose(&mut rng)
                                .copied()
                                .map(String::from);
                        }
                    }
                }
            }
            _ => {}
        }
    }
}

impl App for InvokerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.draw_input(ui);

            ctx.input(|i| {
                if i.key_pressed(egui::Key::Q) {
                    self.on_key('q');
                }
                if i.key_pressed(egui::Key::W) {
                    self.on_key('w');
                }
                if i.key_pressed(egui::Key::E) {
                    self.on_key('e');
                }
                if i.key_pressed(egui::Key::R) {
                    self.on_key('r');
                }
            });
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    eframe::run_native(
        "Invoker Trainer",
        NativeOptions::default(),
        Box::new(|cc: &CreationContext<'_>| Box::new(InvokerApp::new(cc))),
    )
}