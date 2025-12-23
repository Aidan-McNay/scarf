// =======================================================================
// mod.rs
// =======================================================================
// Syntax for compiler directives

use crate::*;
pub mod conditional_compilation;
pub mod define;
pub mod include;
pub mod line;
pub use conditional_compilation::*;
pub use define::*;
pub use include::*;
pub use line::*;

#[derive(Clone, Debug, PartialEq)]
pub enum CompilerDirective<'a> {
    __FILE__(Box<Span<'a>>),
    __LINE__(Box<Span<'a>>),
    DirBeginKeywords(),
    DirCellDefine(),
    DirDefaultNettype(),
    DirDefine(Box<TextMacroDefinition<'a>>),
    DirElse(Box<Span<'a>>),
    DirElsif(Box<ElsifDirective<'a>>),
    DirEndKeywords(),
    DirEndcalldefine(),
    DirEndif(Box<EndifDirective<'a>>),
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
    DirUndefineall(Box<Span<'a>>),
    TextMacro(Box<TextMacroUsage<'a>>),
}
