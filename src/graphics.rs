use gtk::{gdk, prelude::*, Align, Label};
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
    // main_box.append(&center_box());
    // main_box.append(&horizontal_labels());
    main_box.append(&matrix_grid());
    main_box
}

fn matrix_grid() -> gtk::Grid {
    let mut grid = gtk::Grid::builder()
        .row_homogeneous(true)
        .column_homogeneous(true)
        .build();

    label_adder(&mut grid, gtk::PositionType::Bottom);
    label_adder(&mut grid, gtk::PositionType::Left);
    grid
}

fn label_adder(grid: &mut gtk::Grid, position: gtk::PositionType) {
    let labels = ["High", "MidHigh", "Mid", "MidLow", "Low"].map(|str| gtk::Label::new(Some(str)));
    match position {
        gtk::PositionType::Left => {
            for (i, label) in labels.iter().enumerate() {
                grid.attach(label, 0, i as i32, 1, 1);
            }
        }
        gtk::PositionType::Bottom => {
            for (i, label) in labels.iter().rev().enumerate() {
                grid.attach(label, i as i32 + 1, 6, 1, 1);
            }
        }
        _ => ()
    }

}

fn top_bar() -> gtk::Box {
    let label = gtk::Label::builder().label("top bar").build();
    let top_bar = gtk::Box::builder()
        .orientation(Orientation::Horizontal)
        .build();

    top_bar.append(&label);
    top_bar
}