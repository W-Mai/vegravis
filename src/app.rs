use crate::any_data::AnyData;
use crate::common_vec_op::{CodeParser, CommonVecVisualizer, VecLineGen};
use crate::cus_component::{toggle, CodeEditor};
use crate::interfaces::{
    ICodeEditor, IParser, IVisData, IVisDataGenerator, IVisualizer, ParseError,
};
use bincode::{Decode, Encode};
use eframe::{egui, Storage};
use log::error;
use std::collections::{BTreeMap, BTreeSet};
use std::time::Duration;
use std::vec;

use super::sample_codes_list::SAMPLE_CODES_LIST;
use crate::egui::Sense;
use base64::prelude::*;

const WINDOW_NAMES: [[&str; 2]; 4] = [
    ["🐑", "Samples"],
    ["", ""],
    ["⚙", "Options"],
    ["📄", "Code"],
];

struct MainAppCache {
    code: AnyData,
    lines: Vec<Vec<Box<dyn IVisData>>>,

    params: MainAppParams,

    #[cfg(target_arch = "wasm32")]
    transfer_data: TransferData,
}

#[derive(Clone, PartialEq, Decode, Encode)]
struct MainAppParams {
    vis_progress_anim: bool,
    /// true: positive, false: negative
    vis_progress_anim_dir: bool,
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
            vis_progress_anim: false,
            vis_progress_anim_dir: true,
            vis_progress: 0,
            vis_progress_max: 0,
            lcd_coords: false,
            show_inter_dash: true,
            colorful_block: true,
            trans_matrix: [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]], // Identity matrix
        }
    }
}

impl Default for MainAppCache {
    fn default() -> Self {
        Self {
            code: AnyData::new("".to_owned()),
            lines: vec![],
            params: Default::default(),

            #[cfg(target_arch = "wasm32")]
            transfer_data: Default::default(),
        }
    }
}

pub struct MainApp {
    code: AnyData,
    error: Option<ParseError>,

    params: MainAppParams,

    cache: MainAppCache,
    samples_cache: BTreeMap<&'static str, MainAppCache>,
    selected_sample: &'static str,
    hovered_sample: &'static str,

    #[cfg(target_arch = "wasm32")]
    is_loaded_from_url: bool,

    /// panel status
    side_panel_open: bool,
    panel_status: BTreeSet<String>,
}

