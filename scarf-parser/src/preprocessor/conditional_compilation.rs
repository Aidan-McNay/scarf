// =======================================================================
// conditional_compilation.rs
// =======================================================================
// Preprocessing for conditional compiler directives

use crate::Span;
use crate::*;
use scarf_syntax::*;

fn preprocess_untaken_conditional<'s>(
    src: &mut TokenIterator<'s, impl Iterator<Item = SpannedToken<'s>>>,
) -> Result<(), PreprocessorError<'s>> {
    loop {
        match src.next() {
            None => {
                break Ok(());
            }
            Some(SpannedToken(Token::DirEndif, endif_span)) => {
                break Err(PreprocessorError::Endif(endif_span));
            }
            Some(SpannedToken(Token::DirElse, else_span)) => {
                break Err(PreprocessorError::Else(else_span));
            }
            Some(SpannedToken(Token::DirElsif, elsif_span)) => {
                break Err(PreprocessorError::Elsif(elsif_span));
            }
            Some(SpannedToken(Token::DirIfdef, _))
            | Some(SpannedToken(Token::DirIfndef, _)) => {
                'inner_conditional: loop {
                    match preprocess_untaken_conditional(src) {
                        Ok(()) => {
                            return Ok(());
                        }
                        Err(PreprocessorError::EndKeywords(_)) => {
                            break 'inner_conditional;
                        }
                        _ => (),
                    }
                }
            }
            _ => (),
        }
    }
}

fn get_ifdef_condition<'s>(
    src: &mut TokenIterator<'s, impl Iterator<Item = SpannedToken<'s>>>,
    ifdef_span: Span<'s>,
) -> Result<IfdefCondition<'s>, PreprocessorError<'s>> {
    let Some(spanned_token) = src.next() else {
        return Err(PreprocessorError::IncompleteDirective(ifdef_span));
    };
    match spanned_token.0 {
        Token::SimpleIdentifier(id_str) => Ok(IfdefCondition::TextMacro(
            Box::new(TextMacroIdentifier(id_str, spanned_token.1)),
        )),
        Token::EscapedIdentifier(id_str) => Ok(IfdefCondition::TextMacro(
            Box::new(TextMacroIdentifier(id_str, spanned_token.1)),
        )),
        Token::Paren => {
            let ifdef_macro_expression =
                get_ifdef_macro_expression(src, ifdef_span, 0)?;
            let Some(eparen_token) = src.next() else {
                return Err(PreprocessorError::Error(VerboseError {
                    valid: true,
                    span: spanned_token.1,
                    found: None,
                    expected: vec![Expectation::Token(Token::EParen)],
                }));
            };
            Ok(IfdefCondition::ParenMacro(Box::new((
                spanned_token.1,
                ifdef_macro_expression,
                eparen_token.1,
            ))))
        }
        _ => {
            return Err(PreprocessorError::Error(VerboseError {
                valid: true,
                span: spanned_token.1,
                found: Some(spanned_token.0),
                expected: vec![Expectation::Label(
                    "a preprocessor macro expression",
                )],
            }));
        }
    }
}

#[inline(always)]
fn amp_amp_operator_binding_power<'s>() -> (u8, u8) {
    left_assoc(6)
}

#[inline(always)]
fn pipe_pipe_operator_binding_power<'s>() -> (u8, u8) {
    left_assoc(5)
}

#[inline(always)]
fn implication_operator_binding_power<'s>() -> (u8, u8) {
    right_assoc(3)
}

#[inline(always)]
fn equivalence_operator_binding_power<'s>() -> (u8, u8) {
    right_assoc(3)
}

fn get_ifdef_macro_expression<'s>(
    src: &mut TokenIterator<'s, impl Iterator<Item = SpannedToken<'s>>>,
    previous_span: Span<'s>,
    min_bp: u8,
) -> Result<IfdefMacroExpression<'s>, PreprocessorError<'s>> {
    let Some(spanned_token) = src.next() else {
        return Err(PreprocessorError::IncompleteDirective(previous_span));
    };
    let mut lhs = match spanned_token.0 {
        Token::SimpleIdentifier(id_str) => IfdefMacroExpression::Text(
            Box::new(TextMacroIdentifier(id_str, spanned_token.1)),
        ),
        Token::EscapedIdentifier(id_str) => IfdefMacroExpression::Text(
            Box::new(TextMacroIdentifier(id_str, spanned_token.1)),
        ),
        Token::Exclamation => {
            let negated_expr =
                get_ifdef_macro_expression(src, previous_span.clone(), 255)?;
            IfdefMacroExpression::Not(Box::new((spanned_token.1, negated_expr)))
        }
        _ => {
            return Err(PreprocessorError::Error(VerboseError {
                valid: true,
                span: spanned_token.1,
                found: Some(spanned_token.0),
                expected: vec![Expectation::Label(
                    "a preprocessor macro expression",
                )],
            }));
        }
    };
    loop {
        let Some(spanned_token) = src.peek() else {
            return Ok(lhs);
        };
        let (op, r_bp) = match spanned_token.0 {
            Token::AmpAmp => {
                let (l_bp, r_bp) = amp_amp_operator_binding_power();
                if l_bp < min_bp {
                    return Ok(lhs);
                }
                (BinaryLogicalOperator::AmpAmp(spanned_token.1.clone()), r_bp)
            }
            Token::PipePipe => {
                let (l_bp, r_bp) = pipe_pipe_operator_binding_power();
                if l_bp < min_bp {
                    return Ok(lhs);
                }
                (
                    BinaryLogicalOperator::PipePipe(spanned_token.1.clone()),
                    r_bp,
                )
            }
            Token::MinusGt => {
                let (l_bp, r_bp) = implication_operator_binding_power();
                if l_bp < min_bp {
                    return Ok(lhs);
                }
                (
                    BinaryLogicalOperator::Implication(spanned_token.1.clone()),
                    r_bp,
                )
            }
            Token::LtMinusGt => {
                let (l_bp, r_bp) = equivalence_operator_binding_power();
                if l_bp < min_bp {
                    return Ok(lhs);
                }
                (
                    BinaryLogicalOperator::Equivalence(spanned_token.1.clone()),
                    r_bp,
                )
            }
            _ => return Ok(lhs),
        };
        let _ = src.next().unwrap(); // Consume peeked token
        let rhs = get_ifdef_macro_expression(src, previous_span.clone(), r_bp)?;
        lhs = IfdefMacroExpression::Operator(Box::new((lhs, op, rhs)));
    }
}

