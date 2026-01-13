// =======================================================================
// configs.rs
// =======================================================================
// Configurations for preprocessing

use crate::*;
use elsa::FrozenVec;
use scarf_syntax::*;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

const DEFAULT_TIMESCALE: Timescale = Timescale::new(
    Span::empty(),
    (TimescaleValue::One, TimescaleUnit::NS),
    (TimescaleValue::One, TimescaleUnit::NS),
);

const DEFAULT_NETTYPE: DefaultNettype = DefaultNettype::Wire;

const DEFAULT_UNCONNECTED_DRIVE: UnconnectedDrive =
    UnconnectedDrive::NoUnconnected;

#[derive(Default)]
pub struct PreprocessorCache<'a> {
    spans: FrozenVec<Box<Span<'a>>>,
    strings: FrozenVec<Box<str>>,
}

impl<'a> PreprocessorCache<'a> {
    pub fn retain_span(&self, span: Span<'a>) -> &Span<'a> {
        self.spans.push_get(Box::new(span))
    }
    pub fn retain_string(&self, string: String) -> &str {
        self.strings.push_get(string.into_boxed_str())
    }
}

#[derive(Clone)]
pub struct PreprocessConfigs<'a> {
    includes: Vec<&'a Path>,
    defines: Vec<Define<'a>>,
    timescales: Vec<Timescale<'a>>,
    default_nettypes: Vec<(DefaultNettype, Span<'a>)>,
    unconnected_drives: Vec<(UnconnectedDrive, Span<'a>)>,
    cell_defines: Vec<(bool, Span<'a>)>,
    included_files: HashMap<&'a str, &'a str>,
    pub cache: &'a PreprocessorCache<'a>,
    pub curr_standard: StandardVersion,
    pub in_define: bool,
    pub in_define_arg: bool,
    pub include_line_directives: bool,
}

#[derive(Clone, Debug)]
pub struct SpannedString<'a>(pub &'a str, pub Span<'a>);

#[derive(Clone, Debug)]
pub struct Define<'a> {
    pub name: SpannedString<'a>,
    pub body: DefineBody<'a>,
}

impl<'a> Define<'a> {
    pub fn is_from_command_line(&self) -> bool {
        self.name.1.file == ""
    }
}

#[derive(Clone, Debug)]
pub enum DefineBody<'a> {
    Empty,
    Text(Vec<SpannedToken<'a>>),
    Function(DefineFunction<'a>),
}

#[derive(Clone, Debug)]
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
    pub fn get_tokens(
        &self,
    ) -> (
        Vec<SpannedToken<'a>>,
        Option<Vec<(SpannedString<'a>, Option<Vec<SpannedToken<'a>>>)>>,
    ) {
        match self {
            DefineBody::Empty => (vec![], None),
            DefineBody::Text(token_vec) => (token_vec.clone(), None),
            DefineBody::Function(def_func) => {
                let function_args = def_func
                    .args
                    .iter()
                    .map(|(a, b)| match b {
                        Some((_, tokens)) => (a.clone(), Some(tokens.clone())),
                        None => (a.clone(), None),
                    })
                    .collect();
                match &def_func.body {
                    Some(token_vec) => (token_vec.clone(), Some(function_args)),
                    None => (vec![], Some(function_args)),
                }
            }
        }
    }
}

impl<'a> PreprocessConfigs<'a> {
    pub fn new(string_cache: &'a PreprocessorCache<'a>) -> Self {
        Self {
            includes: vec![],
            defines: vec![],
            timescales: vec![],
            default_nettypes: vec![],
            unconnected_drives: vec![],
            cell_defines: vec![],
            included_files: HashMap::new(),
            cache: string_cache,
            curr_standard: StandardVersion::default(),
            in_define: false,
            in_define_arg: false,
            include_line_directives: false,
        }
    }
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

    /// Provide a reference to existing defines
    pub fn defines(&self) -> &Vec<Define<'a>> {
        &self.defines
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
                None => DefineBody::Empty,
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
    ) -> Option<(
        Vec<SpannedToken<'a>>,
        Option<Vec<(SpannedString<'a>, Option<Vec<SpannedToken<'a>>>)>>,
    )> {
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
        let path = self.cache.retain_string(file_path);
        let contents = self.cache.retain_string(file_contents);
        self.included_files.insert(path, contents);
        (path, contents)
    }

    /// Retain a span
    pub fn retain_span(&mut self, span: Span<'a>) -> &'a Span<'a> {
        self.cache.retain_span(span)
    }

    /// Get the included file contents as a vector
    pub fn included_files(&self) -> Vec<(String, String)> {
        self.included_files
            .iter()
            .map(|(a, b)| (a.to_string(), b.to_string()))
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

    /// Get the file from a Span
    pub fn get_file(&self, span: &Span<'a>) -> &'a str {
        span.file
    }

    /// Get the line number of a Span
    pub fn get_line(&mut self, span: &Span<'a>) -> &'a str {
        let offset = span.bytes.start;
        let file_contents: &str = self.included_files.get(span.file).unwrap();
        let line_num = file_contents[..offset].lines().count() + 1;
        self.cache.retain_string(line_num.to_string())
    }

    pub fn retain_string(&mut self, string: String) -> &'a str {
        self.cache.retain_string(string)
    }
}
