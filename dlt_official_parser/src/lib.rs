use std::iter;

use parsers::{dlt::DltParser, Parser as InternParser};
use plugins_api::{
    log,
    parser::{Attachment, ParseError, ParseReturn, ParseYield, Parser},
    parser_export,
};

struct PluginParser {
    parser: DltParser<'static>,
}

impl PluginParser {
    fn new(parser: DltParser<'static>) -> Self {
        log::error!("TEST LOGGING: CLIENT ERROR: New function called");
        Self { parser }
    }
}

fn parse_intern(
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
        log::warn!("TEST LOGGING: CLIENT WARN: Create function called");

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
        // Test code for log functionality
        // log::warn!("CLIENT WARN: test parse called");
        // log::info!("CLIENT INFO: test parse called");
        // log::trace!("CLIENT TRACE: test parse called");

        let mut slice = &data[0..];
        let mut encounter_error = false;

        iter::from_fn(move || {
            if encounter_error {
                return None;
            }
            match parse_intern(&mut self.parser, slice, timestamp) {
                Ok(res) => {
                    slice = &slice[res.consumed as usize..];
                    Some(Ok(res))
                }
                Err(err) => {
                    encounter_error = true;
                    // log::warn!("Parse encounter error: {:#?}", err);
                    Some(Err(err))
                }
            }
        })
    }
}

parser_export!(PluginParser);
