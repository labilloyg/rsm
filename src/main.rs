use macroquad::prelude::*;

use macroquad::ui::{hash, root_ui, widgets, Skin};

#[derive(Debug, Clone)]
struct State {
    // The values for the exercice
    result: String,
    x1: String,
    x2: String,
    // The info box
    dialog: String,
    // The reveal window to update value
    open_window: bool,
    pop_up_buf: String,
    target: DiagramValue,
}

impl State {
    fn check_result(&self) -> String {
        let res = self.result.parse::<i32>();
        let x1_val = self.x1.parse::<i32>();
        let x2_val = self.x2.parse::<i32>();

        match (res, x1_val, x2_val) {
            (Ok(res), Ok(x1_val), Ok(x2_val)) => {
                if res == x1_val + x2_val {
                    String::from("Corret!")
                } else {
                    String::from("Try again :)")
                }
            }
            _ => String::from("Invalid input!"),
        }
    }
}

impl Default for State {
    fn default() -> Self {
        Self {
            result: String::from("12"),
            x1: String::from("<x1>"),
            x2: String::from("4"),
            dialog: String::from("Waiting..."),
            open_window: false,
            pop_up_buf: String::from(""),
            target: DiagramValue::Undefined,
        }
    }
}

#[derive(Debug, Clone)]
enum DiagramValue {
    Result,
    X1,
    X2,
    Undefined,
}

#[macroquad::main("UI showcase")]
async fn main() {
    let mut state = State::default();

    let skin1 = {
        let button_style = root_ui()
            .style_builder()
            .background(
                Image::from_file_with_format(include_bytes!("../ui_myassets/circle150.png"), None)
                    .unwrap(),
            )
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
            widgets::Group::new(hash!(), vec2(screen_width(), screen_height() / 3.0))
                .position(vec2(50.0, 0.5 * screen_height() / 3.0))
                .ui(ui, |ui| {
                    if widgets::Button::new(state.result.clone())
                        .size(vec2(100., 100.))
                        .position(vec2(100., 0.))
                        .ui(ui)
                    {
                        state.target = DiagramValue::Result;
                        state.pop_up_buf = state.result.clone();
                        state.open_window = true;
                        state.dialog = "You pushed the button 1!".to_string();
                    };

                    if widgets::Button::new(state.x1.clone())
                        .size(vec2(100., 100.))
                        .position(vec2(0., 100.))
                        .ui(ui)
                    {
                        state.target = DiagramValue::X1;
                        state.pop_up_buf = state.x1.clone();
                        state.open_window = true;
                        state.dialog = "You pushed the button 2!".to_string();
                    };

                    if widgets::Button::new(state.x2.clone())
                        .size(vec2(100., 100.))
                        .position(vec2(200., 100.))
                        .ui(ui)
                    {
                        state.target = DiagramValue::X2;
                        state.pop_up_buf = state.x2.clone();
                        state.open_window = true;
                        "You pushed the button 3!".to_string();
                    };
                });
            widgets::Editbox::new(hash!(), vec2(200., 30.)).ui(ui, &mut state.dialog);
        });

        root_ui().pop_skin();

        if state.open_window {
            widgets::Window::new(hash!(), vec2(150., 150.), vec2(200., 200.)).ui(
                &mut root_ui(),
                |ui| {
                    widgets::InputText::new(hash!())
                        .size(vec2(100.0, 30.0))
                        .filter_numbers()
                        .ui(ui, &mut state.pop_up_buf);

                    if widgets::Button::new("Go!")
                        .size(vec2(60., 30.))
                        .position(vec2(50., 50.))
                        .ui(ui)
                    {
                        match state.target {
                            DiagramValue::X1 => {
                                state.x1 = state.pop_up_buf.clone();
                            }
                            DiagramValue::X2 => {
                                state.x2 = state.pop_up_buf.clone();
                            }
                            DiagramValue::Result => {
                                state.result = state.pop_up_buf.clone();
                            }
                            DiagramValue::Undefined => {}
                        }
                        state.open_window = false;
                        state.dialog = state.check_result();
                    };
                },
            );
        };
        root_ui().pop_skin();

        next_frame().await;
    }
}
