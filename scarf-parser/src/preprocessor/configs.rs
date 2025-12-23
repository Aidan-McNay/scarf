// =======================================================================
// configs.rs
// =======================================================================
// Configurations for preprocessing

use crate::*;
use std::path::{Path, PathBuf};

#[derive(Default, Clone)]
pub struct PreprocessConfigs<'a> {
    includes: Vec<&'a Path>,
    defines: Vec<Define<'a>>,
    pub curr_standard: StandardVersion,
    pub in_define: bool,
    pub in_define_arg: bool,
}

#[derive(Clone)]
pub struct SpannedString<'a>(pub &'a str, pub Span<'a>);

#[derive(Clone)]
pub struct Define<'a> {
    pub name: SpannedString<'a>,
    pub body: DefineBody<'a>,
}

impl<'a> Define<'a> {
    pub fn is_from_command_line(&self) -> bool {
        self.name.1.file == ""
    }
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
            Span<'a>, // =
            Vec<SpannedToken<'a>>,
        )>,
    )>,
    pub body: Option<Vec<SpannedToken<'a>>>,
}

impl<'a> DefineBody<'a> {
    pub fn get_tokens(&self) -> (Vec<SpannedToken<'a>>, bool) {
        match self {
            DefineBody::Empty() => (vec![], false),
            DefineBody::Text(token_vec) => (token_vec.clone(), false),
            DefineBody::Function(def_func) => match &def_func.body {
                Some(token_vec) => (token_vec.clone(), true),
                None => (vec![], true),
            },
        }
    }
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
        macro_span: Span<'a>,
        macro_body: DefineBody<'a>,
    ) {
        self.defines.retain(|d| d.name.0 != macro_name);
        self.defines.push(Define {
            name: SpannedString(macro_name, macro_span),
            body: macro_body,
        });
    }

    /// Define a new macro from the command line
    pub fn command_line_define(
        &mut self,
        macro_name: &'a str,
        macro_text: Option<Vec<SpannedToken<'a>>>,
    ) {
        self.define(
            macro_name,
            Span::default(),
            match macro_text {
                None => DefineBody::Empty(),
                Some(token_vec) => DefineBody::Text(token_vec),
            },
        )
    }

    /// Undefine all macros
    pub fn undefineall(&mut self) {
        self.defines = vec![];
    }

    /// Get the tokens for a macro replacement
    pub fn get_macro_tokens(
        &self,
        macro_name: &'a str,
    ) -> Option<(Vec<SpannedToken<'a>>, bool)> {
        for define in &self.defines {
            if define.name.0 == macro_name {
                return Some(define.body.get_tokens());
            }
        }
        None
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
