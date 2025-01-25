//! Hello
//!
use macroquad::prelude::*;

use macroquad::ui::{hash, root_ui, widgets, Skin};

#[warn(dead_code)]
pub async fn get_skin() -> Skin {
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
}
