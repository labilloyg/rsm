use macroquad::prelude::*;

use macroquad::ui::{hash, root_ui, widgets, Skin};

use ::rand;
use ::rand::prelude::*;

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

async fn get_skin_title() -> Skin {
    let font = load_ttf_font("ui_myassets/Coolvetica Rg.otf").await;

    let label_style = match font {
        Ok(font) => root_ui()
            .style_builder()
            .with_font(&font)
            .unwrap()
            .text_color(BLACK)
            .font_size(24)
            .build(),
        _ => root_ui().style_builder().build(),
    };

    Skin {
        label_style,
        ..root_ui().default_skin()
    }
}

async fn get_skin_diagram() -> Skin {
    let font = load_ttf_font("ui_myassets/Coolvetica Rg.otf").await;

    let circle100 =
        Image::from_file_with_format(include_bytes!("../ui_myassets/circle75.png"), None);

    let button_style = match (circle100, font) {
        (Ok(circle100), Ok(font)) => root_ui()
            .style_builder()
            .background(circle100)
            .with_font(&font)
            .unwrap()
            .text_color(BLACK)
            .font_size(32)
            .build(),
        (_, _) => root_ui().style_builder().build(),
    };

    Skin {
        button_style,
        ..root_ui().default_skin()
    }
}

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

    fn truncate(&mut self) {
        match self.target {
            DiagramValue::X1 => {
                let n = self.x1.len();
                if n > 0 {
                    self.x1.truncate(n - 1);
                }
            }
            DiagramValue::X2 => {
                let n = self.x2.len();
                if n > 0 {
                    self.x2.truncate(n - 1);
                }
            }
            DiagramValue::Result => {
                let n = self.result.len();
                if n > 0 {
                    self.result.truncate(n - 1);
                }
            }
        }
    }

    fn push(&mut self, c: char) {
        match self.target {
            DiagramValue::X1 => {
                self.x1.push(c);
            }
            DiagramValue::X2 => {
                self.x2.push(c);
            }
            DiagramValue::Result => {
                self.result.push(c);
            }
        }
    }

    fn random_addition(state: &mut State) {
        let mut rng = rand::rng();
        let result: u32 = rng.random_range(0..=20);
        let x: i32 = rng.random_range(0..=result as i32);
        let xy = rng.random_range(0..=1);

        if xy == 0 {
            state.x1 = x.to_string();
            state.x2 = "".to_string();
            state.target = DiagramValue::X2;
        } else {
            state.x1 = "".to_string();
            state.x2 = x.to_string();
            state.target = DiagramValue::X1;
        }
        state.result = result.to_string();
    }
}

impl Default for State {
    fn default() -> Self {
        Self {
            result: String::from("12"),
            x1: String::from(""),
            x2: String::from("4"),
            dialog: String::from("Waiting..."),
            open_window: false,
            pop_up_buf: String::from(""),
            target: DiagramValue::X1,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum DiagramValue {
    Result,
    X1,
    X2,
}

enum KeyAction {
    Truncate,
    Push(char),
    Check,
    Next,
}

fn check_key_down() -> Option<KeyAction> {
    let nums = [
        KeyCode::Key1,
        KeyCode::Key2,
        KeyCode::Key3,
        KeyCode::Key4,
        KeyCode::Key5,
        KeyCode::Key6,
        KeyCode::Key7,
        KeyCode::Key8,
        KeyCode::Key9,
        KeyCode::Key0,
        KeyCode::Kp0,
        KeyCode::Kp1,
        KeyCode::Kp2,
        KeyCode::Kp3,
        KeyCode::Kp4,
        KeyCode::Kp5,
        KeyCode::Kp6,
        KeyCode::Kp7,
        KeyCode::Kp8,
        KeyCode::Kp9,
    ];

    if is_key_down(KeyCode::Backspace) {
        return Some(KeyAction::Truncate);
    };

    if is_key_down(KeyCode::Enter) || is_key_down(KeyCode::KpEnter) {
        if is_key_down(KeyCode::LeftShift) {
            return Some(KeyAction::Next);
        } else {
            return Some(KeyAction::Check);
        }
    }

    if is_key_down(KeyCode::Right) {
        return Some(KeyAction::Next);
    }

    let keys = get_keys_down();
    if !keys.is_empty() {
        if let Some(key) = keys.iter().next() {
            if nums.contains(key) {
                let key_val = *key as u32;
                let char_val = if key_val >= 65456 {
                    key_val - 65456 + 48
                } else {
                    key_val
                };
                let c = char::from_u32(char_val);
                if let Some(c) = c {
                    return Some(KeyAction::Push(c));
                } else {
                    return None;
                }
            };
        }
    };
    None
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut state = State::default();

    let skin_diagram = get_skin_diagram().await;
    let skin_title = get_skin_title().await;

    let typing_thd = 0.15;
    let mut ref_time = get_time();

    loop {
        clear_background(PINK);
        let can_type = (get_time() - ref_time) >= typing_thd;

        if can_type {
            if let Some(action) = check_key_down() {
                ref_time = get_time();
                state.dialog = "Waiting...".to_string();
                match action {
                    KeyAction::Truncate => {
                        state.truncate();
                    }
                    KeyAction::Push(c) => {
                        state.push(c);
                    }
                    KeyAction::Check => {
                        state.dialog = state.check_result();
                    }
                    KeyAction::Next => {
                        State::random_addition(&mut state);
                    }
                }
            };
        }

        widgets::Group::new(hash!(), vec2(140., 320.))
            .position(vec2(0., 0.))
            .ui(&mut root_ui(), |ui| {
                widgets::Label::new("Info...".to_string()).ui(ui);
            });

        root_ui().push_skin(&skin_title);

        widgets::Group::new(hash!(), vec2(500., 50.))
            .position(vec2(140., 0.))
            .ui(&mut root_ui(), |ui| {
                widgets::Label::new("What is the missing number in this addition?".to_string())
                    .ui(ui);
            });

        root_ui().push_skin(&skin_diagram);

        widgets::Group::new(hash!(), vec2(227., 152.))
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
                };

                if widgets::Button::new(state.x1.clone())
                    .size(vec2(75., 75.))
                    .position(vec2(0., 75.))
                    .ui(ui)
                {
                    state.target = DiagramValue::X1;
                    state.pop_up_buf = state.x1.clone();
                    state.open_window = true;
                };

                if widgets::Button::new(state.x2.clone())
                    .size(vec2(75., 75.))
                    .position(vec2(150., 75.))
                    .ui(ui)
                {
                    state.target = DiagramValue::X2;
                    state.pop_up_buf = state.x2.clone();
                    state.open_window = true;
                };
            });

        root_ui().pop_skin();

        widgets::Editbox::new(hash!(), vec2(300., 30.))
            .position(vec2(240., 275.))
            .ui(&mut root_ui(), &mut state.dialog);

        if widgets::Button::new("Next!".to_string())
            .size(vec2(75., 25.))
            .position(vec2(352., 225.))
            .ui(&mut root_ui())
        {
            State::random_addition(&mut state);
        };

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
