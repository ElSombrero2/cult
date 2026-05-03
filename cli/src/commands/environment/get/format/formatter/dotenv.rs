use cult::entities::environment;
use crate::commands::environment::get::format::Formatable;

pub struct DotenvFormat;

impl Formatable for DotenvFormat {
    fn format (&self, envs: Vec<environment::Model>) -> String {
        let mut str = String::new();
        for env in envs {
            str.push_str(&format!("{}={}\n", env.key, env.value));
        }
        str
    }
}

