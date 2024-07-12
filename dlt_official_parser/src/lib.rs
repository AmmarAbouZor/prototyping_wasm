use parsers::{dlt::DltParser, Parser as InternParser};
use plugins_api::{
    parser::{Attachment, ParseError, ParseReturn, ParseYield, Parser},
    parser_export,
};

struct PluginParser {
    parser: DltParser<'static>,
}

impl PluginParser {
    fn new(parser: DltParser<'static>) -> Self {
        Self { parser }
    }
}

fn parse_intern<'a>(
    parser: &mut DltParser<'static>,
    data: &[u8],
    timestamp: Option<u64>,
) -> Result<ParseReturn, ParseError> {
    match parser.parse(data, timestamp) {
        Ok((remain, opt)) => {
            let offset = (data.len() - remain.len()) as u64;
            let ret_val = match opt {
                Some(yld) => match yld {
                    parsers::ParseYield::Message(msg) => Some(ParseYield::Message(msg.to_string())),
                    parsers::ParseYield::Attachment(att) => {
                        Some(ParseYield::Attachment(attachment_from(att)))
                    }
                    parsers::ParseYield::MessageAndAttachment((msg, att)) => Some(
                        ParseYield::MessageAndAttachment((msg.to_string(), attachment_from(att))),
                    ),
                },
                None => None,
            };

            Ok(ParseReturn {
                consumed: offset,
                value: ret_val,
            })
        }
        Err(err) => Err(parse_error_from(err)),
    }
}

fn attachment_from(att: parsers::Attachment) -> Attachment {
    Attachment {
        data: att.data,
        name: att.name,
        size: att.size as u64,
        messages: att.messages.into_iter().map(|m| m as u64).collect(),
        created_date: att.created_date,
        modified_date: att.modified_date,
    }
}

fn parse_error_from(err: parsers::Error) -> ParseError {
    match err {
        parsers::Error::Parse(msg) => ParseError::Parse(msg),
        parsers::Error::Incomplete => ParseError::Incomplete,
        parsers::Error::Eof => ParseError::Eof,
    }
}

impl Parser for PluginParser {
    fn create(
        _general_configs: plugins_api::parser::ParserConfig,
        _config_path: Option<std::path::PathBuf>,
    ) -> Result<Self, plugins_api::parser::InitError>
    where
        Self: Sized,
    {
        // This should be read from plugin configs file
        let with_storage_header = true;

        let mut p = DltParser::default();
        p.with_storage_header = with_storage_header;

        Ok(PluginParser::new(p))
    }

    fn parse(
        &mut self,
        data: &[u8],
        timestamp: Option<u64>,
    ) -> impl IntoIterator<Item = Result<ParseReturn, ParseError>> + Send {
        let mut results = Vec::new();
        let mut slice = &data[0..];
        loop {
            match parse_intern(&mut self.parser, slice, timestamp) {
                Ok(res) => {
                    slice = &slice[res.consumed as usize..];
                    results.push(Ok(res));
                }
                Err(err) => {
                    results.push(Err(err));
                    return results;
                }
            }
        }
    }
}

parser_export!(PluginParser);
