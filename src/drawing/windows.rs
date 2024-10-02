use egui::{Context, Ui};
use esp::aim_element;
use windows::Win32::UI::WindowsAndMessaging::SetForegroundWindow;

use crate::{external::cheat::aim, settings::mgr};
use super::{localization::Lang, overlay::Overlay};

pub fn draw_windows(overlay: &mut Overlay, ctx: &Context, ui: &mut Ui) {
    draw_main(overlay, ctx, ui);
    draw_esp(overlay, ctx, ui);
    draw_aim(overlay, ctx, ui);
}

fn draw_main(overlay: &mut Overlay, ctx: &Context, _ui: &mut Ui) {
    egui::Window::new("Main")
        .resizable(false)
        .collapsible(false)
        .show(ctx, |ui| {
            ui.separator();
            ui.label("Language");
            egui::ComboBox::from_id_salt("lang_selector")
                        .selected_text(format!("{:?}", overlay.lang))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut overlay.lang,
                                Lang::EN,
                                "English",
                            );
                            ui.selectable_value(
                                &mut overlay.lang,
                                Lang::RU,
                                "Русский",
                            );
                            ui.selectable_value(
                                &mut overlay.lang,
                                Lang::CH,
                                "中文",
                            );
                        });

            ui.separator();
            ui.label(overlay.lang.config());
            ui.separator();
            if ui.button(overlay.lang.config_default()).clicked() {
                mgr::change(&mut overlay.settings, "default.cjson");
            }
            ui.horizontal(|ui| {
                if ui.button(overlay.lang.config_load()).clicked() {
                    mgr::change(&mut overlay.settings, "current.cjson");
                }
                if ui.button(overlay.lang.config_save()).clicked() {
                    mgr::save(&mut overlay.settings, "current.cjson");
                }
            });
            ui.collapsing("json:", |ui| {
                ui.label(format!("{:?}", overlay.settings));
            });
            ui.separator();
            ui.hyperlink_to(overlay.lang.repository(), "https://github.com/Loara228/deadlock-esp");
            ui.separator();
            if ui.button(overlay.lang.close()).clicked() {
                std::process::exit(0);
            }
        });
}

fn draw_aim(overlay: &mut Overlay, ctx: &Context, _ui: &mut Ui) {
    egui::Window::new("Aim")
        .resizable(false).show(ctx, |ui| {
            if overlay.settings.aim.angle_per_pixel == 0f32
            {
                ui.label(egui::RichText::new(overlay.lang.aim_not_calibrated()).color(egui::Color32::RED));
            }
            if ui.button(overlay.lang.aim_calibrate()).clicked() {
                unsafe { SetForegroundWindow(overlay.game_hwnd).unwrap(); }
                overlay.settings.aim.angle_per_pixel = aim::aiming::calibrate(&mut overlay.game);
                if overlay.settings.aim.angle_per_pixel.is_nan()
                {
                    overlay.settings.aim.angle_per_pixel = 0f32;
                }
                unsafe { SetForegroundWindow(overlay.overlay_hwnd).unwrap(); }
            }
            ui.label(overlay.lang.aim_players());
            ui.group(|ui| {
                aim_element(ui, &mut overlay.settings.aim, false, overlay.lang);
            });
            ui.label(overlay.lang.aim_creeps());
            ui.group(|ui| {
                aim_element(ui, &mut overlay.settings.aim, true, overlay.lang);
            });

        ui.horizontal(|ui|
            {
                ui.color_edit_button_srgba(&mut overlay.settings.aim.soul_color);
                ui.label(overlay.lang.aim_creeps());
            });
        });
}

fn draw_esp(overlay: &mut Overlay, ctx: &Context, _ui: &mut Ui) {
    egui::Window::new("ESP")
        .resizable(false)
        .default_width(500.)
        .show(ctx, |ui| {
            esp::esp_players(ui, overlay);
            esp::esp_boxes(ui, overlay);
            esp::esp_radar(ui, overlay);
            esp::esp_text(ui, overlay);
        });
}

mod esp {
    use crate::{
        drawing::{localization::Lang, overlay::Overlay},
        settings::structs::*,
    };
    use egui::{Align2, Ui};


    pub fn esp_text(ui: &mut Ui, overlay: &mut Overlay) {
        ui.collapsing("Надписи", |ui| {
            ui.collapsing("Имя персонжа", |ui| {
                ui_text(ui, &mut overlay.settings.esp_players.text_hero, "ui_text_1");
            });
            ui.collapsing("Здоровье", |ui| {
                ui_text(ui, &mut overlay.settings.esp_players.text_health, "ui_text_2");
            });
            ui.collapsing("Дистанция", |ui| {
                ui_text(ui, &mut overlay.settings.esp_players.text_distance, "ui_text_3");
            });
        });
    }
    
