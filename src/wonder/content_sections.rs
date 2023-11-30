#[derive(PartialEq, Default)]
pub enum ContentSections {
    #[default] History,
    Construction,
    Geography
}

impl ContentSections {
    pub fn starts_at(&self) -> f64 {
        match self {
            ContentSections::History => 0.0,
            ContentSections::Construction => 1600.0,
            ContentSections::Geography => 2700.0,
        }
    }
}