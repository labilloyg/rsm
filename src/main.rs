// use std::hash;

use std::i32;

use macroquad::prelude::*;

use macroquad::ui::{hash, root_ui, widgets, Skin};

// struct Diagram {
//     pub total: widgets::Editbox,
//     pub x: widgets::Editbox,
//     pub y: widgets::Editbox,
// }

#[macroquad::main("UI showcase")]
async fn main() {
    let _ = {
        let font = load_ttf_font("../ui_assets/HTOWERT.TTF").await.unwrap();
        let label_style = root_ui()
            .style_builder()
            .with_font(&font)
            .unwrap()
            .text_color(Color::from_rgba(180, 180, 120, 255))
            .font_size(30)
            .build();

        let window_style = root_ui()
            .style_builder()
            .background(
                Image::from_file_with_format(
                    include_bytes!("../ui_assets/window_background.png"),
                    None,
                )
                .unwrap(),
            )
            .background_margin(RectOffset::new(20.0, 20.0, 10.0, 10.0))
            .margin(RectOffset::new(-20.0, -30.0, 0.0, 0.0))
            .build();

        let button_style = root_ui()
            .style_builder()
            .background(
                Image::from_file_with_format(
                    include_bytes!("../ui_assets/button_background.png"),
                    None,
                )
                .unwrap(),
            )
            .background_margin(RectOffset::new(37.0, 37.0, 5.0, 5.0))
            .margin(RectOffset::new(10.0, 10.0, 0.0, 0.0))
            .background_hovered(
                Image::from_file_with_format(
                    include_bytes!("../ui_assets/button_hovered_background.png"),
                    None,
                )
                .unwrap(),
            )
            .background_clicked(
                Image::from_file_with_format(
                    include_bytes!("../ui_assets/button_clicked_background.png"),
                    None,
                )
                .unwrap(),
            )
            .with_font(&font)
            .unwrap()
            .text_color(Color::from_rgba(180, 180, 100, 255))
            .font_size(40)
            .build();

        // mark1
        let editbox_style = root_ui()
            .style_builder()
            .color(Color::from_rgba(0, 0, 0, 0))
            .color_clicked(Color::from_rgba(0, 0, 0, 0))
            .background(
                Image::from_file_with_format(
                    include_bytes!("../ui_assets/button_background.png"),
                    None,
                )
                .unwrap(),
            )
            .background_margin(RectOffset::new(37.0, 37.0, 5.0, 5.0))
            .margin(RectOffset::new(50.0, 50.0, 10.0, 0.0))
            .background_hovered(
                Image::from_file_with_format(
                    include_bytes!("../ui_assets/button_hovered_background.png"),
                    None,
                )
                .unwrap(),
            )
            .background_clicked(
                Image::from_file_with_format(
                    include_bytes!("../ui_assets/button_clicked_background.png"),
                    None,
                )
                .unwrap(),
            )
            .with_font(&font)
            .unwrap()
            .text_color(Color::from_rgba(180, 180, 100, 255))
            .font_size(40)
            .build();

        Skin {
            editbox_style,
            window_style,
            button_style,
            label_style,
            ..root_ui().default_skin()
        }
    };

    // let default_skin = root_ui().default_skin().clone();
    // let mut window1_skin = skin1.clone();

    let mut result = String::from("12");
    let mut x1 = String::from("<x1>");
    let mut x2 = String::from("4");
    let mut dialog = String::from("Waiting...");
    loop {
        clear_background(GRAY);

        // window:wroot_ui().push_skin(&window1_skin);

        // Window definition
        widgets::Window::new(hash!(), vec2(0., 0.), vec2(400., 400.)).ui(&mut root_ui(), |ui| {
            widgets::Group::new(hash!("group"), vec2(screen_width(), screen_height() / 3.0))
                .position(vec2(0.0, 0.5 * screen_height() / 3.0))
                .ui(ui, |ui| {
                    widgets::Label::new("Diagram")
                        // .position(vec2(200.0, 0.0))
                        .ui(ui);
                    widgets::InputText::new(hash!())
                        .size(vec2(100.0, 30.0))
                        .filter_numbers()
                        // .position(vec2(50.0, 70.0))
                        .ui(ui, &mut result);

                    widgets::InputText::new(hash!())
                        .size(vec2(100.0, 30.0))
                        .filter_numbers()
                        // .position(vec2(100.0, 140.0))
                        .ui(ui, &mut x1);

                    widgets::InputText::new(hash!())
                        .size(vec2(100.0, 30.0))
                        .filter_numbers()
                        // .position(vec2(100.0, 140.0))
                        .ui(ui, &mut x2);

                    if widgets::Button::new("Answer!").ui(ui) {
                        let res = result.parse::<i32>();
                        let x1_val = x1.parse::<i32>();
                        let x2_val = x2.parse::<i32>();

                        match (res, x1_val, x2_val) {
                            (Ok(res), Ok(x1_val), Ok(x2_val)) => {
                                dialog = if res == x1_val + x2_val {
                                    String::from("Corret!")
                                } else {
                                    String::from("Try again :)")
                                };
                            }
                            _ => {
                                dialog = String::from("Invalid input!");
                            }
                        }
                    };

                    if widgets::Button::new("Reset!").ui(ui) {
                        result = String::from("12");
                        x1 = String::from("<x1>");
                        x2 = String::from("4");
                        dialog = String::from("Waiting..");
                    };
                    widgets::Editbox::new(hash!(), vec2(200., 30.)).ui(ui, &mut dialog);
                });
        });

        root_ui().pop_skin();

        next_frame().await;
    }
}
