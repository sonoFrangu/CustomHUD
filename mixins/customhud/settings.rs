// La pagina della mod in Options > Mods ha un solo pulsante, che apre la
// schermata delle impostazioni (come l'ingresso di Mod Menu in Java).
fn customhud_settings(game: &crate::game::Game<'_>, mod_id: &str, buttons: &mut Vec<String>) {
    use crate::game::customhud as hud;
    if mod_id == hud::MOD_ID {
        let lang = hud::Lang::from_code(&game.settings.language);
        buttons.push(hud::tr("key.customhud.open_config", lang).to_owned());
    }
}
