use crate::{helper::result::{collect, zip}, parser::Parse, reader::{Line, VecLine}};

use super::models::{PageOrderingRule, RulesWithUpdates, UpdatePages};


pub struct RulesWithUpdatesParser {
    rule_re: regex::Regex,
    update_re: regex::Regex,
}

mod error {
    const PREFIX: &str = "[Parser D-05]";

    pub fn rule_regex_error(e: regex::Error) -> String {
        format!("{} failed to create rule regex, '{}'", PREFIX, e)
    }
    
    pub fn update_regex_error(e: regex::Error) -> String {
        format!("{} failed to create page update regex, '{}'", PREFIX, e)
    }

    pub fn rule_number_error(num: &str, line_num: usize) -> String {
        format!("{} failed to parse number '{}' from page ordering rules on line #{}", PREFIX, num, line_num)
    }

    pub fn update_regex_not_matcher_error(line_num: usize) -> String {
        format!("{} line #{} did not match the pattern for update pages", PREFIX, line_num)
    }

    pub fn update_page_parse_error(num: &str, line_num: usize, e: std::num::ParseIntError) -> String {
        format!("{} failed to parse page number '{}' on line #{} due to error '{}'", PREFIX, num, line_num, e)
    }
}

impl RulesWithUpdatesParser {
    pub fn new() -> Result<RulesWithUpdatesParser, String> {
        let rule_re = regex::Regex::new(r"^(\d+)\|(\d+)$")
            .map_err(error::rule_regex_error);
        let update_re = regex::Regex::new(r"^\d+(?:,\d+)+$")
            .map_err(error::update_regex_error);

        zip(rule_re, update_re, |rule_re, update_re|RulesWithUpdatesParser{ rule_re, update_re })
    }

    fn try_parse_rule(&self, line: &Line) -> Result<PageOrderingRule, String> {
        match self.rule_re.captures(&line.text).map(|c|c.extract()) {
            Some((_, [fst, snd])) => {
                let fst = fst.parse().map_err(|_|error::rule_number_error(fst, line.number));
                let snd = snd.parse().map_err(|_|error::rule_number_error(snd, line.number));
                zip(fst, snd, PageOrderingRule)
            },
            None => Err(format!("try_parse_rule None arm")),
        }
    }

    fn try_parse_update(&self, line: &Line) -> Result<UpdatePages, String> {
        if !self.update_re.is_match(&line.text) {
            return Err(error::update_regex_not_matcher_error(line.number));
        }

        let page_numbers = line.text.split(",")
            .map(|num|num.parse::<u32>().map_err(|e|error::update_page_parse_error(num, line.number, e)))
            .collect();

        collect(page_numbers).map(UpdatePages)
    }
}

impl Parse<RulesWithUpdates> for RulesWithUpdatesParser {
    fn parse(&self, vec_line: VecLine) -> Result<RulesWithUpdates, String> {
        let mut rules = vec![];
        let mut i = 0;
        while i < vec_line.lines.len() && self.rule_re.is_match(&vec_line.lines[i].text)
        {
            match self.try_parse_rule(&vec_line.lines[i]) {
                Ok(rule) => rules.push(rule),
                Err(err) => return Err(err),
            }
            i += 1;
        }
        
        let mut updates = vec![];
        while i < vec_line.lines.len() {
            match self.try_parse_update(&vec_line.lines[i]) {
                Ok(update) => updates.push(update),
                Err(err) => return Err(err),
            }
            i += 1;
        }

        Ok(RulesWithUpdates{ rules, updates })
    }
}