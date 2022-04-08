use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct RuntimeConfig {
    ui_visible: bool,
    blade_create_disable_save: bool,
    return_title: bool,
    infinite_flutterheart: bool,
    chain_attack_rate_fix: bool,
}
