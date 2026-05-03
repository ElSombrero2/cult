use comfy_table::presets::NOTHING;
use cult::entities::environment;
use crate::{commands::environment::get::format::Formatable, utils::table::get_table};

pub struct TableFormat;

impl Formatable for TableFormat {
   fn format(&self, envs: Vec<environment::Model>) -> String {
      let mut table = get_table(NOTHING);

      table.set_header(vec!["Id", "Key", "Value"]);

      for env in envs {
        table.add_row(vec![
            env.id.to_string(),
            env.key,
            env.value,
        ]);
      }

      table.to_string()
   } 
}
