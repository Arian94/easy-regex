pub mod error_handling;
pub mod literal_exp_mod;
pub mod group_mod;
pub mod list_mod;
pub mod settings_mod;

#[derive(Debug, Clone)]
pub struct MetaFuncRegex(String);

impl MetaFuncRegex {
    pub fn new(raw: String) -> Self {
        MetaFuncRegex(raw)
    }

    pub fn new_section() -> Self {
        MetaFuncRegex(String::new())
    }

    pub fn get_regex(self) -> String {
        self.0
    }
}