use std::cell::{Cell, RefCell};

pub const ID: &str = "tbot";
pub const NAME: &str = "T-Bot";
pub const VERSION: &str = "0.4.0";
pub const DESCRIPTION: &str = "Hold a key to attack the entity under your crosshair.";
pub const KEY: &str = "activate";
pub const DEFAULT_KEY: &str = "key.keyboard.right.alt";

thread_local! {
    /// The activation key belongs to the client thread, not the integrated-server thread.
    static BIND_DOWN: Cell<bool> = const { Cell::new(false) };
    /// Settings are loaded once per game thread and refreshed immediately after menu changes.
    static CACHED_SETTINGS: RefCell<Option<Settings>> = const { RefCell::new(None) };
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
    pub target_mode: TargetMode,
    /// Maximum crosshair distance, in blocks. Horizon's normal survival reach is 3.
    pub max_range: f32,
    /// Minimum attack-strength charge, between 0.50 and 1.00.
    pub min_charge: f32,
    /// Require the registered activation key to remain held.
    pub hold_to_activate: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
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
            target_mode,
            max_range: parse_number(get("max_range"), defaults.max_range, 1.5, 3.0),
            min_charge: parse_number(get("min_charge"), defaults.min_charge, 0.5, 1.0),
            hold_to_activate: parse_bool(get("hold_to_activate"), defaults.hold_to_activate),
        }
    }

    pub fn labels(&self) -> Vec<String> {
        vec![
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
        match index {
            0 => {
                self.target_mode = match self.target_mode {
                    TargetMode::Anyone => TargetMode::Players,
                    TargetMode::Players => TargetMode::Mobs,
                    TargetMode::Mobs => TargetMode::Anyone,
                };
            }
            1 => {
                const RANGES: [f32; 4] = [1.5, 2.0, 2.5, 3.0];
                let current = RANGES
                    .iter()
                    .position(|value| (*value - self.max_range).abs() < 0.001)
                    .unwrap_or(RANGES.len() - 1);
                self.max_range = RANGES[(current + 1) % RANGES.len()];
            }
            2 => {
                const CHARGES: [f32; 6] = [0.5, 0.6, 0.7, 0.8, 0.9, 1.0];
                let current = CHARGES
                    .iter()
                    .position(|value| (*value - self.min_charge).abs() < 0.001)
                    .unwrap_or(4);
                self.min_charge = CHARGES[(current + 1) % CHARGES.len()];
            }
            3 => self.hold_to_activate = !self.hold_to_activate,
            4 => *self = Self::default(),
            _ => {}
        }
    }

    pub fn stored(&self) -> [(&'static str, String); 4] {
        [
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

pub fn should_attack(
    module_on: bool,
    in_world: bool,
    bind_down: bool,
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

    fn get_from(values: &[(&str, &str)], key: &str) -> Option<String> {
        values
            .iter()
            .find(|(name, _)| *name == key)
            .map(|(_, value)| (*value).to_owned())
    }

    #[test]
    fn defaults_are_safe_and_hold_to_activate() {
        let settings = Settings::default();
        assert_eq!(settings.target_mode, TargetMode::Anyone);
        assert_eq!(settings.max_range, 3.0);
        assert_eq!(settings.min_charge, 0.90);
        assert!(settings.hold_to_activate);
        assert!(!should_attack(true, true, false, &settings, false, 2.0, 1.0));
        assert!(should_attack(true, true, true, &settings, false, 2.0, 1.0));
    }

    #[test]
    fn module_and_world_gates_are_required() {
        let settings = Settings { hold_to_activate: false, ..Settings::default() };
        assert!(!should_attack(false, true, false, &settings, false, 2.0, 1.0));
        assert!(!should_attack(true, false, false, &settings, false, 2.0, 1.0));
        assert!(should_attack(true, true, false, &settings, false, 2.0, 1.0));
    }

    #[test]
    fn target_mode_filters_players_and_mobs() {
        let mut settings = Settings { hold_to_activate: false, ..Settings::default() };
        settings.target_mode = TargetMode::Players;
        assert!(should_attack(true, true, false, &settings, true, 2.0, 1.0));
        assert!(!should_attack(true, true, false, &settings, false, 2.0, 1.0));
        settings.target_mode = TargetMode::Mobs;
        assert!(should_attack(true, true, false, &settings, false, 2.0, 1.0));
        assert!(!should_attack(true, true, false, &settings, true, 2.0, 1.0));
    }

    #[test]
    fn range_and_attack_charge_are_enforced() {
        let settings = Settings { hold_to_activate: false, ..Settings::default() };
        assert!(should_attack(true, true, false, &settings, false, 3.0, 0.90));
        assert!(!should_attack(true, true, false, &settings, false, 3.01, 1.0));
        assert!(!should_attack(true, true, false, &settings, false, 2.0, 0.89));
        assert!(!should_attack(true, true, false, &settings, false, f64::NAN, 1.0));
        assert!(!should_attack(true, true, false, &settings, false, 2.0, f32::NAN));
    }

    #[test]
    fn hold_release_prevents_triggering() {
        let settings = Settings::default();
        assert!(should_attack(true, true, true, &settings, false, 2.0, 1.0));
        assert!(!should_attack(true, true, false, &settings, false, 2.0, 1.0));
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
        assert_eq!(settings.labels().len(), 5);
    }

    #[test]
    fn cached_settings_change_without_another_disk_read() {
        set_settings(Settings { max_range: 2.5, ..Settings::default() });
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
