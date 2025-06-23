// =======================================================================
// mod.rs
// =======================================================================
// Parsing for 1800-2023 A.9.3

use crate::*;
use chumsky::prelude::*;
use scarf_syntax::*;

pub fn array_identifier_parser<'a, I>()
-> impl Parser<'a, I, ArrayIdentifier<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
{
    identifier_parser().map(|a| ArrayIdentifier(a))
}

pub fn block_identifier_parser<'a, I>()
-> impl Parser<'a, I, BlockIdentifier<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
{
    identifier_parser().map(|a| BlockIdentifier(a))
}

pub fn bin_identifier_parser<'a, I>()
-> impl Parser<'a, I, BinIdentifier<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
{
    identifier_parser().map(|a| BinIdentifier(a))
}

pub fn c_identifier_parser<'a, I>() -> impl Parser<'a, I, CIdentifier<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
{
    select! {
        Token::SimpleIdentifier(text) = e if !(text.contains("$")) => CIdentifier(text, Metadata{
            span: convert_span(e.span()),
            extra_nodes: Vec::new()
        })
    }
    .labelled("a C identifier")
    .then(extra_node_parser())
    .map(|(identifier, b)| match identifier {
        CIdentifier(text, metadata) => CIdentifier(text, replace_nodes(metadata, b)),
    })
}

pub fn cell_identifier_parser<'a, I>()
-> impl Parser<'a, I, CellIdentifier<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
{
    identifier_parser().map(|a| CellIdentifier(a))
}

pub fn checker_identifier_parser<'a, I>()
-> impl Parser<'a, I, CheckerIdentifier<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
{
    identifier_parser().map(|a| CheckerIdentifier(a))
}

pub fn class_identifier_parser<'a, I>()
-> impl Parser<'a, I, ClassIdentifier<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
{
    identifier_parser().map(|a| ClassIdentifier(a))
}

pub fn class_variable_identifier_parser<'a, I>()
-> impl Parser<'a, I, ClassVariableIdentifier<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
{
    variable_identifier_parser().map(|a| ClassVariableIdentifier(a))
}

pub fn clocking_identifier_parser<'a, I>()
-> impl Parser<'a, I, ClockingIdentifier<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
{
    identifier_parser().map(|a| ClockingIdentifier(a))
}

pub fn config_identifier_parser<'a, I>()
-> impl Parser<'a, I, ConfigIdentifier<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
{
    identifier_parser().map(|a| ConfigIdentifier(a))
}

pub fn const_identifier_parser<'a, I>()
-> impl Parser<'a, I, ConstIdentifier<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
{
    identifier_parser().map(|a| ConstIdentifier(a))
}

pub fn constraint_identifier_parser<'a, I>()
-> impl Parser<'a, I, ConstraintIdentifier<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
{
    identifier_parser().map(|a| ConstraintIdentifier(a))
}

pub fn covergroup_identifier_parser<'a, I>()
-> impl Parser<'a, I, CovergroupIdentifier<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
{
    identifier_parser().map(|a| CovergroupIdentifier(a))
}

pub fn covergroup_variable_identifier_parser<'a, I>()
-> impl Parser<'a, I, CovergroupVariableIdentifier<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
{
    variable_identifier_parser().map(|a| CovergroupVariableIdentifier(a))
}

pub fn cover_point_identifier_parser<'a, I>()
-> impl Parser<'a, I, CoverPointIdentifier<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
{
    identifier_parser().map(|a| CoverPointIdentifier(a))
}

pub fn cross_identifier_parser<'a, I>()
-> impl Parser<'a, I, CrossIdentifier<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
{
    identifier_parser().map(|a| CrossIdentifier(a))
}

pub fn dynamic_array_variable_identifier_parser<'a, I>()
-> impl Parser<'a, I, DynamicArrayVariableIdentifier<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
{
    variable_identifier_parser().map(|a| DynamicArrayVariableIdentifier(a))
}

pub fn enum_identifier_parser<'a, I>()
-> impl Parser<'a, I, EnumIdentifier<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
{
    identifier_parser().map(|a| EnumIdentifier(a))
}

pub fn formal_identifier_parser<'a, I>()
-> impl Parser<'a, I, FormalIdentifier<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
{
    identifier_parser().map(|a| FormalIdentifier(a))
}

