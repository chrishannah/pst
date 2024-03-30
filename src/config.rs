pub mod load {
    use core::fmt;
    use std::fs::File;
    use std::io::Read;

    use serde::Deserialize;

    #[derive(Deserialize, Debug)]
    pub struct Config {
        pub token: String,
    }

    impl fmt::Display for Config {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.token)
        }
    }

    pub fn load_config() -> Result<Config, String> {
        let path = "config.json";

        let mut file = File::open(path).unwrap();
        let mut buff = String::new();
        file.read_to_string(&mut buff).unwrap();

        let config: Config = serde_json::from_str(&buff).unwrap();

        Ok(config)
    }
}
