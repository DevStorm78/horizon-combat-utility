use std::cell::{Cell, RefCell};

pub const ID: &str = "tbot";
pub const NAME: &str = "T-Bot";
pub const VERSION: &str = "0.4.0";
pub const DESCRIPTION: &str = "Hold a key to attack the entity under your crosshair.";
pub const MENU_KEY: &str = "key.keyboard.right.shift";
pub const DEFAULT_KEY: &str = "key.mouse.5";
const ESCAPE_KEY: &str = "key.keyboard.escape";
const MENU_ENTRIES: usize = 6;

thread_local! {
    /// The activation key belongs to the client thread, not the integrated-server thread.
    static BIND_DOWN: Cell<bool> = const { Cell::new(false) };
    /// Settings are loaded once per game thread and refreshed immediately after menu changes.
    static CACHED_SETTINGS: RefCell<Option<Settings>> = const { RefCell::new(None) };
    static MENU: RefCell<MenuState> = const { RefCell::new(MenuState::new()) };
}

pub fn bind_down() -> bool {
    BIND_DOWN.with(Cell::get)
}

pub fn set_bind_down(down: bool) {
    BIND_DOWN.with(|state| state.set(down));
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TargetMode {
    Anyone,
    Players,
    Mobs,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Settings {
    pub activation_key: String,
    pub target_mode: TargetMode,
    /// Maximum crosshair distance, in blocks. Horizon's normal survival reach is 3.
    pub max_range: f32,
    /// Minimum attack-strength charge, between 0.50 and 1.00.
    pub min_charge: f32,
    /// Require the configured activation key to remain held.
    pub hold_to_activate: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            activation_key: DEFAULT_KEY.into(),
            target_mode: TargetMode::Anyone,
            max_range: 3.0,
            min_charge: 0.90,
            hold_to_activate: true,
        }
    }
}

impl Settings {
    pub fn read(mut get: impl FnMut(&str) -> Option<String>) -> Self {
        let defaults = Self::default();
        let activation_key = get("activation_key")
            .filter(|value| valid_activation_key(value))
            .unwrap_or_else(|| defaults.activation_key.clone());
        let target_mode = match get("target_mode").as_deref() {
            Some("players") => TargetMode::Players,
            Some("mobs") => TargetMode::Mobs,
            _ => TargetMode::Anyone,
        };
        let parse_number = |value: Option<String>, default: f32, low: f32, high: f32| {
            value
                .and_then(|value| value.parse::<f32>().ok())
                .filter(|value| value.is_finite())
                .map_or(default, |value| value.clamp(low, high))
        };
        let parse_bool = |value: Option<String>, default: bool| match value.as_deref() {
            Some("1" | "true") => true,
            Some("0" | "false") => false,
            _ => default,
        };

        Self {
            activation_key,
            target_mode,
            max_range: parse_number(get("max_range"), defaults.max_range, 1.5, 3.0),
            min_charge: parse_number(get("min_charge"), defaults.min_charge, 0.5, 1.0),
            hold_to_activate: parse_bool(get("hold_to_activate"), defaults.hold_to_activate),
        }
    }

    pub fn labels(&self) -> Vec<String> {
        vec![
            format!("Activation Key: {}", display_key(&self.activation_key)),
            format!(
                "Target Mode: {}",
                match self.target_mode {
                    TargetMode::Anyone => "Anyone",
                    TargetMode::Players => "Players",
                    TargetMode::Mobs => "Mobs",
                }
            ),
            format!("Maximum Range: {:.1} blocks", self.max_range),
            format!("Minimum Charge: {:.0}%", self.min_charge * 100.0),
            format!(
                "Hold to Activate: {}",
                if self.hold_to_activate { "ON" } else { "OFF" }
            ),
            "Reset Settings".into(),
        ]
    }

    pub fn click(&mut self, index: usize) {
        self.change(index, true);
    }

