use glib::clone;
// glib and other dependencies are re-exported by the gtk crate
use gtk::gdk::Display;
use gtk::glib;
use gtk::prelude::*;
use gtk::{
    Application, ApplicationWindow, Box, Button, CssProvider, DropDown, Entry, Label, Orientation,
    StyleContext, STYLE_PROVIDER_PRIORITY_APPLICATION,
};
mod json;
mod utils;

use gtk::traits::{ButtonExt, GtkWindowExt, WidgetExt};

// When the application is launched…
fn on_activate(application: &gtk::Application) {
    // style
    let provider = CssProvider::new();
    provider.load_from_data(include_str!("style.css"));

    gtk::style_context_add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
    let container = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .margin_top(24)
        .margin_bottom(24)
        .margin_start(24)
        .margin_end(24)
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Center)
        .spacing(24)
        .build();

    // … create a new window …
    let window = gtk::ApplicationWindow::new(application);
    // … with a button in it …
    let button = gtk::Button::with_label("Submit");
    let close_window = gtk::Button::with_label("Exit");
    let text_container = gtk::Box::builder()
        .halign(gtk::Align::Center)
        .orientation(gtk::Orientation::Horizontal)
        .spacing(24)
        .build();
    let css = gtk::Button::with_label("get css");
    css.add_css_class("css-button");

    let from_entry = gtk::Entry::builder()
        .placeholder_text("Enter command")
        .build();
    text_container.append(&from_entry);

    button.connect_clicked(
        clone!(@weak window => move |_| utils::run_command(&from_entry.text().as_str())),
    );
    css.connect_clicked(clone!(@weak window => move |_|
        let command = &json::read_json("commands.json")["debug"][1]["command"].to_string();
        utils::run_command(&command.replace('"', ""))
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
    let app = gtk::Application::builder()
        // .application_id("com.github.gtk-rs.examples.basic")
        .application_id("xf-tweaks")
        .build();
    app.connect_activate(on_activate);
    // Run the application
    app.run();
    // utils::t();
    //  utils::convert_to_struct();
}
