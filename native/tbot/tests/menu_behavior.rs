use horizon_mod_tbot::{MenuState, Settings, should_attack};

fn open_menu(menu: &mut MenuState, settings: &mut Settings) {
    menu.handle_key("key.keyboard.right.shift", true, settings);
}

#[test]
fn right_shift_opens_and_closes_the_in_game_menu() {
    let mut menu = MenuState::default();
    let mut settings = Settings::default();
    open_menu(&mut menu, &mut settings);
    assert!(menu.is_open());
    menu.handle_key("key.keyboard.right.shift", true, &mut settings);
    assert!(!menu.is_open());
}

#[test]
fn activation_key_defaults_to_mouse_button_five_and_can_be_captured() {
    let mut menu = MenuState::default();
    let mut settings = Settings::default();
    assert_eq!(settings.activation_key, "key.mouse.5");
    open_menu(&mut menu, &mut settings);
    menu.handle_key("key.keyboard.enter", true, &mut settings);
    assert!(menu.is_capturing_key());
    menu.handle_key("key.mouse.4", true, &mut settings);
    assert_eq!(settings.activation_key, "key.mouse.4");
    assert!(!menu.is_capturing_key());
}

#[test]
fn escape_cancels_activation_key_capture_without_changing_the_binding() {
    let mut menu = MenuState::default();
    let mut settings = Settings::default();
    open_menu(&mut menu, &mut settings);
    menu.handle_key("key.keyboard.enter", true, &mut settings);
    menu.handle_key("key.keyboard.escape", true, &mut settings);
    assert_eq!(settings.activation_key, "key.mouse.5");
    assert!(!menu.is_capturing_key());
    assert!(menu.is_open());
}

#[test]
fn menu_navigation_changes_existing_settings_and_persists_key_choice() {
    let mut menu = MenuState::default();
    let mut settings = Settings::default();
    open_menu(&mut menu, &mut settings);
    menu.handle_key("key.keyboard.down", true, &mut settings);
    assert_eq!(menu.selected_index(), 1);
    menu.handle_key("key.keyboard.enter", true, &mut settings);
    assert_eq!(settings.target_mode, horizon_mod_tbot::TargetMode::Players);
    let stored = settings.stored();
    let restored = Settings::read(|key| {
        stored
            .iter()
            .find(|(name, _)| *name == key)
            .map(|(_, value)| value.clone())
    });
    assert_eq!(restored.activation_key, settings.activation_key);
}

#[test]
fn open_menu_suppresses_automatic_attacks() {
    let settings = Settings::default();
    assert!(!should_attack(
        true, true, true, true, &settings, false, 2.0, 1.0
    ));
    assert!(should_attack(
        true, true, true, false, &settings, false, 2.0, 1.0
    ));
}