    pub fn esp_radar(ui: &mut Ui, overlay: &mut Overlay)
    {
        ui.collapsing("Радар", |ui| {
            egui::Grid::new("radar_grid")
                .num_columns(2)
                .min_col_width(150.)
                .max_col_width(150.)
                .show(ui, |ui| {
                    ui.checkbox(&mut overlay.settings.radar.enable, "enable");
                    ui.label("Отрисовка радара на экране");
                    ui.end_row();
                    ui.add(
                        egui::Slider::new(&mut overlay.settings.radar.player_radius, 1.0..=8.0)
                            .show_value(true)
                    );
                    ui.label("Радиус точки игрока");
                    ui.end_row();
                    ui.add(
                        egui::Slider::new(&mut overlay.settings.radar.scale, 10.0..=50.0)
                            .show_value(true)
                    );
                    ui.label("Маштаб радара");
                    ui.end_row();

                    ui.color_edit_button_srgba(&mut overlay.settings.radar.color_enemy);
                    ui.label("Цвет врага");

                    ui.end_row();

                    ui.color_edit_button_srgba(&mut overlay.settings.radar.color_team);
                    ui.label("Цвет союзника");
                    ui.end_row();
                    

                    ui.color_edit_button_srgba(&mut overlay.settings.radar.color_background);
                    ui.label("Цвет фона");

                    ui.end_row();

                    ui.color_edit_button_srgba(&mut overlay.settings.radar.color_border);
                    ui.label("Цвет обводки");
                    ui.end_row();
                })
        });
    }

    pub fn esp_players(ui: &mut Ui, overlay: &mut Overlay) {
        ui.collapsing("Прямоугольники", |ui| {
            ui.checkbox(&mut overlay.settings.esp_players.render, "Включить");
            egui::Grid::new("esp_grid")
                .num_columns(2)
                .min_col_width(150.)
                .max_col_width(150.)
                .show(ui, |ui| {
                    egui::ComboBox::from_id_salt("esp_boxtype")
                        .selected_text(format!("{:?}", overlay.settings.esp_players.box_type))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut overlay.settings.esp_players.box_type,
                                BoxType::Default,
                                "Default",
                            );
                            ui.selectable_value(
                                &mut overlay.settings.esp_players.box_type,
                                BoxType::Rounded,
                                "Rounded",
                            );
                            ui.selectable_value(
                                &mut overlay.settings.esp_players.box_type,
                                BoxType::Edges,
                                "Edges",
                            );
                        });

                    ui.label("Стиль отрисовки");
                    ui.end_row();

                    ui.checkbox(&mut overlay.settings.esp_players.outline_rect, "stroke");
                    ui.label("Обводка");
                    ui.end_row();

                    ui.checkbox(&mut overlay.settings.esp_players.fill_rect, "fill");
                    ui.label("Заливка");
                    ui.end_row();

                    ui.checkbox(&mut overlay.settings.esp_players.glow, "glow");
                    ui.label("Свечение головы игрока");
                    ui.end_row();

                    ui.checkbox(&mut overlay.settings.esp_players.shadow, "shadow");
                    ui.label("Тень обводки");
                    ui.end_row();
                });
                
            ui.add(
                egui::Slider::new(&mut overlay.settings.esp_players.stroke_width, 1.0..=4.0)
                    .show_value(true)
                    .text("Толщина линий"),
            );
            ui.end_row();
            
            ui.add(
                egui::Slider::new(&mut overlay.settings.esp_players.shadow_size, 4.0..=18.0)
                    .show_value(true)
                    .text("Размер тени"),
            );
            ui.end_row();
            
            ui.add(
                egui::Slider::new(&mut overlay.settings.esp_players.shadow_blur, 1.0..=100.)
                    .show_value(true)
                    .text("Размытие тени"),
            );
            ui.end_row();
            
