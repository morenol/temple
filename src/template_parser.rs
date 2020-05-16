use crate::error::{Error, ErrorKind, Result, SourceLocation};
use crate::keyword::{RegexEnum, KEYWORDS, ROUGH_TOKENIZER};
use crate::renderer::{ComposedRenderer, RawTextRenderer, Render};
use crate::template_env::TemplateEnv;
use crate::token::Token;
use regex::Regex;
use std::sync::RwLock;

#[derive(Debug)]
pub struct TemplateParser<'a, 'b> {
    template_body: RwLock<&'a str>,
    env: RwLock<&'b TemplateEnv>,
    rough_tokenizer: Regex,
    keywords: Regex,
    text_blocks: RwLock<Vec<TextBlockInfo>>,
    current_block_info: RwLock<TextBlockInfo>,
    lines: RwLock<Vec<LineInfo>>,
    current_line_info: RwLock<LineInfo>,
}

impl<'a, 'b> TemplateParser<'a, 'b> {
    pub fn new(body: &'a str, env: &'b TemplateEnv) -> Result<Self> {
        let rough_tokenizer = Regex::new(&ROUGH_TOKENIZER[..ROUGH_TOKENIZER.len() - 1]).unwrap();
        let keywords = Regex::new(&KEYWORDS[..KEYWORDS.len() - 1]).unwrap();

        Ok(Self {
            template_body: RwLock::new(body),
            env: RwLock::new(env),
            rough_tokenizer,
            keywords,
            text_blocks: RwLock::new(vec![]),
            current_block_info: RwLock::new(TextBlockInfo::default()),
            lines: RwLock::new(vec![]),
            current_line_info: RwLock::new(LineInfo::default()),
        })
    }

    fn fine_parsing(&self, renderer: &mut ComposedRenderer<'a>) -> Result<()> {
        let mut statements_stack: StatementInfoList = vec![];
        let root = StatementInfo::new(StatementInfoType::TemplateRoot, Token::Unknown, renderer);
        statements_stack.push(root);
        for orig_block in self.text_blocks.read().unwrap().iter() {
            match orig_block.mode {
                TextBlockType::RawBlock | TextBlockType::RawText => {
                    if orig_block.range.size() == 0 {
                        continue;
                    }
                    let text = self.template_body.read().unwrap();
                    let new_renderer = RawTextRenderer::new(
                        &text[orig_block.range.start..orig_block.range.size()],
                    );
                    statements_stack
                        .last()
                        .unwrap()
                        .current_composition
                        .add_renderer(Box::new(new_renderer));
                }
                _ => {}
            }
        }
        Ok(())
    }

