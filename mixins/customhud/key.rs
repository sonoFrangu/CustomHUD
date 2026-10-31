// Il tasto scelto in Options > Mods (H di default) apre le impostazioni della mod,
// come `CONFIG_KEY` in Java.
fn open_customhud_settings(game: &mut crate::game::Game<'_>, key: &str, pressed: bool) {
    use crate::game::customhud as hud;
    if !pressed || !game.native_on(hud::MOD_ID) || !game.native_playing() {
        return;
    }
    if key == game.native_mod_key(hud::MOD_ID, hud::CONFIG_KEY, hud::CONFIG_KEY_DEFAULT) {
        crate::game::customhud_screen::open_screen(game, hud::Parent::Game);
    }
}