pub fn formal_port_identifier_parser<'a, I>()
-> impl Parser<'a, I, FormalPortIdentifier<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
{
    identifier_parser().map(|a| FormalPortIdentifier(a))
}

pub fn function_identifier_parser<'a, I>()
-> impl Parser<'a, I, FunctionIdentifier<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
{
    identifier_parser().map(|a| FunctionIdentifier(a))
}

pub fn generate_block_identifier_parser<'a, I>()
-> impl Parser<'a, I, GenerateBlockIdentifier<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
{
    identifier_parser().map(|a| GenerateBlockIdentifier(a))
}

pub fn genvar_identifier_parser<'a, I>()
-> impl Parser<'a, I, GenvarIdentifier<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
{
    identifier_parser().map(|a| GenvarIdentifier(a))
}

pub fn hierarchical_array_identifier_parser<'a, I>()
-> impl Parser<'a, I, HierarchicalArrayIdentifier<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
{
    hierarchical_identifier_parser().map(|a| HierarchicalArrayIdentifier(a))
}

pub fn hierarchical_block_identifier_parser<'a, I>()
-> impl Parser<'a, I, HierarchicalBlockIdentifier<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
{
    hierarchical_identifier_parser().map(|a| HierarchicalBlockIdentifier(a))
}

pub fn hierarchical_event_identifier_parser<'a, I>()
-> impl Parser<'a, I, HierarchicalEventIdentifier<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
{
    hierarchical_identifier_parser().map(|a| HierarchicalEventIdentifier(a))
}

