fn official_tbot_key(game: &mut crate::game::Game<'_>, key: &str, pressed: bool) {
    use crate::game::official_tbot as m;

    use std::sync::atomic::Ordering;

    // Track press and release in memory; never perform filesystem I/O per key event.
    if key == "key.keyboard.right.shift" && pressed {
        m::HOLD_BIND_DOWN.store(false, Ordering::Relaxed);
    }

    // Don't let the key used to operate the settings screen activate gameplay.
    if game.native_mod_screen() != Some(m::ID) {
        let bind = game.native_setting(m::ID, "bind")
            .unwrap_or_else(|| "key.keyboard.right.alt".to_owned());
        if key == bind {
            m::HOLD_BIND_DOWN.store(pressed, Ordering::Relaxed);
        }
    }

    if !pressed || key != "key.keyboard.right.shift" { return; }
    if game.native_mod_screen() == Some(m::ID) {
        game.native_mod_screen_close(m::ID);
    } else {
        // A previous session may have ended while bind capture was active.
        game.native_set_setting(m::ID, "capture_bind", "0");
        game.native_mod_screen_open(m::ID);
    }
}
