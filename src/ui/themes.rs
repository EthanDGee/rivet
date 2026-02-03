use ratatui::style::Color;

pub struct ColorPalette {
    pub title: Color,
    pub outer_border: Color,
    pub inner_border: Color,
    pub header_text: Color,
    pub body_text: Color,
    pub highlight: Color,
    pub background: Color,
}

#[allow(dead_code)]
impl ColorPalette {
    pub fn tokyo_night() -> Self {
        Self {
            title: Color::from_u32(0xff9e64),        // orange
            outer_border: Color::from_u32(0xbb9af7), // purple
            inner_border: Color::from_u32(0x7aa2f7), // blue
            header_text: Color::from_u32(0x9ece6a),  // green
            body_text: Color::from_u32(0xc0caf5),    // foreground
            highlight: Color::from_u32(0x73daca),    // cyan
            background: Color::from_u32(0x1a1b26),   // dark
        }
    }

    pub fn catppuccin_mocha() -> Self {
        Self {
            title: Color::from_u32(0xf5e0dc),        // rosewater
            outer_border: Color::from_u32(0x8aadf4), // blue
            inner_border: Color::from_u32(0xf38ba8), // red
            header_text: Color::from_u32(0xa6e3a1),  // green
            body_text: Color::from_u32(0xcdd6f4),    // text
            highlight: Color::from_u32(0x94e2d5),    // teal
            background: Color::from_u32(0x1e1e2e),   // base
        }
    }

    pub fn dracula() -> Self {
        Self {
            title: Color::from_u32(0xffb86c),        // orange
            outer_border: Color::from_u32(0xbd93f9), // purple
            inner_border: Color::from_u32(0xff79c6), // pink
            header_text: Color::from_u32(0x50fa7b),  // green
            body_text: Color::from_u32(0xf8f8f2),    // foreground
            highlight: Color::from_u32(0x8be9fd),    // cyan
            background: Color::from_u32(0x282a36),   // background
        }
    }

    pub fn gruvbox() -> Self {
        Self {
            title: Color::from_u32(0xd65d0e),        // orange
            outer_border: Color::from_u32(0x458588), // blue
            inner_border: Color::from_u32(0xb16286), // purple
            header_text: Color::from_u32(0x98971a),  // green
            body_text: Color::from_u32(0xebdbb2),    // foreground
            highlight: Color::from_u32(0x689d6a),    // aqua
            background: Color::from_u32(0x282828),   // dark0
        }
    }

    pub fn solarized_dark() -> Self {
        Self {
            title: Color::from_u32(0xcb4b16),        // orange
            outer_border: Color::from_u32(0x268bd2), // blue
            inner_border: Color::from_u32(0x6c71c4), // violet
            header_text: Color::from_u32(0x859900),  // green
            body_text: Color::from_u32(0x839496),    // base0
            highlight: Color::from_u32(0x2aa198),    // cyan
            background: Color::from_u32(0x002b36),   // base03
        }
    }

    pub fn nord() -> Self {
        Self {
            title: Color::from_u32(0xD08770),        // orange
            outer_border: Color::from_u32(0x81A1C1), // blue
            inner_border: Color::from_u32(0xB48EAD), // purple
            header_text: Color::from_u32(0xA3BE8C),  // green
            body_text: Color::from_u32(0xECEFF4),    // foreground
            highlight: Color::from_u32(0x8FBCBB),    // cyan
            background: Color::from_u32(0x2E3440),   // polar night
        }
    }

    pub fn monokai() -> Self {
        Self {
            title: Color::from_u32(0xFD971F),        // orange
            outer_border: Color::from_u32(0xAE81FF), // purple
            inner_border: Color::from_u32(0xF92672), // pink
            header_text: Color::from_u32(0xA6E22E),  // green
            body_text: Color::from_u32(0xF8F8F2),    // foreground
            highlight: Color::from_u32(0x66D9EF),    // cyan
            background: Color::from_u32(0x272822),   // background
        }
    }
}
