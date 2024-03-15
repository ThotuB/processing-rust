

use thiserror;

#[derive(Debug, Clone)]
pub struct LSystemConfig {
    start: String,
    rules: Vec<(char, String)>,
}

#[derive(Debug, thiserror::Error)]
pub enum LSystemConfigError {
    #[error("Invalid character in start string: {0}")]
    InvalidStartCharacter(char),
    #[error("Invalid character in rule: {0}\nrule: {1}\nindex: {2}")]
    InvalidRuleCharacter(char, String, usize),
    #[error("Too many closing brackets")]
    TooManyClosingBrackets,
    #[error("Too many opening brackets")]
    TooManyOpeningBrackets,
}

impl LSystemConfig {
    pub fn rules(start: &str, rules: &[(char, &str)]) -> Result<LSystemConfig, LSystemConfigError> {
        let variables = rules.iter().map(|(k, _)| *k).collect::<Vec<_>>();

        for c in start.chars() {
            if !variables.contains(&c) {
                return Err(LSystemConfigError::InvalidStartCharacter(c));
            }
        }

        LSystemConfig::check_rules(variables, rules)?;

        Ok(LSystemConfig {
            start: start.to_string(),
            rules: rules.iter().map(|(k, v)| (*k, v.to_string())).collect(),
        })
    }

    fn check_rules(variables: Vec<char>, rules: &[(char, &str)]) -> Result<(), LSystemConfigError> {
        let mut valid_chars = variables.clone();
        valid_chars.extend(vec!['[', ']', '+', '-']);

        for (_, rule) in rules.iter() {
            for (i, c) in rule.chars().enumerate() {
                if !valid_chars.contains(&c) {
                    return Err(LSystemConfigError::InvalidRuleCharacter(
                        c,
                        rule.to_string(),
                        i,
                    ));
                }
            }

            let backet_diff = rule.chars().filter(|c| *c == '[').count() as i32
                - rule.chars().filter(|c| *c == ']').count() as i32;
            if backet_diff > 0 {
                return Err(LSystemConfigError::TooManyOpeningBrackets);
            }
            if backet_diff < 0 {
                return Err(LSystemConfigError::TooManyClosingBrackets);
            }
        }

        Ok(())
    }

    fn apply_rules(&self, current: &str) -> String {
        let mut new_string = String::new();
        for c in current.chars() {
            let rule = self.rules.iter().find(|(k, _)| *k == c);
            if let Some((_, v)) = rule {
                new_string.push_str(v);
            } else {
                new_string.push(c);
            }
        }
        new_string
    }

    pub fn generate(&self, iterations: usize) -> String {
        let mut current = self.start.clone();
        for _ in 0..iterations {
            current = self.apply_rules(&current);
        }
        current
    }
}

#[macro_export]
macro_rules! l_system_config {
    ($start:expr, $($key:expr => $value:expr),+) => {
        $crate::examples::l_systems::l_system::LSystemConfig::rules($start, &[$(($key, $value)),+])
    };
}
