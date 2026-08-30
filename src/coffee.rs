// SPDX-License-Identifier: GPL-3.0-only

//! The coffee easter eggs: a `#coffee` tag wears a cup, the hidden "Long
//! Black" theme, and Ctrl+Shift+Enter — a braille-dot cup with steam that
//! lights up like an 80s Tokyo neon sign.

/// Tags that are really about coffee (the leaf of the tag path).
pub fn is_coffee_tag(tag: &str) -> bool {
    let leaf = tag.rsplit('/').next().unwrap_or(tag).to_ascii_lowercase();
    matches!(
        leaf.as_str(),
        "coffee"
            | "espresso"
            | "latte"
            | "flatwhite"
            | "flat-white"
            | "cappuccino"
            | "cortado"
            | "macchiato"
            | "americano"
            | "longblack"
            | "long-black"
            | "mocha"
            | "brew"
            | "cafe"
            | "café"
            | "barista"
            | "beans"
    )
}

/// The cup with its steam, four frames of the steam drifting. Braille
/// cells (2×4 dots) so the drawing stays crisp at any size.
pub const FRAMES: [&str; 4] = [
    "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢤⡀⠀⠀⢠⡄⢀⡤\n⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⡇⢀⡴⠋⢰⡏\n⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⡴⠋⢠⡟⠀⠀⠈⢷⡀\n⠀⠀⠀⠀⠀⠀⠀⠀⢀⡿⠀⠀⠈⢳⡄⠀⠀⠀⠙⣦\n⠀⠀⠀⠀⢀⣤⣤⣤⣤⣤⣤⣤⣤⣤⣤⣤⣤⣤⣤⣤⣤⣤⣤⣴⢦⣤⡀\n⠀⠀⠀⠀⠀⢿⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣸⠇⠀⠈⢻⡆\n⠀⠀⠀⠀⠀⢸⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣿⠀⠀⠀⢸⡗\n⠀⠀⠀⠀⠀⠀⣿⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⡇⢀⣠⣴⠟⠁\n⠀⠀⠀⠀⠀⠀⠙⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠁⠈⠁\n⠀⠀⠛⠛⠿⠿⠿⠿⠿⠿⠿⠿⠿⠿⠿⠿⠿⠿⠿⠿⠿⠿⠿⠿⠿⠛⠛",
    "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣠⠄⣠⠄⠀⢠⡄\n⠀⠀⠀⠀⠀⠀⠀⠀⠀⣠⠞⠁⢸⡇⠀⠀⠈⠳⣄\n⠀⠀⠀⠀⠀⠀⠀⠀⢰⡏⠀⠀⠀⠻⣄⠀⠀⠀⠈⢷⡀\n⠀⠀⠀⠀⠀⠀⠀⠀⠀⠻⣄⠀⠀⠀⠈⣷⠀⠀⢀⡾⠁\n⠀⠀⠀⠀⢀⣤⣤⣤⣤⣤⣤⣤⣤⣤⣤⣤⣤⣤⣤⣤⣤⣤⣤⣴⢦⣤⡀\n⠀⠀⠀⠀⠀⢿⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣸⠇⠀⠈⢻⡆\n⠀⠀⠀⠀⠀⢸⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣿⠀⠀⠀⢸⡗\n⠀⠀⠀⠀⠀⠀⣿⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⡇⢀⣠⣴⠟⠁\n⠀⠀⠀⠀⠀⠀⠙⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠁⠈⠁\n⠀⠀⠛⠛⠿⠿⠿⠿⠿⠿⠿⠿⠿⠿⠿⠿⠿⠿⠿⠿⠿⠿⠿⠿⠿⠛⠛",
    "⠀⠀⠀⠀⠀⠀⠀⠀⢀⡤⠀⠀⢠⡄⠀⠀⠀⠀⢤⡀\n⠀⠀⠀⠀⠀⠀⠀⠀⢸⡇⠀⠀⠀⠙⢦⡀⠀⠀⠀⢹⡆\n⠀⠀⠀⠀⠀⠀⠀⠀⠀⠙⢦⡀⠀⠀⠀⢻⡄⠀⢀⡾⠁\n⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢿⡀⠀⢠⡞⠁⣴⠋\n⠀⠀⠀⠀⢀⣤⣤⣤⣤⣤⣤⣤⣤⣤⣤⣤⣤⣤⣤⣤⣤⣤⣤⣴⢦⣤⡀\n⠀⠀⠀⠀⠀⢿⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣸⠇⠀⠈⢻⡆\n⠀⠀⠀⠀⠀⢸⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣿⠀⠀⠀⢸⡗\n⠀⠀⠀⠀⠀⠀⣿⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⡇⢀⣠⣴⠟⠁\n⠀⠀⠀⠀⠀⠀⠙⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠁⠈⠁\n⠀⠀⠛⠛⠿⠿⠿⠿⠿⠿⠿⠿⠿⠿⠿⠿⠿⠿⠿⠿⠿⠿⠿⠿⠿⠛⠛",
    "⠀⠀⠀⠀⠀⠀⠀⠀⠠⣄⠀⠀⠀⠀⠠⣄⠀⠀⠀⢠⡄\n⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠳⣄⠀⠀⠀⢸⡇⠀⣠⠞⠁\n⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢹⡆⠀⣠⠟⢀⡾⠁\n⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣠⠟⠀⣾⠁⠀⠈⢷⡀\n⠀⠀⠀⠀⢀⣤⣤⣤⣤⣤⣤⣤⣤⣤⣤⣤⣤⣤⣤⣤⣤⣤⣤⣴⢦⣤⡀\n⠀⠀⠀⠀⠀⢿⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣸⠇⠀⠈⢻⡆\n⠀⠀⠀⠀⠀⢸⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣿⠀⠀⠀⢸⡗\n⠀⠀⠀⠀⠀⠀⣿⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⡇⢀⣠⣴⠟⠁\n⠀⠀⠀⠀⠀⠀⠙⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠛⠁⠈⠁\n⠀⠀⠛⠛⠿⠿⠿⠿⠿⠿⠿⠿⠿⠿⠿⠿⠿⠿⠿⠿⠿⠿⠿⠿⠿⠛⠛",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn coffee_tags() {
        assert!(is_coffee_tag("coffee"));
        assert!(is_coffee_tag("work/espresso"));
        assert!(is_coffee_tag("Flat-White"));
        assert!(!is_coffee_tag("tea"));
        assert!(!is_coffee_tag("coffeetable"));
    }

    #[test]
    fn frames_share_a_shape() {
        let rows: Vec<usize> = FRAMES.iter().map(|f| f.lines().count()).collect();
        assert!(rows.iter().all(|&r| r == rows[0]));
        assert!(
            FRAMES[0]
                .chars()
                .any(|c| ('\u{2800}'..='\u{28ff}').contains(&c))
        );
    }
}
