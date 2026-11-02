//! T-Bot settings and deterministic policy values.
//!
//! This file is both the native bundle's injected game module and its test
//! crate. This bundle currently stores and presents preferences; it does not
//! issue automated player/entity attacks.

pub const ID: &str = "tbot";
pub const NAME: &str = "T-Bot";
pub const VERSION: &str = "0.2.0";
pub const DESCRIPTION: &str = "Configurable combat preferences and timing profile";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TargetMode { Anyone, Players, Mobs }

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SweepMode { Dynamic, Always, Never }

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Window { pub min: f32, pub max: f32 }

impl Window {
    fn parse(value: Option<String>, default: Self, low: f32, high: f32) -> Self {
        let Some(value) = value else { return default };
        let mut parts = value.split(',');
        let a = parts.next().and_then(|v| v.parse::<f32>().ok());
        let b = parts.next().and_then(|v| v.parse::<f32>().ok());
        if parts.next().is_some() { return default; }
        match (a, b) {
            (Some(a), Some(b)) if a.is_finite() && b.is_finite() =>
                Self { min: a.clamp(low, high).min(b.clamp(low, high)), max: a.clamp(low, high).max(b.clamp(low, high)) },
            _ => default,
        }
    }
    fn encode(self) -> String { format!("{:.2},{:.2}", self.min, self.max) }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Settings {
    pub enabled: bool,
    pub target_mode: TargetMode,
    pub range_variability: bool,
    pub range: Window,
    pub sword_cooldown: Window,
    pub axe_cooldown: Window,
    pub reaction_ms: u16,
    pub miss_chance: u8,
    pub miss_range: Window,
    pub smart_crits: bool,
    pub crit_delay_ms: Window,
    pub reduce_sweeps: bool,
    pub sweep_mode: SweepMode,
    pub ignore_shields: bool,
    pub piercing: bool,
    pub players_only: bool,
    pub hit_invisible_players: bool,
    pub weapon_only: bool,
    pub holding_bind_only: bool,
    pub bind: String,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            enabled: false,
            target_mode: TargetMode::Anyone,
            range_variability: true,
            range: Window { min: 3.0, max: 3.0 },
            sword_cooldown: Window { min: 0.89, max: 0.93 },
            axe_cooldown: Window { min: 0.89, max: 0.93 },
            reaction_ms: 0,
            miss_chance: 7,
            miss_range: Window { min: 3.0, max: 3.5 },
            smart_crits: true,
            crit_delay_ms: Window { min: 10.0, max: 10.0 },
            reduce_sweeps: true,
            sweep_mode: SweepMode::Dynamic,
            ignore_shields: false,
            piercing: false,
            players_only: false,
            hit_invisible_players: true,
            weapon_only: false,
            holding_bind_only: true,
            bind: "BUTTON5".into(),
        }
    }
}

