fn official_tbot_key(game: &mut crate::game::Game<'_>, key: &str, pressed: bool) {
    use crate::game::official_tbot as m;

    // Track both press and release. This is deliberately transient state:
    // it is not part of Settings::stored(), so it cannot survive a restart.
    if key == "key.keyboard.right.shift" && pressed {
        game.native_set_setting(m::ID, "bind_down", "0");
    }

    // Don't let the key used to operate the settings screen activate gameplay.
    if game.native_mod_screen() != Some(m::ID) {
        let bind = game.native_setting(m::ID, "bind")
            .unwrap_or_else(|| "key.keyboard.right.alt".to_owned());
        if key == bind {
            game.native_set_setting(m::ID, "bind_down", if pressed { "1" } else { "0" });
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
