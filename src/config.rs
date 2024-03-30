pub mod load {
    use core::fmt;
    use std::fs::File;
    use std::io::Read;
    use serde::Deserialize;

    #[derive(Deserialize)]
    pub struct Config {
        pub token: String,
    }

    impl fmt::Display for Config {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.token)
        }
    }

    pub fn load_config() -> Result<Config, String> {
        let mut path: String;
        match home::home_dir() {
            Some(home_path) => path = home_path.into_os_string().into_string()
                .expect("Cannot determine home directory"),
            None => return Err("Cannot detect home directory".to_string()),
        }
        path += "/.config/pst/config.json";

        let mut file = File::open(path).unwrap();
        let mut buff = String::new();
        file.read_to_string(&mut buff).unwrap();

        let config: Config = serde_json::from_str(&buff).unwrap();

        Ok(config)
    }
}
