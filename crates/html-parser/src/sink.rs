//! Bridges `html5ever`'s tokenizer to our [`crate::builder::Builder`].

use crate::builder::Builder;
use crate::token::Token;
use html5ever::tokenizer::states::RawKind;
use html5ever::tokenizer::{TagKind, Token as HtToken, TokenSink, TokenSinkResult};

/// [`TokenSink`] implementation that forwards tokenizer output to [`Builder`].
pub(crate) struct Html5Sink {
    /// Incremental DOM builder owned for the lifetime of the `html5ever` tokenizer.
    pub(crate) builder: Builder,
}

impl Html5Sink {
    /// Wraps a fresh [`Builder`] that will receive tokens.
    pub(crate) fn new(builder: Builder) -> Self {
        Self { builder }
    }
}

impl TokenSink for Html5Sink {
    type Handle = ();

    /// Translates each `html5ever` token into our [`Token`] stream for [`Builder`].
    ///
    /// Returns [`TokenSinkResult::RawData`] after certain start tags so markup inside
    /// `<script>`, `<style>`, etc. stays literal per the HTML tokenizer state machine.
    fn process_token(&mut self, token: HtToken, _line_number: u64) -> TokenSinkResult<Self::Handle> {
        match token {
            HtToken::DoctypeToken(d) => {
                let name = d.name.as_ref().map(|t| t.to_string());
                let public_id = d.public_id.as_ref().map(|t| t.to_string());
                let system_id = d.system_id.as_ref().map(|t| t.to_string());
                self.builder.apply(Token::Doctype {
                    name,
                    public_id,
                    system_id,
                });
            }
            HtToken::TagToken(tag) => match tag.kind {
                TagKind::StartTag => {
                    let name = tag.name.to_string();
                    let attrs = tag
                        .attrs
                        .iter()
                        .map(|a| (a.name.local.to_string(), a.value.to_string()))
                        .collect();
                    self.builder.apply(Token::Start {
                        name: name.clone(),
                        attrs,
                        self_closing: tag.self_closing,
                    });
                    // Without this, `<` inside `<script>` would start bogus tags instead of raw text.
                    let lc = name.to_ascii_lowercase();
                    return match lc.as_str() {
                        "script" => TokenSinkResult::RawData(RawKind::ScriptData),
                        "style" => TokenSinkResult::RawData(RawKind::Rawtext),
                        "title" | "textarea" => TokenSinkResult::RawData(RawKind::Rcdata),
                        _ => TokenSinkResult::Continue,
                    };
                }
                TagKind::EndTag => {
                    let name = tag.name.to_string();
                    self.builder.apply(Token::End { name });
                }
            },
            HtToken::CharacterTokens(s) => {
                self.builder.apply(Token::Text(s.to_string()));
            }
            HtToken::NullCharacterToken => {
                self.builder.apply(Token::Text("\0".into()));
            }
            HtToken::CommentToken(s) => {
                self.builder.apply(Token::Comment(s.to_string()));
            }
            HtToken::ParseError(e) => {
                self.builder.apply(Token::ParseError(e.to_string()));
            }
            HtToken::EOFToken => {}
        }
        TokenSinkResult::Continue
    }
}
