use crate::apps;
use crate::json;
use crate::utils;
use gio::prelude::*;
use glib::clone;
use gtk::glib;
use gtk::prelude::*;
use gtk::traits::{ButtonExt, GtkWindowExt, WidgetExt};
use gtk::{
    Application, ApplicationWindow, Box, Button, CheckButton, CssProvider, DropDown, Entry, Label,
    Notebook, Orientation, StyleContext, STYLE_PROVIDER_PRIORITY_APPLICATION,
};
// create the gtk window

pub fn build_ui(app: &Application) {
    // the window
    let window = ApplicationWindow::builder()
        .title("xf-tweaks")
        .application(app)
        .build();

    let notebook = Notebook::new(); //where the tabs are

    let submit_button = Button::builder()
        .label("Finished")
        .tooltip_markup("Apply selection")
        .build();

    // cancel button
    let cancel = Button::builder().label("Cancel").tooltip_markup("Close window").build();
    cancel.connect_clicked(clone!(@weak window =>move|_| window.close()));

    // the list of what is in the app
    let content = Box::new(Orientation::Horizontal, 4);
    let apply_cmd = Box::new(Orientation::Vertical, 2);

    // apply_cmd.append(&enter);
    apply_cmd.append(&submit_button);
    apply_cmd.append(&cancel);

    submit_button.connect_clicked(clone!(@weak notebook => move |_|
        for page_num in 0..notebook.n_pages() {
        if let Some(page) = notebook.nth_page(Some(page_num)) {
            if let Some(box_container) = page.downcast_ref::<Box>() {

                for i in &box_container.observe_children() {
                if let Some(check_button) = i.expect("ll").downcast_ref::<Box>() {

                    for button in &check_button.observe_children(){
                        if let Some(check) = button.expect("ll").downcast_ref::<CheckButton>() {

                            let state = check.is_active();
                            if state{

                                let run = json::find_element(&check.label().unwrap(), "applications");
                                // println!("{:?}", run)
                                utils::split_command(run);
                                println!("true")
                            }
                        }
                    }
                }
            }
        }
    }
})); // button action

    // content of the page
    content.append(&notebook);
    content.append(&apply_cmd);
    // content.append(&debug);

    // the labels of the tabs
    let main_label = Label::builder()
        .label("Applications")
        .tooltip_markup("Install any apps")
        .build();
    let display_label = Label::builder()
        .label("Display options")
        .tooltip_markup("Customize the display options")
        .build();
    let theme_label = Label::builder()
        .label("System theme")
        .tooltip_markup("Change the look and feel of your os\n⚠This will change entirely how the gui will behave")
        .build();
    let debug_label = Label::builder()
        .tooltip_markup("Run commands that wont install")
        .label("Debug options")
        .build();

    // adding the tabs to the window
    notebook.append_page(&create_main_tab(), Some(&main_label));
    notebook.append_page(&create_display_tab(), Some(&display_label));
    notebook.append_page(&system_theme(), Some(&theme_label));
    notebook.append_page(&debug_tab(), Some(&debug_label));

    window.set_child(Some(&content));

    window.show();
}

// returns the tab to install a desktop environment/ display manager
fn create_display_tab() -> Box {
    let label: Label = Label::builder().label("txt").build();
    // tab.append_page(Some(label), Some("lab"));
    let content = Box::new(Orientation::Vertical, 2);
    content.append(&label);

    return content;
}

fn create_main_tab() -> Box {
    // let notebook = Notebook::new();

    let cli_list = Label::builder()
        .label("CLI tools")
        .margin_top(12)
        .margin_bottom(12)
        .margin_end(12)
        .margin_start(12)
        .build();

    let app_label = Label::builder()
        .label("Applications")
        .margin_top(12)
        .margin_bottom(12)
        .margin_end(12)
        .margin_start(12)
        .build();

    let language_list = Label::builder()
        .label("Programming languages")
        .margin_top(12)
        .margin_bottom(12)
        .margin_end(12)
        .margin_start(12)
        .build();

    // cancel buttons
    // the list of what is in the app
    let content = Box::new(Orientation::Horizontal, 4);
    let app_list = Box::new(Orientation::Vertical, 2);
    let cli_tools = Box::new(Orientation::Vertical, 2);
    let prog_language = Box::new(Orientation::Vertical, 2);

    // adds the buttons to the window
    app_list.append(&app_label);
    cli_tools.append(&cli_list);
    prog_language.append(&language_list);

    // loop to create a button for every possible command
    for obj in apps::return_json("applications") {
        let cmd_button = CheckButton::builder()
            .label(obj["name"].to_string().replace('"', "").to_string())
            .tooltip_markup(obj["description"].to_string().replace('"', "").to_string())
            .build();

        match obj["type"].as_str() {
            Some("application") => app_list.append(&cmd_button),
            Some("utilities") => cli_tools.append(&cmd_button),
            Some("programming_language") => prog_language.append(&cmd_button),
            _ => println!("error {}", obj["name"]),
        }
    }

    // adds the list of buttons
    content.append(&app_list);
    content.append(&cli_tools);
    content.append(&prog_language);

    // content.append(&apply_cmd);
    // content.append(&debug);

    // let label = Label::builder().label("display options").build();
    // notebook.append_page(&content, Some(&label));
    return content;
}

fn system_theme() -> Box {
    let content = Box::new(Orientation::Horizontal, 4); // main box for the page
    let dm_list = Box::new(Orientation::Vertical, 2); //list of lock screen
    let de_list = Box::new(Orientation::Vertical, 2); // list of system theme

    let dm_label = Label::builder().label("Lock screen theme").build();
    let de_label = Label::builder().label("System theme").build();

    dm_list.append(&dm_label);
    de_list.append(&de_label);

    for obj in apps::return_json("applications") {
        let cmd_button = CheckButton::builder()
            .label(obj["name"].to_string().replace('"', "").to_string())
            .tooltip_markup(obj["description"].to_string().replace('"', "").to_string())
            .build();

        match obj["type"].as_str() {
            Some("dm") => dm_list.append(&cmd_button),
            Some("de") => de_list.append(&cmd_button),
            _ => println!("error {}", obj["name"]),
        }
    }

    content.append(&dm_list);
    content.append(&de_list);
    return content;
}

fn debug_tab() -> Box {
    let content = Box::new(Orientation::Vertical, 2);
    let c = Box::new(Orientation::Horizontal, 2);

    for obj in apps::return_json("applications") {
        let cmd_button = CheckButton::builder()
            .label(obj["name"].to_string().replace('"', "").to_string())
            .tooltip_markup(obj["description"].to_string().replace('"', "").to_string())
            .build();

        match obj["type"].as_str() {
            Some("debug") => c.append(&cmd_button),
            _ => println!("error {}", obj["name"]),
        }
    }
    content.append(&c);
    return content;
}

// auto-checks the boxes for each profile
fn click_buttons(profile: &str) {}

fn uncheck_box(content: Notebook) {}