    pub fn parse(&mut self) -> Result<ComposedRenderer<'a>> {
        match self.rough_parsing() {
            Ok(_) => {
                let len_of_temp = self.template_body.read().unwrap().len();
                self.finish_current_line(len_of_temp);
                if let TextBlockType::RawBlock = self.current_block_info.read().unwrap().mode {
                    return Err(Error::from(ErrorKind::ExpectedRawEnd(SourceLocation::new(
                        len_of_temp,
                        len_of_temp + 2, // TODO: THERE is not handling of expected end of comment????
                    ))));
                }
                self.finish_current_block(len_of_temp, TextBlockType::RawText, None);
                let mut renderer = ComposedRenderer::new();
                self.fine_parsing(&mut renderer)?;
                Ok(renderer)
            }
            Err(error) => Err(error),
        }
    }

    fn rough_parsing(&mut self) -> Result<()> {
        let match_begin = self
            .rough_tokenizer
            .captures_iter(&self.template_body.read().unwrap());

        for capture in match_begin {
            // This does not seem idiomatic to rust
            let mut match_start = 0;
            let mut n_regex = 0;
            for i in 1..9 {
                if let Some(m) = capture.get(i) {
                    n_regex = i - 1;
                    match_start = m.start();
                    break;
                };
            }

            match RegexEnum::n(n_regex).unwrap() {
                RegexEnum::NewLine => {
                    self.finish_current_line(match_start);
                    self.current_line_info.write().unwrap().range.start =
                        self.current_line_info.read().unwrap().range.end + 1;

                    if self.current_line_info.read().unwrap().range.start
                        < self.template_body.read().unwrap().len()
                    {
                        match self.current_block_info.read().unwrap().mode {
                            TextBlockType::RawText => {}
                            TextBlockType::LineStatement => {
                                self.finish_current_block(
                                    match_start,
                                    TextBlockType::RawText,
                                    None,
                                );
                                self.current_block_info.write().unwrap().range.start =
                                    self.current_line_info.read().unwrap().range.start;
                            }
                            _ => {}
                        }
                    }
                }
                RegexEnum::CommentBegin => match self.current_block_info.read().unwrap().mode {
                    TextBlockType::RawBlock => continue,
                    TextBlockType::RawText => {
                        self.finish_current_block(match_start, TextBlockType::Comment, None);
                        self.current_block_info.write().unwrap().range.start = match_start + 2;
                        self.current_block_info.write().unwrap().mode = TextBlockType::Comment;
                    }
                    _ => {
                        self.finish_current_line(match_start + 2);
                        return Err(Error::from(ErrorKind::UnexpectedCommentBegin(
                            SourceLocation::new(match_start, match_start + 2),
                        )));
                    }
                },
                RegexEnum::CommentEnd => match self.current_block_info.read().unwrap().mode {
                    TextBlockType::RawBlock => continue,
                    TextBlockType::Comment => {
                        self.current_block_info.write().unwrap().range.start =
                            self.finish_current_block(match_start, TextBlockType::RawText, None);
                    }
                    _ => {
                        self.finish_current_line(match_start + 2);
                        return Err(Error::from(ErrorKind::UnexpectedCommentEnd(
                            SourceLocation::new(match_start, match_start + 2),
                        )));
                    }
                },

                RegexEnum::ExprBegin => {}
                RegexEnum::ExprEnd => {}
                RegexEnum::StmtBegin => {}
                RegexEnum::StmtEnd => {}
                RegexEnum::RawBegin => {}
                RegexEnum::RawEnd => {}
            };
        }

        Ok(())
    }
    fn finish_current_block(
        &self,
        mut position: usize,
        next_block: TextBlockType,
        match_start: Option<usize>,
    ) -> usize {
        let mut new_position = position;

        match self.current_block_info.read().unwrap().mode {
            TextBlockType::RawBlock => {
                let current_position = match_start.unwrap_or(position);
                let original_position = position;
                position = self.strip_block_left(current_position + 2, current_position);
                new_position = self.strip_block_right(original_position);
            }
            TextBlockType::RawText => position = self.strip_block_left(position + 2, position),
            _ => {
                if let TextBlockType::RawText = next_block {
                    new_position = self.strip_block_right(position);
                }
            }
        };
        self.current_block_info.write().unwrap().range.end = position;
        self.text_blocks
            .write()
            .unwrap()
            .push(*self.current_block_info.read().unwrap());

        new_position
    }
    fn strip_block_left(&self, ctrl_char_pos: usize, end_offset: usize) -> usize {
        end_offset
    }
    fn strip_block_right(&self, position: usize) -> usize {
        position + 2
    }
    fn finish_current_line(&self, position: usize) {
        self.current_line_info.write().unwrap().range.end = position;
        self.lines
            .write()
            .unwrap()
            .push(*self.current_line_info.read().unwrap());
        self.current_line_info.write().unwrap().line_number += 1;
    }
}

#[derive(Debug, Clone, Copy)]
struct TextBlockInfo {
    range: Range,
    mode: TextBlockType,
}
impl Default for TextBlockInfo {
    fn default() -> Self {
        Self {
            range: Range { start: 0, end: 0 },
            mode: TextBlockType::RawText,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Range {
    start: usize,
    end: usize,
}

impl Range {
    pub fn size(&self) -> usize {
        self.end - self.start
    }
}

#[derive(Debug, Clone, Copy)]
enum TextBlockType {
    RawText,
    Expression,
    Statement,
    Comment,
    LineStatement,
    RawBlock,
}

#[derive(Debug, Clone, Copy)]
struct LineInfo {
    range: Range,
    line_number: usize,
}

impl Default for LineInfo {
    fn default() -> Self {
        Self {
            range: Range { start: 0, end: 0 },
            line_number: 0,
        }
    }
}

struct StatementInfo<'a, 'b> {
    mode: StatementInfoType,
    current_composition: &'a ComposedRenderer<'b>,
    compositions: Vec<ComposedRenderer<'a>>,
    token: Token,
    renderer: Option<Box<dyn Render + 'a>>,
}

enum StatementInfoType {
    TemplateRoot,
    IfStatement,
    ElseIfStatement,
    ForStatement,
    SetStatement,
    ExtendsStatement,
    BlockStatement,
    ParentBlockStatement,
    MacroStatement,
    MacroCallStatement,
    WithStatement,
    FilterStatement,
}

impl<'a, 'b> StatementInfo<'a, 'b> {
    pub fn new(
        mode: StatementInfoType,
        token: Token,
        renderers: &'a mut ComposedRenderer<'b>,
    ) -> Self {
        let compositions = vec![];
        let current_composition = renderers;
        Self {
            mode,
            token,
            current_composition,
            compositions,
            renderer: None,
        }
    }
}

type StatementInfoList<'a, 'b> = Vec<StatementInfo<'a, 'b>>;
