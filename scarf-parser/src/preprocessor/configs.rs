// =======================================================================
// configs.rs
// =======================================================================
// Configurations for preprocessing

use crate::*;
use std::path::{Path, PathBuf};

#[derive(Default, Clone)]
pub struct PreprocessConfigs<'a> {
    pub includes: Vec<&'a Path>,
    pub defines: Vec<Define<'a>>,
    pub in_define: bool,
}

#[derive(Clone)]
pub struct SpannedString<'a>(pub &'a str, pub Span);

#[derive(Clone)]
pub struct Define<'a> {
    pub name: SpannedString<'a>,
    pub body: DefineBody<'a>,
}

#[derive(Clone)]
pub enum DefineBody<'a> {
    Empty(),
    Text(Vec<SpannedToken<'a>>),
    Function(DefineFunction<'a>),
}

#[derive(Clone)]
pub struct DefineFunction<'a> {
    pub args: Vec<(
        SpannedString<'a>,
        Option<(
            Span, // =
            SpannedString<'a>,
        )>,
    )>,
    pub body: Option<Vec<SpannedToken<'a>>>,
}

impl<'a> PreprocessConfigs<'a> {
    /// Check whether the given macro is defined
    pub fn is_defined(&self, macro_name: &'a str) -> bool {
        self.defines.iter().any(|d| d.name.0 == macro_name)
    }

    /// Remove a given macro, evaluating to whether a macro was removed
    pub fn undefine(&mut self, macro_name: &'a str) -> bool {
        if let Some(idx) =
            self.defines.iter().position(|d| d.name.0 == macro_name)
        {
            self.defines.remove(idx);
            true
        } else {
            false
        }
    }

    /// Define a new macro
    pub fn define(
        &mut self,
        macro_name: &'a str,
        macro_span: Span,
        macro_body: DefineBody<'a>,
    ) {
        self.defines.retain(|d| d.name.0 != macro_name);
        self.defines.push(Define {
            name: SpannedString(macro_name, macro_span),
            body: macro_body,
        });
    }

    /// Undefine all macros
    pub fn undefineall(&mut self) {
        self.defines = vec![];
    }

    /// Get the full path from an include statement
    pub fn get_file_path(&self, include_path: &str) -> Option<PathBuf> {
        for dir_path in &self.includes {
            let full_path = Path::new(dir_path).join(include_path);
            if full_path.exists() {
                return Some(full_path);
            }
        }
        return None;
    }
}
