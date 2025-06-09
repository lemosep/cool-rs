use crate::semantic::errors::SemanticError;

#[derive(Debug, Default)]
pub struct ErrorCollector {
    pub errors: Vec<SemanticError>,
}

impl ErrorCollector {
    pub fn add(&mut self, err: SemanticError) {
        self.errors.push(err);
    }

    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    pub fn report_all(&self) {
        for e in &self.errors {
            eprintln!("{}", e);
        }
    }
}