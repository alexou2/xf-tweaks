use glib::clone;
// glib and other dependencies are re-exported by the gtk crate
use gio::prelude::*;
use gtk4::gdk::Display;
use gtk4::glib;
use gtk4::prelude::*;
use gtk4::{gdk, gio};
use gtk4::{
    Application, ApplicationWindow, Box, Button, CssProvider, DropDown, Entry, Label, Orientation,
    StyleContext, STYLE_PROVIDER_PRIORITY_APPLICATION,
};
use std::process::Command;
mod json;
use serde_json::{Result, Value};

use gtk4::traits::{ButtonExt, GtkWindowExt, WidgetExt};

// When the application is launched…
fn on_activate(application: &gtk4::Application) {
    // style
    let provider = CssProvider::new();
    provider.load_from_data(include_str!("style.css"));

    gtk4::style_context_add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
    let container = gtk4::Box::builder()
        .orientation(gtk4::Orientation::Vertical)
        .margin_top(24)
        .margin_bottom(24)
        .margin_start(24)
        .margin_end(24)
        .halign(gtk4::Align::Center)
        .valign(gtk4::Align::Center)
        .spacing(24)
        .build();

    // … create a new window …
    let window = gtk4::ApplicationWindow::new(application);
    // … with a button in it …
    let button = gtk4::Button::with_label("Submit");
    let close_window = gtk4::Button::with_label("Exit");
    let text_container = gtk4::Box::builder()
        .halign(gtk4::Align::Center)
        .orientation(gtk4::Orientation::Horizontal)
        .spacing(24)
        .build();
    let css = gtk4::Button::with_label("get css");
    css.add_css_class("css-button");

    let from_entry = gtk4::Entry::builder()
        .placeholder_text("Enter command")
        .build();
    text_container.append(&from_entry);

    button
        .connect_clicked(clone!(@weak window => move |_| run_command(&from_entry.text().as_str())));
    css.connect_clicked(clone!(@weak window => move |_|
        let command = &json::read_json("commands.json")["debug"][1]["command"].to_string();
        run_command(&command.replace('"', ""))
        // println!("{}", command)
    ));

    close_window.connect_clicked(clone!(@weak window => move |_| window.close()));

    container.append(&css);
    container.append(&button);
    container.append(&close_window);
    container.append(&text_container);
    window.set_child(Some(&container));
    window.present();
}

fn main() {
    // Create a new application with the builder pattern
    let app = gtk4::Application::builder()
        // .application_id("com.github.gtk-rs.examples.basic")
        .application_id("xf-tweaks")
        .build();
    app.connect_activate(on_activate);
    // Run the application
    // app.run();
    convert_to_struct();
    // print!("{}", commands::Commands())
}
fn run_command(command_to_run: &str) {
    let split = command_to_run.split(' ');
    let mut args: Vec<&str> = split.collect();
    let command = args[0];
    args.remove(0);

    let output = Command::new(command)
        .args(args)
        .output()
        .expect("Failed to execute command");

    println!("{}", String::from_utf8_lossy(&output.stdout));
}

fn convert_to_struct() {
    let data = &json::read_json("commands.json")["applications"];
    let arr = data.as_array().unwrap();
    let mut buffer: Vec<String> = Vec::new();

    for i in arr {
        let str = format!(
            "let {} = install_commands{{
            name:{},
            command:{},
            description:{},
            needs_sudo:{}
        }};\n",
            i["name"].to_string().replace('"', ""),
            i["name"],
            i["command"],
            i["description"],
            i["needs_sudo"]
        );
        buffer.push(str);
    }
    // for i in buffer {
    //     print!("{}", i)
    // }
    json::write_file(buffer.join("\n"), "result.struct");
}
