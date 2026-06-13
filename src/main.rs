use crate::menu::menu::show_main_menu;
use crate::menu::message_view_ui::MessageScreen;

pub mod message;
pub mod menu;
pub mod app;
pub mod form;
pub mod util;

fn main() {

    let mut mvc2 = MessageScreen::from_empty();
    mvc2.load();

    //show_main_menu();
    //show_main_menu()


    //check if there is an app.bin file
        //if there is not create a blank one

    //load application options

    //load program data and print menus

}
