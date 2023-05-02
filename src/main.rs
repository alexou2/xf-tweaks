use glib::clone;
// glib and other dependencies are re-exported by the gtk crate
use gtk4::glib;
use gtk4::prelude::*;
use gio::prelude::*;
use gtk4::gdk::Display;
use gtk4::{gdk, gio};
use gtk4::{
    Application, ApplicationWindow, Box as Box, Button, CssProvider, DropDown, Entry, Orientation,
    STYLE_PROVIDER_PRIORITY_APPLICATION,
};
use std::process::Command;
mod json;

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

        let from_entry = gtk4::Entry::builder()
            .placeholder_text("Type text to copy")
            .build();
        text_container.append(&from_entry);

    // … which closes the window when clicked
    // button.connect_clicked(clone!(@weak window => move |_| window.close()));
    // button.connect_clicked(clone!(@weak window => move |_| println!("{}", from_entry.text().as_str())));
    button.connect_clicked(clone!(@weak window => move |_| run_command(from_entry.text().as_str())));

    close_window.connect_clicked(clone!(@weak window => move |_| window.close()));
   

    container.append(&button);
    container.append(&close_window);
    container.append(&text_container);
    window.set_child(Some(&container));
    window.present();
}

fn main() {
    // Create a new application with the builder pattern
    let app = gtk4::Application::builder()
        .application_id("com.github.gtk-rs.examples.basic")
        .build();
    app.connect_activate(on_activate);
    // Run the application
    // app.run();
    run_command("ls")
}
fn run_command(command_to_run: &str){
    let output = Command::new(command_to_run)
        // .arg("-a")
        .output()
        .expect("Failed to execute command");
    println!("{}", String::from_utf8_lossy(&output.stdout));
}