pub mod message_binary_form {
    use dialoguer::console::Style;
    use dialoguer::Input;
    use dialoguer::theme::ColorfulTheme;
    use crate::message::message::MessageBinary;

    pub fn binary_wizard() {
        let theme = ColorfulTheme {
            values_style: Style::new().blue(),
            ..ColorfulTheme::default()
        };

        let binary_name: String = Input::with_theme(&theme)
            .with_prompt("Message Binary Name: ")
            .interact()
            .expect("Error getting binary name");

        let binary_file_name: String = Input::with_theme(&theme)
            .with_prompt("Binary File Name: ")
            .default(parse_name(binary_name.as_str()))
            .interact()
            .expect("Error parsing file name");

        create_binary(&binary_name, &binary_file_name);
    }

    pub fn parse_name(name: &str) -> String {
        String::from(name.replace(" ", "") + ".bin")
    }

    pub fn create_binary(name: &String, file_name: &String) {
        let bin_file_struct = MessageBinary::from_empty(name.to_string(),
                                                                    file_name.to_string());
        bin_file_struct.write_bin();
    }

}