use parsers::{dlt::DltParser, Parser};
use wit_bindgen::generate;

generate!({
    path: "../../chipmunk/application/apps/indexer/plugins_api/wit/v_0.1.0",
    world: "parser",
    //TODO AAZ: Check the impact of duplicate_if_necessary on the performance
    ownership: Borrowing {
        duplicate_if_necessary: false
    },
});

static mut PARSER: Option<DltParser<'static>> = None;

struct PluginParser;

export!(PluginParser);

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
                        Some(ParseYield::Attachment(att.into()))
                    }
                    parsers::ParseYield::MessageAndAttachment((msg, att)) => Some(
                        ParseYield::MessageAndAttachment((msg.to_string(), att.into())),
                    ),
                },
                None => None,
            };

            Ok(ParseReturn {
                consumed: offset,
                value: ret_val,
            })
        }
        Err(err) => Err(err.into()),
    }
}

impl Guest for PluginParser {
    fn init(
        _general_configs: ParserConfig,
        _plugin_configs: Option<_rt::String>,
    ) -> Result<(), InitError> {
        // This should be read from plugin configs file
        let with_storage_header = true;

        let mut p = DltParser::default();
        p.with_storage_header = with_storage_header;
        // SAFETY: all functions on the host take mutable reference and can't be called
        // concurrently
        unsafe {
            PARSER = Some(p);
        }

        Ok(())
    }

    fn parse(
        data: _rt::Vec<u8>,
        timestamp: Option<u64>,
    ) -> _rt::Vec<Result<ParseReturn, ParseError>> {
        let mut results = Vec::new();
        let mut slice = &data[0..];
        // SAFETY: all functions on the host take mutable reference and can't be called
        // concurrently
        let parser = unsafe { PARSER.as_mut().unwrap() };
        loop {
            match parse_intern(parser, slice, timestamp) {
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

    fn parse_with_add(data: _rt::Vec<u8>, timestamp: Option<u64>) {
        let mut slice = &data[0..];
        // SAFETY: all functions on the host take mutable reference and can't be called
        // concurrently
        let parser = unsafe { PARSER.as_mut().unwrap() };
        loop {
            match parse_intern(parser, slice, timestamp) {
                Ok(res) => {
                    slice = &slice[res.consumed as usize..];
                    add(Ok(&res));
                }
                Err(err) => {
                    add(Err(&err));
                    return;
                }
            }
        }
    }
}

impl From<parsers::Attachment> for Attachment {
    fn from(att: parsers::Attachment) -> Self {
        Self {
            data: att.data,
            name: att.name,
            size: att.size as u64,
            messages: att.messages.into_iter().map(|m| m as u64).collect(),
            created_date: att.created_date,
            modified_date: att.modified_date,
        }
    }
}

impl From<parsers::Error> for ParseError {
    fn from(err: parsers::Error) -> Self {
        match err {
            parsers::Error::Parse(msg) => ParseError::Parse(msg),
            parsers::Error::Incomplete => ParseError::Incomplete,
            parsers::Error::Eof => ParseError::Eof,
        }
    }
}
