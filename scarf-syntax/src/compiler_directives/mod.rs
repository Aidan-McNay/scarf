// =======================================================================
// mod.rs
// =======================================================================
// Syntax for compiler directives

use crate::*;
pub mod conditional_compilation;
pub mod include;
pub mod line;
pub mod text_macro;
pub use conditional_compilation::*;
pub use include::*;
pub use line::*;
pub use text_macro::*;

#[derive(Clone, Debug, PartialEq)]
pub enum CompilerDirective<'a> {
    __FILE__(Box<Span>),
    __LINE__(Box<Span>),
    DirBeginKeywords(),
    DirCellDefine(),
    DirDefaultNettype(),
    DirDefine(Box<TextMacroDefinition<'a>>),
    DirElse(Box<Span>),
    DirElsif(Box<ElsifDirective<'a>>),
    DirEndKeywords(),
    DirEndcalldefine(),
    DirEndif(Box<EndifDirective>),
    DirIfdef(Box<IfdefDirective<'a>>),
    DirIfndef(Box<IfndefDirective<'a>>),
    DirInclude(Box<IncludeCompilerDirective<'a>>),
    DirLine(Box<LineCompilerDirective<'a>>),
    DirNounconnectedDrive(),
    DirPragma(),
    DirResetall(),
    DirTimescale(),
    DirUnconnectedDrive(),
    DirUndef(Box<UndefineCompilerDirective<'a>>),
    DirUndefineall(Box<Span>),
    TextMacro(Box<TextMacroUsage<'a>>),
}
