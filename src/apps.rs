pub fn cmd() {

    struct install_commands {
        name:str,
        command:str,
        description:str,
        requires_sudo:bool
    }

    
    let rust_lang = install_commands{
        name:"rust",
        command:"sudo apt install cargo",
        description:"Let's get rusty",
        needs_sudo:true
    };
}
