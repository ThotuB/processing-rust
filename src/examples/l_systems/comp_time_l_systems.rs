// use crate::examples::l_systems::l_system::LSystemConfig;
//
// impl LSystemConfig {
//     const fn const_create<const L: usize>(start: &str, rules: [(char, &str); L]) {
//         let mut variables = [0 as char; L];
//         let mut i = 0;
//         while i < L {
//             variables[i] = rules[i].0;
//             i += 1;
//         }
//
//         let mut i = 0;
//         let start = start.as_bytes();
//         while i < start.len() {
//             let c = start[i] as char;
//             let mut is_valid = false;
//
//             let mut j = 0;
//             while j < L {
//                 if c == variables[j] {
//                     is_valid = false;
//                     break;
//                 }
//                 j += 1;
//             }
//             if !is_valid {
//                 panic!("InvalidStartCharacter");
//             }
//
//             i += 1;
//         }
//
//         LSystemConfig::const_check_rules(variables, rules);
//
//         // LSystemConfig {
//         //     start: start.to_string(),
//         //     rules: rules.iter().map(|(k, v)| (*k, v.to_string())).collect(),
//         // }
//     }
//
//     const fn const_check_rules<const L: usize>(variables: [char; L], rules: [(char, &str); L]) {
//         let mut valid_chars: [char; L + 4] = [0 as char; L + 4];
//         // valid_chars.extend(vec!['[', ']', '+', '-']);
//
//         // for to while
//         let mut i = 0;
//         while i < rules.len() {
//             let (k, rule) = rules[i];
//             let mut j = 0;
//             while j < rule.len() {
//                 let c = rule.chars().nth(j).unwrap();
//                 if !valid_chars.contains(&c) {
//                     panic!(
//                         "InvalidRuleCharacter: {} in rule: {} at index: {}",
//                         c, rule, j
//                     )
//                 }
//                 j += 1;
//             }
//
//             let backet_diff = rule.chars().filter(|c| *c == '[').count() as i32
//                 - rule.chars().filter(|c| *c == ']').count() as i32;
//             if backet_diff > 0 {
//                 panic!("TooManyOpeningBrackets");
//             }
//             if backet_diff < 0 {
//                 panic!("TooManyClosingBrackets");
//             }
//             i += 1;
//         }
//     }
// }
//
// #[macro_export]
// macro_rules! l_system_config {
//     ($start:expr, $($key:expr => $value:expr),+) => {
//         $crate::examples::l_systems::l_system::LSystemConfig::create($start, &[$(($key, $value)),+])
//     };
// }
