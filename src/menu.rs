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
    use ratatui::buffer::Buffer;
    use ratatui::crossterm::style::Stylize;
    use ratatui::layout::{Constraint, Direction, Layout, Rect};
    use ratatui::style::Style;
    use ratatui::widgets::{Block, Paragraph, Widget};
    use ratatui::{DefaultTerminal, Frame};
    use std::io;
    use ratatui::crossterm::event;
    use ratatui::crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind};
    use ratatui::text::Line;
    use log::error;

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

        pub fn exit(&mut self) {
            self.exit = true;
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
            match event::read()? {
                Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                  self.handle_key_event(key_event) },
                _ => {}
            };
            Ok(())
        }

        fn handle_key_event(&mut self, key_event: KeyEvent) {
            match key_event.code {
                KeyCode::Esc => self.exit(),
                _ => {}
            }
        }

        fn get_message_lines(&self) -> Vec<Line<'static>> {
            let _lines: Vec<Line<'static>> = vec![
                "Create New Message".into(),
                "Message 1".into(),
                "Message 2".into()
            ];
            _lines
        }

    }

    impl Widget for &MessageViewScreen {
        fn render(self, area: Rect, buf: &mut Buffer) where Self: Sized {

            let title_layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![
                    Constraint::Percentage(10),
                    Constraint::Percentage(90)
                ])
                .split(area);

            let column_layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(vec![
                    Constraint::Percentage(25),
                    Constraint::Percentage(25),
                    Constraint::Percentage(50)])
                .split(title_layout[1]);

            let binary_overview_layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![
                    Constraint::Percentage(5),
                    Constraint::Percentage(95)
                ])
                .split(column_layout[0]);

            let messages_layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![
                    Constraint::Percentage(5),
                    Constraint::Percentage(95)
                ])
                .split(column_layout[1]);

            let message_pane_layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![
                    Constraint::Percentage(5),
                    Constraint::Percentage(95)
                ])
                .split(column_layout[2]);


            let _styled = Block::bordered()
                .style(Style::new().bold().light_blue());

            /* Title Paragraph */
            Paragraph::new("Rust Message Encoder").centered().block(
                _styled.clone()).render(title_layout[0], buf);


            /* Binary Overview */
            Paragraph::new("")
                .block(_styled.clone())
                .render(column_layout[0], buf);
            Paragraph::new(" Binary Overview ")
                .centered()
                .render(binary_overview_layout[0], buf);

            /* Messages */
            Paragraph::new("")
                .block(_styled.clone())
                .render(column_layout[1], buf);
            Paragraph::new(" Messages ")
                .centered()
                .render(messages_layout[0], buf);
            Paragraph::new(self.get_message_lines())
                .centered()
                .render(messages_layout[1], buf);

            /* View Message Pane */
            Paragraph::new("")
                .block(_styled.clone())
                .render(column_layout[2], buf);
            Paragraph::new(" View Message ")
                .centered()
                .render(message_pane_layout[0], buf);


        }

    }

}
