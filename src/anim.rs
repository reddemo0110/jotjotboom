// SPDX-License-Identifier: GPL-3.0-only

//! UI movement: values glide toward their target instead of jumping.
//!
//! Two knobs, both user-tunable: how long the whole trip takes, and how
//! gently it brakes into place at the end (the "landing").

/// Trip time in milliseconds. 0 = snap instantly.
pub const DEFAULT_MS: u16 = 250;
pub const MAX_MS: u16 = 600;
/// One click of the settings stepper.
pub const STEP_MS: u16 = 10;

/// Landing softness in tenths: the ease-out exponent ×10. 10 = linear
/// (no slow-down), higher = a longer, floatier final descent.
pub const DEFAULT_EASE10: u16 = 30;
pub const MIN_EASE10: u16 = 10;
pub const MAX_EASE10: u16 = 60;
pub const STEP_EASE10: u16 = 5;

/// Parse the trip time from config: a number in ms, or one of the old
/// preset keys (off / fast / normal / slow) from before it was numeric.
pub fn ms_from_key(key: &str) -> u16 {
    match key.trim() {
        "" | "normal" => DEFAULT_MS,
        "off" => 0,
        "fast" => 150,
        "slow" => 400,
        n => n.parse::<u16>().map_or(DEFAULT_MS, |v| v.min(MAX_MS)),
    }
}

/// Parse the landing softness (tenths) from config.
pub fn ease_from_key(key: &str) -> u16 {
    key.trim()
        .parse::<u16>()
        .map_or(DEFAULT_EASE10, |v| v.clamp(MIN_EASE10, MAX_EASE10))
}

/// The pair `Glide` runs on: trip time in seconds and the ease-out
/// exponent. `None` = animation off, snap.
pub fn params(ms: u16, ease10: u16) -> Option<(f32, f32)> {
    (ms > 0).then(|| (f32::from(ms) / 1000.0, f32::from(ease10) / 10.0))
}

/// A vertical position gliding toward its target: a fixed-time trip with
/// a power ease-out, so it sets off quickly and brakes into place.
#[derive(Debug, Clone, Copy, Default)]
pub struct Glide {
    pub pos: f32,
    start: f32,
    target: f32,
    /// Seconds into the current trip.
    t: f32,
    /// Mid-flight: a frame tick is wanted.
    pub live: bool,
    /// The value has appeared and `pos` means something.
    shown: bool,
}

impl Glide {
    /// Head for `target`. The first target after a `clear` is taken
    /// directly (nothing to glide from); `None` params snap too. A
    /// retarget restarts the trip from wherever the glide is now.
    pub fn to(&mut self, target: f32, params: Option<(f32, f32)>) {
        if self.shown && (self.target - target).abs() < 0.5 {
            return;
        }
        self.target = target;
        if !self.shown || params.is_none() {
            self.pos = target;
            self.live = false;
        } else {
            self.start = self.pos;
            self.t = 0.0;
            self.live = (self.pos - target).abs() >= 0.5;
        }
        self.shown = true;
    }

    /// The glide has a meaningful position to draw at.
    pub fn shown(&self) -> bool {
        self.shown
    }

    /// The indicator is gone; the next `to` starts fresh.
    pub fn clear(&mut self) {
        self.shown = false;
        self.live = false;
    }

    /// Advance by `dt` seconds. Returns whether it is still moving.
    pub fn step(&mut self, dt: f32, params: Option<(f32, f32)>) -> bool {
        let Some((duration, ease)) = params.filter(|(d, _)| *d > 0.0) else {
            self.pos = self.target;
            self.live = false;
            return false;
        };
        if !self.live {
            return false;
        }
        self.t += dt;
        let u = (self.t / duration).min(1.0);
        let eased = 1.0 - (1.0 - u).powf(ease.max(1.0));
        self.pos = self.start + (self.target - self.start) * eased;
        if u >= 1.0 {
            self.pos = self.target;
            self.live = false;
        }
        self.live
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const P: Option<(f32, f32)> = Some((0.1, 3.0));

    #[test]
    fn first_target_is_taken_directly() {
        let mut g = Glide::default();
        g.to(40.0, P);
        assert_eq!(g.pos, 40.0);
        assert!(!g.live);
    }

    #[test]
    fn retarget_glides_and_settles_in_trip_time() {
        let mut g = Glide::default();
        g.to(0.0, P);
        g.to(100.0, P);
        assert!(g.live);
        assert_eq!(g.pos, 0.0);
        let mut frames = 0;
        while g.step(1.0 / 60.0, P) {
            frames += 1;
            assert!(frames < 10, "0.1s trip should settle in ~6 frames");
        }
        assert_eq!(g.pos, 100.0);
    }

    #[test]
    fn landing_brakes_harder_with_higher_ease() {
        // Halfway through the trip, a higher exponent has covered more
        // ground (it front-loads the speed, saving the crawl for the end).
        let mut soft = Glide::default();
        let mut linear = Glide::default();
        for g in [&mut soft, &mut linear] {
            g.to(0.0, P);
            g.to(100.0, P);
        }
        soft.step(0.05, Some((0.1, 4.0)));
        linear.step(0.05, Some((0.1, 1.0)));
        assert!(soft.pos > linear.pos + 20.0, "{} vs {}", soft.pos, linear.pos);
    }

    #[test]
    fn off_snaps_and_keys_parse() {
        let mut g = Glide::default();
        g.to(0.0, None);
        g.to(100.0, None);
        assert_eq!(g.pos, 100.0);
        assert!(!g.live);
        assert!(params(0, 30).is_none());
        assert_eq!(params(250, 30), Some((0.25, 3.0)));
        assert_eq!(ms_from_key("off"), 0);
        assert_eq!(ms_from_key("120"), 120);
        assert_eq!(ease_from_key(""), DEFAULT_EASE10);
        assert_eq!(ease_from_key("45"), 45);
        assert_eq!(ease_from_key("99"), MAX_EASE10);
    }
}
