use std::fmt::Display;

use crate::helper::display::vector_display;


#[derive(Eq, PartialEq, Clone, Debug)]
pub struct PageOrderingRule(pub u32, pub u32);


impl Display for PageOrderingRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let PageOrderingRule(left, right) = self;
        write!(f, "{}|{}", left, right)
    }
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct UpdatePages(pub Vec<u32>);

impl Display for UpdatePages {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let UpdatePages(pages) = self;
        write!(f, "[{}]", vector_display(pages, ","))
    }
}


#[derive(Eq, PartialEq, Clone, Debug)]
pub struct RulesWithUpdates {
    pub rules: Vec<PageOrderingRule>,
    pub updates: Vec<UpdatePages>,
}

impl Display for RulesWithUpdates {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ==> {}", vector_display(&self.rules, ","), vector_display(&self.updates, ","))
    }
}