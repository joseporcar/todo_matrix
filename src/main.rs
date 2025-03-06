use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, Label};

const APP_ID: &str = "gtk.personal.TodoMatrix";
fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);

    app.run()
}
fn build_ui(app: &Application) {
    let label = Label::builder().label("label").build();

    ApplicationWindow::builder().application(app)
        .title("Todo—Matrix")
        .child(&label)
        .build().present();
}