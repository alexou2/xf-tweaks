pub fn cmd() {
    struct install_commands {
        name: str,
        command: str,
        description: str,
        requires_sudo: bool,
        app_type: str,
    }

// lists of types of applications
    let applications: Vec<install_commands>::new();
    let programming_languages: Vec<install_commands>::new();
    let utilities: Vec<install_commands>::new();
    let system_tweaks: Vec<install_commands>::new();
    let keyboard_shortcut: Vec<install_commands>::new();


// 
    let nautilus = install_commands {
        name: "nautilus",
        command: "sudo apt install nautilus",
        description: "the default gnome file manager",
        needs_sudo: true,
        app_type: "application",
    };
    application.add(nautilus);

    let vscode = install_commands {
        name: "vs code",
        command: "sudo apt install code",
        description: "lightweight code IDE",
        needs_sudo: true,
        app_type: "application",
    };
    application.add(vscode);

    let kate = install_commands {
        name: "kate",
        command: "sudo apt install kate",
        description: "lightweight text editor",
        needs_sudo: true,
        app_type: "application",
    };
    application.add(kate);

    let vlc = install_commands {
        name: "vlc",
        command: "sudo apt install vlc",
        description: "an awesome media player",
        needs_sudo: true,
        app_type: "application",
    };
    application.add(vlc);

    let chromium = install_commands {
        name: "chromium",
        command: "sudo apt install chromium-browser",
        description: "the open-source base of chrome and ms edge",
        needs_sudo: true,
        app_type: "application",
    };
    application.add(chromium);

    let virt_manager = install_commands {
        name: "virt-manager",
        command: "sudo apt install virt-manager",
        description: "a virtual machine manager",
        needs_sudo: true,
        app_type: "application",
    };
    application.add(virt_manager);

    let lutris = install_commands {
        name: "lutris",
        command: "sudo apt install lutris",
        description: "play windows games on linux",
        needs_sudo: true,
        app_type: "application",
    };
    application.add(lutris);

    let gnome_disks = install_commands {
        name: "gnome-disks",
        command: "sudo apt install gnome-disks",
        description: "the default disk manager of gnome",
        needs_sudo: true,
        app_type: "application",
    };
    application.add(gnome_disks);

    let rust = install_commands {
        name: "rust",
        command: "sudo apt install cargo",
        description: "the rust programming language",
        needs_sudo: true,
        app_type: "programming_language",
    };
    programming_language.add(rust);

    let nodejs = install_commands {
        name: "nodejs",
        command: "sudo apt install nodejs npm",
        description: "javascript for the server",
        needs_sudo: true,
        app_type: "programming_language",
    };
    programming_language.add(nodejs);

    let tree = install_commands {
        name: "tree",
        command: "sudo apt install tree",
        description: "lists all files as a tree",
        needs_sudo: true,
        app_type: "utilities",
    };
    utilities.add(tree);

    let tlp = install_commands {
        name: "tlp",
        command: "sudo apt install tlp",
        description: "saves power on laptops by turning off features that aren't used",
        needs_sudo: true,
        app_type: "utilities",
    };
    utilities.add(tlp);

    let neofetch = install_commands {
        name: "neofetch",
        command: "sudo apt install neofetch",
        description: "shows system specs in a cool way",
        needs_sudo: true,
        app_type: "utilities",
    };
    utilities.add(neofetch);

    let htop = install_commands {
        name: "htop",
        command: "sudo apt install htop",
        description: "a lightweight task manager",
        needs_sudo: true,
        app_type: "utilities",
    };
    utilities.add(htop);

    let opensshserver = install_commands {
        name: "openssh server",
        command: [
            "sudo apt install openssh-server",
            "sudo systemctl enable sshd",
        ],
        description: "enables remote logging into the pc",
        needs_sudo: true,
        app_type: "tweak",
    };
    tweak.add(opensshserver);

    let ls_a = install_commands {
        name: "ls -a",
        command: "ls",
        description: "lists files",
        needs_sudo: false,
        app_type: "debug",
    };
    debug.add(ls_a);

    let pwd = install_commands {
        name: "pwd",
        command: "pwd",
        description: "pwd",
        needs_sudo: false,
        app_type: "debug",
    };
    debug.add(pwd);
}
