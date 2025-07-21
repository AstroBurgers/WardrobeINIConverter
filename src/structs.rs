#[derive(Clone)]
pub struct CompCombo {
    pub comp_name: &'static str, // Interned string
    pub comp_id: i16,
    pub comp_tex: i8,
}

impl Default for CompCombo {
    fn default() -> Self {
        Self {
            comp_name: "",
            comp_id: 0,
            comp_tex: 0,
        }
    }
}

pub struct Entry {
    pub entry_name: String,
    pub combos: Vec<CompCombo>,
    pub gender: String,
}