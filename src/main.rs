use fltk::{prelude::*, *};

fn main() {
    let app = app::App::default();
    let mut wind = window::Window::default().with_size(400, 300);

    let mut btn = button::Button::default()
        .with_size(100, 30)
        .with_label("Select file")
        .center_of_parent();

    wind.end();
    wind.show();

    btn.set_callback(|_| {
        let mut dialog = dialog::FileChooser::new(
            /*start dir*/ ".",
            /*pattern*/ "*.{txt,rs,toml}",
            /*type*/ dialog::FileChooserType::Multi,
            /*title*/ "Select file:",
        );
        dialog.show();
        while dialog.shown() {
            app::wait();
        }
        if dialog.count() > 1 {
            for i in 1..=dialog.count() { // values start at 1
                println!(" VALUE[{}]: '{}'", i, dialog.value(i).unwrap());
            }
        }
    });

    app.run().unwrap();
}
