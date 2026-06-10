pub mod message {
    use std::fmt::{write, Formatter};
    use std::fs::File;
    use std::io::{Read, Write};

    pub struct Message {
        pub created:    DateTime<Utc>,
        pub encoded:    bool,
        pub shifts:     int,
        pub author:     String,
        pub name:       String,
        pub content:    String,
    }

    impl Message {
        pub fn new(created: DateTime<Utc>, encoded: bool, shifts: int,
                   author: String, name: String, content: String) -> Self {
            Self { created, encoded, shifts, author, name, content }
        }

        pub fn from(path: &str) {
            let mut file = File::open(path)
                .expect("Unable to open file at {}", path);

            file.read(&Self.created).expect("Unable to read file");
            file.read(&Self.encoded).expect("Unable to read file");
            file.read(&Self.shifts).expect("Unable to read file");
            file.read(&Self.author).expect("Unable to read file");
            file.read(&Self.name).expect("Unable to read file");
            file.read(&Self.content).expect("Unable to read file");
            
            
        }

        pub fn decode() {

        }

        pub fn write_bin() {
            let mut file = File::create(
                String::from(&Self.name).push_str(".bin"))
                .expect("Unable to create file");
            file.write_all(&Self.created).expect("Unable to write to file");
            file.write_all(&Self.encoded).expect("Unable to write to file");
            file.write_all(&Self.shifts).expect("Unable to write to file");
            file.write_all(&Self.author).expect("Unable to write to file");
            file.write_all(&Self.name).expect("Unable to write to file");
            file.write_all(&Self.content).expect("Unable to write to file");

        }
    }

    impl std::fmt::Display for Message {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write(f, "\
            Name: {}\n\
            Created: {}\n\
            Author: {}\n,\
            Content:\n\
            {}", self.name, self.created, self.author, self.content);
        }
    }
}