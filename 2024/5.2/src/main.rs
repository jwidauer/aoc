use std::{
    collections::HashMap,
    fs::{self},
    num::ParseIntError,
};

use anyhow::{anyhow, Result};

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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
        self.get_applicable_rules(update).iter().all(|rule| {
            let first = update.index_of(rule.first);
            let second = update.index_of(rule.second);
            first < second
        })
    }

    fn get_applicable_rules(&self, update: &Update) -> Self {
        let rules = self
            .rules
            .iter()
            .filter(|rule| update.contains_page(rule.first) && update.contains_page(rule.second))
            .cloned()
            .collect();

        Self::new(rules)
    }

    fn iter(&self) -> impl Iterator<Item = &Rule> {
        self.rules.iter()
    }
}

#[derive(Debug, Clone)]
struct Update {
    pages: Vec<u32>,
    page_to_index: HashMap<u32, usize>,
}

impl Update {
    fn new(pages: Vec<u32>) -> Self {
        let page_to_index =
            HashMap::from_iter(pages.iter().copied().enumerate().map(|(i, p)| (p, i)));

        Self {
            pages,
            page_to_index,
        }
    }

    fn from_str(s: &str) -> Result<Self> {
        let pages = s
            .split(',')
            .map(str::parse)
            .collect::<Result<Vec<_>, ParseIntError>>()?;
        Ok(Self::new(pages))
    }

    #[inline]
    fn middle_page(&self) -> u32 {
        let mid = self.pages.len() / 2;
        self.pages[mid]
    }

    #[inline]
    fn index_of(&self, page: u32) -> Option<usize> {
        self.page_to_index.get(&page).copied()
    }

    #[inline]
    fn contains_page(&self, page: u32) -> bool {
        self.page_to_index.contains_key(&page)
    }

    fn swap(&mut self, first_idx: usize, second_idx: usize) {
        let first = self.pages[first_idx];
        let second = self.pages[second_idx];

        // Swap the pages
        self.pages.swap(first_idx, second_idx);

        // Update the page to index mapping
        self.page_to_index.insert(first, second_idx);
        self.page_to_index.insert(second, first_idx);
    }

    fn sort_according_to(&self, rules: &RuleSet) -> Self {
        let applicable_rules = rules.get_applicable_rules(self);

        let mut prev = self.clone();
        loop {
            let mut cur = prev.clone();
            for rule in applicable_rules.iter() {
                let first = cur.index_of(rule.first).unwrap();
                let second = cur.index_of(rule.second).unwrap();
                if first > second {
                    cur.swap(first, second);
                }
            }

            if cur.pages == prev.pages {
                return cur;
            }
            prev = cur;
        }
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

    let incorrectly_ordered_updates: Vec<_> = updates
        .iter()
        .filter(|update| !ruleset.is_correctly_ordered(update))
        .cloned()
        .collect();

    println!(
        "{} out of {} updates are incorrectly ordered",
        incorrectly_ordered_updates.len(),
        updates.len()
    );

    let sorted_updates: Vec<_> = incorrectly_ordered_updates
        .iter()
        .map(|u| u.sort_according_to(&ruleset))
        .collect();

    let res: u32 = sorted_updates.iter().map(|u| u.middle_page()).sum();

    println!("Sum of middle pages: {}", res);

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
