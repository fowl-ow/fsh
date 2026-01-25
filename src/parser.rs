use crate::scanner::{ScannedInput, Span, Token, TokenKind};

pub struct ParsedInput {
    pub source: String,
    pub program: Program,
}

pub type Program = Vec<Command>;

pub enum Command {
    Simple { argv: Vec<Word> },
}

pub struct Word {
    pub span: Span,
}

impl Word {
    pub fn as_str<'a>(&self, source: &'a str) -> &'a str {
        &source[self.span.start..self.span.end]
    }
}

pub fn parse(input: ScannedInput) -> ParsedInput {
    let ScannedInput { source, tokens } = input;

    let mut program: Vec<Command> = Vec::new();

    let mut argv: Vec<Word> = Vec::new();
    for Token { kind, span } in tokens {
        match (kind, span) {
            (TokenKind::Word, span) => argv.push(Word { span }),
            (TokenKind::Newline, _) => {
                program.push(Command::Simple { argv });
                argv = Vec::new();
            }
        }
    }

    ParsedInput { source, program }
}