    pub fn change(&mut self, index: usize, forward: bool) {
        match index {
            0 => {
                self.target_mode = match (self.target_mode, forward) {
                    (TargetMode::Anyone, true) | (TargetMode::Mobs, false) => TargetMode::Players,
                    (TargetMode::Players, true) => TargetMode::Mobs,
                    (TargetMode::Players, false) => TargetMode::Anyone,
                    (TargetMode::Mobs, true) | (TargetMode::Anyone, false) => TargetMode::Anyone,
                };
            }
            1 => {
                const RANGES: [f32; 4] = [1.5, 2.0, 2.5, 3.0];
                let current = RANGES
                    .iter()
                    .position(|value| (*value - self.max_range).abs() < 0.001)
                    .unwrap_or(RANGES.len() - 1);
                let next = if forward {
                    (current + 1) % RANGES.len()
                } else {
                    (current + RANGES.len() - 1) % RANGES.len()
                };
                self.max_range = RANGES[next];
            }
            2 => {
                const CHARGES: [f32; 6] = [0.5, 0.6, 0.7, 0.8, 0.9, 1.0];
                let current = CHARGES
                    .iter()
                    .position(|value| (*value - self.min_charge).abs() < 0.001)
                    .unwrap_or(4);
                let next = if forward {
                    (current + 1) % CHARGES.len()
                } else {
                    (current + CHARGES.len() - 1) % CHARGES.len()
                };
                self.min_charge = CHARGES[next];
            }
            3 => self.hold_to_activate = !self.hold_to_activate,
            4 if forward => *self = Self::default(),
            _ => {}
        }
    }

