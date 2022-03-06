use crate::MetaFuncRegex;

impl MetaFuncRegex {
    pub fn start_of_line() -> MetaFuncRegex {
        MetaFuncRegex("^".to_string())
    }

    pub fn or(self) -> MetaFuncRegex {
        let result = format!("|{}", self.0);
        MetaFuncRegex(result)
    }

    pub fn not(self, expression: &str) -> MetaFuncRegex {
        let result = format!("{}[^{}]", self.0, expression);
        MetaFuncRegex(result)
    }

    pub fn end_of_line(self) -> MetaFuncRegex {
        MetaFuncRegex("$".to_string())
    }
}