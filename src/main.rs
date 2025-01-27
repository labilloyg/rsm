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

fn window_conf() -> Conf {
    Conf {
        window_title: "RSM [Rust School of Mathematics]".to_owned(),
        fullscreen: false,
        window_width: 640,
        window_height: 320,
        window_resizable: false,
        platform: miniquad::conf::Platform {
            linux_backend: miniquad::conf::LinuxBackend::WaylandOnly,
            ..Default::default()
        },
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut state = State::default();

    let skin1 = {
        let button_style = root_ui()
            .style_builder()
            .background(
                Image::from_file_with_format(include_bytes!("../ui_myassets/circle100.png"), None)
                    .unwrap(),
            )
            .build();

        Skin {
            button_style,
            ..root_ui().default_skin()
        }
    };

    loop {
        clear_background(PINK);

        widgets::Group::new(hash!(), vec2(140., 320.))
            .position(vec2(0., 0.))
            .ui(&mut root_ui(), |ui| {
                widgets::Label::new("Info...".to_string()).ui(ui);
            });

        widgets::Group::new(hash!(), vec2(500., 50.))
            .position(vec2(140., 0.))
            .ui(&mut root_ui(), |ui| {
                widgets::Label::new("What is the missing number in this addition?".to_string())
                    .ui(ui);
            });

        root_ui().push_skin(&skin1);

        widgets::Group::new(hash!(), vec2(225., 150.))
            .position(vec2(277., 50.))
            .ui(&mut root_ui(), |ui| {
                if widgets::Button::new(state.result.clone())
                    .size(vec2(75., 75.))
                    .position(vec2(75., 0.))
                    .ui(ui)
                {
                    state.target = DiagramValue::Result;
                    state.pop_up_buf = state.result.clone();
                    state.open_window = true;
                    state.dialog = "You pushed the button 1!".to_string();
                };

                if widgets::Button::new(state.x1.clone())
                    .size(vec2(75., 75.))
                    .position(vec2(0., 75.))
                    .ui(ui)
                {
                    state.target = DiagramValue::X1;
                    state.pop_up_buf = state.x1.clone();
                    state.open_window = true;
                    state.dialog = "You pushed the button 2!".to_string();
                };

                if widgets::Button::new(state.x2.clone())
                    .size(vec2(75., 75.))
                    .position(vec2(150., 75.))
                    .ui(ui)
                {
                    state.target = DiagramValue::X2;
                    state.pop_up_buf = state.x2.clone();
                    state.open_window = true;
                    "You pushed the button 3!".to_string();
                };
            });

        widgets::Editbox::new(hash!(), vec2(300., 30.))
            .position(vec2(240., 275.))
            .ui(&mut root_ui(), &mut state.dialog);

        root_ui().pop_skin();

        if state.open_window {
            widgets::Window::new(hash!(), vec2(277., 50.), vec2(150., 100.)).ui(
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
