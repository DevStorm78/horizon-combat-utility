fn official_tbot_screen_render(
    game: &crate::game::Game<'_>,
    batch: &mut horizon_client::hud::HudBatch,
    screen: &crate::game::native_hooks::NativeScreenView<'_>,
) {
    use crate::game::official_tbot as m;
    if game.native_mod_screen() != Some(m::ID) { return; }

    let width = 286.0_f32;
    let row_h = 13.0_f32;
    let rows = m::Settings::read(|key| game.native_setting(m::ID, key)).labels();
    let height = 34.0 + rows.len() as f32 * row_h;
    let x = ((screen.rect[2] - width) * 0.5).max(4.0);
    let y = ((screen.rect[3] - height) * 0.5).max(4.0);

    game.native_fill(batch, [x - 1.0, y - 1.0, width + 2.0, height + 2.0], 0xE00A0D13);
    game.native_fill(batch, [x, y, width, 22.0], 0xFF202A3A);
    game.native_text(batch, "T-BOT  /  COMBAT SETTINGS", [x + 10.0, y + 7.0], 0xFFFFC857, true);
    game.native_text(batch, "Right Shift: close  |  Click a row to change", [x + 10.0, y + 27.0], 0xFF9AA8BA, false);

    for (i, label) in rows.iter().enumerate() {
        let ry = y + 40.0 + i as f32 * row_h;
        let hovered = screen.mouse[0] >= x && screen.mouse[0] <= x + width
            && screen.mouse[1] >= ry - 2.0 && screen.mouse[1] < ry + row_h - 2.0;
        if hovered {
            game.native_fill(batch, [x + 3.0, ry - 2.0, width - 6.0, row_h], 0xFF34465C);
        } else if i % 2 == 0 {
            game.native_fill(batch, [x + 3.0, ry - 2.0, width - 6.0, row_h], 0x40232D3B);
        }
        game.native_text(batch, label, [x + 9.0, ry], 0xFFE6EDF5, false);
    }
}
