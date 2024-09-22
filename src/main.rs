#![windows_subsystem = "windows"]
mod functions;
use {
    cascade::cascade,
    fltk::{
        app,
        app::App,
        button::Button,
        enums::{Color, FrameType},
        frame::Frame,
        image::PngImage,
        input::Input,
        menu::Choice,
        prelude::*,
        text::{StyleTableEntry, TextBuffer, TextDisplay, WrapMode},
        window::Window,
    },
    functions::http_request,
    json_tools::{Buffer, BufferType, Lexer, Span, TokenType},
};

#[tokio::main]
async fn main() -> Result<(), FltkError> {
    let app: App = App::default();
    cascade!(
        Window::new(100, 100, 1000, 600, "Fltk HTTP Request Rust");
        ..set_icon(Some(PngImage::load("assets/api.png").unwrap()));
        ..set_border(true);
        ..set_color(Color::from_rgb(155,155,155));
        ..make_resizable(true);
        ..add(&cascade!(
            Frame::new(80, 15, 300, 25, "Enter the URL");
            ..set_label_size(16);
        ));
        ..add(&cascade!(
            Input::new(50, 50, 350, 25, "").with_id("url");
            ..set_frame(FrameType::RoundedBox);
            ..set_value(r#"https://ipinfo.io/json"#);
        ));
        ..add(&cascade!(
            Frame::new(80, 85, 300, 25, "HTTP method");
            ..set_label_size(16);
        ));
        ..add(&cascade!(
            Choice::new(50, 120, 350, 25,"").with_id("method");
            ..add_choice("GET|POST|PUT|DELETE");
            ..set_value(0);
        ));
        ..add(&cascade!(
            Frame::new(80, 160, 300, 25, "Body");
            ..set_label_size(16);
        ));
        ..add(&cascade!(
            Input::new(50, 190, 350, 200, "").with_id("body");
            ..set_frame(FrameType::RoundedBox);
        ));
        ..add(&cascade!(
            Frame::new(80, 410, 300, 25, "User Data Base");
            ..set_label_size(16);
        ));
        ..add(&cascade!(
            Input::new(50, 445, 350, 25, "").with_id("user");
            ..set_frame(FrameType::RoundedBox);
        ));
        ..add(&cascade!(
            Frame::new(80, 480, 300, 25, "Password Data Base");
            ..set_label_size(16);
        ));
        ..add(&cascade!(
            Input::new(50, 515, 350, 25, "").with_id("password");
            ..set_frame(FrameType::RoundedBox);
        ));
        ..add(&cascade!(
            Button::new(50, 560, 350, 25, "Submit");
            ..set_label_size(16);
            ..set_callback(move |_| {
                tokio::spawn(async move {
                    let mut display = app::widget_from_id::<TextDisplay>("responce").unwrap();
                    display.buffer().unwrap().set_text(&http_request::http_request(
                        app::widget_from_id::<Choice>("method").unwrap().value(),
                        app::widget_from_id::<Input>("url").unwrap().value(),
                        app::widget_from_id::<Input>("body").unwrap().value(),
                        app::widget_from_id::<Input>("user").unwrap().value(),
                        app::widget_from_id::<Input>("password").unwrap().value(),
                    ).await.expect("Error HTTP"));
                    display.do_callback();
                });
            });
        ));
        ..add(&cascade!(
            TextDisplay::new(500, 10, 490, 580, "").with_id("responce");
            ..set_color(Color::from_rgb(255,255,255));
            ..set_buffer(TextBuffer::default());
            ..wrap_mode(WrapMode::AtBounds, 0);
            ..set_text_color(Color::Black);
            ..set_text_size(16);
            ..set_callback(add_highlight);
            ..insert("Response");
        ));
        ..end();
    )
    .show();
    app.run()
}

fn add_highlight(display: &mut TextDisplay) {
    let text = display.buffer().unwrap().text();
    let styles: Vec<StyleTableEntry> = [Color::Red, Color::Blue, Color::Green, Color::Yellow]
        .into_iter()
        .map(|color| StyleTableEntry {
            color,
            font: display.text_font(),
            size: display.text_size(),
        })
        .collect();
    let mut buffer = vec![b'A'; text.len()];
    for token in Lexer::new(text.bytes(), BufferType::Span) {
        let c = match token.kind {
            TokenType::String => 'B',
            TokenType::BooleanTrue | TokenType::BooleanFalse | TokenType::Null => 'C',
            TokenType::Number => 'D',
            _ => 'A',
        };
        if let Buffer::Span(Span { first, end }) = token.buf {
            let start = first as _;
            let last = end as _;
            buffer[start..last].copy_from_slice(c.to_string().repeat(last - start).as_bytes());
        }
    }
    let mut buf = TextBuffer::default();
    buf.set_text(&String::from_utf8_lossy(&buffer));
    display.scroll(text.split_whitespace().count() as i32, 0);
    display.set_highlight_data(buf, styles);
}
