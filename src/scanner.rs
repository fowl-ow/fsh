pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

#[derive(Debug)]
pub enum TokenKind {
    Word,
    Newline,
    // Operator,
    // Pipe,
}

#[derive(Debug, Clone, Copy)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

pub struct ScannedInput {
    pub source: String,
    pub tokens: Vec<Token>,
}

enum State {
    Idle,
    InWord { start: usize },
}

pub fn scan(input: String) -> ScannedInput {
    let mut state: State = State::Idle;
    let mut tokens: Vec<Token> = Vec::new();

    let it = input.char_indices().peekable();
    // while let Some((idx, ch)) = it.next() {
    for (idx, ch) in it {
        // let peek = it.peek();
        match state {
            State::Idle => match ch {
                '\n' => tokens.push(Token {
                    kind: TokenKind::Newline,
                    span: Span {
                        start: idx,
                        end: idx + 1,
                    },
                }),
                ' ' | '\t' => {}
                _ => state = State::InWord { start: idx },
            },
            State::InWord { start } => match ch {
                '\n' => {
                    tokens.push(Token {
                        kind: TokenKind::Word,
                        span: Span { start, end: idx },
                    });
                    tokens.push(Token {
                        kind: TokenKind::Newline,
                        span: Span {
                            start: idx,
                            end: idx + 1,
                        },
                    });
                    state = State::Idle;
                }
                ' ' | '\t' => {
                    tokens.push(Token {
                        kind: TokenKind::Word,
                        span: Span { start, end: idx },
                    });
                    state = State::Idle;
                }
                _ => {}
            },
        }
    }
    if let State::InWord { start } = state {
        tokens.push(Token {
            kind: TokenKind::Word,
            span: Span {
                start,
                end: input.len(),
            },
        })
    }

    ScannedInput {
        source: input,
        tokens,
    }
}
