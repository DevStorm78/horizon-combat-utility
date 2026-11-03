fn official_tbot_tick(game: &mut crate::game::Game<'_>) {
    use crate::game::official_tbot as m;

    // native_tick runs for the window Game and the integrated/dedicated server Game.
    // The server-side Game must never synthesize a local input event.
    if !game.native_window() {
        return;
    }

    let in_world = game.native_playing() && game.player.is_some();
    let module_on = game.native_on(m::ID);
    if !in_world {
        m::close_menu();
        m::set_bind_down(false);
        return;
    }
    if !module_on {
        m::set_bind_down(false);
        return;
    }
    if m::menu_open() {
        m::set_bind_down(false);
        return;
    }

    let settings = m::settings(|key| game.native_setting(m::ID, key));
    let Some((target_index, distance)) = game.mob_hit() else {
        return;
    };
    let Some(target) = game.mobs.get(target_index) else {
        return;
    };
    // Require the exact living entity to be Horizon's actual crosshair pick.
    // This also checks block occlusion and prevents a nearer boat, frame or
    // projectile from receiving an attack intended for the mob behind it.
    let Some(Some(picked_id)) = game.pick_entity_hit() else {
        return;
    };
    if picked_id != target.id as i32 {
        return;
    }

    let target_is_player = &*target.kind.id == "minecraft:player";

    if !m::should_attack(
        module_on,
        in_world,
        m::bind_down(),
        m::menu_open(),
        &settings,
        target_is_player,
        distance,
        game.hand_strength
            .scale(0.5, game.main_hand_attack_speed()),
    ) {
        return;
    }

    if game.remote.is_some() {
        // Multiplayer and Horizon's default singleplayer client both use the
        // remote input path. Queue a real one-shot attack click so Horizon sends
        // its normal ServerboundAttack + swing packets on the next client tick.
        // Bypass ToggleAttack only for this synthetic edge, then restore the
        // user's actual held/toggled attack state.
        let previous_left = game.left;
        let previous_toggle_state = game.toggles.0[2].clone();
        let previous_toggle_option = game.settings.toggle_attack;

        game.settings.toggle_attack = false;
        let _ = game.remote_mouse(winit::event::MouseButton::Left, true);
        let _ = game.remote_mouse(winit::event::MouseButton::Left, false);

        game.settings.toggle_attack = previous_toggle_option;
        game.toggles.0[2] = previous_toggle_state;
        game.left = previous_left;
    } else {
        // Local/offline Game: use Horizon's normal in-process attack implementation.
        let _ = game.attack();
    }
}
