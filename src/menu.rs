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
    use ratatui::text::{Line, Span};



    #[derive(Debug, Default)]
    pub struct MessageScreen {
        file_name: String,
        bin_data: Vec<Vec<String>>,
        exit: bool,
        active_pane: i32,
        active_index: i32
    }

    impl MessageScreen {

        const BINARY_PANE: i32 = 0;
        const MESSAGES_PANE: i32 = 1;
        const VIEW_PANE: i32 = 2;

        const PG_DEFAULT_STYLE: Style = Style::new().light_blue();

        pub fn from_empty() -> Self {

            let _data: Vec<Vec<String>> = vec![
                Vec::new(),
                Vec::new(),
                Vec::new()
            ];

            Self {
                file_name: String::from("TestFileName.bin"),
                bin_data: _data,
                exit: false,
                active_pane: Self::MESSAGES_PANE,
                active_index: 0
            }
        }

        pub fn exit(&mut self) {
            self.exit = true;
        }

        pub fn render_title(&self, block: Block, area: Rect, buf: &mut Buffer) {
            /* Title Paragraph */
            Paragraph::new("Rust Message Encoder")
                .centered()
                .block(block).render(area, buf);
        }

        pub fn render_binary_overview(&self, block: Block, area: Rect, buf: &mut Buffer) {
            let binary_overview_layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![
                    Constraint::Percentage(5),
                    Constraint::Percentage(95)])
                .split(area);

            /* Binary Overview */
            Paragraph::new("")
                .block(block)
                .render(area, buf);
            Paragraph::new(" Binary Overview ")
                .centered()
                .render(binary_overview_layout[0], buf);
        }

        pub fn render_messages_pane(&self, block: Block,
                                    area: Rect, buf: &mut Buffer) {

            let messages_pane_layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![
                    Constraint::Percentage(5),
                    Constraint::Percentage(95)])
                .split(area);

            /* Messages */
            Paragraph::new("")
                .block(block)
                .render(area, buf);
            Paragraph::new(" Messages ")
                .centered()
                .render(messages_pane_layout[0], buf);
            Paragraph::new(vec![
                Line::styled("Create New Message", Self::PG_DEFAULT_STYLE),
                Line::styled("Message 1", Self::PG_DEFAULT_STYLE),
                Line::styled("Message 2", Self::PG_DEFAULT_STYLE),
            ])
                .centered()
                .render(messages_pane_layout[1], buf);
        }

        pub fn render_view_pane(&self, block: Block, area: Rect, buf: &mut Buffer) {
            let view_pane_layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![
                    Constraint::Percentage(5),
                    Constraint::Percentage(95)])
                .split(area);

            /* View Message Pane */
            Paragraph::new("")
                .block(block)
                .render(area, buf);
            Paragraph::new(" View Message ")
                .centered()
                .render(view_pane_layout[0], buf);
        }

        pub fn load(&mut self) {
            ratatui::run(|terminal| self.run(terminal))
                .expect("Error starting MessageViewScreen");
        }

        pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {

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
                Event::Key(key_event) => {
                    match key_event.kind {
                        KeyEventKind::Press => self.handle_key_press_event(key_event),
                        _ => {} } }
                _ => { }
            }
            Ok(())
        }

        pub fn handle_key_press_event(&mut self, key_event: KeyEvent) {
            match key_event.code {
                KeyCode::Esc | KeyCode::Char('q') => { self.exit() }
                _ => { }
            }
        }

    }

    impl Widget for &MessageScreen {
        fn render(self, area: Rect, buf: &mut Buffer) where Self: Sized {
            let title_layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![
                    Constraint::Percentage(10),
                    Constraint::Percentage(90)])
                .split(area);

            let column_layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(vec![
                    Constraint::Percentage(25),
                    Constraint::Percentage(25),
                    Constraint::Percentage(50)])
                .split(title_layout[1]);

            let _styled = Block::bordered()
                .style(Style::new().bold().light_blue());

            self.render_title(_styled.clone(), title_layout[0], buf);
            self.render_binary_overview(_styled.clone(), column_layout[0], buf);
            self.render_messages_pane(_styled.clone(), column_layout[1], buf);
            self.render_view_pane(_styled.clone(), column_layout[2], buf);

        }
    }

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
                Event::Key(key_event)
                if key_event.kind == KeyEventKind::Press => {
                  self.handle_key_event(key_event) },
                _ => {}
            };
            Ok(())
        }

        fn handle_key_event(&mut self, key_event: KeyEvent) {
            match key_event.code {
                KeyCode::Esc => self.exit(),
                KeyCode::Up | KeyCode::Down | KeyCode::Left | KeyCode::Right => {
                    self.handle_user_input(key_event) }
                _ => {}
            }
        }

        fn handle_user_input(&mut self, key_event: KeyEvent) {

        }

        fn get_message_lines(&self) -> Vec<Line<'static>> {
            let _lines: Vec<Line<'static>> = vec![
                Line::styled("Create New Message", Style::new().reversed()),
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
                    Constraint::Percentage(90)])
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
                    Constraint::Percentage(95)])
                .split(column_layout[0]);

            let messages_layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![
                    Constraint::Percentage(5),
                    Constraint::Percentage(95)])
                .split(column_layout[1]);

            let message_pane_layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![
                    Constraint::Percentage(5),
                    Constraint::Percentage(95)])
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