impl Default for MainApp {
    fn default() -> Self {
        Self {
            code: AnyData::new(SAMPLE_CODES_LIST[0].1.to_owned()),
            params: MainAppParams::default(),
            cache: MainAppCache {
                code: AnyData::new("".to_owned()),
                lines: vec![],
                params: MainAppParams::default(),

                #[cfg(target_arch = "wasm32")]
                transfer_data: TransferData::default(),
            },
            samples_cache: Default::default(),

            error: None,

            #[cfg(target_arch = "wasm32")]
            is_loaded_from_url: false,
            side_panel_open: false,
            panel_status: Default::default(),
            selected_sample: "",
            hovered_sample: "",
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

        if !self.panel_status.contains(WINDOW_NAMES[0][1]) {
            self.selected_sample = "";
            self.hovered_sample = "";
        }

        if self.params.vis_progress_anim {
            ctx.request_repaint_after_secs(0.033);

            self.params.vis_progress += if self.params.vis_progress_anim_dir {
                1
            } else {
                -1
            };

            if self.params.vis_progress > self.params.vis_progress_max {
                self.params.vis_progress_anim_dir = false;
            }

            if self.params.vis_progress < 0 {
                self.params.vis_progress_anim_dir = true;
            }
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

        egui::SidePanel::left("Samples")
            .resizable(false)
            .max_width(300.0)
            .show_animated(ctx, self.panel_status.contains(WINDOW_NAMES[0][1]), |ui| {
                self.ui_samples_panel(ui);
            });

        egui::TopBottomPanel::bottom("SampleCodeEditor")
            .resizable(false)
            .exact_height(ctx.available_rect().height() / 2.0)
            .show_animated(ctx, self.panel_status.contains(WINDOW_NAMES[0][1]), |ui| {
                self.ui_sample_code_editor(ui);
            });

        egui::Window::new("Options")
            .open(&mut self.panel_status.contains(WINDOW_NAMES[2][1]))
            .auto_sized()
            .default_pos(ctx.available_rect().left_top())
            .movable(true)
            .show(ctx, |ui| {
                self.ui_options_panel(ui);
            });

        if ctx.available_rect().aspect_ratio() < 1.0 {
            egui::TopBottomPanel::bottom("CodeEditor")
                .resizable(false)
                .exact_height(ctx.available_rect().height() / 2.0)
                .show_animated(ctx, self.panel_status.contains(WINDOW_NAMES[3][1]), |ui| {
                    self.ui_code_editor(ui);
                });
        } else {
            egui::SidePanel::left("CodeEditor")
                .resizable(false)
                .exact_width(ctx.available_rect().width() / 2.0)
                .show_animated(ctx, self.panel_status.contains(WINDOW_NAMES[3][1]), |ui| {
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
                    if icon.is_empty() && name.is_empty() {
                        ui.separator();
                        continue;
                    }
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

    fn ui_samples_panel(&mut self, ui: &mut egui::Ui) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            let mut hover_count = 0;
            for (name, code) in SAMPLE_CODES_LIST {
                let selected = self.selected_sample == name;
                egui::containers::Frame::default()
                    .inner_margin(10.0)
                    .outer_margin(10.0)
                    .rounding(10.0)
                    .show(ui, |ui| {
                        let one_sample = ui.vertical_centered(|ui| {
                            ui.set_height(300.0);
                            ui.vertical_centered(|ui| {
                                let visualizer = CommonVecVisualizer::new([
                                    [1.0, 0.0, 0.0],
                                    [0.0, 1.0, 0.0],
                                    [0.0, 0.0, 1.0],
                                ]);

                                if !self.samples_cache.contains_key(name) {
                                    self.samples_cache.insert(name, Default::default());

                                    let v = self.samples_cache.get_mut(name).unwrap();

                                    let mut generator = VecLineGen::default();
                                    let mut parser = CodeParser::new(
                                        AnyData::new(code.to_owned()),
                                        &mut generator,
                                    );
                                    let vlg = parser.parse().unwrap_or_else(|e| {
                                        error!("Error: {:?}", e);
                                        unreachable!("The sample code can't go wrong.");
                                    });
                                    let lines = vlg.gen(0..vlg.len() as i64);
                                    v.lines = lines;
                                }

                                let v = self.samples_cache.get(name).unwrap();

                                visualizer.plot(
                                    ui,
                                    v.lines.clone(),
                                    false,
                                    true,
                                    true,
                                    false,
                                    |plot| {
                                        plot.show_axes([false, false])
                                            .id(egui::Id::from(name))
                                            .width(250.0)
                                            .height(250.0)
                                            .allow_scroll([false, false])
                                            .allow_drag([false, false])
                                            .allow_zoom([false, false])
                                            .show_x(false)
                                            .show_y(false)
                                    },
                                );
                                ui.heading(name);
                            })
                        });

                        let response = one_sample.response;

                        let visuals = ui.style().interact_selectable(&response, selected);

                        let rect = response.rect;
                        let response = ui.allocate_rect(rect, Sense::click());
                        if response.clicked() {
                            if selected {
                                self.selected_sample = ""
                            } else {
                                self.selected_sample = name;
                            }
                        }
                        if response.hovered() {
                            self.hovered_sample = name;
                            hover_count += 1;
                        }

                        if selected
                            || response.hovered()
                            || response.highlighted()
                            || response.has_focus()
                        {
                            let rect = rect.expand(10.0);
                            let mut painter = ui.painter_at(rect);
                            let rect = rect.expand(-2.0);
                            painter.rect(
                                rect,
                                10.0,
                                egui::Color32::TRANSPARENT,
                                egui::Stroke::new(2.0, ui.style().visuals.hyperlink_color),
                            );
                            painter.set_opacity(0.3);
                            painter.rect(rect, 10.0, visuals.text_color(), egui::Stroke::NONE);
                        }
                    });
            }
            if hover_count == 0 {
                self.hovered_sample = "";
            }
        });
    }

    fn ui_sample_code_editor(&mut self, ui: &mut egui::Ui) {
        if self.selected_sample.is_empty() && self.hovered_sample.is_empty() {
            return;
        }

        let sample_to_be_chosen = if !self.hovered_sample.is_empty() {
            self.hovered_sample
        } else {
            self.selected_sample
        };

        let mut sample_code = AnyData::new(
            SAMPLE_CODES_LIST
                .iter()
                .find(|x| x.0 == sample_to_be_chosen)
                .unwrap()
                .1
                .to_owned(),
        );

        ui.heading(format!("Sample: {}", sample_to_be_chosen));
        ui.separator();
        ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
            if ui.button("👆 Replaced With THIS 👇").clicked() {
                self.code = sample_code.clone::<String>();
                self.panel_status.remove(WINDOW_NAMES[0][1]);
            }
            ui.shrink_height_to_current();

            ui.separator();

            if ui.button("📋 Copy Code").clicked() {
                ui.output_mut(|o| o.copied_text = sample_code.cast_ref::<String>().clone());
            }
            if ui.button("🌐 Copy URL").clicked() {
                let transfer_data = TransferData {
                    code: sample_code.cast_ref::<String>().clone(),
                    params: Some(self.params.clone()),
                };
                let t = self.create_transfer_url(&transfer_data);
                ui.output_mut(|o| o.copied_text = format!("https://w-mai.github.io/vegravis/{t}"));
            }
        });

        ui.separator();

        CodeEditor {}.show(ui, &mut sample_code, VecLineGen::default().command_syntax());
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
                ui.allocate_ui_with_layout(
                    ui.available_size_before_wrap(),
                    egui::Layout::left_to_right(egui::Align::Center),
                    |ui| {
                        let mut anim_status = self.params.vis_progress_anim;
                        ui.toggle_value(
                            &mut anim_status,
                            if self.params.vis_progress_anim {
                                "⏸"
                            } else {
                                "▶"
                            },
                        );

                        self.params.vis_progress_anim = anim_status;
                        ui.add(
                            egui::Slider::new(
                                &mut self.params.vis_progress,
                                0..=self.params.vis_progress_max,
                            )
                            .text("Progress")
                            .show_value(true),
                        );
                    },
                );
            });
            ui.vertical_centered(|ui| {
                ui.heading("Transform Matrix");
                egui_extras::TableBuilder::new(ui)
                    .columns(egui_extras::Column::auto(), 3)
                    .body(|mut body| {
                        for i in 0..3 {
                            body.row(30.0, |mut row| {
                                for j in 0..3 {
                                    row.col(|ui| {
                                        ui.add(
                                            egui::DragValue::new(
                                                &mut self.params.trans_matrix[i][j],
                                            )
                                            .speed(0.01),
                                        )
                                        .on_hover_text(format!("m_{i}{j}"));
                                    });
                                }
                            });
                        }
                    });
            });
        });
    }

    fn ui_code_editor(&mut self, ui: &mut egui::Ui) {
        ui.heading("Code Editor");

        ui.separator();

        ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
            if ui.button("📋 Copy Code").clicked() {
                ui.output_mut(|o| o.copied_text = self.code.cast_ref::<String>().clone());
            }
            ui.shrink_height_to_current();

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
        if self.selected_sample.is_empty() && self.hovered_sample.is_empty() {
            let mut has_error = false;
            if !self.code.equal::<String, String>(&self.cache.code)
                || self.params != self.cache.params
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

                        let parsed = vlg.gen(0..self.params.vis_progress);

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
            CommonVecVisualizer::new(self.params.trans_matrix).plot(
                ui,
                self.cache.lines.clone(),
                has_error,
                self.params.show_inter_dash,
                self.params.colorful_block,
                self.params.lcd_coords,
                |x| x,
            );
        } else {
            let visualizer =
                CommonVecVisualizer::new([[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]]);
            let sample_to_be_chosen = if !self.hovered_sample.is_empty() {
                self.hovered_sample
            } else {
                self.selected_sample
            };
            let v = self.samples_cache.get(sample_to_be_chosen).unwrap();

            visualizer.plot(ui, v.lines.clone(), false, true, true, false, |plot| plot);
        }
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
