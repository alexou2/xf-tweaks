use crate::apps;
use crate::json;
use crate::utils;
use crate::utils::type_of;
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
use serde_json::Value;
// create the gtk window

pub fn build_ui(app: &Application) {
    // the window
    let window = ApplicationWindow::builder()
        .title("xf-tweaks")
        .application(app)
        .build();

    // some text
    // let cli_list = Label::builder()
    //     .label("CLI tools")
    //     .margin_top(12)
    //     .margin_bottom(12)
    //     .margin_end(12)
    //     .margin_start(12)
    //     .build();

    // let app_label = Label::builder()
    //     .label("Applications")
    //     .margin_top(12)
    //     .margin_bottom(12)
    //     .margin_end(12)
    //     .margin_start(12)
    //     .build();

    // let debug_menu = Label::builder()
    //     .label("Debug options")
    //     .margin_top(12)
    //     .margin_bottom(12)
    //     .margin_end(12)
    //     .margin_start(12)
    //     .build();

    // let language_list = Label::builder()
    //     .label("Programming language")
    //     .margin_top(12)
    //     .margin_bottom(12)
    //     .margin_end(12)
    //     .margin_start(12)
    //     .build();

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
    // let cli_tools = Box::new(Orientation::Vertical, 2);
    // let debug = Box::new(Orientation::Vertical, 2);
    let apply_cmd = Box::new(Orientation::Vertical, 2);
    // let prog_language = Box::new(Orientation::Vertical, 2);

    // adds the buttons to the window
    // debug.append(&debug_menu);
    // app_list.append(&app_label);
    // debug.append(&button);
    apply_cmd.append(&enter);
    // cli_tools.append(&cli_list);
    apply_cmd.append(&submit_button);
    apply_cmd.append(&cancel);
    // prog_language.append(&language_list);

    // loop to create a button for every possible command
    // for obj in apps::return_json("applications") {
    //     let cmd_button = CheckButton::builder()
    //         .label(format!("{}", obj["name"].to_string().replace('"', "")))
    //         .tooltip_markup(format!(
    //             "{}",
    //             obj["description"].to_string().replace('"', "")
    //         ))
    //         .build();

    //     match obj["type"].as_str() {
    //         Some("application") => app_list.append(&cmd_button),
    //         Some("utilities") => cli_tools.append(&cmd_button),
    //         Some("programming_language") => prog_language.append(&cmd_button),
    //         _ => println!("error {}", obj["name"]),
    //     }
    // }

    enter.connect_clicked(clone!(@weak button => move |_|if button.is_active(){
        println!("click");
    }
    )); // button action

    submit_button.connect_clicked(clone!(@weak app_list, @weak content => move |_| {
            let content = content.clone();

            for tab in &content.observe_children() {
                if let Some(boxy) = tab.expect("ll").downcast_ref::<Box>() {

            let app_list = boxy.clone();
    type_of(&app_list);
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
    let notebook = Notebook::new();
    // adds the list of buttons
    // content.append(&app_list);
    // content.append(&cli_tools);
    // content.append(&prog_language);
    content.append(&notebook);
    content.append(&apply_cmd);
    // content.append(&debug);

    let main_label = Label::builder().label("Applications").tooltip_markup("Install any apps").build();
    let display_label = Label::builder().label("Display options").tooltip_markup("Customize the display options").build();
    let theme_label = Label::builder().label("System theme").tooltip_markup("Change the look and feel of your os\n⚠This will change entirely how the gui will behave").build();

    notebook.append_page(&create_main_tab(), Some(&main_label));
    notebook.append_page(&create_display_tab(), Some(&display_label));
    notebook.append_page(&system_theme(), Some(&theme_label));

    enter.connect_clicked(clone!(@weak notebook => move |_|
        for page_num in 0..notebook.n_pages() {
        if let Some(page) = notebook.nth_page(Some(page_num)) {
            if let Some(box_container) = page.downcast_ref::<Box>() {

                for i in &box_container.observe_children() {
                if let Some(check_button) = i.expect("ll").downcast_ref::<Box>() {

                    for button in &check_button.observe_children(){
                        if let Some(check) = button.expect("ll").downcast_ref::<CheckButton>() {

                            let state = check.is_active();
                            if state{

                            println!("true")
                            }else {
                                println!("false")
                            }
                        }
                    }
                }
                // type_of(&box_container);
            }

                // }
            }
        }
    }
    )); // button action

    // window.set_child(Some(&notebook));
    window.set_child(Some(&content)); //uses the buttons/text/ etc... from content

    window.show();
}

// returns the tab to install a desktop environment/ display manager
fn create_display_tab() -> Box {
    let tab = Notebook::builder().build();
    let label: Label = Label::builder().label("txt").build();
    let label2 = Label::builder().label("txt").build();
    // tab.append_page(Some(label), Some("lab"));
    let content = Box::new(Orientation::Vertical, 2);
    content.append(&label);

    tab.append_page(&content, Some(&label2));

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

    // let debug_menu = Label::builder()
    //     .label("Debug options")
    //     .margin_top(12)
    //     .margin_bottom(12)
    //     .margin_end(12)
    //     .margin_start(12)
    //     .build();

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
    // the list of what is in the app
    let content = Box::new(Orientation::Horizontal, 4);
    let app_list = Box::new(Orientation::Vertical, 2);
    let cli_tools = Box::new(Orientation::Vertical, 2);
    let debug = Box::new(Orientation::Vertical, 2);
    // let apply_cmd = Box::new(Orientation::Vertical, 2);
    let prog_language = Box::new(Orientation::Vertical, 2);

    // adds the buttons to the window
    // debug.append(&debug_menu);
    app_list.append(&app_label);
    debug.append(&button);
    debug.append(&enter);
    cli_tools.append(&cli_list);
    // apply_cmd.append(&submit_button);
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

    for obj in apps::return_json("theme") {
        let cmd_button = CheckButton::builder()
            .label(format!("{}", obj["name"].to_string().replace('"', "")))
            .tooltip_markup(format!(
                "{}",
                obj["description"].to_string().replace('"', "")
            ))
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

// auto-checks the boxes for each profile
fn click_buttons(profile: &str) {}

// adds the requested items to the gtk::box
fn add_to_box(type_requested: &str, menu: Box, apps_list: Vec<Value>) -> Box {
    for obj in apps_list {
        let cmd_button = CheckButton::builder()
            .label(format!("{}", obj["name"].to_string().replace('"', "")))
            .tooltip_markup(format!(
                "{}",
                obj["description"].to_string().replace('"', "")
            ))
            .build();

        match obj["type"].as_str() {
            // Some(&type_requested) => menu.append(obj)
            Some(_type_requested) => menu.append(&cmd_button),
            _ => println!("error {}", obj["name"]),
        }
    // println!("{}", type_requested);
    }
println!("{:?}", menu);
    return menu;
}
