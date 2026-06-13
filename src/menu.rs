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

//TODO: some of this ui stuff can be abstracted, I'm just not good enough at rust yet to effectively do it right now
pub mod message_view_ui {
    use crate::menu::binary_select_ui::BinarySelectScreen;
    use crate::message::message::MessageBinary;
    use crate::util::term_util::ScrollDirection;
    use ratatui::buffer::Buffer;
    use ratatui::crossterm::event;
    use ratatui::crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind};
    use ratatui::layout::{Constraint, Direction, Layout, Rect};
    use ratatui::style::Style;
    use ratatui::text::Line;
    use ratatui::widgets::{Block, Paragraph, Widget};
    use ratatui::{DefaultTerminal, Frame};
    use std::cmp::min;
    use std::io;

    #[derive(Debug, Default)]
    pub struct MessageScreen {
        loaded_binary: Option<MessageBinary>,
        bin_data: Vec<Vec<String>>,
        exit: bool,
        active_pane: usize,
        active_index: usize,
        current_screen: bool,
        scroll_direction: ScrollDirection,
        last_offset: usize,
        view_message_index: usize
    }

    impl MessageScreen {

        const BINARY_PANE: usize = 0;
        const MESSAGES_PANE: usize = 1;
        const VIEW_PANE: usize = 2;

        const PG_DEFAULT_STYLE: Style = Style::new().light_blue();
        const PG_FOCUS_STYLE:   Style = Style::new().bold().light_blue().reversed();

        const MAX_MESSAGE_LINES: usize = 22;

        pub fn from_empty() -> Self {

            let _data: Vec<Vec<String>> = vec![
                Vec::new(),
                Vec::new(),
                Vec::new()
            ];

            Self {
                loaded_binary: Some(MessageBinary::from_empty("Test Binary".to_string(),
                                                              "test_file.bin".to_string())),
                bin_data: _data,
                exit: false,
                active_pane: Self::MESSAGES_PANE,
                active_index: 0,
                current_screen: true,
                scroll_direction: ScrollDirection::None,
                last_offset: 0,
                view_message_index: 0
            }
        }

        pub fn exit(&mut self) {
            self.exit = true;
        }

        pub fn load_binary(&mut self, bin: MessageBinary) {
            self.loaded_binary = Some(bin);
            let mut _data = self.bin_data.get_mut(Self::MESSAGES_PANE).unwrap();

            let _messages = &self.loaded_binary.as_ref().unwrap().messages;
            for message in _messages {
                _data.push(message.name.clone());
            }

        }

        pub fn set_last_offset(&mut self, offset: usize) {
            self.last_offset = offset;
        }

        pub fn get_offset(&self) -> usize {
            let mut _offset: usize = 0;

            if self.active_index > (Self::MAX_MESSAGE_LINES - 1) {
                if matches!(self.scroll_direction, ScrollDirection::Down){
                    _offset = self.active_index - (Self::MAX_MESSAGE_LINES - 1);
                }

                if matches!(self.scroll_direction, ScrollDirection::Up) {
                    if self.active_index < self.last_offset {
                        _offset = self.last_offset - 1;
                    } else {
                        _offset = self.last_offset;
                    }
                }
            }

            _offset
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
                    Constraint::Percentage(10),
                    Constraint::Percentage(90)])
                .split(area);

            let _bin = self.loaded_binary.as_ref().unwrap();

            let _bin_name = format!("Binary Name: {}", _bin.name);
            let _mes_in_bin = format!("Messages in Binary: {}", _bin.mes_in_binary);

            let _binary_overview_lines = vec![
                Line::styled(_bin_name, Self::PG_DEFAULT_STYLE),
                Line::styled(_mes_in_bin, Self::PG_DEFAULT_STYLE),
            ];

            /* Binary Overview */
            Paragraph::new("")
                .block(block)
                .render(area, buf);
            Paragraph::new(" Binary Overview ")
                .centered()
                .render(binary_overview_layout[0], buf);
            Paragraph::new(_binary_overview_lines)
                .centered()
                .render(binary_overview_layout[1], buf);
        }

        pub fn render_messages_pane(&self, block: Block,
                                    area: Rect, buf: &mut Buffer) {

            let messages_pane_layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![
                    Constraint::Percentage(5),
                    Constraint::Percentage(95)])
                .split(area);


            let mut _lines: Vec<Line> = Vec::new();

            let _bin = self.loaded_binary.as_ref().unwrap();
            let _bin_data = self.bin_data.get(Self::MESSAGES_PANE).unwrap();
            let _mes_in_bin = self.loaded_binary.as_ref().unwrap().mes_in_binary;

            let _max_iterations = min(_mes_in_bin, Self::MAX_MESSAGE_LINES as i32);


            for i in 0.._max_iterations as usize {
                let mut _offset: usize = self.get_offset();

                let _adjusted_index = i + _offset;
                let _mes = _bin_data.get(_adjusted_index).unwrap();

                if self.active_pane == Self::MESSAGES_PANE && self.active_index == _adjusted_index {
                    _lines.push(Line::styled(_mes, Self::PG_FOCUS_STYLE));
                } else {
                    _lines.push(Line::styled(_mes, Self::PG_DEFAULT_STYLE));
                }

            }

            /* Messages */
            Paragraph::new("")
                .block(block)
                .render(area, buf);
            Paragraph::new(" Messages ")
                .centered()
                .render(messages_pane_layout[0], buf);

            Paragraph::new(_lines)
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

            let view_pane_inner = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![
                    Constraint::Percentage(5),
                    Constraint::Percentage(5),
                    Constraint::Percentage(90)])
                .margin(1)
                .split(view_pane_layout[1]);

            let _messages = &self.loaded_binary.as_ref().unwrap().messages;
            if _messages.len() != 0 {
                let _message = _messages.get(self.view_message_index).unwrap();

                let _name = _message.name.clone();
                let _author = _message.author.clone();
                let _content = _message.content.clone();

                Paragraph::new(format!(" Message Name: {}", _name))
                    .render(view_pane_inner[0], buf);
                Paragraph::new(format!(" Author: {}", _author))
                    .render(view_pane_inner[1], buf);
                Paragraph::new(_content).left_aligned()
                    .render(view_pane_inner[2], buf);
            }

            /* View Message Pane */
            Paragraph::new("")
                .block(block.clone())
                .render(area, buf);
            Paragraph::new(" View Message ")
                .centered()
                .render(area, buf);


            // Paragraph::new(vec![
            //     Line::styled(format!("Last Offset: {}", self.last_offset), Self::PG_DEFAULT_STYLE),
            //     Line::styled(format!("Active Index: {}", self.active_index), Self::PG_DEFAULT_STYLE),
            //     Line::styled(format!("View Index: {}", self.view_message_index), Self::PG_DEFAULT_STYLE)])
            //     .centered()
            //     .render(view_pane_layout[1], buf);
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
                KeyCode::Up => {
                    if self.active_index > 0 {
                        self.active_index -= 1;
                        self.scroll_direction = ScrollDirection::Up;
                        self.last_offset = self.get_offset();
                    }

                }
                KeyCode::Down => {
                    let _num_messages = self.loaded_binary.as_ref().unwrap().mes_in_binary - 1;
                    if self.active_index < _num_messages as usize {
                        self.active_index += 1;
                        self.last_offset = self.get_offset();
                        self.scroll_direction = ScrollDirection::Down
                    }
                }
                KeyCode::Enter => {
                    let _mes_index = self.active_index;
                    self.view_message_index = _mes_index;
                    // let _messages = self.loaded_binary.as_ref()
                    //     .expect("Unable to access loaded binary");
                    // let _message = _messages.messages.get(self.active_index);
                }
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
    use crate::app::app_config;
    use crate::app::app_config::get_option;
    use crate::menu::message_view_ui::MessageScreen;
    use crate::message::message;
    use crate::util::file_util::get_directory_files;
    use ratatui::buffer::Buffer;
    use ratatui::crossterm::event;
    use ratatui::crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind};
    use ratatui::layout::{Layout, Rect};
    use ratatui::prelude::{Constraint, Direction, Line, Widget};
    use ratatui::style::Style;
    use ratatui::widgets::{Block, Paragraph};
    use ratatui::{DefaultTerminal, Frame};
    use std::io;

    #[derive(Debug, Default)]
    pub struct BinarySelectScreen {
        exit:       bool,
        binaries:   Vec<String>,
        current_screen: bool,
        focused_index: i32
    }

    impl BinarySelectScreen {

        pub fn new() -> Self {
            Self { exit: false, binaries: Vec::new(), current_screen: true, focused_index: 0 }
        }

        pub fn exit(&mut self) {
            self.exit = true;
        }

        pub fn get_message_binaries(&mut self) {
            self.binaries = get_directory_files(
                &get_option(&app_config::MESSAGE_BINARY_PATH.to_string()));

        }

        pub fn load_message_binary(&mut self) {
            //create message binary struct
            let _index = self.focused_index as usize;
            let bin = message::MessageBinary::from_file(
                self.binaries.get(_index)
                    .expect("Unable to get binary at selected index")
            );

            //exit this screen
            self.current_screen = false;

            //load message view screen
            let mut message_screen = MessageScreen::from_empty();
            message_screen.load_binary(bin);
            message_screen.load();

            //populate
        }

        pub fn get_message_widget_lines(&self) -> Vec<Line<'static>> {
            let mut lines: Vec<Line<'static>> = Vec::new();

            let mut i = 0;
            for file in &self.binaries {
                let _base_path = get_option(&app_config::MESSAGE_BINARY_PATH.to_string());
                let mut _file_name = String::from(_base_path);
                _file_name.push_str("/");
                _file_name.push_str(file.as_str());

                let default_style = Style::new().light_blue();
                let focused_style = Style::new().light_blue().bold().reversed();

                if i == self.focused_index {
                    lines.push(Line::styled(_file_name, focused_style)); }
                else {
                    lines.push(Line::styled(_file_name, default_style));
                }


                i += 1;
            }

            lines
        }

        pub fn load(&mut self) {
            ratatui::run(|terminal| self.run(terminal))
                .expect("Error starting MessageViewScreen");
        }

        pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {

            self.get_message_binaries();

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
                KeyCode::Enter => {
                    self.load_message_binary()
                }
                KeyCode::Down => {
                    let _total_binaries: i32 = self.binaries.len() as i32;
                    if self.focused_index < (_total_binaries - 1) {
                        self.focused_index += 1; }
                }
                KeyCode::Up => {
                    let _total_binaries: i32 = self.binaries.len() as i32;
                    if self.focused_index != 0 {
                        self.focused_index -= 1; }
                }
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
                .block(block.clone()).render(area, buf);

            Paragraph::new(self.get_message_widget_lines())
                .centered()
                .block(block.clone()).render(area, buf);
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

pub mod main_menu_ui {
    
}
