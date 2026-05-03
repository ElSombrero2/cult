use cult::entities::environment;

pub mod formatter;
pub mod enums;

pub trait Formatable { fn format(&self, envs: Vec<environment::Model>) -> String; }

