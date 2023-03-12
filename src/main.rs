mod functions;
use functions::http_get;

use fltk::{
    prelude::*,
    *,
    enums::{
        Color,
        FrameType,
        Align
    },
    window::Window,
    button::Button,
    input::Input,
    frame::Frame,
    app::App,
    group::Flex,
    image::PngImage
};

#[tokio::main]
async fn main() {
    let icon: PngImage = PngImage::load(&std::path::Path::new("src/api.png")).unwrap();
    let app: App = App::default();
    let mut wind: Window = Window::new(100, 100, 1000, 600, "Fltk HTTP Request Rust");
    wind.set_icon(Some(icon));
    wind.set_border(true);
    wind.set_color(Color::from_rgb(64,69,73));
    wind.make_resizable(true);

    /*let mut history_http: Window = Window::new(0, 0, 100, 600, "");
    history_http.set_color(Color::from_rgb(8,66,255));
    input_request.end();
    history_http.end();

    let mut input_request: Window = Window::new(100, 0, 900, 200, "");
    input_request.set_color(Color::from_rgb(8,66,200));*/
    let mut bar =
        Frame::new(0, 0, 1000, 60, "FLTK App!").with_align(Align::Left | Align::Inside);
        bar.set_frame(FrameType::FlatBox);
            bar.set_label_size(22);
            bar.set_label_color(Color::White);
            bar.set_color(Color::from_hex(0x42A5F5));
            bar.draw(|b| {
                draw::set_draw_rgb_color(211, 211, 211);
                draw::draw_rectf(0, b.height(), b.width(), 3);
            });
    let _label: Frame = Frame::default()
        .with_label("Enter age");
    let mut input: Input = Input::default()
        .center_of(&wind)
        .with_size(200, 25);
    let mut btn = Button::new(500, 300, 80, 30, "");
    input.set_frame(FrameType::RoundedBox);

    /*let mut btn: Button = Button::default()
        .with_size(200, 50)
        .with_label("Submit");*/


    /*btn.set_callback(move |_| {
        let input_value: String = input.value();
        let http = async move {
            let response = http_get::http_get(&input_value).await;
            println!("{:?}", response);
        };
        tokio::spawn(http);
    });*/

    wind.end();
    wind.show();
    app.run().unwrap();
}
