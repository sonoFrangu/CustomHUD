// Quando la schermata delle impostazioni è aperta, la disegna al posto di questa.
fn draw_customhud_screen(
    game: &mut crate::game::Game<'_>,
    batch: &mut horizon_client::hud::HudBatch,
) -> std::ops::ControlFlow<()> {
    crate::game::customhud_screen::draw_screen(game, batch)
}
