use crate::apps;
use crate::json;
use crate::utils;
use gio::prelude::*;
use glib::clone;
use gtk::builders::NotebookBuilder;
use gtk::gdk_pixbuf::Pixbuf;
use gtk::glib;
use gtk::prelude::*;
use gtk::traits::{ButtonExt, GtkWindowExt, WidgetExt};
use gtk::NotebookTab;
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

    // some text
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

    let debug_menu = Label::builder()
        .label("Debug options")
        .margin_top(12)
        .margin_bottom(12)
        .margin_end(12)
        .margin_start(12)
        .build();

    let language_list = Label::builder()
        .label("Programming language")
        .margin_top(12)
        .margin_bottom(12)
        .margin_end(12)
        .margin_start(12)
        .build();

    // creates a useless button
    // let button = Button::builder()
    let button = CheckButton::builder()
        .label("click me ")
        .margin_top(12)
        .margin_bottom(12)
        .margin_end(12)
        .margin_start(12)
        .focus_on_click(true)
        .build();
    // button.connect_clicked(move |_| println!("opopop")); // button action

    // creates a submit button
    let enter = Button::builder().label("debug").build();

    let submit_button = Button::builder().label("finished").build();

    // cancel buttons
    let cancel = Button::builder().label("Cancel").build();
    cancel.connect_clicked(clone!(@weak window =>move|_| window.close()));

    // the list of what is in the app
    let content = Box::new(Orientation::Horizontal, 4);
    let app_list = Box::new(Orientation::Vertical, 2);
    let cli_tools = Box::new(Orientation::Vertical, 2);
    let debug = Box::new(Orientation::Vertical, 2);
    let apply_cmd = Box::new(Orientation::Vertical, 2);
    let prog_language = Box::new(Orientation::Vertical, 2);

    // adds the buttons to the window
    debug.append(&debug_menu);
    app_list.append(&app_label);
    debug.append(&button);
    debug.append(&enter);
    cli_tools.append(&cli_list);
    apply_cmd.append(&submit_button);
    apply_cmd.append(&cancel);
    prog_language.append(&language_list);

    // loop to create a button for every possible command
    for obj in apps::return_json("applications") {
        let cmd_button = CheckButton::builder()
            .label(format!("{}", obj["name"].to_string().replace('"', "")))
            .tooltip_markup(format!(
                "{}",
                obj["description"].to_string().replace('"', "")
            ))
            .build();

        match obj["type"].as_str() {
            Some("application") => app_list.append(&cmd_button),
            Some("utilities") => cli_tools.append(&cmd_button),
            Some("programming_language") => prog_language.append(&cmd_button),
            _ => println!("error {}", obj["name"]),
        }
    }

    enter.connect_clicked(clone!(@weak button => move |_|if button.is_active(){
        println!("click");
    }
    )); // button action

    submit_button.connect_clicked(clone!(@weak app_list, @weak content => move |_| {
        let content = content.clone();

        for tab in &content.observe_children() {
            if let Some(boxy) = tab.expect("ll").downcast_ref::<Box>() {

        let app_list = boxy.clone();

        for i in &app_list.observe_children() {
            if let Some(check_button) = i.expect("ll").downcast_ref::<CheckButton>() {
                let state = check_button.is_active();
                if state{
                // println!("CheckButton state: {}", state);
                // println!("clicked:{}", &check_button.label().unwrap().replace('"', ""));

                let run = json::find_element(&check_button.label().unwrap(), "applications");
                // println!("{:?}", run)
                utils::split_command(run);
                // let command = json::find_element(&check_button.label().unwrap().replace('"', "").to_string());
                // utils::run_command(command.as_str());
                // run_cmd(command);
                }
            }}}
        }
    }));

    // adds the list of buttons
    content.append(&app_list);
    content.append(&cli_tools);
    content.append(&prog_language);

    content.append(&apply_cmd);
    content.append(&debug);

    // content.append(&create_display_tab());

    window.set_child(Some(&content)); //uses the buttons/text/ etc... from content

    window.show();
}


// returns the tab to install a desktop environment/ display manager
fn create_display_tab() -> Notebook {
    let tab = Notebook::builder().build();
    let label: Label = Label::builder().label("txt").build();
    let label2 = Label::builder().label("txt").build();
    // tab.append_page(Some(label), Some("lab"));
    let content = Box::new(Orientation::Vertical, 2);
    content.append(&label);
    
    tab.append_page(&content, Some(&label2));
    tab.append_page(&content, Some(&label2));

    return tab;
}

// auto-checks the boxes for each profile
fn click_buttons(profile: &str) {}