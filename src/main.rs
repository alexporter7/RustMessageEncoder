use crate::menu::menu::show_main_menu;
use crate::menu::terminal_ui::MessageViewScreen;

pub mod message;
pub mod menu;
pub mod app;
pub mod form;
pub mod util;

fn main() {

    let mut mvc = MessageViewScreen::from_empty();

    mvc.load();

    //mvc.load();

    //show_main_menu();


    //check if there is an app.bin file
        //if there is not create a blank one

    //load application options

    //load program data and print menus

}
