// Il pulsante della pagina della mod: apre la schermata, che poi torna a Options.
fn customhud_settings_click(game: &mut crate::game::Game<'_>, mod_id: &str, button: usize) {
    use crate::game::customhud as hud;
    if mod_id == hud::MOD_ID && button == 0 {
        crate::game::customhud_screen::open_screen(game, hud::Parent::Options);
    }
}
