#[derive(Eq, PartialEq, Hash, Clone, Copy)]
pub enum Format {
    Table,
    Json,
    Dotenv,
    Unknown,
}

impl From<String> for Format {
    fn from(value: String) -> Self {
        match value.as_str() {
            "json" => Format::Json,
            "table" => Format::Table,
            "dotenv" => Format::Dotenv,
            _ => Format::Unknown,
        }
    }
}
