pub mod file_util {
    use std::fs;
    use crate::app::app_config::get_option;

    pub fn get_directory_files(path: &String) -> Vec<String> {
        let files = fs::read_dir(path)
            .expect("Unable to list files in directory");
        let mut file_names: Vec<String> = Vec::new();
        for path in files {
            let _name = path.unwrap().file_name();
            let _c_string = _name.to_str().unwrap();
            file_names.push(_c_string.to_string());
        }
        file_names
    }

    pub fn get_relative_message_path(name: &str) -> String {
        let mut rel_path = String::new();
        let _base_dir: String = get_option(&String::from("message_binary_path"));

        rel_path.push_str(_base_dir.as_str());
        rel_path.push_str("/");
        rel_path.push_str(name);

        rel_path
    }

}

pub mod menu_util {
    use console_menu::{MenuOption, MenuProps};
    use console_menu::color::{BLACK, BLUE};

    pub fn create_menu_from_list(items: &Vec<String>) -> Vec<MenuOption> {
        let mut options: Vec<MenuOption> = Vec::new();
        for item in items {

            let _label = item.clone();
            let _action_data = item.clone();
            let _action = move || { open_binary(&_action_data); };

            options.push(MenuOption::new(_label.as_str(), _action));
        }

        options
    }

    pub fn open_binary(name: &str) {
        println!("Arg called {name}");
    }

    pub fn get_default_props<'a>(title: &'a String, message: &'a String) -> MenuProps<'a> {
        MenuProps {
            title,
            message,
            bg_color: BLACK,
            title_color: Option::from(BLUE),
            selected_color: Option::from(BLUE),
            ..MenuProps::default()
        }

    }

}

pub mod term_util {
    use ratatui::prelude::Line;
    use crate::menu::message_view_ui::MessageScreen;

    pub fn get_messages(screen: &MessageScreen) -> Vec<Line<'static>> {

        let message_lines: Vec<Line<'static>> = Vec::new();


        message_lines

    }

    #[derive(Debug, Default)]
    pub enum ScrollDirection {
        #[default]
        None,
        Up,
        Down
    }

}