fn ifdef_condition_true<'s>(
    condition: IfdefCondition<'s>,
    configs: &PreprocessConfigs<'s>,
) -> bool {
    match condition {
        IfdefCondition::TextMacro(inner_box) => {
            let TextMacroIdentifier(text, _) = *inner_box;
            configs.is_defined(text)
        }
        IfdefCondition::ParenMacro(inner_box) => match *inner_box {
            (_, expression, _) => ifdef_expression_true(expression, configs),
        },
    }
}

fn ifdef_expression_true<'s>(
    expression: IfdefMacroExpression<'s>,
    configs: &PreprocessConfigs<'s>,
) -> bool {
    match expression {
        IfdefMacroExpression::Text(inner_box) => {
            let TextMacroIdentifier(text, _) = *inner_box;
            configs.is_defined(text)
        }
        IfdefMacroExpression::Paren(inner_box) => {
            let (_, expression, _) = *inner_box;
            ifdef_expression_true(expression, configs)
        }
        IfdefMacroExpression::Not(inner_box) => {
            let (_, expression) = *inner_box;
            !ifdef_expression_true(expression, configs)
        }
        IfdefMacroExpression::Operator(inner_box) => {
            let (first_expr, operator, second_expr) = *inner_box;
            match operator {
                BinaryLogicalOperator::AmpAmp(_) => {
                    ifdef_expression_true(first_expr, configs)
                        && ifdef_expression_true(second_expr, configs)
                }
                BinaryLogicalOperator::PipePipe(_) => {
                    ifdef_expression_true(first_expr, configs)
                        || ifdef_expression_true(second_expr, configs)
                }
                BinaryLogicalOperator::Implication(_) => {
                    (!ifdef_expression_true(first_expr, configs))
                        || ifdef_expression_true(second_expr, configs)
                }
                BinaryLogicalOperator::Equivalence(_) => {
                    ((!ifdef_expression_true(first_expr.clone(), configs))
                        || ifdef_expression_true(second_expr.clone(), configs))
                        && ((!ifdef_expression_true(second_expr, configs))
                            || ifdef_expression_true(first_expr, configs))
                }
            }
        }
    }
}

pub fn preprocess_ifdef<'s>(
    src: &mut TokenIterator<'s, impl Iterator<Item = SpannedToken<'s>>>,
    dest: &mut Vec<SpannedToken<'s>>,
    configs: &mut PreprocessConfigs<'s>,
    ifdef_span: Span<'s>,
    is_ifdef: bool, // False for ifndef
) -> Result<(), PreprocessorError<'s>> {
    let ifdef_condition = get_ifdef_condition(src, ifdef_span.clone())?;
    let mut valid_condition_found =
        ifdef_condition_true(ifdef_condition, configs) ^ !is_ifdef;
    let mut curr_condition_valid = valid_condition_found;
    loop {
        let result = if curr_condition_valid {
            preprocess(src, dest, configs)
        } else {
            preprocess_untaken_conditional(src)
        };
        match result {
            Ok(()) => {
                let conditional_token = if is_ifdef {
                    Token::DirIfdef
                } else {
                    Token::DirIfndef
                };
                return Err(PreprocessorError::NoEndif(
                    conditional_token,
                    ifdef_span,
                ));
            }
            Err(PreprocessorError::Endif(_)) => return Ok(()),
            Err(PreprocessorError::Elsif(elsif_span)) => {
                let ifdef_condition = get_ifdef_condition(src, elsif_span)?;
                if valid_condition_found {
                    curr_condition_valid = false;
                } else {
                    curr_condition_valid =
                        ifdef_condition_true(ifdef_condition, configs);
                    valid_condition_found = curr_condition_valid;
                };
                ()
            }
            Err(PreprocessorError::Else(else_span)) => {
                let result = if !valid_condition_found {
                    preprocess(src, dest, configs)
                } else {
                    preprocess_untaken_conditional(src)
                };
                match result {
                    Ok(()) => {
                        return Err(PreprocessorError::NoEndif(
                            Token::DirElse,
                            else_span,
                        ));
                    }
                    Err(PreprocessorError::Endif(_)) => return Ok(()),
                    Err(err) => return Err(err),
                }
            }
            Err(err) => return Err(err),
        }
    }
}
