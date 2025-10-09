use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Button};
use gtk4::gdk::Display;
mod constants;
use constants::*;

fn main() {
    let app: Application = Application::builder()
        .application_id(APP_ID)
        .build();

    app.connect_activate(|app: &Application| {
        let (width, height) = get_start_size();

        // let button: Button = Button::with_label("Click");
        // button.connect_clicked(|_| {
        // });

        let window: ApplicationWindow = ApplicationWindow::builder()
            .application(app)
            .title(APP_TITLE)
            .default_width(width)
            .default_height(height)
            //.child(&button)
            .build();

        window.show();
    });

    app.run();
}

fn get_start_size() -> (i32, i32) {
    if let Some(display) = Display::default(){
            let monitor = display.monitors().item(0).and_downcast::<gtk4::gdk::Monitor>().unwrap();
            let geo = monitor.geometry();
            (
                (geo.width() as f32 * START_WINDOW_WIDTH_PCT) as i32,
                (geo.height() as f32 * START_WINDOW_HEIGHT_PCT) as i32
            )
    } else {
        (START_WINDOW_WIDTH_INT, START_WINDOW_HEIGHT_INT)
    }
}

// const
pub const START_WINDOW_WIDTH_PCT: f32 = 0.5;
pub const START_WINDOW_HEIGHT_PCT: f32 = 0.4;
pub const START_WINDOW_WIDTH_INT: i32 = 250;
pub const START_WINDOW_HEIGHT_INT: i32 = 200;