            egui::Grid::new("esp_grid 2")
                .num_columns(2)
                .min_col_width(150.)
                .max_col_width(150.)
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.color_edit_button_srgba(&mut overlay.settings.esp_players.outline_color);
                        ui.label("stroke");
                    });
                    ui.label("Обводка");
                    ui.end_row();
                    ui.horizontal(|ui| {
                        ui.color_edit_button_srgba(&mut overlay.settings.esp_players.fill_color);
                        ui.label("fill");
                    });
                    ui.label("Заливка");
                    ui.end_row();
                    ui.horizontal(|ui| {
                        ui.color_edit_button_srgba(&mut overlay.settings.esp_players.glow_color);
                        ui.label("glow");
                    });
                    ui.label("Подсветка");
                    ui.end_row();
                    ui.horizontal(|ui| {
                        ui.color_edit_button_srgba(&mut overlay.settings.esp_players.shadow_color);
                        ui.label("shadow");
                    });
                    ui.label("Тень обводки");
                    ui.end_row();
                });
        });
    }

    pub fn esp_boxes(ui: &mut Ui, overlay: &mut Overlay) {
        ui.collapsing("Здоровье", |ui| {
            ui.checkbox(&mut overlay.settings.healthbars.enable, "Включить");
            egui::Grid::new("healthbars_grid")
            .num_columns(2)
            .min_col_width(150.)
            .max_col_width(150.)
            .show(ui, |ui| {
                ui.color_edit_button_srgba(&mut overlay.settings.healthbars.background_color);
                ui.label("Цвет заднего фона");
                ui.end_row();
                ui.color_edit_button_srgba(&mut overlay.settings.healthbars.hp_color);
                ui.label("Цвет здоровья");
                ui.end_row();
                ui.color_edit_button_srgba(&mut overlay.settings.healthbars.outline_color);
                ui.label("Цвет обводки");
                ui.end_row();
            });
        });
    }

    /// UI блок для ESP TEXT
    fn ui_text(ui: &mut Ui, settings: &mut TextSettings, id: &str) {
        fn to_string(align: Align2) -> String {
            if align == Align2::CENTER_TOP {
                return "Сверху".to_owned();
            } else if align == Align2::LEFT_TOP {
                return "Левый верхний угол".to_owned();
            } else if align == Align2::RIGHT_TOP {
                return "Правый верхний угол".to_owned();
            } else if align == Align2::CENTER_BOTTOM {
                return "Снизу".to_owned();
            }
            "?".to_owned()
        }

        ui.checkbox(&mut settings.enable, "Включить");
        ui.horizontal(|ui| {
            ui.color_edit_button_srgba(&mut settings.font_color);
            ui.checkbox(&mut settings.shadow, "Контрастный");
        });
        ui.add(
            egui::Slider::new(&mut settings.font_size, 6.0..=24.0)
                .show_value(true)
                .text("Размер шрифта"),
        );

        egui::ComboBox::from_id_salt(id)
            .selected_text(format!("{}", to_string(settings.align)))
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut settings.align, Align2::CENTER_TOP, "Сверху");
                ui.selectable_value(&mut settings.align, Align2::LEFT_TOP, "Левый верхний угол");
                ui.selectable_value(
                    &mut settings.align,
                    Align2::RIGHT_TOP,
                    "Правый верхний угол",
                );
                ui.selectable_value(&mut settings.align, Align2::CENTER_BOTTOM, "Снизу");
            });
    }

    pub fn aim_element(ui: &mut Ui, global_aim_settings: &mut AimSettings, entities: bool, lang: Lang)
    {
        let settings: &mut AimProperties = match entities {
            true => &mut global_aim_settings.creeps,
            false => &mut global_aim_settings.players,
        };
        ui.checkbox(&mut settings.enable, lang.aim_enable());
        ui.checkbox(&mut settings.velocity_prediction, lang.aim_velocity_prediction());
        ui.checkbox(&mut settings.rcs, lang.aim_rcs());
        ui.checkbox(&mut settings.targeting, lang.aim_targeting());
        ui.horizontal(|ui| {
            ui.color_edit_button_srgba(&mut settings.color);
            ui.label(lang.aim_fov_color());
        });
        ui.add(
            egui::Slider::new(&mut settings.fov, 20.0..=800.0).show_value(true).text(lang.aim_fov())
        );
        ui.add(
            egui::Slider::new(&mut settings.smooth, 1.25..=10.0).show_value(true).text(lang.aim_smooth())
        );
        ui.add(
            egui::Slider::new(&mut settings.velocity_div_dav, 1f32..=30.0).show_value(true).text(lang.aim_velocity_prediction())
        );
        ui.horizontal(|ui| {
            ui.add(
                egui::Slider::new(&mut settings.range, 200.0..=5000.0).show_value(true).text(lang.aim_max_distance())
            );
            ui.label(format!("{} ({})", (settings.range * 0.0254f32).round(), lang.aim_meters()))
        });
    }

}