// La libreria Cargo autonoma: gli stessi file che horizon-mixins inietta nel
// gioco come moduli fratelli di `crate::game` (vedi `horizon-mixin.toml`), qui
// fratelli nella radice del crate, così `super::customhud_*` funziona in tutti e
// due i casi e la logica si prova con `cargo test` senza Horizon.

#[path = "../mixins/customhud/runtime.rs"]
pub mod customhud;
#[path = "../mixins/customhud/config.rs"]
pub mod customhud_config;
#[path = "../mixins/customhud/config_screen.rs"]
pub mod customhud_config_screen;
#[path = "../mixins/customhud/text.rs"]
pub mod customhud_text;