    pub fn stored(&self) -> [(&'static str, String); 5] {
        [
            ("activation_key", self.activation_key.clone()),
            (
                "target_mode",
                match self.target_mode {
                    TargetMode::Anyone => "anyone",
                    TargetMode::Players => "players",
                    TargetMode::Mobs => "mobs",
                }
                .into(),
            ),
            ("max_range", format!("{:.1}", self.max_range)),
            ("min_charge", format!("{:.2}", self.min_charge)),
            (
                "hold_to_activate",
                if self.hold_to_activate { "1" } else { "0" }.into(),
            ),
        ]
    }
}

fn valid_activation_key(key: &str) -> bool {
    if key.len() > 64 || key == MENU_KEY || key == ESCAPE_KEY || !key.is_ascii() {
        return false;
    }
    if let Some(name) = key.strip_prefix("key.keyboard.") {
        return !name.is_empty()
            && name.chars().all(|character| {
                character.is_ascii_lowercase()
                    || character.is_ascii_digit()
                    || "._".contains(character)
            });
    }
    key.strip_prefix("key.mouse.").is_some_and(|name| {
        !name.is_empty()
            && name
                .chars()
                .all(|character| character.is_ascii_alphanumeric())
    })
}

pub fn display_key(key: &str) -> String {
    match key {
        "key.mouse.left" => "Mouse 1 (Left)".into(),
        "key.mouse.right" => "Mouse 2 (Right)".into(),
        "key.mouse.middle" => "Mouse 3 (Middle)".into(),
        "key.mouse.4" => "Mouse 4 (Back)".into(),
        "key.mouse.5" => "Mouse 5 (Forward)".into(),
        _ => key
            .strip_prefix("key.keyboard.")
            .or_else(|| key.strip_prefix("key.mouse."))
            .unwrap_or(key)
            .replace('.', " ")
            .to_ascii_uppercase(),
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MenuState {
    open: bool,
    selected_index: usize,
    capturing_key: bool,
}

impl MenuState {
    pub const fn new() -> Self {
        Self {
            open: false,
            selected_index: 0,
            capturing_key: false,
        }
    }

    pub fn is_open(&self) -> bool {
        self.open
    }

    pub fn selected_index(&self) -> usize {
        self.selected_index
    }

    pub fn is_capturing_key(&self) -> bool {
        self.capturing_key
    }

    /// Returns true when the key event belongs to the in-game menu.
    pub fn handle_key(&mut self, key: &str, pressed: bool, settings: &mut Settings) -> bool {
        if key == MENU_KEY && pressed {
            self.capturing_key = false;
            self.open = !self.open;
            return true;
        }
        if !self.open {
            return false;
        }
        if !pressed {
            return true;
        }
        if self.capturing_key {
            if key == ESCAPE_KEY {
                self.capturing_key = false;
            } else if valid_activation_key(key) {
                settings.activation_key = key.to_owned();
                self.capturing_key = false;
            }
            return true;
        }
        match key {
            ESCAPE_KEY => {
                self.open = false;
                self.capturing_key = false;
            }
            "key.keyboard.up" => {
                self.selected_index = (self.selected_index + MENU_ENTRIES - 1) % MENU_ENTRIES;
            }
            "key.keyboard.down" => {
                self.selected_index = (self.selected_index + 1) % MENU_ENTRIES;
            }
            "key.keyboard.left" => {
                if self.selected_index > 0 {
                    settings.change(self.selected_index - 1, false);
                }
            }
            "key.keyboard.right" | "key.keyboard.enter" => {
                if self.selected_index == 0 {
                    self.capturing_key = true;
                } else {
                    settings.change(self.selected_index - 1, true);
                }
            }
            _ => {}
        }
        true
    }
}

impl Default for MenuState {
    fn default() -> Self {
        Self::new()
    }
}

pub fn handle_menu_key(key: &str, pressed: bool, settings: &mut Settings) -> bool {
    MENU.with(|menu| menu.borrow_mut().handle_key(key, pressed, settings))
}

pub fn menu_state() -> MenuState {
    MENU.with(|menu| menu.borrow().clone())
}

pub fn menu_open() -> bool {
    MENU.with(|menu| menu.borrow().is_open())
}

pub fn close_menu() {
    MENU.with(|menu| *menu.borrow_mut() = MenuState::new());
}

/// Load settings on the first game tick; later ticks only read the in-memory copy.
pub fn settings(get: impl FnMut(&str) -> Option<String>) -> Settings {
    if let Some(settings) = CACHED_SETTINGS.with(|cache| cache.borrow().clone()) {
        return settings;
    }
    let settings = Settings::read(get);
    CACHED_SETTINGS.with(|cache| *cache.borrow_mut() = Some(settings.clone()));
    settings
}

pub fn set_settings(settings: Settings) {
    CACHED_SETTINGS.with(|cache| *cache.borrow_mut() = Some(settings));
}

#[allow(clippy::too_many_arguments)]
pub fn should_attack(
    module_on: bool,
    in_world: bool,
    bind_down: bool,
    menu_open: bool,
    settings: &Settings,
    target_is_player: bool,
    distance: f64,
    attack_strength: f32,
) -> bool {
    let target_allowed = match settings.target_mode {
        TargetMode::Anyone => true,
        TargetMode::Players => target_is_player,
        TargetMode::Mobs => !target_is_player,
    };

    module_on
        && in_world
        && !menu_open
        && (!settings.hold_to_activate || bind_down)
        && target_allowed
        && distance.is_finite()
        && distance >= 0.0
        && distance <= f64::from(settings.max_range)
        && attack_strength.is_finite()
        && attack_strength >= settings.min_charge
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_are_safe_and_hold_to_activate() {
        let settings = Settings::default();
        assert_eq!(settings.target_mode, TargetMode::Anyone);
        assert_eq!(settings.max_range, 3.0);
        assert_eq!(settings.min_charge, 0.90);
        assert!(settings.hold_to_activate);
        assert!(!should_attack(
            true, true, false, false, &settings, false, 2.0, 1.0
        ));
        assert!(should_attack(
            true, true, true, false, &settings, false, 2.0, 1.0
        ));
    }

    #[test]
    fn module_and_world_gates_are_required() {
        let settings = Settings {
            hold_to_activate: false,
            ..Settings::default()
        };
        assert!(!should_attack(
            false, true, false, false, &settings, false, 2.0, 1.0
        ));
        assert!(!should_attack(
            true, false, false, false, &settings, false, 2.0, 1.0
        ));
        assert!(should_attack(
            true, true, false, false, &settings, false, 2.0, 1.0
        ));
    }

    #[test]
    fn target_mode_filters_players_and_mobs() {
        let mut settings = Settings {
            hold_to_activate: false,
            ..Settings::default()
        };
        settings.target_mode = TargetMode::Players;
        assert!(should_attack(
            true, true, false, false, &settings, true, 2.0, 1.0
        ));
        assert!(!should_attack(
            true, true, false, false, &settings, false, 2.0, 1.0
        ));
        settings.target_mode = TargetMode::Mobs;
        assert!(should_attack(
            true, true, false, false, &settings, false, 2.0, 1.0
        ));
        assert!(!should_attack(
            true, true, false, false, &settings, true, 2.0, 1.0
        ));
    }

    #[test]
    fn range_and_attack_charge_are_enforced() {
        let settings = Settings {
            hold_to_activate: false,
            ..Settings::default()
        };
        assert!(should_attack(
            true, true, false, false, &settings, false, 3.0, 0.90
        ));
        assert!(!should_attack(
            true, true, false, false, &settings, false, 3.01, 1.0
        ));
        assert!(!should_attack(
            true, true, false, false, &settings, false, 2.0, 0.89
        ));
        assert!(!should_attack(
            true,
            true,
            false,
            false,
            &settings,
            false,
            f64::NAN,
            1.0
        ));
        assert!(!should_attack(
            true,
            true,
            false,
            false,
            &settings,
            false,
            2.0,
            f32::NAN
        ));
    }

    #[test]
    fn hold_release_prevents_triggering() {
        let settings = Settings::default();
        assert!(should_attack(
            true, true, true, false, &settings, false, 2.0, 1.0
        ));
        assert!(!should_attack(
            true, true, false, false, &settings, false, 2.0, 1.0
        ));
    }

    #[test]
    fn settings_round_trip_and_invalid_values_fall_back_safely() {
        let original = Settings::default();
        let stored = original.stored();
        let restored = Settings::read(|key| {
            stored
                .iter()
                .find(|(name, _)| *name == key)
                .map(|(_, value)| value.clone())
        });
        assert_eq!(restored, original);

        let invalid = Settings::read(|key| match key {
            "target_mode" => Some("unknown".into()),
            "max_range" => Some("NaN".into()),
            "min_charge" => Some("4.0".into()),
            "hold_to_activate" => Some("maybe".into()),
            _ => None,
        });
        assert_eq!(invalid.max_range, 3.0);
        assert_eq!(invalid.min_charge, 1.0);
        assert!(invalid.hold_to_activate);
    }

    #[test]
    fn setting_buttons_change_only_supported_behaviour() {
        let mut settings = Settings::default();
        settings.click(0);
        assert_eq!(settings.target_mode, TargetMode::Players);
        settings.click(1);
        assert_eq!(settings.max_range, 1.5);
        settings.click(2);
        assert_eq!(settings.min_charge, 1.0);
        settings.click(3);
        assert!(!settings.hold_to_activate);
        settings.click(4);
        assert_eq!(settings, Settings::default());
        assert_eq!(settings.labels().len(), 6);
    }

    #[test]
    fn cached_settings_change_without_another_disk_read() {
        set_settings(Settings {
            max_range: 2.5,
            ..Settings::default()
        });
        let result = settings(|_| panic!("cached settings should not read storage"));
        assert_eq!(result.max_range, 2.5);
        set_settings(Settings::default());
    }

    #[test]
    fn activation_state_is_thread_local() {
        set_bind_down(true);
        assert!(bind_down());
        set_bind_down(false);
        assert!(!bind_down());
    }
}
