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
}

#[derive(Clone)]
pub struct Define<'a> {
    pub name: &'a str,
    pub body: DefineBody<'a>,
}

#[derive(Clone)]
pub enum DefineBody<'a> {
    Text(Vec<SpannedToken<'a>>),
    Function(DefineFunction<'a>),
}

#[derive(Clone)]
pub struct DefineFunction<'a> {
    pub args: Vec<(&'a str, Option<&'a str>)>,
    pub body: Vec<SpannedToken<'a>>,
}

impl<'a> PreprocessConfigs<'a> {
    /// Check whether the given macro is defined
    pub fn is_defined(&self, macro_name: &'a str) -> bool {
        self.defines.iter().any(|d| d.name == macro_name)
    }

    /// Remove a given macro, evaluating to whether a macro was removed
    pub fn undefine(&mut self, macro_name: &'a str) -> bool {
        if let Some(idx) =
            self.defines.iter().position(|d| d.name == macro_name)
        {
            self.defines.remove(idx);
            true
        } else {
            false
        }
    }

    /// Define a new macro
    pub fn define(&mut self, macro_name: &'a str, macro_body: DefineBody<'a>) {
        self.defines.retain(|d| d.name != macro_name);
        self.defines.push(Define {
            name: macro_name,
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
