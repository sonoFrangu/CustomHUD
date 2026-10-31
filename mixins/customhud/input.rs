// Mentre la schermata delle impostazioni è aperta riceve lei mouse e tastiera.
fn customhud_input(
    game: &mut crate::game::Game<'_>,
    event: &winit::event::WindowEvent,
) -> std::ops::ControlFlow<()> {
    crate::game::customhud_screen::handle_event(game, event)
}
