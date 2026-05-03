use std::collections::HashMap;
use cult::entities::environment;
use crate::commands::environment::get::format::{
    enums::Format,
    formatter::{
        dotenv::DotenvFormat, json::JsonFormat, table::TableFormat, unknown::UnknownFormat
    }, 
    Formatable
};

pub mod dotenv;
pub mod json;
pub mod table;
pub mod unknown;

pub struct Formatter { formatters: HashMap<Format, Box<dyn Formatable>> }

impl Formatter {
   pub fn new() -> Self {
       let mut formatters: HashMap<Format, Box<dyn Formatable>> = HashMap::new(); 
       formatters.insert(Format::Unknown, Box::new(UnknownFormat));
       formatters.insert(Format::Dotenv, Box::new(DotenvFormat));
       formatters.insert(Format::Table, Box::new(TableFormat));
       formatters.insert(Format::Json, Box::new(JsonFormat));
       Self { formatters }
   }

   pub fn print(&self, envs: Vec<environment::Model>, format: Format) {
       if let Some(formater) = self.formatters.get(&format) {
            println!("{}", formater.format(envs));
       }
   }
}

