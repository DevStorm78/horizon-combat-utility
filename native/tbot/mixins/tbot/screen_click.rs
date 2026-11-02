fn official_tbot_screen_click(
    game: &crate::game::Game<'_>,
    screen: &crate::game::native_hooks::NativeScreenView<'_>,
    button: i32,
    _modifiers: u32,
    canceled: &mut bool,
) {
    use crate::game::official_tbot as m;
    if button != 0 || game.native_mod_screen() != Some(m::ID) { return; }

    let width = 286.0_f32;
    let rows = m::Settings::read(|key| game.native_setting(m::ID, key)).labels();
    let height = 34.0 + rows.len() as f32 * 13.0;
    let x = ((screen.rect[2] - width) * 0.5).max(4.0);
    let y = ((screen.rect[3] - height) * 0.5).max(4.0);
    let local_x = screen.mouse[0] - x;
    let local_y = screen.mouse[1] - y - 38.0;
    if local_x < 0.0 || local_x > width || local_y < 0.0 {
        return;
    }
    let index = (local_y / 13.0) as usize;
    if index >= rows.len() { return; }

    let mut settings = m::Settings::read(|key| game.native_setting(m::ID, key));
    settings.click(index);
    for (key, value) in settings.stored() {
        game.native_set_setting(m::ID, key, &value);
    }
    *canceled = true;
}
