fn official_tbot_screen_click(
    game: &crate::game::Game<'_>,
    screen: &crate::game::native_hooks::NativeScreenView<'_>,
    button: i32,
    _modifiers: u32,
    canceled: &mut bool,
) {
    use crate::game::official_tbot as m;
    if game.native_mod_screen() != Some(m::ID) { return; }

    // Mouse buttons 1..=4 are available as keybinds while capture is armed.
    if game.native_setting(m::ID, "capture_bind").as_deref() == Some("1") {
        if (1..=4).contains(&button) {
            let bind = format!("BUTTON{}", button + 1);
            game.native_set_setting(m::ID, "bind", &bind);
            game.native_set_setting(m::ID, "capture_bind", "0");
            *canceled = true;
        }
        return;
    }

    if button != 0 && button != 1 { return; }
    let width = 380.0_f32;
    let row_h = 17.0_f32;
    let settings = m::Settings::read(|key| game.native_setting(m::ID, key));
    let rows = settings.labels();
    let height = 56.0 + rows.len() as f32 * row_h;
    let x = ((screen.rect[2] - width) * 0.5).max(4.0);
    let y = ((screen.rect[3] - height) * 0.5).max(4.0);
    let local_x = screen.mouse[0] - x;
    let local_y = screen.mouse[1] - y - 46.0;
    if local_x < 0.0 || local_x > width || local_y < 0.0 {
        return;
    }

    let index = (local_y / row_h) as usize;
    if index >= rows.len() { return; }

    let mut updated = settings;
    if index == 19 && button == 0 {
        game.native_set_setting(m::ID, "capture_bind", "1");
        *canceled = true;
        return;
    }
    if index == 20 {
        if button != 0 { return; }
        updated = m::Settings::default();
        game.native_set_setting(m::ID, "capture_bind", "0");
    } else if updated.slider_fill(index).is_some() {
        let track_x = 286.0_f32;
        let track_w = 82.0_f32;
        if local_x < track_x || local_x > track_x + track_w { return; }
        let ratio = ((local_x - track_x) / track_w).clamp(0.0, 1.0);
        updated.set_slider(index, ratio, button == 1);
    } else {
        if button != 0 { return; }
        updated.click(index);
    }

    for (key, value) in updated.stored() {
        game.native_set_setting(m::ID, key, &value);
    }
    *canceled = true;
}
