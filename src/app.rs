use crate::any_data::AnyData;
use crate::common_vec_op::{CodeParser, CommonVecVisualizer, VecLineGen};
use crate::cus_component::{toggle, CodeEditor};
use crate::interfaces::{
    ICodeEditor, IParser, IVisData, IVisDataGenerator, IVisualizer, ParseError,
};
use bincode::{Decode, Encode};
use eframe::{egui, Storage};
use egui_extras::{Size, StripBuilder};
use log::error;
use std::collections::BTreeSet;
use std::time::Duration;
use std::vec;

use base64::prelude::*;

const DEFAULT_CODE: &str = include_str!("default_code");

const WINDOW_NAMES: [[&str; 2]; 2] = [["⚙", "Options"], ["📄", "Code"]];

struct MainAppCache {
    code: AnyData,
    lines: Vec<Vec<Box<dyn IVisData>>>,

    params: MainAppParams,

    #[cfg(target_arch = "wasm32")]
    transfer_data: TransferData,
}

#[derive(Clone, PartialEq, Decode, Encode)]
struct MainAppParams {
    vis_progress: i64,
    vis_progress_max: i64,
    lcd_coords: bool,
    show_inter_dash: bool,
    colorful_block: bool,

    trans_matrix: [[f64; 3]; 3],
}

#[derive(Clone, PartialEq, Default, Decode, Encode)]
struct TransferData {
    code: String,
    params: Option<MainAppParams>,
}

impl Default for MainAppParams {
    fn default() -> Self {
        Self {
            vis_progress: 0,
            vis_progress_max: 0,
            lcd_coords: false,
            show_inter_dash: true,
            colorful_block: true,
            trans_matrix: [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]], // Identity matrix
        }
    }
}

pub struct MainApp {
    code: AnyData,
    error: Option<ParseError>,

    params: MainAppParams,

    cache: MainAppCache,

    #[cfg(target_arch = "wasm32")]
    is_loaded_from_url: bool,

    /// panel status
    side_panel_open: bool,
    panel_status: BTreeSet<String>,
}

impl Default for MainApp {
    fn default() -> Self {
        Self {
            code: AnyData::new(DEFAULT_CODE.to_owned()),
            params: MainAppParams::default(),
            cache: MainAppCache {
                code: AnyData::new("".to_owned()),
                lines: vec![],
                params: MainAppParams::default(),

                #[cfg(target_arch = "wasm32")]
                transfer_data: TransferData::default(),
            },
            error: None,

            #[cfg(target_arch = "wasm32")]
            is_loaded_from_url: false,
            side_panel_open: false,
            panel_status: Default::default(),
        }
    }
}

impl eframe::App for MainApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        #[cfg(target_arch = "wasm32")]
        if self.is_loaded_from_url == false {
            self.load_from_url_search();
            self.is_loaded_from_url = true;
        }

        egui::TopBottomPanel::top("top").show(ctx, |ui| {
            self.ui_about(ui);
        });
        egui::TopBottomPanel::bottom("bottom").show(ctx, |ui| {
            self.ui_toast_bar(ui);
        });
        egui::SidePanel::left("Panels")
            .resizable(false)
            .exact_width(if self.side_panel_open { 100.0 } else { 40.0 })
            .show(ctx, |ui| {
                self.ui_panels(ui);
            });

        egui::Window::new("Options")
            .open(&mut self.panel_status.contains(WINDOW_NAMES[0][1]))
            .fixed_size([600.0, 200.0])
            .default_pos(ctx.available_rect().left_top())
            .movable(true)
            .show(ctx, |ui| {
                self.ui_options_panel(ui);
            });

        if ctx.available_rect().aspect_ratio() < 1.0 {
            egui::TopBottomPanel::bottom("CodeEditor")
                .resizable(false)
                .exact_height(ctx.available_rect().height() / 2.0)
                .show_animated(ctx, self.panel_status.contains(WINDOW_NAMES[1][1]), |ui| {
                    self.ui_code_editor(ui);
                });
        } else {
            egui::SidePanel::left("CodeEditor")
                .resizable(false)
                .exact_width(ctx.available_rect().width() / 2.0)
                .show_animated(ctx, self.panel_status.contains(WINDOW_NAMES[1][1]), |ui| {
                    self.ui_code_editor(ui);
                });
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            self.ui_visualizer(ui);
        });
    }

    fn save(&mut self, _storage: &mut dyn Storage) {
        #[cfg(target_arch = "wasm32")]
        self.save_to_url_search();
    }

    fn auto_save_interval(&self) -> Duration {
        Duration::from_millis(30)
    }
}

