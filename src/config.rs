/// Author : James Njenga(jnjenga.com)
///
/// Simple hashmap based configuration module that loads
/// properties/values from a text file
///
/// # File format
///
/// PROPERTY=VALUE
/// `# This is a comment without the quotes`
///
use std::collections::HashMap;
use std::str::FromStr;

pub struct Config
{
    map: HashMap<String, String>,
}

impl Config
{
    /// Parse the string and populate the map
    pub fn new(config_str: &String) -> Config
    {
        let mut config = Config{ map: HashMap::new(), };
        let bytes = config_str.as_bytes();

        let mut prev_index  = 0;
        let mut should_skip = false;

        let mut key:   String = String::new();
        let mut value: String = String::new();

        for (i, &item) in bytes.iter().enumerate()
        {
            if should_skip
            {
                if item == b'\n'
                {
                    should_skip = false;
                    prev_index  = i + 1;
                }
                else if item == b'\r'
                {
                    should_skip = false;
                    prev_index  = i + 2;
                }
            }
            else
            {
                if item == b'='
                {
                    key = String::from(&config_str[prev_index..i]);
                    prev_index = i + 1;
                    config.add(&key, &value);
                }
                else if item == b'#'
                {
                    should_skip = true;
                }
                else if item == b'\r'
                {
                    if !key.is_empty() && prev_index != i
                    {
                        value = String::from(&config_str[prev_index..i]);
                        config.add(&key, &value);
                        key.clear();
                        value.clear();
                        prev_index = i + 2;
                    }
                }
                else if item == b'\n' || i == bytes.len() - 1
                {
                    if !key.is_empty() && prev_index != i
                    {
                        value = String::from(&config_str[prev_index..i]);
                        config.add(&key, &value);
                        key.clear();
                        value.clear();
                        prev_index = i + 1;
                    }
                    else
                    {
                    prev_index  = i+1;
                    }
                }
            }
        }

        config
    }

    /// Read file, parse and populate the map
    pub fn new_from_file(file_name: &str) -> Config
    {
        let config_str = std::fs::read_to_string(file_name).unwrap();
        Config::new(&config_str)
    }

    /// Add or Update a property in the map
    fn add(&mut self, key: &String, value: &String)
    {
        self.map.insert(key.to_string(), value.to_string());
    }

    /// Read string property
    pub fn get(&self, key: &str) -> Option<&String>
    {
        self.map.get(key)
    }

    /// Read u32 property
    pub fn getu32(&self, key: &str) -> Option<u32>
    {
        let string = self.map.get(key)?.as_str();
        Some(u32::from_str(string).unwrap())
    }

    /// Read i32 property
    pub fn geti32(&self, key: &str) -> Option<i32>
    {
        let string = self.map.get(key)?.as_str();
        Some(i32::from_str(string).unwrap())
    }

    // Print contents of map
    pub fn print(&self)
    {
        println!("{:?}", self.map);
    }
}
