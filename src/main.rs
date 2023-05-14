use gtk::prelude::*;

mod apps;
mod cli_app;
mod gui_app;
mod json;
mod utils;
// the list of commands that will be executed
// lazy_static! {
//     static ref CMD_LIST: Vec<Value> = apps::return_json(); // list all of the commands
//     static ref command_to_run: Vec<Value> = Vec::new();
//     static ref MY_VECTOR: Mutex<Vec<Value>> = Mutex::new(Vec::new()); // the list of commands that were selected
// }

// fn on_activate(application: &gtk::Application) {
//     let provider = CssProvider::new();
//     provider.load_from_data(include_str!("style.css"));

//     gtk::style_context_add_provider_for_display(
//         &Display::default().expect("Could not connect to a display."),
//         &provider,
//         STYLE_PROVIDER_PRIORITY_APPLICATION,
//     );
//     let container = gtk::Box::builder()
//         .orientation(gtk::Orientation::Vertical)
//         .margin_top(24)
//         .margin_bottom(24)
//         .margin_start(24)
//         .margin_end(24)
//         .halign(gtk::Align::Center)
//         .valign(gtk::Align::Center)
//         .spacing(24)
//         .build();

//     let window = gtk::ApplicationWindow::new(application);
//     let button = gtk::Button::with_label("Submit");
//     let close_window = gtk::Button::with_label("Exit");
//     let text_container = gtk::Box::builder()
//         .halign(gtk::Align::Center)
//         .orientation(gtk::Orientation::Horizontal)
//         .spacing(24)
//         .build();
//     let css = gtk::Button::with_label("get css");
//     css.add_css_class("css-button");

//     let from_entry = gtk::Entry::builder()
//         .placeholder_text("Enter command")
//         .build();
//     text_container.append(&from_entry);

//     button.connect_clicked(
//         clone!(@weak window => move |_| utils::run_command(&from_entry.text().as_str())),
//     );
//     css.connect_clicked(clone!(@weak window => move |_|
//         let command = &json::read_json("commands.json")["debug"][1]["command"].to_string();
//         utils::run_command(&command.replace('"', ""))
//     ));

//     close_window.connect_clicked(clone!(@weak window => move |_| window.close()));

//     container.append(&css);
//     container.append(&button);
//     container.append(&close_window);
//     container.append(&text_container);
//     window.set_child(Some(&container));
//     window.present();
// }
// use winres::WindowsResource;

fn main() {
    // WindowsResource::new()
    // .set_icon("/home/alexou/xf-tweaks/icon2.png")
    // .compile()
    // .unwrap();
    

    // creates the gui version of the app
    // Create a new application with the builder pattern
    let app = gtk::Application::builder()
        // .application_id("com.github.gtk-rs.examples.basic")
        .application_id("xf-tweaks")
        .build();
    // app.connect_activate(on_activate);
    // Run the application
    // app.run();

    app.connect_activate(gui_app::build_ui);
    app.run(); // runs the window

    // runs the cli version of the app

    cli_app::cli_home();
}
