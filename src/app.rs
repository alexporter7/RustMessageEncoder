pub mod app_config {
    use std::collections::HashMap;
    use config::Config;

    pub fn get_config() -> HashMap<String, String> {
        let settings = Config::builder()
            .add_source(config::File::with_name("config.toml"))
            .set_default("archive_path", "./archive").unwrap()
            .set_default("mode", "warning")
            .unwrap()
            .build();

        settings.unwrap().try_deserialize::<HashMap<String, String>>()
            .expect("Unable to deserialize config")


    }

    pub fn default_config() {
        let config_file = Config::builder()
            .build()
            .unwrap();

    }

}

pub mod app_data {

}