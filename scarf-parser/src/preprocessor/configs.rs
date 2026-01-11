// =======================================================================
// configs.rs
// =======================================================================
// Configurations for preprocessing

use crate::*;
use scarf_syntax::*;
use std::path::{Path, PathBuf};

const DEFAULT_TIMESCALE: Timescale = Timescale::new(
    Span::empty(),
    (TimescaleValue::One, TimescaleUnit::NS),
    (TimescaleValue::One, TimescaleUnit::NS),
);

const DEFAULT_NETTYPE: DefaultNettype = DefaultNettype::Wire;

const DEFAULT_UNCONNECTED_DRIVE: UnconnectedDrive =
    UnconnectedDrive::NoUnconnected;

#[derive(Default, Clone)]
pub struct PreprocessConfigs<'a> {
    includes: Vec<&'a Path>,
    defines: Vec<Define<'a>>,
    timescales: Vec<Timescale<'a>>,
    default_nettypes: Vec<(DefaultNettype, Span<'a>)>,
    unconnected_drives: Vec<(UnconnectedDrive, Span<'a>)>,
    cell_defines: Vec<(bool, Span<'a>)>,
    included_files: Vec<(Box<str>, Box<str>)>,
    included_spans: Vec<Span<'a>>,
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
    /// Reset all resetable configs
    pub fn reset_all(&mut self, reset_all_span: Span<'a>) {
        self.add_timescale(
            reset_all_span.clone(),
            DEFAULT_TIMESCALE.unit,
            DEFAULT_TIMESCALE.precision,
        );
        self.add_default_nettype(reset_all_span.clone(), DEFAULT_NETTYPE);
        self.add_unconnected_drive(
            reset_all_span.clone(),
            DEFAULT_UNCONNECTED_DRIVE,
        );
        self.add_cell_define(false, reset_all_span);
    }

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
        Some(PathBuf::from(include_path))
    }

    /// Add a compiler directive timescale
    pub fn add_timescale(
        &mut self,
        def_span: Span<'a>,
        unit: (TimescaleValue, TimescaleUnit),
        precision: (TimescaleValue, TimescaleUnit),
    ) {
        self.timescales
            .push(Timescale::new(def_span, unit, precision));
    }

    /// Get the correct compiler timescale, based on a span
    pub fn get_timescale(&self, span: &Span<'a>) -> &Timescale<'a> {
        for timescale in self.timescales.iter().rev() {
            if timescale.is_valid(span) {
                return timescale;
            }
        }
        // Default timescale
        &DEFAULT_TIMESCALE
    }

    /// Add a compiler directive default nettype
    pub fn add_default_nettype(
        &mut self,
        def_span: Span<'a>,
        default_nettype: DefaultNettype,
    ) {
        self.default_nettypes.push((default_nettype, def_span));
    }

    /// Get the correct compiler default nettype, based on a span
    pub fn get_default_nettype(&self, span: &Span<'a>) -> &DefaultNettype {
        for default_nettype in self.default_nettypes.iter().rev() {
            if default_nettype.1.compare(span) == SpanRelation::Earlier {
                return &default_nettype.0;
            }
        }
        &DEFAULT_NETTYPE
    }

    /// Retain the contents of a file
    pub fn retain_file(
        &mut self,
        file_path: String,
        file_contents: String,
    ) -> (&'a str, &'a str) {
        self.included_files
            .push((file_path.into_boxed_str(), file_contents.into_boxed_str()));
        let entry = self.included_files.last().unwrap();
        let path: *const str = entry.0.as_ref();
        let contents: *const str = entry.1.as_ref();
        unsafe { (&*path, &*contents) }
    }

    /// Retain a span
    pub fn retain_span(&mut self, span: Span<'a>) -> &'a Span<'a> {
        self.included_spans.push(span);
        let entry: *const Span<'a> = self.included_spans.last().unwrap();
        unsafe { &*entry }
    }

    /// Get the included file contents as a vector
    pub fn included_files(&self) -> Vec<(String, String)> {
        self.included_files
            .iter()
            .map(|(a, b)| (a.clone().into(), b.clone().into()))
            .collect()
    }

    /// Add an unconnected drive
    pub fn add_unconnected_drive(
        &mut self,
        unconnected_drive_span: Span<'a>,
        unconnected_drive: UnconnectedDrive,
    ) {
        self.unconnected_drives
            .push((unconnected_drive, unconnected_drive_span));
    }

    /// Get the unconnected drive based on a span
    pub fn get_unconnected_drive(&self, span: &Span<'a>) -> &UnconnectedDrive {
        for unconnected_drive in self.unconnected_drives.iter().rev() {
            if unconnected_drive.1.compare(span) == SpanRelation::Earlier {
                return &unconnected_drive.0;
            }
        }
        &DEFAULT_UNCONNECTED_DRIVE
    }

    /// Add a cell define declaration
    pub fn add_cell_define(&mut self, is_cell_define: bool, span: Span<'a>) {
        self.cell_defines.push((is_cell_define, span));
    }

    /// Determine whether a module is a cell module, based on a span
    pub fn is_cell_module(&self, declaration_span: &Span<'a>) -> bool {
        for cell_define in self.cell_defines.iter().rev() {
            if cell_define.1.compare(declaration_span) == SpanRelation::Earlier
            {
                return cell_define.0;
            }
        }
        false
    }
}
