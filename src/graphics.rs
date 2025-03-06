use gtk::prelude::*;
use gtk::{self, glib};

pub fn run_app(app_id: &str) -> glib::ExitCode {
    let app = build_app(app_id);
    app.run()
}
pub fn build_app(app_id: &str) -> gtk::Application {
    let app = gtk::Application::builder().application_id(app_id).build();
    app.connect_activate(build_ui);
    app
}

fn build_ui(app: &gtk::Application) {
    let label = gtk::Label::builder().label("label").build();

    gtk::ApplicationWindow::builder().application(app)
        .title("Todo—Matrix")
        .height_request(300)
        .width_request(300)
        .child(&label)
        .build().present();
}


