use serde::Serialize;
use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SimpleSelector {
    Universal,
    Tag(String),
    Class(String),
    Id(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Combinator {
    Descendant,
    Child,
    AdjacentSibling,
    GeneralSibling,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct SelectorSequence {
    pub selectors: Vec<SimpleSelector>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ComplexSelector {
    pub first: SelectorSequence,
    pub rest: Vec<(Combinator, SelectorSequence)>,
}

// Internal DFA State 

#[derive(Debug, PartialEq)]
enum State {
    Idle,
    ReadingTag,
    ReadingClass,
    ReadingId,
}

// Parse Error 

#[derive(Debug, PartialEq)]
pub enum ParseError {
    EmptyInput,
    EmptySequence(String),
    LeadingCombinator(char),
    TrailingCombinator(char),
    EmptySelectorName(char),
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::EmptyInput => write!(f, "Input selector kosong"),
            ParseError::EmptySequence(ctx) => write!(f, "Sequence selector kosong di: '{ctx}'"),
            ParseError::LeadingCombinator(c) => write!(f, "Combinator '{c}' di awal selector tidak valid"),
            ParseError::TrailingCombinator(c) => write!(f, "Combinator '{c}' di akhir selector tidak valid"),
            ParseError::EmptySelectorName(c) => write!(f, "Nama selector kosong setelah '{c}'"),
        }
    }
}

// Parser 

#[allow(unused_assignments)]
pub fn parse(input: &str) -> Result<ComplexSelector, ParseError> {
    let input = input.trim();

    if input.is_empty() {
        return Err(ParseError::EmptyInput);
    }

    if let Some(c) = input.chars().next() {
        if matches!(c, '>' | '+' | '~') {
            return Err(ParseError::LeadingCombinator(c));
        }
    }

    let mut first: Option<SelectorSequence> = None;
    let mut rest: Vec<(Combinator, SelectorSequence)> = Vec::new();
    let mut current_seq = SelectorSequence::default();
    let mut pending_combinator: Option<Combinator> = None;
    let mut state = State::Idle;
    let mut buffer = String::new();
    let mut last_explicit_comb: Option<char> = None;

    let mut chars = input.chars().peekable();

    macro_rules! flush {
        () => {
            if !buffer.is_empty() {
                let val = buffer.clone();
                buffer.clear();
                match state {
                    State::ReadingTag => current_seq.selectors.push(SimpleSelector::Tag(val)),
                    State::ReadingClass => current_seq.selectors.push(SimpleSelector::Class(val)),
                    State::ReadingId => current_seq.selectors.push(SimpleSelector::Id(val)),
                    State::Idle => {}
                }
            }
            state = State::Idle;
        };
    }

    macro_rules! commit_seq {
        ($comb:expr) => {
            flush!();
            if current_seq.selectors.is_empty() {
                return Err(ParseError::EmptySequence(format!("{:?}", $comb)));
            }
            if first.is_none() && pending_combinator.is_none() {
                first = Some(current_seq.clone());
            } else if let Some(prev_comb) = pending_combinator.take() {
                rest.push((prev_comb, current_seq.clone()));
            }
            pending_combinator = Some($comb);
            current_seq = SelectorSequence::default();
        };
    }

    while let Some(&c) = chars.peek() {
        match c {
            ' ' | '\t' | '\n' => {
                chars.next();
                flush!();

                while matches!(chars.peek(), Some(' ') | Some('\t') | Some('\n')) {
                    chars.next();
                }

                let comb = match chars.peek() {
                    Some(&'>') => {
                        last_explicit_comb = Some('>');
                        chars.next();
                        while matches!(chars.peek(), Some(' ') | Some('\t')) { chars.next(); }
                        Combinator::Child
                    }
                    Some(&'+') => {
                        last_explicit_comb = Some('+');
                        chars.next();
                        while matches!(chars.peek(), Some(' ') | Some('\t')) { chars.next(); }
                        Combinator::AdjacentSibling
                    }
                    Some(&'~') => {
                        last_explicit_comb = Some('~');
                        chars.next();
                        while matches!(chars.peek(), Some(' ') | Some('\t')) { chars.next(); }
                        Combinator::GeneralSibling
                    }
                    _ => Combinator::Descendant,
                };

                if chars.peek().is_none() {
                    let c = last_explicit_comb.unwrap_or(' ');
                    return Err(ParseError::TrailingCombinator(c));
                }

                commit_seq!(comb);
            }

            '>' | '+' | '~' => {
                last_explicit_comb = Some(c);
                chars.next();

                while matches!(chars.peek(), Some(' ') | Some('\t')) { chars.next(); }

                if chars.peek().is_none() {
                    return Err(ParseError::TrailingCombinator(c));
                }

                let comb = match c {
                    '>' => Combinator::Child,
                    '+' => Combinator::AdjacentSibling,
                    '~' => Combinator::GeneralSibling,
                    _ => unreachable!(),
                };

                commit_seq!(comb);
            }

            '.' => {
                flush!();
                chars.next();
                match chars.peek() {
                    None | Some(' ') | Some('>') | Some('+') | Some('~') | Some('.') | Some('#') => {
                        return Err(ParseError::EmptySelectorName('.'));
                    }
                    _ => {}
                }
                state = State::ReadingClass;
            }

            '#' => {
                flush!();
                chars.next();
                match chars.peek() {
                    None | Some(' ') | Some('>') | Some('+') | Some('~') | Some('.') | Some('#') => {
                        return Err(ParseError::EmptySelectorName('#'));
                    }
                    _ => {}
                }
                state = State::ReadingId;
            }

            '*' => {
                flush!();
                current_seq.selectors.push(SimpleSelector::Universal);
                chars.next();
            }

            _ => {
                if state == State::Idle {
                    state = State::ReadingTag;
                }
                buffer.push(c);
                chars.next();
            }
        }
    }

    flush!();

    if current_seq.selectors.is_empty() {
        return Err(ParseError::EmptyInput);
    }

    match (first.take(), pending_combinator.take()) {
        (None, None) => Ok(ComplexSelector { first: current_seq, rest }),
        (Some(f), Some(comb)) => {
            rest.push((comb, current_seq));
            Ok(ComplexSelector { first: f, rest })
        }
        (Some(f), None) => Ok(ComplexSelector { first: f, rest }),
        (None, Some(_)) => unreachable!("pending_combinator ada tapi first belum di-set"),
    }
}