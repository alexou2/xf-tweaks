// stores all of the data that will be used to run the commands from the app

use serde_json::{from_str, Value};
// use crate::{json, utils::type_of};

// the json that stores the list of actions that the app can do
pub const JSON_DATA: &str = r#"
{
    "applications": [
        {
            "name": "firefox",
            "command": "sudo apt install firefox",
            "description": "an open source browser",
            "needs_sudo": true,
            "type":"application"
        },
        {
            "name": "nautilus",
            "command": "sudo apt install nautilus",
            "description": "the default gnome file manager",
            "needs_sudo": true,
            "type":"application"
        },
        {
            "name": "node server",
            "command": [
                "cd ~/Documents/test-mangaJs",
            "node index.js -lan"],
            "description": "for debug purposes",
            "needs_sudo": false,
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
            "name": "kde connect",
            "command": "sudo apt install kdeconnect",
            "description": "control your pc from your phone",
            "needs_sudo": true,
            "type":"application"
        },
        {
            "name": "byobu terminal",
            "command": "sudo apt install byobu",
            "description": "a light, powerfull terminal emulator",
            "needs_sudo": true,
            "type":"application"
        },
        {
            "name": "thunderbird mail",
            "command": "sudo apt install thunderbird",
            "description": "an open source mail client",
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
            "description": "play windows games on linux 🦦",
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
            "command": [
                "sudo apt install nodejs npm",
                "curl -O https://raw.githubusercontent.com/nvm-sh/nvm/master/nvm.sh"
            ],
            "description": "javascript for the server (installs nodejs, npm, nvm)",
            "needs_sudo": true,
            "type":"programming_language"
        },
        {
            "name": "python",
            "command": "sudo apt install python3 pip",
            "description": "the python programming language 🐍(installs python, pip)",
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
        }
    ],
    "desktop_environment":[
        {
            "name": "gnome full",
            "command": "sudo apt install gnome-shell",
            "description": "the fully featured version of the gnome DE",
            "needs_sudo": true,
            "type":"de"
        },
        {
            "name": "gnome minimal",
            "command": "sudo apt install gnome-session",
            "description": "a lightweght version of gnome (with ubuntu tewaks)",
            "needs_sudo": true,
            "type":"de"
        },
        {
            "name": "kde minimal",
            "command": "sudo apt install kde-plasma-desktop",
            "description": "a highly customizable desktop environment",
            "needs_sudo": true,
            "type":"de"
        },
        {
            "name": "xfce",
            "command": "sudo apt install xfce4",
            "description": "a lightweight DE",
            "needs_sudo": true,
            "type":"de"
        }
    ],
    "display_manager":[
        {
            "name": "gdm3",
            "command": "sudo apt install lightdm",
            "description": "the default login screen of gnome",
            "needs_sudo": true,
            "type":"de"
        },
        {
            "name": "lightdm",
            "command": "sudo apt install lightdm",
            "description": "the default login screen of xfce",
            "needs_sudo": true,
            "type":"de"
        },
        {
            "name": "sddm",
            "command": "sudo apt install sddm",
            "description": "the default login screen of kde",
            "needs_sudo": true,
            "type":"de"
        }
    ]
}
"#;

// returns the json obj as a vec of Value (array of json objects)
pub fn return_json(element: &str) -> Vec<Value> {
    let json_data: Value = from_str(JSON_DATA).expect("error parsing json");
    let mut json_list: Vec<Value> = Vec::new();

    for item in json_data[element].as_array().unwrap() {
        json_list.push(item.clone());
    }
    return json_list;
}
pub const PROFILES: &str = r#"
{
    "server": [
        "htop",
        "openssh server",
        "virt-manager"
    ],
    "gaming": [
        "lutris"
    ],
    "programming": [
        "rust",
        "python",
        "vs code",
        "kate",
        "nodejs"
    ],
    "laptop": [
        "tlp"
    ],
    "useful_tools":[
        "nautilus",
        "vlc",
        "tree",
        "neofetch",
        "lutris"
    ],
    "recommended":[
        "nautilus",
        "vlc",
        "tree",
        "neofetch",
        "lutris",
        "thunderbird", 
        "kde connect",
        "virt-manager"
    ]
}
"#;
