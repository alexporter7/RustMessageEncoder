use crate::menu::menu::{show_main_menu, show_menu};
use crate::message::message::Message;
use crate::test_menu::s;

pub mod message;
pub mod menu;
pub mod app;
pub mod test_menu;
pub mod form;
pub mod util;

fn main() {
    
    let mut config = app::app_config::get_config();
    show_main_menu();

    //check if there is an app.bin file
        //if there is not create a blank one

    //load application options

    //load program data and print menus

}
