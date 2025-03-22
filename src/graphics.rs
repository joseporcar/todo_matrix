use gtk::{gdk, prelude::*, Align};
use gtk::{self, glib, Orientation};

pub fn run_app(app_id: &str) -> glib::ExitCode {
    let app = build_app(app_id);
    app.run()
}
fn build_app(app_id: &str) -> gtk::Application {
    let app = gtk::Application::builder().application_id(app_id).build();
    app.connect_startup(|_| load_css());
    app.connect_activate(build_ui);
    app
}
fn load_css() {
    // Load the CSS file and add it to the provider
    let provider = gtk::CssProvider::new();
    provider.load_from_string(include_str!("style.css"));

    // Add the provider to the default screen
    gtk::style_context_add_provider_for_display(
        &gdk::Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

fn build_ui(app: &gtk::Application) {

    gtk::ApplicationWindow::builder()
        .application(app)
        .title("Todo—Matrix")
        .height_request(500)
        .width_request(500)
        .child(&main_box())
        .build()
        .present();
}

fn main_box() -> gtk::Box {
    let main_box = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .build();
    main_box.append(&top_bar());
    main_box.append(&center_box());
    main_box.append(&horizontal_labels());
    main_box
}
fn center_box() -> gtk::Box {
    let center_box = gtk::Box::builder()
        .orientation(Orientation::Horizontal)
        .vexpand(true)
        .hexpand(true)
        .build();
    center_box.append(&vertical_labels());
    center_box.append(&content_box());
    center_box
}
fn vertical_labels() -> gtk::Box {
    let mut vertical_labels = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .vexpand(true)
        .valign(Align::Center)
        
        .build();
    label_adder(&mut vertical_labels);
    vertical_labels
}

fn horizontal_labels() -> gtk::Box {
    let label = gtk::Label::builder().label("mr labler").build();
    let horizontal_labels = gtk::Box::builder()
        .orientation(Orientation::Horizontal)
        .build();

    horizontal_labels.append(&label);
    horizontal_labels
}

fn label_adder(area: &mut gtk::Box) {
    for label in ["High", "MidHigh", "Mid", "MidLow", "Low"] {
        area.append(&gtk::Label::builder().label(label).build());
    }
}

fn content_box() -> gtk::Box {
    let label = gtk::Label::builder().label("content_box").build();
    let content_box = gtk::Box::builder()
        .orientation(Orientation::Horizontal)
        .halign(Align::Center)
        .hexpand(true)
        .vexpand(true)
        .build();

    content_box.append(&label);
    content_box
}

fn top_bar() -> gtk::Box {
    let label = gtk::Label::builder().label("top bar").build();
    let top_bar = gtk::Box::builder()
        .orientation(Orientation::Horizontal)
        .build();

    top_bar.append(&label);
    top_bar
}