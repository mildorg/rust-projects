use stylist::Style;

/// create a `Style` via stylist
pub fn create_style(style: &str) -> Style {
    Style::new(style).expect("Failed to create style")
}

/// create multi `Style` via stylist
pub fn create_styles(styles: Vec<&str>) -> Vec<Style> {
    styles.iter().map(|&style| create_style(style)).collect()
}
