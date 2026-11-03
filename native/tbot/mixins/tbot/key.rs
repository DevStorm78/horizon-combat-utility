fn official_tbot_key(game: &mut crate::game::Game<'_>, key: &str, pressed: bool) {
    use crate::game::official_tbot as m;

    // The server's Game is a different thread; never let it alter client key state.
    if !game.native_window() {
        return;
    }

    let bind = game.native_mod_key(m::ID, m::KEY, m::DEFAULT_KEY);
    if key == bind {
        let active = pressed
            && game.native_on(m::ID)
            && game.native_playing()
            && game.player.is_some();
        m::set_bind_down(active);
    } else if !game.native_playing() || !game.native_on(m::ID) {
        m::set_bind_down(false);
    }
}
