// =======================================================================
// covergroup_declarations.rs
// =======================================================================
// Parsing for 1800-2023 A.2.11

use crate::*;
use scarf_syntax::*;
use winnow::ModalResult;
use winnow::Parser;
use winnow::combinator::{alt, opt, repeat};

#[inline(always)]
fn covergroup_matches_operator_binding_power<'s>() -> u8 {
    no_assoc(3)
}

#[inline(always)]
fn amp_amp_operator_binding_power<'s>() -> (u8, u8) {
    left_assoc(2)
}

#[inline(always)]
fn pipe_pipe_operator_binding_power<'s>() -> (u8, u8) {
    left_assoc(1)
}

enum SelectExpressionOp<'a> {
    AmpAmp(Metadata<'a>),
    PipePipe(Metadata<'a>),
    With(Metadata<'a>),
}

fn basic_select_expression_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<SelectExpression<'s>, VerboseError<'s>> {
    let _condition_parser = select_condition_parser
        .map(|a| SelectExpression::Condition(Box::new(a)));
    let _not_parser = (token(Token::Exclamation), select_condition_parser)
        .map(|(a, b)| SelectExpression::Not(Box::new((a, b))));
    let _paren_parser = (
        token(Token::Paren),
        select_expression_parser,
        token(Token::EParen),
    )
        .map(|(a, b, c)| SelectExpression::Paren(Box::new((a, b, c))));
    let _cross_identifier_parser = cross_identifier_parser
        .map(|a| SelectExpression::CrossIdentifier(Box::new(a)));
    let _cross_set_parser = (
        cross_set_expression_parser,
        opt((token(Token::Matches), integer_covergroup_expression_parser)),
    )
        .map(|(a, b)| SelectExpression::CrossSet(Box::new((a, b))));
    alt((
        _condition_parser,
        _not_parser,
        _paren_parser,
        _cross_set_parser,
        _cross_identifier_parser,
    ))
    .parse_next(input)
}

fn select_expression_bp_parser<'s>(
    input: &mut Tokens<'s>,
    min_bp: u8,
) -> ModalResult<SelectExpression<'s>, VerboseError<'s>> {
    let mut lhs = basic_select_expression_parser(input)?;
    loop {
        let Ok((op, r_bp)) = alt((
            token(Token::AmpAmp).verify_map(|a| {
                let (l_bp, r_bp) = amp_amp_operator_binding_power();
                if l_bp < min_bp {
                    return None;
                }
                Some((SelectExpressionOp::AmpAmp(a), r_bp))
            }),
            token(Token::PipePipe).verify_map(|a| {
                let (l_bp, r_bp) = pipe_pipe_operator_binding_power();
                if l_bp < min_bp {
                    return None;
                }
                Some((SelectExpressionOp::PipePipe(a), r_bp))
            }),
            token(Token::With).verify_map(|a| {
                let bp = covergroup_matches_operator_binding_power();
                if bp < min_bp {
                    return None;
                }
                Some((SelectExpressionOp::With(a), 0))
            }),
        ))
        .parse_next(input) else {
            return Ok(lhs);
        };
        lhs = match op {
            SelectExpressionOp::AmpAmp(metadata) => {
                let rhs = select_expression_bp_parser(input, r_bp)?;
                SelectExpression::And(Box::new((lhs, metadata, rhs)))
            }
            SelectExpressionOp::PipePipe(metadata) => {
                let rhs = select_expression_bp_parser(input, r_bp)?;
                SelectExpression::Or(Box::new((lhs, metadata, rhs)))
            }
            SelectExpressionOp::With(metadata) => {
                let paren = token(Token::Paren)(input)?;
                let with_covergroup_expression =
                    with_covergroup_expression_parser(input)?;
                let eparen = token(Token::EParen)(input)?;
                let matches_section = opt((
                    token(Token::Matches),
                    integer_covergroup_expression_parser,
                ))
                .parse_next(input)?;
                SelectExpression::With(Box::new((
                    lhs,
                    metadata,
                    paren,
                    with_covergroup_expression,
                    eparen,
                    matches_section,
                )))
            }
        }
    }
}

pub fn select_expression_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<SelectExpression<'s>, VerboseError<'s>> {
    select_expression_bp_parser(input, 0)
}

pub fn select_condition_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<SelectCondition<'s>, VerboseError<'s>> {
    (
        token(Token::Binsof),
        token(Token::Paren),
        bins_expression_parser,
        token(Token::EParen),
        opt((
            token(Token::Intersect),
            token(Token::Brace),
            covergroup_range_list_parser,
            token(Token::EBrace),
        )),
    )
        .map(|(a, b, c, d, e)| SelectCondition(a, b, c, d, e))
        .parse_next(input)
}

pub fn bins_expression_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<BinsExpression<'s>, VerboseError<'s>> {
    alt((
        (
            cover_point_identifier_parser,
            opt((token(Token::Period), bin_identifier_parser)),
        )
            .map(|(a, b)| BinsExpression::CoverPoint(Box::new((a, b)))),
        variable_identifier_parser
            .map(|a| BinsExpression::Variable(Box::new(a))),
    ))
    .parse_next(input)
}

pub fn covergroup_range_list_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<CovergroupRangeList<'s>, VerboseError<'s>> {
    (
        covergroup_value_range_parser,
        repeat(0.., (token(Token::Comma), covergroup_value_range_parser)),
    )
        .map(|(a, b)| CovergroupRangeList(a, b))
        .parse_next(input)
}

enum CovergroupExpressionRangeOp<'a> {
    Colon(Metadata<'a>),
    AbsoluteTolerance(Metadata<'a>),
    RelativeTolerance(Metadata<'a>),
}

pub fn covergroup_value_range_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<CovergroupValueRange<'s>, VerboseError<'s>> {
    let _two_expression_range_parser = (
        token(Token::Bracket),
        covergroup_expression_parser,
        alt((
            token(Token::Colon).map(|a| CovergroupExpressionRangeOp::Colon(a)),
            token(Token::PlusSlashMinus)
                .map(|a| CovergroupExpressionRangeOp::AbsoluteTolerance(a)),
            token(Token::PlusPercentMinus)
                .map(|a| CovergroupExpressionRangeOp::RelativeTolerance(a)),
        )),
        covergroup_expression_parser,
        token(Token::EBracket),
    )
        .map(|(a, b, c, d, e)| match c {
            CovergroupExpressionRangeOp::Colon(metadata) => {
                CovergroupValueRange::ExprRange(Box::new((
                    a, b, metadata, d, e,
                )))
            }
            CovergroupExpressionRangeOp::AbsoluteTolerance(metadata) => {
                CovergroupValueRange::AbsoluteTolerance(Box::new((
                    a, b, metadata, d, e,
                )))
            }
            CovergroupExpressionRangeOp::RelativeTolerance(metadata) => {
                CovergroupValueRange::RelativeTolerance(Box::new((
                    a, b, metadata, d, e,
                )))
            }
        });
    let _dollar_low_parser = (
        token(Token::Bracket),
        token(Token::Dollar),
        token(Token::Colon),
        covergroup_expression_parser,
        token(Token::EBracket),
    )
        .map(|(a, b, c, d, e)| {
            CovergroupValueRange::DollarLow(Box::new((a, b, c, d, e)))
        });
    let _dollar_high_parser = (
        token(Token::Bracket),
        covergroup_expression_parser,
        token(Token::Colon),
        token(Token::Dollar),
        token(Token::EBracket),
    )
        .map(|(a, b, c, d, e)| {
            CovergroupValueRange::DollarHigh(Box::new((a, b, c, d, e)))
        });
    let _expr_parser = covergroup_expression_parser
        .map(|a| CovergroupValueRange::Expr(Box::new(a)));
    alt((
        _dollar_low_parser,
        _two_expression_range_parser,
        _dollar_high_parser,
        _expr_parser,
    ))
    .parse_next(input)
}

pub fn with_covergroup_expression_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<WithCovergroupExpression<'s>, VerboseError<'s>> {
    covergroup_expression_parser
        .map(|a| WithCovergroupExpression(a))
        .parse_next(input)
}

pub fn integer_covergroup_expression_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<IntegerCovergroupExpression<'s>, VerboseError<'s>> {
    alt((
        covergroup_expression_parser
            .map(|a| IntegerCovergroupExpression::Expression(Box::new(a))),
        token(Token::Dollar)
            .map(|a| IntegerCovergroupExpression::Dollar(Box::new(a))),
    ))
    .parse_next(input)
}

pub fn set_covergroup_expression_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<SetCovergroupExpression<'s>, VerboseError<'s>> {
    covergroup_expression_parser
        .map(|a| SetCovergroupExpression(a))
        .parse_next(input)
}

pub fn cross_set_expression_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<CrossSetExpression<'s>, VerboseError<'s>> {
    covergroup_expression_parser
        .map(|a| CrossSetExpression(a))
        .parse_next(input)
}

pub fn covergroup_expression_parser<'s>(
    input: &mut Tokens<'s>,
) -> ModalResult<CovergroupExpression<'s>, VerboseError<'s>> {
    expression_parser
        .map(|a| CovergroupExpression(a))
        .parse_next(input)
}
