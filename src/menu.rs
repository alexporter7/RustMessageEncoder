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

pub mod message_view_ui {
    use crate::menu::binary_select_ui::BinarySelectScreen;
    use ratatui::buffer::Buffer;
    use ratatui::crossterm::event;
    use ratatui::crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind};
    use ratatui::crossterm::style::Stylize;
    use ratatui::layout::{Constraint, Direction, Layout, Rect};
    use ratatui::style::Style;
    use ratatui::text::Line;
    use ratatui::widgets::{Block, Paragraph, Widget};
    use ratatui::{DefaultTerminal, Frame};
    use std::io;

    #[derive(Debug, Default)]
    pub struct MessageScreen {
        file_name: String,
        bin_data: Vec<Vec<String>>,
        exit: bool,
        active_pane: i32,
        active_index: i32,
        current_screen: bool
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
                active_index: 0,
                current_screen: true
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

        pub fn render_footer(&self, block: Block, area: Rect, buf: &mut Buffer) {
            Paragraph
            ::new("Exit: [Esc / Q] | Open Binary: [O] | New Message: [N] | Select: [Enter]")
                .centered()
                .block(block)
                .render(area, buf)
        }

        pub fn load(&mut self) {
            ratatui::run(|terminal| self.run(terminal))
                .expect("Error starting MessageViewScreen");
        }

        pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {

            while !self.exit && self.current_screen {
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
                KeyCode::Char('o') => { BinarySelectScreen::new().load(); self.current_screen = false }
                KeyCode::Char('n') => {}
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
                    Constraint::Percentage(80),
                    Constraint::Percentage(10)])
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
            self.render_footer(_styled.clone(), title_layout[2], buf);


        }
    }

}

pub mod binary_select_ui {
    use crate::app::app_config::get_option;
    use crate::menu::message_view_ui::MessageScreen;
    use ratatui::buffer::Buffer;
    use ratatui::crossterm::event;
    use ratatui::crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind};
    use ratatui::layout::{Layout, Rect};
    use ratatui::prelude::{Constraint, Direction, Widget};
    use ratatui::style::Style;
    use ratatui::widgets::{Block, Paragraph};
    use ratatui::{DefaultTerminal, Frame};
    use std::io;

    #[derive(Debug, Default)]
    pub struct BinarySelectScreen {
        exit:       bool,
        binaries:   Vec<String>,
        current_screen: bool
    }

    impl BinarySelectScreen {

        pub fn new() -> Self {
            Self { exit: false, binaries: Vec::new(), current_screen: true }
        }

        pub fn exit(&mut self) {
            self.exit = true;
        }

        pub fn load(&mut self) {
            ratatui::run(|terminal| self.run(terminal))
                .expect("Error starting MessageViewScreen");
        }

        pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {

            while !self.exit && self.current_screen {
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
                KeyCode::Esc | KeyCode::Char('q') => { self.current_screen = false; MessageScreen::from_empty().load() }
                KeyCode::Enter => { }
                _ => { }
            }
        }

        pub fn render_title(&self, block: Block, area: Rect, buf: &mut Buffer) {
            Paragraph::new("Rust Message Encoder")
                .centered()
                .block(block).render(area, buf);
        }

        pub fn render_footer(&self, block: Block, area: Rect, buf: &mut Buffer) {
            Paragraph::new("Exit: [Esc / Q] | Select [Enter]")
                .centered()
                .block(block)
                .render(area, buf)
        }

        pub fn render_file_list(&self, block: Block, area: Rect, buf: &mut Buffer) {

            let mut _t = String::from(" Files: [");
            _t.push_str(get_option(&"message_binary_path".to_string()).as_str());
            _t.push_str("] ");

            Paragraph::new(_t)
                .centered()
                .block(block).render(area, buf);
        }
    }

    impl Widget for &BinarySelectScreen {
        fn render(self, area: Rect, buf: &mut Buffer) where Self: Sized {
            let root_layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![
                    Constraint::Percentage(10),
                    Constraint::Percentage(80),
                    Constraint::Percentage(10)])
                .split(area);

            let styled_block = Block::bordered().style(Style::new().bold().light_blue());

            self.render_title(styled_block.clone(), root_layout[0], buf);
            self.render_file_list(styled_block.clone(), root_layout[1], buf);
            self.render_footer(styled_block.clone(), root_layout[2], buf);
        }
    }

}
