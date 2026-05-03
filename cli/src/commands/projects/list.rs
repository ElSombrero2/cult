use comfy_table::presets;
use cult::connection::Connection;
use crate::utils::table;

pub async fn exec(conn: Connection) {
    let projects = conn.project.find_all().await;
    let mut table = table::get_table(presets::NOTHING);
    
    table.set_header(vec!["Id", "Name", "Creation Date"]);

    for project in projects {
       table.add_row(vec![
           project.id.to_string(),
           project.name,
           project.created_at.to_string()
       ]); 
    }

    println!("{table}");
}
