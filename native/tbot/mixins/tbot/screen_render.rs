fn official_tbot_screen_render(
    game: &crate::game::Game<'_>,
    batch: &mut horizon_client::hud::HudBatch,
    screen: &crate::game::native_hooks::NativeScreenView<'_>,
) {
    use crate::game::official_tbot as m;
    if game.native_mod_screen() != Some(m::ID) { return; }

    let width = 380.0_f32;
    let row_h = 17.0_f32;
    let rows = m::Settings::read(|key| game.native_setting(m::ID, key)).labels();
    let height = 56.0 + rows.len() as f32 * row_h;
    let x = ((screen.rect[2] - width) * 0.5).max(4.0);
    let y = ((screen.rect[3] - height) * 0.5).max(4.0);

    game.native_fill(batch, [x - 1.0, y - 1.0, width + 2.0, height + 2.0], 0xE00A0D13);
    game.native_fill(batch, [x, y, width, 22.0], 0xFF202A3A);
    game.native_text(batch, "T-BOT  /  COMBAT SETTINGS", [x + 10.0, y + 7.0], 0xFFFFC857, true);
    let capturing = game.native_setting(m::ID, "capture_bind").as_deref() == Some("1");
    let subtitle = if capturing {
        "Press a keyboard or mouse button  |  Esc cancels"
    } else {
        "Right Shift / Esc: close  |  L/R click sliders"
    };
    game.native_text(batch, subtitle, [x + 10.0, y + 28.0], 0xFF9AA8BA, false);

    let settings = m::Settings::read(|key| game.native_setting(m::ID, key));

    for (i, label) in rows.iter().enumerate() {
        let ry = y + 48.0 + i as f32 * row_h;
        let hovered = screen.mouse[0] >= x && screen.mouse[0] <= x + width
            && screen.mouse[1] >= ry - 2.0 && screen.mouse[1] < ry + row_h - 2.0;
        if hovered {
            game.native_fill(batch, [x + 3.0, ry - 2.0, width - 6.0, row_h], 0xFF34465C);
        } else if i % 2 == 0 {
            game.native_fill(batch, [x + 3.0, ry - 2.0, width - 6.0, row_h], 0x40232D3B);
        }
        game.native_text(batch, label, [x + 9.0, ry + 1.0], 0xFFE6EDF5, false);
        if let Some((start, end)) = settings.slider_fill(i) {
            let track_x = x + 286.0;
            let track_y = ry + 6.0;
            let track_w = 82.0;
            game.native_fill(batch, [track_x, track_y, track_w, 4.0], 0xFF354253);
            let left = start.min(end);
            let right = start.max(end);
            if right > left {
                game.native_fill(
                    batch,
                    [track_x + left * track_w, track_y, (right - left) * track_w, 4.0],
                    0xFFFFC857,
                );
            } else {
                game.native_fill(
                    batch,
                    [track_x + left * track_w - 1.0, track_y - 2.0, 2.0, 8.0],
                    0xFFFFC857,
                );
            }
        }
    }
}
