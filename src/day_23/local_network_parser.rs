use crate::{day_23::model::{Connection, LocalNetwork}, helper::result::collect, parser::Parse, reader::{Line, VecLine}};


mod error {
    const PREFIX: &str = "[D-23 parser]";

    pub fn connection_re(err: regex::Error) -> String {
        format!("{} could not create connection RE due to error: {}", PREFIX, err)
    }

    pub fn regex_match(line_num: usize) -> String {
        format!("{} line #{} does not match connection RE", PREFIX, line_num)
    }
}

pub struct LocalNetworkParser {
    connection_re: regex::Regex
}

impl LocalNetworkParser {
    pub fn new() -> Result<LocalNetworkParser, String> {
        regex::Regex::new(r"^([a-z]+)-([a-z]+)$")
            .map(|connection_re|LocalNetworkParser { connection_re })
            .map_err(error::connection_re)
    }

    fn parse_line(&self, line: Line) -> Result<Connection, String> {
        match self.connection_re.captures(&line.text).map(|c|c.extract()) {
            Some((_, [left, right])) => Ok(Connection(left.to_string(), right.to_string())),
            None => Err(error::regex_match(line.number)),
        }
    }
}

impl Parse<LocalNetwork> for LocalNetworkParser {
    fn parse(&self, vec_line: VecLine) -> Result<LocalNetwork, String> {
        collect(vec_line.lines.into_iter().map(|line|self.parse_line(line)).collect()).map(LocalNetwork)
    }
}