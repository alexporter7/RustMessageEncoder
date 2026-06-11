pub mod app_config {
    use std::collections::HashMap;
    use config::Config;
    use serde::Serialize;

    pub fn get_config() -> HashMap<String, String> {
        let settings = Config::builder()
            .add_source(config::File::with_name("config.toml"))
            .set_default("archive_path", "./archive").unwrap()
            .set_default("mode", "warning").unwrap()
            .set_default("message_binary_path", "./messages").unwrap()
            .build()
            .expect("Error getting config");

        settings.try_deserialize::<HashMap<String, String>>()
            .expect("Unable to deserialize config")

    }

    pub fn get_option(name: &String) -> String {
        let _config = get_config();
        let _val = _config.get(name);
        _val.unwrap().to_string()

    }

    pub fn save_config(config: HashMap<String, String>) {
        //TODO: need to re-serialize config
    }

}

pub mod app_data {

    pub fn setup() {

    }

    pub fn check_directories() {

    }

    pub fn check_config() {

    }

}

pub mod app_files {

    pub fn get_message_pages() {

    }

}