impl Settings {
    pub fn read(mut get: impl FnMut(&str) -> Option<String>) -> Self {
        let mut s = Self::default();
        // Unknown persisted values fall back to the default instead of silently disabling a setting.
        let boolean = |v: Option<String>, d| match v.as_deref() {
            Some("1" | "true") => true,
            Some("0" | "false") => false,
            _ => d,
        };
        s.enabled = boolean(get("enabled"), s.enabled);
        s.range_variability = boolean(get("range_variability"), s.range_variability);
        s.smart_crits = boolean(get("smart_crits"), s.smart_crits);
        s.reduce_sweeps = boolean(get("reduce_sweeps"), s.reduce_sweeps);
        s.ignore_shields = boolean(get("ignore_shields"), s.ignore_shields);
        s.piercing = boolean(get("piercing"), s.piercing);
        s.players_only = boolean(get("players_only"), s.players_only);
        s.hit_invisible_players = boolean(get("hit_invisible_players"), s.hit_invisible_players);
        s.weapon_only = boolean(get("weapon_only"), s.weapon_only);
        s.holding_bind_only = boolean(get("holding_bind_only"), s.holding_bind_only);
        s.range = Window::parse(get("range"), s.range, 0.0, 6.0);
        s.sword_cooldown = Window::parse(get("sword_cooldown"), s.sword_cooldown, 0.0, 1.0);
        s.axe_cooldown = Window::parse(get("axe_cooldown"), s.axe_cooldown, 0.0, 1.0);
        s.miss_range = Window::parse(get("miss_range"), s.miss_range, 0.0, 6.0);
        s.crit_delay_ms = Window::parse(get("crit_delay_ms"), s.crit_delay_ms, 0.0, 1000.0);
        s.reaction_ms = get("reaction_ms").and_then(|v| v.parse().ok()).unwrap_or(s.reaction_ms).min(2000);
        s.miss_chance = get("miss_chance")
            .and_then(|v| v.parse::<u16>().ok())
            .unwrap_or(s.miss_chance as u16)
            .min(100) as u8;
        s.target_mode = match get("target_mode").as_deref() { Some("players") => TargetMode::Players, Some("mobs") => TargetMode::Mobs, _ => TargetMode::Anyone };
        s.sweep_mode = match get("sweep_mode").as_deref() { Some("always") => SweepMode::Always, Some("never") => SweepMode::Never, _ => SweepMode::Dynamic };
        s.bind = get("bind")
            .filter(|v| !v.trim().is_empty() && v.len() <= 48 && !v.chars().any(char::is_control))
            .unwrap_or(s.bind);
        s
    }

    pub fn labels(&self) -> Vec<String> {
        vec![
            format!("Module: {}", if self.enabled { "ON" } else { "OFF" }),
            format!("Target Mode: {}", match self.target_mode { TargetMode::Anyone => "Anyone", TargetMode::Players => "Players", TargetMode::Mobs => "Mobs" }),
            format!("Range Variability: {}", on(self.range_variability)),
            format!("Range: {:.2}-{:.2}m", self.range.min, self.range.max),
            format!("Sword Cooldown: {:.2}-{:.2}", self.sword_cooldown.min, self.sword_cooldown.max),
            format!("Axe Cooldown: {:.2}-{:.2}", self.axe_cooldown.min, self.axe_cooldown.max),
            format!("Reaction Time: {}ms", self.reaction_ms),
            format!("Miss Chance: {}%", self.miss_chance),
            format!("Miss Range: {:.2}-{:.2}m", self.miss_range.min, self.miss_range.max),
            format!("Smart Crits: {}", on(self.smart_crits)),
            format!("Crit Delay: {:.0}-{:.0}ms", self.crit_delay_ms.min, self.crit_delay_ms.max),
            format!("Reduce Sweeps: {}", on(self.reduce_sweeps)),
            format!("Reduce Sweeps Mode: {}", match self.sweep_mode { SweepMode::Dynamic => "Dynamic", SweepMode::Always => "Always", SweepMode::Never => "Never" }),
            format!("Ignore Shields: {}", on(self.ignore_shields)),
            format!("Piercing: {}", on(self.piercing)),
            format!("Players Only: {}", on(self.players_only)),
            format!("Hit Invisible Players: {}", on(self.hit_invisible_players)),
            format!("Weapon Only: {}", on(self.weapon_only)),
            format!("Holding Bind Only: {}", on(self.holding_bind_only)),
            format!("Hold Bind: {}  [click to capture]", display_bind(&self.bind)),
            "Reset All Settings".into(),
        ]
    }

    /// Toggle or cycle non-numeric preferences from the custom panel.
    /// Numeric preferences are changed through `set_slider`.
    pub fn click(&mut self, i: usize) {
        match i {
            0 => self.enabled = !self.enabled,
            1 => self.target_mode = match self.target_mode { TargetMode::Anyone => TargetMode::Players, TargetMode::Players => TargetMode::Mobs, TargetMode::Mobs => TargetMode::Anyone },
            2 => self.range_variability = !self.range_variability,
            3..=8 => {},

            9 => self.smart_crits = !self.smart_crits,
            10 => {},
            11 => self.reduce_sweeps = !self.reduce_sweeps,
            12 => self.sweep_mode = match self.sweep_mode { SweepMode::Dynamic => SweepMode::Always, SweepMode::Always => SweepMode::Never, SweepMode::Never => SweepMode::Dynamic },
            13 => self.ignore_shields = !self.ignore_shields,
            14 => self.piercing = !self.piercing,
            15 => self.players_only = !self.players_only,
            16 => self.hit_invisible_players = !self.hit_invisible_players,
            17 => self.weapon_only = !self.weapon_only,
            18 => self.holding_bind_only = !self.holding_bind_only,
            19 => {}, // The screen hook starts key capture for this row.

            20 => *self = Self::default(),
            _ => {}
        }
    }

