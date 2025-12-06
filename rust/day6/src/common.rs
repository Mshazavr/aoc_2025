use std::env;

pub struct RunConfig {
    pub part: u8,
    pub test_name: String
}

impl RunConfig {
    pub fn get_test_path(&self) -> String {
        String::from(format!("tests/{0}.txt", self.test_name))
    }

    pub fn from_env() -> Self {
        let args: Vec<String> = env::args().collect();
    
        let part: u8 = args[1].parse::<u8>().unwrap();
        let test_name: &str = &args[2];
    
        RunConfig {
            part,
            test_name: String::from(test_name)
        }
    }
}