use crate::menu::message_view_ui::MessageScreen;
use chrono::Timelike;
use crate::app::app_state::{AppState, Screen};

pub mod message;
pub mod menu;
pub mod app;
pub mod form;
pub mod util;

fn main() {

    let mut current_screen = AppState::new();

    let mut mvc2 = MessageScreen::from_empty();
    mvc2.load();

    // loop {
    //     match current_screen.get_app_screen() {
    //         Screen::None => { }
    //         Screen::ViewMessageScreen => { }
    //         Screen::OpenBinaryScreen => { }
    //         Screen::NewMessageScreen => { }
    //         _ => { }
    //     }
    // }




    //let mut test_bin = MessageBinary::from_file("RealTest1.bin");

    //println!("{test_bin}");


    // let mut test_bin =
    //     MessageBinary::from_empty("Real Test 1".to_string(),
    //                               "RealTest1.bin".to_string());

    // for i in 0..117 {
    //
    //     let test_meta = MessageMetadata {
    //         created: Utc::now().second(),
    //         shifts: i % 7,
    //         has_read: i > 55,
    //         read_timestamp: None,
    //     };
    //
    //     let mut _message_name = String::from("Message Number ");
    //     _message_name.push_str(i.to_string().as_str());
    //
    //     let mut _content = String::from("Oh man here we go we're on message number: ");
    //     _content.push_str(i.to_string().as_str());
    //
    //     let _message = Message::new(test_meta, "Alex".to_string(), _message_name, _content);
    //
    //     println!("[{i}] {}", _message);
    //
    //     test_bin.add_message(_message);
    //
    // }
    //
    // test_bin.write_bin();


    //mvc.load();

    //show_main_menu();


    //check if there is an app.bin file
        //if there is not create a blank one

    //load application options

    //load program data and print menus

}
