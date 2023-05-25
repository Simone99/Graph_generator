mod drawing;
mod edge_object;
mod node_object;
mod window;

use gtk::prelude::*;
use gtk::{gio, glib, Application};
use window::Window;

const APP_ID: &'static str = "com.simomaster1.GraphGenerator";

fn main() -> glib::ExitCode {
    // Register and include resources
    gio::resources_register_include!("graph_generator.gresource")
        .expect("Failed to register resources.");

    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn build_ui(app: &Application) {
    // Create a new custom window and show it
    let window = Window::new(app);
    window.present();
}
