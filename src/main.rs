#![windows_subsystem = "windows"]
mod functions;
use functions::http_request;
use fltk::{
    prelude::*,
    enums::{
        Color,
        FrameType,
    },
    menu::Choice,
    window::Window,
    button::Button,
    input::Input,
    frame::Frame,
    app::App,
    image::PngImage,
    text::{
        TextDisplay,
        WrapMode,
        TextBuffer
    }
};

#[tokio::main]
async fn main() {

    let icon: PngImage = PngImage::load(&std::path::Path::new("src/api.png")).unwrap();
    let app: App = App::default();
    
    let mut wind: Window = Window::new(100, 100, 1000, 600, "Fltk HTTP Request Rust");
    wind.set_icon(Some(icon));
    wind.set_border(true);
    wind.set_color(Color::from_rgb(155,155,155));
    wind.make_resizable(true);

    let mut enter_url: Frame = Frame::new(80, 15, 300, 25, "Enter the URL");
    enter_url.set_label_size(16);
    let mut input: Input = Input::new(50, 50, 350, 25, "");
    input.set_frame(FrameType::RoundedBox);

    let mut http_method: Frame = Frame::new(80, 85, 300, 25, "HTTP method");
    http_method.set_label_size(16);
    let mut choice: Choice = Choice::new(50, 120, 350, 25,"");
    choice.add_choice("GET");
    choice.add_choice("POST");
    choice.add_choice("PUT");
    choice.add_choice("DELETE");

    let mut body_text: Frame = Frame::new(80, 160, 300, 25, "Body");
    body_text.set_label_size(16);
    let mut body: Input = Input::new(50, 190, 350, 200, "");
    body.set_frame(FrameType::RoundedBox);

    let mut user_text: Frame = Frame::new(80, 410, 300, 25, "User Data Base");
    user_text.set_label_size(16);
    let mut user: Input = Input::new(50, 445, 350, 25, "");
    user.set_frame(FrameType::RoundedBox);

    let mut password_text: Frame = Frame::new(80, 480, 300, 25, "Password Data Base");
    password_text.set_label_size(16);
    let mut password: Input = Input::new(50, 515, 350, 25, "");
    password.set_frame(FrameType::RoundedBox);

    let mut btn: Button = Button::new(50, 560, 350, 25, "Submit");
    btn.set_label_size(16);

    let mut buf: TextBuffer = TextBuffer::default();
    buf.set_text("Response");
    buf.append("\n");
    let mut response: TextDisplay = TextDisplay::new(500, 10, 490, 580, "");
    response.set_color(Color::from_rgb(255,255,255));
    response.set_buffer(buf);
    response.wrap_mode(WrapMode::AtBounds, 0);
    response.set_text_color(Color::Black);
    response.set_text_size(16);

    btn.set_callback(move |_| {
        let mut response = response.clone();
        let mut new_buffer: TextBuffer = TextBuffer::default();
        let input_value: String = input.value();
        let method: i32 = choice.value();
        let body_value: String = body.value();
        let user: String = user.value();
        let password: String = password.value();
        let http = async move {
            new_buffer.set_text("");
            new_buffer.append("\n");
            response.set_buffer(new_buffer.clone());
            let res = http_request::http_request(
                method,
                input_value.clone(),
                body_value,
                user,
                password,
            ).await.expect("Error HTTP");
            new_buffer.set_text(&res);
            new_buffer.append("\n");
            response.set_buffer(new_buffer);
        };
        tokio::spawn(http);
    });

    wind.end();
    wind.show();
    app.run().unwrap();
}
