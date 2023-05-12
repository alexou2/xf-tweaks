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
    for obj in apps::return_json() {
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
        {
            utils::type_of(&cmd_button);
        }
        // app_list.append(&cmd_button);

        // action when button is clicked
        // cmd_button.connect_clicked(move |_| add_to_cmd_list(obj.clone()));
    }

    enter.connect_clicked(clone!(@weak button => move |_|if button.is_active(){
        println!("click");
    }
    )); // button action

    submit_button.connect_clicked(clone!(@weak app_list => move |_| {
        let app_list = app_list.clone();

        for i in &app_list.observe_children() {
            if let Some(check_button) = i.expect("ll").downcast_ref::<CheckButton>() {
                let state = check_button.is_active();
                if state{
                // println!("CheckButton state: {}", state);
                // println!("clicked:{}", &check_button.label().unwrap().replace('"', ""));

                let run = json::find_element(&check_button.label().unwrap());
                // println!("{:?}", run)
                run_cmd(run);
                // let command = json::find_element(&check_button.label().unwrap().replace('"', "").to_string());
                // utils::run_command(command.as_str());
                // run_cmd(command);
                }
            }
        }
    }));
    // adds the list of buttons
    content.append(&app_list);
    content.append(&cli_tools);
    content.append(&prog_language);

    content.append(&apply_cmd);
    content.append(&debug);

    // the actual window

    window.set_child(Some(&content)); //uses the buttons/text/ etc... from content

    window.show();
}