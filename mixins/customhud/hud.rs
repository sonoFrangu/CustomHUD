// Ogni frame dell'HUD (non sotto F1): contatore, orologio e barra.
fn draw_customhud(game: &mut crate::game::Game<'_>, batch: &mut horizon_client::hud::HudBatch) {
    crate::game::customhud_screen::draw_hud(game, batch);
}
