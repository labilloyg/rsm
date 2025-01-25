use macroquad::prelude::*;

use macroquad::ui::{hash, root_ui, widgets, Skin};

#[macroquad::main("UI showcase")]
async fn main() {
    // let default_skin = root_ui().default_skin().clone();
    // let mut window1_skin = skin1.clone();

    let mut result = String::from("12");
    let mut x1 = String::from("<x1>");
    let mut x2 = String::from("4");
    let mut dialog = String::from("Waiting...");

    let skin1 = {
        let button_style = root_ui()
            .style_builder()
            .background(
                Image::from_file_with_format(include_bytes!("../ui_myassets/circle150.png"), None)
                    .unwrap(),
            )
            // .background_margin(RectOffset::new(37.0, 37.0, 5.0, 5.0))
            // .margin(RectOffset::new(10.0, 10.0, 0.0, 0.0))
            // .background_hovered(
            //     Image::from_file_with_format(
            //         include_bytes!("../ui_assets/button_hovered_background.png"),
            //         None,
            //     )
            //     .unwrap(),
            // )
            // .background_clicked(
            //     Image::from_file_with_format(
            //         include_bytes!("../ui_assets/button_clicked_background.png"),
            //         None,
            //     )
            //     .unwrap(),
            // )
            // .text_color(Color::from_rgba(180, 180, 100, 255))
            // .font_size(40)
            .build();

        Skin {
            button_style,
            ..root_ui().default_skin()
        }
    };

    loop {
        clear_background(GRAY);

        root_ui().push_skin(&skin1);

        widgets::Window::new(hash!(), vec2(300., 0.), vec2(400., 400.)).ui(&mut root_ui(), |ui| {
            widgets::Group::new(hash!("group"), vec2(screen_width(), screen_height() / 3.0))
                .position(vec2(50.0, 0.5 * screen_height() / 3.0))
                .ui(ui, |ui| {
                    if widgets::Button::new("8").size(vec2(100., 100.)).ui(ui) {
                        println!("You pushed the button 1!");
                    };

                    if widgets::Button::new("10")
                        .size(vec2(100., 100.))
                        .position(vec2(200., 0.))
                        .ui(ui)
                    {
                        println!("You pushed the button 2!");
                    };

                    if widgets::Button::new("18")
                        .size(vec2(100., 100.))
                        .position(vec2(100., 100.))
                        .ui(ui)
                    {
                        println!("You pushed the button 3!");
                    };
                });
        });

        root_ui().pop_skin();

        // Window definition
        widgets::Window::new(hash!(), vec2(0., 0.), vec2(300., 400.)).ui(&mut root_ui(), |ui| {
            widgets::Group::new(hash!("group"), vec2(screen_width(), screen_height() / 3.0))
                .position(vec2(0.0, 0. * screen_height() / 3.0))
                .ui(ui, |ui| {
                    widgets::Label::new("Diagram").ui(ui);
                    widgets::InputText::new(hash!())
                        .size(vec2(100.0, 30.0))
                        .filter_numbers()
                        .ui(ui, &mut result);

                    widgets::InputText::new(hash!())
                        .size(vec2(100.0, 30.0))
                        .filter_numbers()
                        .ui(ui, &mut x1);

                    widgets::InputText::new(hash!())
                        .size(vec2(100.0, 30.0))
                        .filter_numbers()
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
