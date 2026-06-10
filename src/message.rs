pub mod message {
    use oxicode::{Decode, Encode};
    use std::fmt::Formatter;
    use std::fs::File;
    use std::io::{Read, Write};


    #[derive(Encode, Decode, Debug)]
    pub struct MessageBinary {
        pub mes_all_read:   bool,
        pub mes_in_binary:  i32,
        pub messages:       Vec<Message>
    }

    #[derive(Encode, Decode, Debug)]
    pub struct Message {
        pub metadata:   MessageMetadata,
        pub author:     String,
        pub name:       String,
        pub content:    String,
    }

    #[derive(Encode, Decode, Debug)]
    pub struct MessageMetadata {
        pub created:        u32,
        pub shifts:         i32,
        pub has_read:       bool,
        pub read_timestamp: Option<u32>,

    }

    impl MessageBinary {

        pub fn default() -> Self {
            Self {
                mes_all_read: false,
                mes_in_binary: 0,
                messages: Vec::new()
            }
        }

        pub fn archive(path: &str) {
            //close references to binary (if necessary) and move to archived folder
        }

    }

    impl Message {

        pub fn new(metadata: MessageMetadata, author: String, name: String, content: String) -> Self {
            Self { metadata, author, name, content }
        }

        pub fn from(path: &str) -> Self {
            let mut file = File::open(path)
                .expect("Unable to open file");

            let mut data_buffer = String::new();
            file.read_to_string(&mut data_buffer)
                .expect("Error reading data into buffer");

            let (m, _): (Message, _) = oxicode::decode_from_hex(&data_buffer)
                .expect("Error decoding bin contents");

            return m;
        }

        pub fn write_bin(&self) {

            /* Get proper file name */
            let mut file_name = String::from(&self.name.as_str()
                .replace(" ", ""));
            file_name.push_str(".bin");

            /* Create File */
            let mut file = File::create(file_name)
                .expect("Unable to create file");

            /* Write Data */
            let data = oxicode::encode_to_hex(self)
                .expect("Error encoding struct");
            file.write_all(data.as_bytes())
                .expect("Error writing to file");

        }
    }

    impl MessageMetadata {
        pub fn new(created: u32, shifts: i32, has_read: bool, read_timestamp: Option<u32>) -> Self {
            Self { created, shifts, has_read, read_timestamp }
        }
    }

    impl std::fmt::Display for Message {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "\
            Name: {}\n\
            Created: {}\n\
            Author: {}\n\
            Content:\n\
            {}", self.name, "N/A", self.author, self.content)
        }
    }
}