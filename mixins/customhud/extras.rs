// Dopo le bossbar e prima di minimappa e HUD delle mod: le sposta sotto l'HUD.
fn shift_boss_bars(game: &crate::game::Game<'_>, batch: &mut horizon_client::hud::HudBatch) {
    crate::game::customhud_screen::shift_boss_bars(game, batch);
}