impl MainApp {
    fn ui_toast_bar(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.horizontal(|ui| {
                let info = self.error.as_ref().map_or_else(
                    || "".to_owned(),
                    |e| format!("({}, {}): Error: {}", e.cursor.row + 1, e.cursor.col, e.msg),
                );
                let rt = egui::RichText::new(info)
                    .size(20.0)
                    .color(egui::Color32::RED)
                    .text_style(egui::TextStyle::Monospace);
                ui.label(rt).highlight();
            });
        });
    }

    fn ui_panels(&mut self, ui: &mut egui::Ui) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.with_layout(egui::Layout::top_down_justified(egui::Align::LEFT), |ui| {
                let side_panel_icon = if self.side_panel_open {
                    "👈 Collapse"
                } else {
                    "👉"
                };
                ui.toggle_value(&mut self.side_panel_open, side_panel_icon);
                ui.separator();
                for [icon, name] in WINDOW_NAMES {
                    let mut is_open = self.panel_status.contains(name);
                    ui.toggle_value(
                        &mut is_open,
                        if self.side_panel_open {
                            format!("{icon} {name}")
                        } else {
                            icon.to_owned()
                        },
                    );
                    if is_open {
                        self.panel_status.insert(name.to_owned());
                    } else {
                        self.panel_status.remove(name);
                    }
                }
            });
        });
    }

    fn ui_options_panel(&mut self, ui: &mut egui::Ui) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("Controls");
            });
            ui.horizontal_wrapped(|ui| {
                ui.add(toggle("LCD Coordinates", &mut self.params.lcd_coords));
                ui.add(toggle(
                    "Show Intermediate Dash",
                    &mut self.params.show_inter_dash,
                ));
                ui.add(toggle("Colorful Blocks", &mut self.params.colorful_block));
                ui.add_sized(
                    ui.available_size(),
                    egui::Slider::new(
                        &mut self.params.vis_progress,
                        0..=self.params.vis_progress_max,
                    )
                    .text("Progress")
                    .show_value(true),
                );
            });
            StripBuilder::new(ui)
                .size(Size::exact(30.0))
                .size(Size::remainder())
                .vertical(|mut strip| {
                    strip.cell(|ui| {
                        ui.vertical_centered(|ui| {
                            ui.heading("Transform Matrix");
                        });
                    });
                    strip.strip(|builder| {
                        builder
                            .size(Size::exact(20.0))
                            .size(Size::exact(20.0))
                            .size(Size::exact(20.0)) // 3x3 matrix
                            .vertical(|mut strip| {
                                strip.strip(|builder| {
                                    builder
                                        .size(Size::relative(0.33))
                                        .size(Size::relative(0.33))
                                        .size(Size::relative(0.33)) // 3x3 matrix
                                        .horizontal(|mut strip| {
                                            strip.cell(|ui| {
                                                ui.add_sized(
                                                    ui.available_size(),
                                                    egui::Slider::new(
                                                        &mut self.params.trans_matrix[0][0],
                                                        -5.0..=5.0,
                                                    )
                                                    .text("m00")
                                                    .show_value(true),
                                                );
                                            });
                                            strip.cell(|ui| {
                                                ui.add_sized(
                                                    ui.available_size(),
                                                    egui::Slider::new(
                                                        &mut self.params.trans_matrix[0][1],
                                                        -5.0..=5.0,
                                                    )
                                                    .text("m01")
                                                    .show_value(true),
                                                );
                                            });
                                            strip.cell(|ui| {
                                                ui.add_sized(
                                                    ui.available_size(),
                                                    egui::Slider::new(
                                                        &mut self.params.trans_matrix[0][2],
                                                        -500.0..=500.0,
                                                    )
                                                    .text("m02")
                                                    .show_value(true),
                                                );
                                            });
                                        });
                                });
                                strip.strip(|builder| {
                                    builder
                                        .size(Size::relative(0.33))
                                        .size(Size::relative(0.33))
                                        .size(Size::relative(0.33)) // 3x3 matrix
                                        .horizontal(|mut strip| {
                                            strip.cell(|ui| {
                                                ui.add_sized(
                                                    ui.available_size(),
                                                    egui::Slider::new(
                                                        &mut self.params.trans_matrix[1][0],
                                                        -5.0..=5.0,
                                                    )
                                                    .text("m10")
                                                    .show_value(true),
                                                );
                                            });
                                            strip.cell(|ui| {
                                                ui.add_sized(
                                                    ui.available_size(),
                                                    egui::Slider::new(
                                                        &mut self.params.trans_matrix[1][1],
                                                        -5.0..=5.0,
                                                    )
                                                    .text("m11")
                                                    .show_value(true),
                                                );
                                            });
                                            strip.cell(|ui| {
                                                ui.add_sized(
                                                    ui.available_size(),
                                                    egui::Slider::new(
                                                        &mut self.params.trans_matrix[1][2],
                                                        -500.0..=500.0,
                                                    )
                                                    .text("m12")
                                                    .show_value(true),
                                                );
                                            });
                                        });
                                });
                                strip.strip(|builder| {
                                    builder
                                        .size(Size::relative(0.33))
                                        .size(Size::relative(0.33))
                                        .size(Size::relative(0.33)) // 3x3 matrix
                                        .horizontal(|mut strip| {
                                            strip.cell(|ui| {
                                                ui.add_sized(
                                                    ui.available_size(),
                                                    egui::Slider::new(
                                                        &mut self.params.trans_matrix[2][0],
                                                        -5.0..=5.0,
                                                    )
                                                    .text("m20")
                                                    .show_value(true),
                                                );
                                            });
                                            strip.cell(|ui| {
                                                ui.add_sized(
                                                    ui.available_size(),
                                                    egui::Slider::new(
                                                        &mut self.params.trans_matrix[2][1],
                                                        -5.0..=5.0,
                                                    )
                                                    .text("m21")
                                                    .show_value(true),
                                                );
                                            });
                                            strip.cell(|ui| {
                                                ui.add_sized(
                                                    ui.available_size(),
                                                    egui::Slider::new(
                                                        &mut self.params.trans_matrix[2][2],
                                                        -500.0..=500.0,
                                                    )
                                                    .text("m22")
                                                    .show_value(true),
                                                );
                                            });
                                        });
                                });
                            });
                    });
                });
        });
    }

    fn ui_code_editor(&mut self, ui: &mut egui::Ui) {
        ui.heading("Code Editor");
        ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
            if ui.button("📋 Copy Code").clicked() {
                ui.output_mut(|o| o.copied_text = self.code.cast_ref::<String>().clone());
            }

            if ui.button("🌐 Copy URL").clicked() {
                let transfer_data = TransferData {
                    code: self.code.cast_ref::<String>().clone(),
                    params: Some(self.params.clone()),
                };
                let t = self.create_transfer_url(&transfer_data);
                ui.output_mut(|o| o.copied_text = format!("https://w-mai.github.io/vegravis/{t}"));
            }
        });

        ui.separator();

        CodeEditor {}.show(ui, &mut self.code, VecLineGen::default().command_syntax());
    }

    fn ui_visualizer(&mut self, ui: &mut egui::Ui) {
        let visualizer = CommonVecVisualizer::new(self.params.trans_matrix);

        let mut has_error = false;
        if !self.code.equal::<String, String>(&self.cache.code) || self.params != self.cache.params
        {
            let mut generator = VecLineGen::default();
            let mut parser = CodeParser::new(self.code.clone::<String>(), &mut generator);
            // 通过parser产生generator需要的前置数据
            has_error = match parser.parse() {
                Ok(vlg) => {
                    let ops_count = vlg.len() as i64;
                    self.params.vis_progress_max = ops_count;
                    if !self.code.equal::<String, String>(&self.cache.code) {
                        self.params.vis_progress = ops_count;
                    }

                    let parsed = vlg.gen(0..(self.params.vis_progress));

                    self.cache.lines = parsed.clone();
                    self.cache.code = self.code.clone::<String>();
                    self.cache.params = self.params.clone();
                    false
                }
                Err(e) => {
                    error!("Error: {:?}", e);
                    self.error = Some(e);
                    true
                }
            }
        }
        if !has_error {
            self.error = None;
        }

        visualizer.plot(
            ui,
            self.cache.lines.clone(),
            has_error,
            self.params.show_inter_dash,
            self.params.colorful_block,
            self.params.lcd_coords,
        );
    }

    fn ui_about(&mut self, ui: &mut egui::Ui) {
        const VERSION: &str = env!("CARGO_PKG_VERSION");
        use egui::special_emojis::GITHUB;
        ui.horizontal_wrapped(|ui| {
            egui::widgets::global_theme_preference_switch(ui);
            ui.separator();
            ui.heading("Vector Graphics Visualizer");
            ui.separator();
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.horizontal_wrapped(|ui| {
                    ui.label(format!("Version: {VERSION}"));
                    ui.hyperlink_to("🌐Web Version", "https://w-mai.github.io/vegravis/");
                    ui.hyperlink_to(
                        format!("{GITHUB} vegravis on GitHub"),
                        env!("CARGO_PKG_HOMEPAGE"),
                    );
                });
            });
        });
    }
}

