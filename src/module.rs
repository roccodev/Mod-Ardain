use crate::get_platform_data;

pub struct Module {
    pub name: String,
    pub version: String,
}

pub fn register_module(module: Module) {
    let platform = get_platform_data();
}
