use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Button};

fn main() {
    let app = Application::builder()
        .application_id("com.example.mygtkapp")
        .build();

    app.connect_activate(|app| {
        let button = Button::with_label("Click");
        button.connect_clicked(|_| {
        });

        let window = ApplicationWindow::builder()
            .application(app)
            .title("Hello GTK + Rust")
            .default_width(300)
            .default_height(100)
            .child(&button)
            .build();

        window.show();
    });

    app.run();
}