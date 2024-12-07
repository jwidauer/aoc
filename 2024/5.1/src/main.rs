use std::{
    collections::HashMap,
    fs::{self},
    num::ParseIntError,
};

use anyhow::{anyhow, Result};

struct Rule {
    first: u32,
    second: u32,
}

impl Rule {
    fn new(first: u32, second: u32) -> Self {
        Self { first, second }
    }

    fn from_str(s: &str) -> Result<Self> {
        let mut parts = s.split('|');
        let first = parts.next().unwrap().parse()?;
        let second = parts.next().unwrap().parse()?;

        Ok(Self::new(first, second))
    }
}

struct RuleSet {
    rules: Vec<Rule>,
}

impl RuleSet {
    fn new(rules: Vec<Rule>) -> Self {
        Self { rules }
    }

    fn from_str(s: &str) -> Result<Self> {
        let rules = s.lines().map(Rule::from_str).collect::<Result<Vec<_>>>()?;
        Ok(Self::new(rules))
    }

    fn is_correctly_ordered(&self, update: &Update) -> bool {
        let update_pages: HashMap<u32, usize> = HashMap::from_iter(
            update
                .pages
                .iter()
                .copied()
                .enumerate()
                .map(|(i, p)| (p, i)),
        );
        let applicable_rules: Vec<_> = self
            .rules
            .iter()
            .filter(|rule| {
                update_pages.contains_key(&rule.first) && update_pages.contains_key(&rule.second)
            })
            .collect();

        applicable_rules.iter().all(|rule| {
            let first = update_pages[&rule.first];
            let second = update_pages[&rule.second];
            first < second
        })
    }
}

struct Update {
    pages: Vec<u32>,
}

impl Update {
    fn new(pages: Vec<u32>) -> Self {
        Self { pages }
    }

    fn from_str(s: &str) -> Result<Self> {
        let pages = s
            .split(',')
            .map(str::parse)
            .collect::<Result<Vec<_>, ParseIntError>>()?;
        Ok(Self::new(pages))
    }

    fn middle_page(&self) -> u32 {
        let mid = self.pages.len() / 2;
        self.pages[mid]
    }
}

fn main() -> Result<()> {
    let input = fs::read_to_string("input.txt")?;
    let (rules, updates) = input
        .split_once("\n\n")
        .ok_or(anyhow!("Couldn't find blank line!"))?;

    let ruleset = RuleSet::from_str(rules)?;

    let updates = updates
        .lines()
        .map(Update::from_str)
        .collect::<Result<Vec<_>>>()?;

    let correctly_ordered_updates: Vec<_> = updates
        .iter()
        .filter(|update| ruleset.is_correctly_ordered(update))
        .collect();

    println!(
        "{} out of {} updates are correctly ordered",
        correctly_ordered_updates.len(),
        updates.len()
    );

    let res: u32 = correctly_ordered_updates
        .iter()
        .map(|update| update.middle_page())
        .sum();

    println!("{}", res);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rule_from_str() {
        let rule = Rule::from_str("1|2").unwrap();
        assert_eq!(rule.first, 1);
        assert_eq!(rule.second, 2);
    }

    #[test]
    fn test_ruleset_from_str() {
        let ruleset = RuleSet::from_str("1|2\n3|4").unwrap();
        assert_eq!(ruleset.rules.len(), 2);
        assert_eq!(ruleset.rules[0].first, 1);
        assert_eq!(ruleset.rules[0].second, 2);
        assert_eq!(ruleset.rules[1].first, 3);
        assert_eq!(ruleset.rules[1].second, 4);
    }

    #[test]
    fn test_update_from_str() {
        let update = Update::from_str("1,2,3").unwrap();
        assert_eq!(update.pages.len(), 3);
        assert_eq!(update.pages[0], 1);
        assert_eq!(update.pages[1], 2);
        assert_eq!(update.pages[2], 3);
    }

    #[test]
    fn test_is_correctly_ordered() {
        let ruleset = RuleSet::from_str("1|2\n3|4\n3|5\n5|4").unwrap();
        let update = Update::from_str("1,2,3,5,4").unwrap();
        assert!(ruleset.is_correctly_ordered(&update));
        let update = Update::from_str("3,4,2,5,1").unwrap();
        assert!(!ruleset.is_correctly_ordered(&update));
        let update = Update::from_str("1,2,3,4,5").unwrap();
        assert!(!ruleset.is_correctly_ordered(&update));
    }

    #[test]
    fn test_middle_page() {
        let update = Update::from_str("1,2,3,4,5").unwrap();
        assert_eq!(update.middle_page(), 3);
    }
}
