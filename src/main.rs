use apps::install_commands;
use glib::clone;
use gtk::gdk::Display;
use gtk::glib;
use gtk::prelude::*;
use gtk::traits::{ButtonExt, GtkWindowExt, WidgetExt};
use gtk::{
    Application, ApplicationWindow, Box, Button, CssProvider, DropDown, Entry, Label, Orientation,
    StyleContext, STYLE_PROVIDER_PRIORITY_APPLICATION,
};
use json::print_json;
use lazy_static::lazy_static;
use std::sync::Mutex;

mod apps;
mod json;
mod utils;
use serde_json::Value;

// the list of commands that will be executed
// const
lazy_static! {
    static ref CMD_LIST: Vec<Value> = apps::return_json();
    static ref command_to_run: Vec<Value> = Vec::new();
    static ref MY_VECTOR: Mutex<Vec<Value>> = Mutex::new(Vec::new());
}

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

fn main() {
    // Create a new application with the builder pattern
    let app = gtk::Application::builder()
        // .application_id("com.github.gtk-rs.examples.basic")
        .application_id("xf-tweaks")
        .build();
    // app.connect_activate(on_activate);
    // Run the application
    // app.run();

    // utils::convert_to_struct()

    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &Application) {
    let label = Label::builder()
        .label("click me lololololololol")
        .margin_top(12)
        .margin_bottom(12)
        .margin_end(12)
        .margin_start(12)
        .build();

    let button = Button::builder()
        .label("click me ")
        .margin_top(12)
        .margin_bottom(12)
        .margin_end(12)
        .margin_start(12)
        .focus_on_click(true)
        .build();
    button.connect_clicked(move |_| println!("opopop"));

    let enter = Button::builder().label("submit").build();
    enter.connect_clicked(move |_| run_cmd());

    let content = Box::new(Orientation::Vertical, 0);
    content.append(&button);
    content.append(&enter);
    content.append(&label);

    for obj in apps::return_json() {
        let cmd_button = Button::builder()
            .label(format!("{}", obj["name"]))
            .opacity(1.0)
            .build();
        content.append(&cmd_button);
        cmd_button.connect_clicked(move |_| add_to_cmd_list(obj.clone()));
    }

    let window = ApplicationWindow::builder()
        .title("xf-tweaks")
        .application(app)
        .child(&content)
        .build();

    window.show();
}

// mod apps;

pub fn add_to_cmd_list(command: Value) {
    let mut vect = MY_VECTOR.lock().unwrap();
    vect.push(command);
    println!("added")
}


pub fn run_cmd() {
// print!("{}", MY_VECTOR.lock().unwrap()[1]);
    let commands = MY_VECTOR.lock().unwrap().to_vec();
println!("{}", commands[0]);

    for cmd in commands {
        let value = cmd["command"].to_string();
        utils::run_command(value.replace('"', "").as_str());
    }
}