impl MainApp {
    fn create_transfer_url(&self, transfer_data: &TransferData) -> String {
        let config = bincode::config::standard();
        if let Ok(data) = bincode::encode_to_vec(transfer_data, config) {
            let mut t = BASE64_URL_SAFE_NO_PAD.encode(data);

            t.insert(0, '?');
            return t;
        };
        Default::default()
    }
}

#[cfg(target_arch = "wasm32")]
impl MainApp {
    fn load_from_url_search(&mut self) {
        use eframe::web::web_location;
        let location = web_location();
        let query = &location.query;
        if query.is_empty() {
            return;
        }

        if let Ok(data) = BASE64_URL_SAFE_NO_PAD.decode(query) {
            let config = bincode::config::standard();
            if let Ok((t, _s)) =
                bincode::decode_from_slice(&data, config) as Result<(TransferData, _), _>
            {
                self.code = AnyData::new(t.code);
                self.params = t.params.unwrap_or_default();

                return;
            }
        }

        error!("Invalid query string");
    }

    fn save_to_url_search(&mut self) {
        let history = web_sys::window().unwrap().history().unwrap();
        let transfer_data = TransferData {
            code: self.code.cast_ref::<String>().clone(),
            params: Some(self.params.clone()),
        };

        if self.cache.transfer_data == transfer_data {
            return;
        }

        self.cache.transfer_data = transfer_data;

        let t = self.create_transfer_url(&self.cache.transfer_data);
        if t.is_empty() {
            return;
        }

        use eframe::wasm_bindgen::JsValue;
        history
            .push_state_with_url(&JsValue::NULL, "", Some(&t))
            .unwrap()
    }
}
