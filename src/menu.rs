pub mod menu {
    use crate::app::app_config::get_option;
    use crate::form;
    use crate::util::file_util::get_directory_files;
    use crate::util::menu_util::{create_menu_from_list, get_default_props};
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

        //let props = get_default_props(&title, &String::from("Message editor"));
    }

}

pub mod terminal_ui {
    use std::io;
    use std::ops::Deref;
    use ratatui::{DefaultTerminal, Frame};
    use ratatui::buffer::Buffer;
    use ratatui::crossterm::style::Stylize;
    use ratatui::layout::Rect;
    use ratatui::prelude::Span;
    use ratatui::style::Style;
    use ratatui::symbols::border;
    use ratatui::text::Line;
    use ratatui::widgets::{Block, Paragraph, Widget};

    #[derive(Debug, Default)]
    pub struct MessageViewScreen {
        title:              String,
        file_name:          String,
        binary_info:        Vec<String>,
        unread_messages:    Vec<String>,
        message_pane:       Vec<String>,
        exit:               bool
    }


    impl MessageViewScreen {

        pub fn new(title: String, file_name: String, binary_info: Vec<String>,
                   unread_messages: Vec<String>, message_pane: Vec<String>, exit: bool) -> Self {
            Self { title, file_name, binary_info, unread_messages, message_pane, exit }
        }

        pub fn from_empty() -> Self {
            Self {
                title: String::from("Message Title"),
                file_name: String::from("TestMesFile.bin"),
                binary_info: Vec::new(),
                unread_messages: Vec::new(),
                message_pane: Vec::new(),
                exit: false
            }
        }

        pub fn load(&mut self) {
            ratatui::run(|terminal| self.run(terminal))
                .expect("Error starting MessageViewScreen");
        }

        pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()>{

            while !self.exit {
                terminal.draw(|frame| self.draw(frame))?;
                self.handle_events()?;
            }

            Ok(())
        }

        fn draw(&self, frame: &mut Frame) {
            frame.render_widget(self, frame.area());
        }

        fn handle_events(&mut self) -> io::Result<()> {
            Ok(())
        }

    }

    impl Widget for &MessageViewScreen {
        fn render(self, area: Rect, buf: &mut Buffer) where Self: Sized {
            let mut title = Line::from("test");


            let mut test: Vec<Span> = Vec::new();


            let t: Vec<Span> = vec![
                " Decrement ".into(),
                "<Left>".into(),
                " Increment ".into(),
                "<Right>".into(),
                " Quit ".into(),
                "<Q> ".into(),
            ];



            let mut _bottom_title = String::from("Viewing [");
            _bottom_title.push_str(&self.file_name.as_str());
            _bottom_title.push_str("] ");

            let s = Style::new().bold().blue();


            let instructions = Line::styled(_bottom_title.as_str(), s);

            let block = Block::bordered()
                .title(title.centered())
                .title_bottom(instructions.centered())
                .border_set(border::THICK);

            Paragraph::new("testing paragraph").centered().block(block).render(area, buf)
        }
    }

}
