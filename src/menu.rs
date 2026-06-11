pub mod menu {
    use std::ops::Deref;
    use console_menu::{Menu, MenuOption, MenuProps};
    use console_menu::color::{BLACK, BLUE};
    use crate::app::app_config::get_option;
    use crate::form;
    use crate::util::file_util::get_directory_files;
    use crate::util::menu_util::{create_menu_from_list, get_default_props};

    pub fn show_menu(menu_options: Vec<MenuOption>, menu_props: MenuProps) {
        let mut menu = Menu::new(menu_options, menu_props);
        menu.show();
    }

    pub fn show_main_menu(){
        let options: Vec<MenuOption> = vec![
            MenuOption::new("Open Message Binary File", show_message_binary_options),
            MenuOption::new("Edit Config", show_config_options)
        ];

        let title = "Rust Message Encoder".to_string();
        let message = "Select an option".to_string();
        let props = get_default_props(&title, &message);

        show_menu(options, props);
    }

    pub fn show_message_binary_options() {
        let options: Vec<MenuOption> = vec![
            MenuOption::new("Create New Message Binary", show_create_binary_options),
            MenuOption::new("Open Message Binary", show_open_binary_options)
        ];

        let title = "Message Binary Options".to_string();
        let message = "Select an option".to_string();
        let props = get_default_props(&title, &message);

        show_menu(options, props);
    }

    pub fn show_create_binary_options() {
        form::message_binary_form::binary_wizard();
    }

    pub fn show_open_binary_options() {
        let _message_directory = get_option(&String::from("message_binary_path"));
        let _message_files = get_directory_files(&_message_directory);

        let options = create_menu_from_list(&_message_files);
        let title = "Rust Message Encoder".to_string();
        let message = "Select an option".to_string();
        let props = get_default_props(&title, &message);

        show_menu(options, props)
    }

    pub fn show_config_options() {

    }

    pub fn show_messages_menu(file_name: &str) {
        let mut title = String::from("Current Message Binary: ");
        title.push_str(file_name);

        let props = get_default_props(&title, &String::from("Message editor"));
    }

}
