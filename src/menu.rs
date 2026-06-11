pub mod menu {
    use std::ops::Deref;
    use console_menu::{Menu, MenuOption, MenuProps};

    pub fn show_menu(menu_options: Vec<MenuOption>, menu_props: MenuProps) {
        let mut menu = Menu::new(menu_options, menu_props);
        menu.show();
    }

    pub fn show_main_menu(){
        let options: Vec<MenuOption> = vec![
            MenuOption::new("Open Message Binary File", show_message_binary_options),
            MenuOption::new("Edit Config", show_config_options)
        ];
        let props = MenuProps {
            title: "Rust Message Encoder",
            message: "A way for me to test out using rust",
            ..MenuProps::default()
        };

        show_menu(options, props);
    }

    pub fn show_message_binary_options() {
        let options: Vec<MenuOption> = vec![
            MenuOption::new("Create New Message Binary", show_create_binary_options),
            MenuOption::new("Open Message Binary", show_open_binary_options)
        ];
        let props = MenuProps {
            title: "Message Binary Options",
            message: "Options for handling message binaries",
            ..MenuProps::default()
        };

        show_menu(options, props);
    }

    pub fn show_create_binary_options() {

    }

    pub fn show_open_binary_options() {

    }

    pub fn show_config_options() {

    }

}
