pub fn cmd() {

    struct install_commands {
        name:str,
        command:str,
        description:str,
        requires_sudo:bool
    }

    
    let nautilus = install_commands{
        name:"nautilus",
        command:"sudo apt install nautilus",
        description:null,
        needs_sudo:true
    };

let vs_code = install_commands{
        name:"vs code",
        command:"sudo apt install code",
        description:null,
        needs_sudo:true
    };

let kate = install_commands{
        name:"kate",
        command:"sudo apt install kate",
        description:null,
        needs_sudo:true
    };

let vlc = install_commands{
        name:"vlc",
        command:"sudo apt install vlc",
        description:null,
        needs_sudo:true
    };

let chromium = install_commands{
        name:"chromium",
        command:"sudo apt install chromium-browser",
        description:null,
        needs_sudo:true
    };

let virt_manager = install_commands{
        name:"virt-manager",
        command:"sudo apt install virt-manager",
        description:null,
        needs_sudo:true
    };

let lutris = install_commands{
        name:"lutris",
        command:"sudo apt install lutris",
        description:null,
        needs_sudo:true
    };

let rust = install_commands{
        name:"rust",
        command:"sudo apt install cargo",
        description:null,
        needs_sudo:true
    };

let nodejs = install_commands{
        name:"nodejs",
        command:"sudo apt install nodejs npm",
        description:null,
        needs_sudo:true
    };

let tree = install_commands{
        name:"tree",
        command:"sudo apt install tree",
        description:null,
        needs_sudo:true
    };

let tlp = install_commands{
        name:"tlp",
        command:"sudo apt install tlp",
        description:null,
        needs_sudo:true
    };

let openssh_server = install_commands{
        name:"openssh server",
        command:["sudo apt install openssh-server","sudo systemctl enable sshd"],
        description:null,
        needs_sudo:true
    };

let ls = install_commands{
        name:"ls -a",
        command:"ls",
        description:null,
        needs_sudo:false
    };

let pwd = install_commands{
        name:"pwd",
        command:"pwd",
        description:null,
        needs_sudo:false
    };

}