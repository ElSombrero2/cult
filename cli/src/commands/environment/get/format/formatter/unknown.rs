use cult::entities::environment;
use crate::commands::environment::get::format::Formatable;

pub struct UnknownFormat;

impl Formatable for UnknownFormat {
    fn format(&self, _: Vec<environment::Model>) -> String {
        String::from("❓Unknown format")
    }
}