pub fn hierarchical_identifier_parser<'a, I>()
-> impl Parser<'a, I, HierarchicalIdentifier<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
{
    let identifiers_parser = identifier_parser()
        .then(constant_bit_select_parser(constant_expression_parser()))
        .then(token(Token::Period))
        .map(|((a, b), c)| (a, b, c))
        .repeated()
        .collect::<Vec<(Identifier<'a>, ConstantBitSelect<'a>, Metadata<'a>)>>();
    token(Token::DollarRoot)
        .then(token(Token::Period))
        .or_not()
        .then(identifiers_parser)
        .then(identifier_parser())
        .map(|((a, b), c)| HierarchicalIdentifier(a, b, c))
        .boxed()
}

pub fn hierarchical_net_identifier_parser<'a, I>()
-> impl Parser<'a, I, HierarchicalNetIdentifier<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
{
    hierarchical_identifier_parser().map(|a| HierarchicalNetIdentifier(a))
}

pub fn hierarchical_parameter_identifier_parser<'a, I>()
-> impl Parser<'a, I, HierarchicalParameterIdentifier<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
{
    hierarchical_identifier_parser().map(|a| HierarchicalParameterIdentifier(a))
}

pub fn hierarchical_property_identifier_parser<'a, I>()
-> impl Parser<'a, I, HierarchicalPropertyIdentifier<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
{
    hierarchical_identifier_parser().map(|a| HierarchicalPropertyIdentifier(a))
}

pub fn hierarchical_sequence_identifier_parser<'a, I>()
-> impl Parser<'a, I, HierarchicalSequenceIdentifier<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
{
    hierarchical_identifier_parser().map(|a| HierarchicalSequenceIdentifier(a))
}

pub fn hierarchical_task_identifier_parser<'a, I>()
-> impl Parser<'a, I, HierarchicalTaskIdentifier<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
{
    hierarchical_identifier_parser().map(|a| HierarchicalTaskIdentifier(a))
}

pub fn hierarchical_tf_identifier_parser<'a, I>()
-> impl Parser<'a, I, HierarchicalTfIdentifier<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
{
    hierarchical_identifier_parser().map(|a| HierarchicalTfIdentifier(a))
}

pub fn hierarchical_variable_identifier_parser<'a, I>()
-> impl Parser<'a, I, HierarchicalVariableIdentifier<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
{
    hierarchical_identifier_parser().map(|a| HierarchicalVariableIdentifier(a))
}

pub fn identifier_parser<'a, I>() -> impl Parser<'a, I, Identifier<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
{
    select! {
        Token::SimpleIdentifier(text) = e => Identifier::SimpleIdentifier((text, Metadata{
            span: convert_span(e.span()),
            extra_nodes: Vec::new()
        })),
        Token::EscapedIdentifier(text) = e => Identifier::EscapedIdentifier((text, Metadata{
            span: convert_span(e.span()),
            extra_nodes: Vec::new()
        })),
    }
    .labelled("an identifier")
    .then(extra_node_parser())
    .map(|(identifier, b)| match identifier {
        Identifier::SimpleIdentifier((text, metadata)) => {
            Identifier::SimpleIdentifier((text, replace_nodes(metadata, b)))
        }
        Identifier::EscapedIdentifier((text, metadata)) => {
            Identifier::EscapedIdentifier((text, replace_nodes(metadata, b)))
        }
    })
    .boxed()
}

pub fn index_identifier_parser<'a, I>()
-> impl Parser<'a, I, IndexVariableIdentifier<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
{
    identifier_parser().map(|a| IndexVariableIdentifier(a))
}

pub fn interface_identifier_parser<'a, I>()
-> impl Parser<'a, I, InterfaceIdentifier<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
{
    identifier_parser().map(|a| InterfaceIdentifier(a))
}

pub fn interface_port_identifier_parser<'a, I>()
-> impl Parser<'a, I, InterfacePortIdentifier<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
{
    identifier_parser().map(|a| InterfacePortIdentifier(a))
}

pub fn inout_port_identifier_parser<'a, I>()
-> impl Parser<'a, I, InoutPortIdentifier<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
{
    identifier_parser().map(|a| InoutPortIdentifier(a))
}

pub fn input_port_identifier_parser<'a, I>()
-> impl Parser<'a, I, InputPortIdentifier<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
{
    identifier_parser().map(|a| InputPortIdentifier(a))
}

pub fn instance_identifier_parser<'a, I>()
-> impl Parser<'a, I, InstanceIdentifier<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
{
    identifier_parser().map(|a| InstanceIdentifier(a))
}

pub fn library_identifier_parser<'a, I>()
-> impl Parser<'a, I, LibraryIdentifier<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
{
    identifier_parser().map(|a| LibraryIdentifier(a))
}

pub fn member_identifier_parser<'a, I>()
-> impl Parser<'a, I, MemberIdentifier<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
{
    identifier_parser().map(|a| MemberIdentifier(a))
}

pub fn method_identifier_parser<'a, I>()
-> impl Parser<'a, I, MethodIdentifier<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
{
    identifier_parser().map(|a| MethodIdentifier(a))
}

pub fn modport_identifier_parser<'a, I>()
-> impl Parser<'a, I, ModportIdentifier<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
{
    identifier_parser().map(|a| ModportIdentifier(a))
}

pub fn module_identifier_parser<'a, I>()
-> impl Parser<'a, I, ModuleIdentifier<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
{
    identifier_parser().map(|a| ModuleIdentifier(a))
}

pub fn net_identifier_parser<'a, I>()
-> impl Parser<'a, I, NetIdentifier<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
{
    identifier_parser().map(|a| NetIdentifier(a))
}

pub fn nettype_identifier_parser<'a, I>()
-> impl Parser<'a, I, NettypeIdentifier<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
{
    identifier_parser().map(|a| NettypeIdentifier(a))
}

pub fn output_port_identifier_parser<'a, I>()
-> impl Parser<'a, I, OutputPortIdentifier<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
{
    identifier_parser().map(|a| OutputPortIdentifier(a))
}

pub fn package_identifier_parser<'a, I>()
-> impl Parser<'a, I, PackageIdentifier<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
{
    identifier_parser().map(|a| PackageIdentifier(a))
}

pub fn package_scope_parser<'a, I>() -> impl Parser<'a, I, PackageScope<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
{
    let _identifier_parser = package_identifier_parser()
        .then(token(Token::ColonColon))
        .map(|(a, b)| PackageScope::Identifier(Box::new((a, b))));
    let _unit_parser = token(Token::DollarUnit)
        .then(token(Token::ColonColon))
        .map(|(a, b)| PackageScope::Unit(Box::new((a, b))));
    choice((_identifier_parser, _unit_parser))
}

pub fn parameter_identifier_parser<'a, I>()
-> impl Parser<'a, I, ParameterIdentifier<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
{
    identifier_parser().map(|a| ParameterIdentifier(a))
}

pub fn port_identifier_parser<'a, I>()
-> impl Parser<'a, I, PortIdentifier<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
{
    identifier_parser().map(|a| PortIdentifier(a))
}

pub fn program_identifier_parser<'a, I>()
-> impl Parser<'a, I, ProgramIdentifier<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
{
    identifier_parser().map(|a| ProgramIdentifier(a))
}

pub fn property_identifier_parser<'a, I>()
-> impl Parser<'a, I, PropertyIdentifier<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
{
    identifier_parser().map(|a| PropertyIdentifier(a))
}

pub fn ps_class_identifier_parser<'a, I>()
-> impl Parser<'a, I, PsClassIdentifier<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
{
    package_scope_parser()
        .or_not()
        .then(class_identifier_parser())
        .map(|(a, b)| PsClassIdentifier(a, b))
        .boxed()
}

pub fn ps_covergroup_identifier_parser<'a, I>()
-> impl Parser<'a, I, PsCovergroupIdentifier<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
{
    package_scope_parser()
        .or_not()
        .then(covergroup_identifier_parser())
        .map(|(a, b)| PsCovergroupIdentifier(a, b))
        .boxed()
}

pub fn ps_checker_identifier_parser<'a, I>()
-> impl Parser<'a, I, PsCheckerIdentifier<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
{
    package_scope_parser()
        .or_not()
        .then(checker_identifier_parser())
        .map(|(a, b)| PsCheckerIdentifier(a, b))
        .boxed()
}

pub fn ps_identifier_parser<'a, I>() -> impl Parser<'a, I, PsIdentifier<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
{
    package_scope_parser()
        .or_not()
        .then(identifier_parser())
        .map(|(a, b)| PsIdentifier(a, b))
        .boxed()
}

pub fn ps_or_hierarchical_array_identifier_parser<'a, I>()
-> impl Parser<'a, I, PsOrHierarchicalArrayIdentifier<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
{
    let _scope_parser = choice((
        implicit_class_handle_parser()
            .then(token(Token::Period))
            .map(|(a, b)| PsOrHierarchicalArrayIdentifierScope::ImplicitClassHandle(a, b)),
        class_scope_parser().map(|a| PsOrHierarchicalArrayIdentifierScope::ClassScope(a)),
        package_scope_parser().map(|a| PsOrHierarchicalArrayIdentifierScope::PackageScope(a)),
    ));
    _scope_parser
        .or_not()
        .then(hierarchical_array_identifier_parser())
        .map(|(a, b)| PsOrHierarchicalArrayIdentifier(a, b))
        .boxed()
}

pub fn ps_or_hierarchical_net_identifier_parser<'a, I>()
-> impl Parser<'a, I, PsOrHierarchicalNetIdentifier<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
{
    choice((
        package_scope_parser()
            .or_not()
            .then(net_identifier_parser())
            .map(|(a, b)| PsOrHierarchicalNetIdentifier::PackageScope(a, b)),
        hierarchical_net_identifier_parser()
            .map(|a| PsOrHierarchicalNetIdentifier::Hierarchical(a)),
    ))
    .boxed()
}

pub fn ps_or_hierarchical_property_identifier_parser<'a, I>()
-> impl Parser<'a, I, PsOrHierarchicalPropertyIdentifier<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
{
    choice((
        package_scope_parser()
            .or_not()
            .then(property_identifier_parser())
            .map(|(a, b)| PsOrHierarchicalPropertyIdentifier::PackageScope(a, b)),
        hierarchical_property_identifier_parser()
            .map(|a| PsOrHierarchicalPropertyIdentifier::Hierarchical(a)),
    ))
    .boxed()
}

pub fn ps_or_hierarchical_sequence_identifier_parser<'a, I>()
-> impl Parser<'a, I, PsOrHierarchicalSequenceIdentifier<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
{
    choice((
        package_scope_parser()
            .or_not()
            .then(sequence_identifier_parser())
            .map(|(a, b)| PsOrHierarchicalSequenceIdentifier::PackageScope(a, b)),
        hierarchical_sequence_identifier_parser()
            .map(|a| PsOrHierarchicalSequenceIdentifier::Hierarchical(a)),
    ))
    .boxed()
}

pub fn ps_or_hierarchical_tf_identifier_parser<'a, I>()
-> impl Parser<'a, I, PsOrHierarchicalTfIdentifier<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
{
    choice((
        package_scope_parser()
            .or_not()
            .then(tf_identifier_parser())
            .map(|(a, b)| PsOrHierarchicalTfIdentifier::PackageScope(a, b)),
        hierarchical_tf_identifier_parser().map(|a| PsOrHierarchicalTfIdentifier::Hierarchical(a)),
    ))
    .boxed()
}

pub fn ps_parameter_identifier_parser<'a, I>()
-> impl Parser<'a, I, PsParameterIdentifier<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
{
    let _scope_parser = choice((
        class_scope_parser().map(|a| PsParameterIdentifierScope::ClassScope(a)),
        package_scope_parser().map(|a| PsParameterIdentifierScope::PackageScope(a)),
    ));
    let _scoped_parser = _scope_parser
        .or_not()
        .then(parameter_identifier_parser())
        .map(|(a, b)| PsParameterIdentifier::Scoped(a, b));
    let _generated_parser = (generate_block_identifier_parser()
        .then(
            token(Token::Bracket)
                .then(constant_expression_parser())
                .then(token(Token::EBracket))
                .map(|((a, b), c)| (a, b, c))
                .or_not(),
        )
        .then(token(Token::Period))
        .map(|((a, b), c)| (a, b, c))
        .repeated()
        .collect::<Vec<(
            GenerateBlockIdentifier<'a>,
            Option<(Metadata<'a>, ConstantExpression<'a>, Metadata<'a>)>,
            Metadata<'a>,
        )>>())
    .then(parameter_identifier_parser())
    .map(|(a, b)| PsParameterIdentifier::Generated(a, b));
    choice((_scoped_parser, _generated_parser)).boxed()
}

pub fn ps_type_identifier_parser<'a, I>()
-> impl Parser<'a, I, PsTypeIdentifier<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
{
    let _scope_parser = choice((
        token(Token::Local)
            .then(token(Token::ColonColon))
            .map(|(a, b)| PsTypeIdentifierScope::LocalScope(a, b)),
        package_scope_parser().map(|a| PsTypeIdentifierScope::PackageScope(a)),
        class_scope_parser().map(|a| PsTypeIdentifierScope::ClassScope(a)),
    ));
    _scope_parser
        .or_not()
        .then(type_identifier_parser())
        .map(|(a, b)| PsTypeIdentifier(a, b))
        .boxed()
}

pub fn rs_production_identifier_parser<'a, I>()
-> impl Parser<'a, I, RsProductionIdentifier<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
{
    identifier_parser().map(|a| RsProductionIdentifier(a))
}

pub fn sequence_identifier_parser<'a, I>()
-> impl Parser<'a, I, SequenceIdentifier<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
{
    identifier_parser().map(|a| SequenceIdentifier(a))
}

pub fn signal_identifier_parser<'a, I>()
-> impl Parser<'a, I, SignalIdentifier<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
{
    identifier_parser().map(|a| SignalIdentifier(a))
}

pub fn specparam_identifier_parser<'a, I>()
-> impl Parser<'a, I, SpecparamIdentifier<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
{
    identifier_parser().map(|a| SpecparamIdentifier(a))
}

pub fn task_identifier_parser<'a, I>()
-> impl Parser<'a, I, TaskIdentifier<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
{
    identifier_parser().map(|a| TaskIdentifier(a))
}

pub fn tf_identifier_parser<'a, I>() -> impl Parser<'a, I, TfIdentifier<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
{
    identifier_parser().map(|a| TfIdentifier(a))
}

pub fn terminal_identifier_parser<'a, I>()
-> impl Parser<'a, I, TerminalIdentifier<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
{
    identifier_parser().map(|a| TerminalIdentifier(a))
}

pub fn topmodule_identifier_parser<'a, I>()
-> impl Parser<'a, I, TopmoduleIdentifier<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
{
    identifier_parser().map(|a| TopmoduleIdentifier(a))
}

pub fn type_identifier_parser<'a, I>()
-> impl Parser<'a, I, TypeIdentifier<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
{
    identifier_parser().map(|a| TypeIdentifier(a))
}

pub fn udp_identifier_parser<'a, I>()
-> impl Parser<'a, I, UdpIdentifier<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
{
    identifier_parser().map(|a| UdpIdentifier(a))
}

pub fn variable_identifier_parser<'a, I>()
-> impl Parser<'a, I, VariableIdentifier<'a>, ParserError<'a>> + Clone
where
    I: ValueInput<'a, Token = Token<'a>, Span = SimpleSpan>,
{
    identifier_parser().map(|a| VariableIdentifier(a))
}
