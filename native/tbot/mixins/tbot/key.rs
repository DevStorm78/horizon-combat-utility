fn official_tbot_key(game: &mut crate::game::Game<'_>, key: &str, pressed: bool) {
    use crate::game::official_tbot as m;

    // The server's Game is a different thread; never let it alter client key state.
    if !game.native_window() {
        return;
    }
    if !game.native_playing() || game.player.is_none() {
        m::close_menu();
        m::set_bind_down(false);
        return;
    }

    let previous = m::settings(|name| game.native_setting(m::ID, name));
    let mut settings = previous.clone();
    let handled = m::handle_menu_key(key, pressed, &mut settings);
    if settings != previous {
        m::set_settings(settings.clone());
        for (name, value) in settings.stored() {
            game.native_set_setting(m::ID, name, &value);
        }
        m::set_bind_down(false);
    }
    if handled {
        if m::menu_open() {
            m::set_bind_down(false);
        }
        return;
    }

    if key == m::MENU_KEY {
        return;
    }
    if !game.native_on(m::ID) {
        m::set_bind_down(false);
        return;
    }
    if key == settings.activation_key {
        m::set_bind_down(pressed);
    }
}
