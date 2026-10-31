// Prima delle bossbar vanilla: ricorda da dove partono i loro vertici, così
// `extras.rs` le può spostare sotto l'HUD quando è in alto.
fn mark_boss_bars(batch: &horizon_client::hud::HudBatch) {
    crate::game::customhud::mark_boss_bars(batch.vertices.len());
}
