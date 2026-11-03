fn official_tbot_hud(
    game: &mut crate::game::Game<'_>,
    batch: &mut horizon_client::hud::HudBatch,
) {
    use crate::game::official_tbot as m;

    if !game.native_window() || !game.native_playing() || !m::menu_open() {
        return;
    }

    let menu = m::menu_state();
    let settings = m::settings(|key| game.native_setting(m::ID, key));
    let labels = settings.labels();

    game.native_fill(batch, [12.0, 12.0, 330.0, 170.0], 0xD9000000);
    game.native_fill(batch, [12.0, 12.0, 330.0, 24.0], 0xFF315B88);
    game.native_text(batch, "T-Bot Settings", [22.0, 19.0], 0xFFFFFF, true);

    for (index, label) in labels.iter().enumerate() {
        let y = 44.0 + index as f32 * 16.0;
        if index == menu.selected_index() {
            game.native_fill(batch, [18.0, y - 2.0, 318.0, 14.0], 0x805C9EFF);
        }
        let line = if index == menu.selected_index() {
            format!("> {label}")
        } else {
            format!("  {label}")
        };
        game.native_text(batch, &line, [22.0, y], 0xFFFFFF, true);
    }

    let help = if menu.is_capturing_key() {
        "Press a key or mouse button; Esc cancels"
    } else {
        "Up/Down: select   Left/Right: change   Enter: choose   Right Shift: close"
    };
    game.native_text(batch, help, [20.0, 158.0], 0xD4D4D4, true);
}
