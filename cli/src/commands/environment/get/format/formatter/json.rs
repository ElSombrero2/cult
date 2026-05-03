use std::collections::HashMap;
use crate::commands::environment::get::format::Formatable;

pub struct JsonFormat;

impl Formatable for JsonFormat {
    fn format (&self, envs: Vec<cult::entities::environment::Model>) -> String {
        let mut map: HashMap<String, String> = HashMap::new();

        for env in envs { map.insert(env.key, env.value); }

        serde_json::to_string_pretty(&map).unwrap_or_default()
    }
}