    /// Normalized fill coordinates for the custom screen's slider bars.
    pub fn slider_fill(&self, i: usize) -> Option<(f32, f32)> {
        let (low, high, a, b) = match i {
            3 => (0.0, 6.0, self.range.min, self.range.max),
            4 => (0.0, 1.0, self.sword_cooldown.min, self.sword_cooldown.max),
            5 => (0.0, 1.0, self.axe_cooldown.min, self.axe_cooldown.max),
            6 => (0.0, 2000.0, 0.0, self.reaction_ms as f32),
            7 => (0.0, 100.0, 0.0, self.miss_chance as f32),
            8 => (0.0, 6.0, self.miss_range.min, self.miss_range.max),
            10 => (0.0, 1000.0, self.crit_delay_ms.min, self.crit_delay_ms.max),
            _ => return None,
        };
        let span = high - low;
        Some((((a - low) / span).clamp(0.0, 1.0), ((b - low) / span).clamp(0.0, 1.0)))
    }

    /// Left-click changes the lower/primary value; right-click changes the upper value.
    pub fn set_slider(&mut self, i: usize, ratio: f32, upper: bool) {
        let ratio = if ratio.is_finite() { ratio.clamp(0.0, 1.0) } else { 0.0 };
        match i {
            3 => set_window(&mut self.range, 6.0, ratio, upper, 0.05),
            4 => set_window(&mut self.sword_cooldown, 1.0, ratio, upper, 0.01),
            5 => set_window(&mut self.axe_cooldown, 1.0, ratio, upper, 0.01),
            6 => self.reaction_ms = quantize(2000.0 * ratio, 25.0) as u16,
            7 => self.miss_chance = quantize(100.0 * ratio, 1.0) as u8,
            8 => set_window(&mut self.miss_range, 6.0, ratio, upper, 0.05),
            10 => set_window(&mut self.crit_delay_ms, 1000.0, ratio, upper, 5.0),
            _ => {}
        }
    }

    pub fn stored(&self) -> Vec<(&'static str, String)> {
        vec![
            ("enabled", b(self.enabled)), ("target_mode", match self.target_mode { TargetMode::Anyone => "anyone", TargetMode::Players => "players", TargetMode::Mobs => "mobs" }.into()),
            ("range_variability", b(self.range_variability)), ("range", self.range.encode()),
            ("sword_cooldown", self.sword_cooldown.encode()), ("axe_cooldown", self.axe_cooldown.encode()),
            ("reaction_ms", self.reaction_ms.to_string()), ("miss_chance", self.miss_chance.to_string()),
            ("miss_range", self.miss_range.encode()), ("smart_crits", b(self.smart_crits)),
            ("crit_delay_ms", self.crit_delay_ms.encode()), ("reduce_sweeps", b(self.reduce_sweeps)),
            ("sweep_mode", match self.sweep_mode { SweepMode::Dynamic => "dynamic", SweepMode::Always => "always", SweepMode::Never => "never" }.into()),
            ("ignore_shields", b(self.ignore_shields)), ("piercing", b(self.piercing)),
            ("players_only", b(self.players_only)), ("hit_invisible_players", b(self.hit_invisible_players)),
            ("weapon_only", b(self.weapon_only)), ("holding_bind_only", b(self.holding_bind_only)),
            ("bind", self.bind.clone()),
        ]
    }
}

fn on(v: bool) -> &'static str { if v { "ON" } else { "OFF" } }
fn b(v: bool) -> String { if v { "1" } else { "0" }.into() }

