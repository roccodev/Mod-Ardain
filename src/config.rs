use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct RuntimeConfig {
    ui_visible: bool,
    blade_create_disable_save: bool,
    return_title: bool,
    infinite_flutterheart: bool,
    chain_attack_rate_fix: bool,
    blade_create_default_sel: BladeCreateDefault,
}

#[derive(Debug, Deserialize)]
pub enum BladeCreateDefault {
    Common = 1 << 0,
    Rare = 1 << 1,
    Legendary = 1 << 2,
    Worst = 1 << 3,
    Best = 1 << 4,
    Bravery = 1 << 5,
    Truth = 1 << 6,
    Compassion = 1 << 7,
    Justice = 1 << 8,
    NoRarity = 1 << 9,
    NoIdea = 1 << 10,
}
