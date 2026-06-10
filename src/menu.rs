pub mod menu {
    use std::fmt::Formatter;

    pub struct Menu {
        pub name: String,
        pub options: Vec<MenuOption>,
    }

    pub struct MenuOption {
        pub name: String,
        pub label: String,
    }


    impl Menu {

        pub fn run(&self) {
            //clear screen

            //print title

            //print available options

            //get user input
        }
        
        pub fn handle_option(&self) {
            //check if user input index is in bounds
            
            //find what menu option was selected
            
            //do something (probably with a lambda)
        }

    }

    impl std::fmt::Display for Menu {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "not yet implemented")
        }
    }
}
