fn official_tbot_hud(
    game: &mut crate::game::Game<'_>,
    batch: &mut horizon_client::hud::HudBatch,
    corners: &mut horizon_client::extras_hud::Corners,
) {
    use crate::game::official_tbot as m;
    use horizon_client::extras::Corner;

    if !game.native_window()
        || !game.native_playing()
        || !game.native_corner_shown(Corner::TopLeft)
        || !m::menu_open()
    {
        return;
    }

    let menu = m::menu_state();
    let settings = m::settings(|key| game.native_setting(m::ID, key));
    let labels = settings.labels();
    let (x, y) = corners.place(Corner::TopLeft, 310.0, 170.0);

    game.native_fill(batch, [x, y, 310.0, 170.0], 0xD9000000);
    game.native_fill(batch, [x, y, 310.0, 24.0], 0xFF315B88);
    game.native_text(batch, "T-Bot Settings", [x + 10.0, y + 7.0], 0xFFFFFF, true);

    for (index, label) in labels.iter().enumerate() {
        let row_y = y + 32.0 + index as f32 * 16.0;
        if index == menu.selected_index() {
            game.native_fill(batch, [x + 6.0, row_y - 2.0, 298.0, 14.0], 0x805C9EFF);
        }
        let line = if index == menu.selected_index() {
            format!("> {label}")
        } else {
            format!("  {label}")
        };
        game.native_text(batch, &line, [x + 10.0, row_y], 0xFFFFFF, true);
    }

    let help = if menu.is_capturing_key() {
        "Press a key or mouse button; Esc cancels"
    } else {
        "Arrows move/change; Enter selects; Right Shift closes"
    };
    game.native_text(batch, help, [x + 8.0, y + 152.0], 0xD4D4D4, true);
}
