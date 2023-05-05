// use crate::apps::my::install_commands;
use serde_json::{from_str, Value};

use crate::{json, utils::type_of};

pub struct install_commands<'a> {
    pub name: &'a str,
    // command: &'a str,
    pub command: Vec<&'a str>,
    pub description: &'a str,
    pub needs_sudo: bool,
    pub app_type: &'a str,
}

// }
fn cmd() {
    // lists of types of applications

    let mut application: Vec<&install_commands> = Vec::new();
    let mut programming_language: Vec<&install_commands> = Vec::new();
    let mut utilities: Vec<&install_commands> = Vec::new();
    let mut tweak: Vec<&install_commands> = Vec::new();
    let mut keyboard_shortcut: Vec<&install_commands> = Vec::new();
    let mut debug: Vec<&install_commands> = Vec::new();

    let nautilus = install_commands {
        name: "nautilus",
        command: vec!["sudo apt install nautilus"],
        description: "the default gnome file manager",
        needs_sudo: true,
        app_type: "application",
    };
    application.push(&nautilus);

    let vscode = install_commands {
        name: "vs code",
        command: vec!["sudo apt install code"],
        description: "lightweight code IDE",
        needs_sudo: true,
        app_type: "application",
    };
    application.push(&vscode);

    let kate = install_commands {
        name: "kate",
        command: vec!["sudo apt install kate"],
        description: "lightweight text editor",
        needs_sudo: true,
        app_type: "application",
    };
    application.push(&kate);

    let vlc = install_commands {
        name: "vlc",
        command: vec!["sudo apt install vlc"],
        description: "an awesome media player",
        needs_sudo: true,
        app_type: "application",
    };
    application.push(&vlc);

    let chromium = install_commands {
        name: "chromium",
        command: vec!["sudo apt install chromium-browser"],
        description: "the open-source base of chrome and ms edge",
        needs_sudo: true,
        app_type: "application",
    };
    application.push(&chromium);

    let virt_manager = install_commands {
        name: "virt-manager",
        command: vec!["sudo apt install virt-manager"],
        description: "a virtual machine manager",
        needs_sudo: true,
        app_type: "application",
    };
    application.push(&virt_manager);

    let lutris = install_commands {
        name: "lutris",
        command: vec!["sudo apt install lutris"],
        description: "play windows games on linux",
        needs_sudo: true,
        app_type: "application",
    };
    application.push(&lutris);

    let rust = install_commands {
        name: "rust",
        command: vec!["sudo apt install cargo"],
        description: "the rust programming language",
        needs_sudo: true,
        app_type: "programming_language",
    };
    programming_language.push(&rust);

    let nodejs = install_commands {
        name: "nodejs",
        command: vec!["sudo apt install nodejs npm"],
        description: "javascript for the server",
        needs_sudo: true,
        app_type: "programming_language",
    };
    programming_language.push(&nodejs);

    let tree = install_commands {
        name: "tree",
        command: vec!["sudo apt install tree"],
        description: "lists all files as a tree",
        needs_sudo: true,
        app_type: "utilities",
    };
    utilities.push(&tree);

    let tlp = install_commands {
        name: "tlp",
        command: vec!["sudo apt install tlp"],
        description: "saves power on laptops by turning off features that aren't used",
        needs_sudo: true,
        app_type: "utilities",
    };
    utilities.push(&tlp);

    let neofetch = install_commands {
        name: "neofetch",
        command: vec!["sudo apt install neofetch"],
        description: "shows system specs in a cool way",
        needs_sudo: true,
        app_type: "utilities",
    };
    utilities.push(&neofetch);

    let htop = install_commands {
        name: "htop",
        command: vec!["sudo apt install htop"],
        description: "a lightweight task manager",
        needs_sudo: true,
        app_type: "utilities",
    };
    utilities.push(&htop);

    let opensshserver = install_commands {
        name: "openssh server",
        command: vec![
            "sudo apt install openssh-server",
            "sudo systemctl enable sshd",
        ],
        description: "enables remote logging into the pc",
        needs_sudo: true,
        app_type: "tweak",
    };
    tweak.push(&opensshserver);

    let ls_a = install_commands {
        name: "ls -a",
        command: vec!["ls"],
        description: "lists files",
        needs_sudo: false,
        app_type: "debug",
    };
    debug.push(&ls_a);

    let pwd = install_commands {
        name: "pwd",
        command: vec!["pwd"],
        description: "pwd",
        needs_sudo: false,
        app_type: "debug",
    };
    debug.push(&pwd);
}

pub const JSON_DATA: &str = r#"
{
    "applications": [
        {
            "name": "nautilus",
            "command": "sudo apt install nautilus",
            "description": "the default gnome file manager",
            "needs_sudo": true,
            "type":"application"
        },
        {
            "name": "vs code",
            "command": "sudo apt install code",
            "description": "lightweight code IDE",
            "needs_sudo": true,
            "type":"application"
        },
        {
            "name": "kate",
            "command": "sudo apt install kate",
            "description": "lightweight text editor",
            "needs_sudo": true,
            "type":"application"
        },
        {
            "name": "vlc",
            "command": "sudo apt install vlc",
            "description": "an awesome media player",
            "needs_sudo": true,
            "type":"application"
        },
        {
            "name": "chromium",
            "command": "sudo apt install chromium-browser",
            "description": "the open-source base of chrome and ms edge",
            "needs_sudo": true,
            "type":"application"
        },
        {
            "name": "virt-manager",
            "command": "sudo apt install virt-manager",
            "description": "a virtual machine manager",
            "needs_sudo": true,
            "type":"application"
        },
        {
            "name": "lutris",
            "command": "sudo apt install lutris",
            "description": "play windows games on linux",
            "needs_sudo": true,
            "type":"application"
        },
        {
            "name": "rust",
            "command": "sudo apt install cargo",
            "description": "the rust programming language",
            "needs_sudo": true,
            "type":"programming_language"
        },
        {
            "name": "nodejs",
            "command": "sudo apt install nodejs npm",
            "description": "javascript for the server",
            "needs_sudo": true,
            "type":"programming_language"
        },
        {
            "name": "tree",
            "command": "sudo apt install tree",
            "description": "lists all files as a tree",
            "needs_sudo": true,
            "type":"utilities"
        },
        {
            "name": "tlp",
            "command": "sudo apt install tlp",
            "description": "saves power on laptops by turning off features that aren't used",
            "needs_sudo": true,
            "type":"utilities"
        },
        {
            "name": "neofetch",
            "command": "sudo apt install neofetch",
            "description": "shows system specs in a cool way",
            "needs_sudo": true,
            "type":"utilities"
        },
        {
            "name": "htop",
            "command": "sudo apt install htop",
            "description": "a lightweight task manager",
            "needs_sudo": true,
            "type":"utilities"
        },

        {
            "name": "openssh server",
            "command": [
                "sudo apt install openssh-server",
                "sudo systemctl enable sshd"
            ],
            "description": "enables remote logging into the pc",
            "needs_sudo": true,
            "type":"tweak"
        },
        {
            "name": "ls -a",
            "command": "ls",
            "description": "lists files",
            "needs_sudo": false,
            "type":"debug"
        },
        {
            "name": "pwd",
            "command": "pwd",
            "description": "pwd",
            "needs_sudo": false,
            "type":"debug"
        }
    ]
}
"#;

pub fn return_json() -> Vec<Value> {
    let json_data: Value = from_str(JSON_DATA).expect("error parsing json");
    let mut json_list: Vec<Value> = Vec::new();

    for item in json_data["applications"].as_array().unwrap() {
        json_list.push(item.clone());
    }
    return json_list;
}