fn quantize(value: f32, step: f32) -> f32 {
    (value / step).round() * step
}

fn set_window(window: &mut Window, maximum: f32, ratio: f32, upper: bool, step: f32) {
    let value = quantize(maximum * ratio, step).clamp(0.0, maximum);
    if upper {
        window.max = value.max(window.min);
    } else {
        window.min = value.min(window.max);
    }
}

fn display_bind(bind: &str) -> String {
    match bind {
        "BUTTON2" => "Mouse 2".into(),
        "BUTTON3" => "Mouse 3".into(),
        "BUTTON4" => "Mouse 4".into(),
        "BUTTON5" => "Mouse 5".into(),
        value if value.starts_with("key.keyboard.") =>
            value.trim_start_matches("key.keyboard.").replace('.', " ").to_ascii_uppercase(),
        _ => bind.to_owned(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_match_the_reference_panel() {
        let s = Settings::default();
        assert_eq!(s.target_mode, TargetMode::Anyone);
        assert_eq!(s.range, Window { min: 3.0, max: 3.0 });
        assert_eq!(s.sword_cooldown, Window { min: 0.89, max: 0.93 });
        assert_eq!(s.axe_cooldown, Window { min: 0.89, max: 0.93 });
        assert_eq!(s.miss_chance, 7);
        assert_eq!(s.miss_range, Window { min: 3.0, max: 3.5 });
        assert_eq!(s.sweep_mode, SweepMode::Dynamic);
        assert_eq!(s.bind, "BUTTON5");
    }

    #[test]
    fn settings_round_trip() {
        let original = Settings::default();
        let values = original.stored();
        let restored = Settings::read(|key| values.iter().find(|(k, _)| *k == key).map(|(_, v)| v.clone()));
        assert_eq!(restored, original);
    }

    #[test]
    fn windows_are_clamped_and_sorted() {
        let w = Window::parse(Some("9.0,-1.0".into()), Window {min:1.0,max:2.0}, 0.0, 6.0);
        assert_eq!(w, Window { min: 0.0, max: 6.0 });
    }

    #[test]
    fn malformed_persisted_values_fall_back_to_defaults() {
        let s = Settings::read(|key| match key {
            "enabled" => Some("perhaps".into()),
            "range" => Some("NaN,4.0".into()),
            "reaction_ms" => Some("-1".into()),
            "miss_chance" => Some("999".into()),
            "bind" => Some("".into()),
            _ => None,
        });
        assert!(!s.enabled);
        assert_eq!(s.range, Settings::default().range);
        assert_eq!(s.reaction_ms, 0);
        assert_eq!(s.miss_chance, 100);
        assert_eq!(s.bind, "BUTTON5");
    }

    #[test]
    fn slider_values_are_bounded_and_keep_windows_ordered() {
        let mut s = Settings::default();
        s.set_slider(3, 0.25, false);
        assert!(s.range.min <= s.range.max);
        assert!((s.range.min - 1.5).abs() < 0.001);
        s.set_slider(3, 0.75, true);
        assert!((s.range.max - 4.5).abs() < 0.001);
        s.set_slider(7, 0.42, false);
        assert_eq!(s.miss_chance, 42);
        s.set_slider(6, 1.0, false);
        assert_eq!(s.reaction_ms, 2000);
        assert!(s.slider_fill(4).is_some());
    }

    #[test]
    fn extra_window_fields_are_rejected() {
        let w = Window::parse(Some("1.0,2.0,3.0".into()), Window { min: 4.0, max: 5.0 }, 0.0, 6.0);
        assert_eq!(w, Window { min: 4.0, max: 5.0 });
    }

    #[test]
    fn bind_rejects_control_characters_and_oversized_values() {
        let control = Settings::read(|key| (key == "bind").then(|| "KEY\\nINJECT".into()));
        let oversized = Settings::read(|key| (key == "bind").then(|| "X".repeat(49)));
        assert_eq!(control.bind, "BUTTON5");
        assert_eq!(oversized.bind, "BUTTON5");
    }
}
