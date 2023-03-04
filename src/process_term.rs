#![allow(warnings)]
use futures::executor::block_on;
use meilisearch_sdk::client::*;
use meilisearch_sdk::search::SearchResult;
use serde::{
    Deserialize,
    Serialize,
};

// NOTE: The `query_search_engine` function
// is called in the live code to populate
// the raw_results. For testing, they are
// populated manually.

// fn process_term(params: Vec<String>) -> String {
//     if params.len() == 1 {
//         "Ready...".to_string()
//     }
//     else {
//         if params[1].chars().last().unwrap() == '.'
//         {
//             "new file name.".to_string()
//         }
//         else {
//             let s = Search {
//                 term: params[1].to_string(),
//                 raw_results: None,
//                 filtered_results: None,
//             };
//             r#"osa- this is an example.org
// osa- anotehr example.org
// osa- it doesn't matter what's here.org"#
//                 .to_string()
//         }
//     }
// }

// impl Search {
//     pub fn process_raw_results(&mut self) {
//         let filtered_results: Vec<String> = vec![];
//         for result in
//             self.raw_results.as_ref().unwrap()
//         {
//             dbg!(result);
//         }
//         self.filtered_results = Some(vec![
//             "this is alfa.org".to_string(),
//         ])
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

// #[test]
// fn return_ready_when_no_term_is_passed_in() {
// }

// #[test]
// fn basic_search_test() {
//     // GIVEN
//     let mut s = Search {
//         term: "osa- example".to_string(),
//         raw_results: Some(vec![Page {
//             id: "alfa_id".to_string(),
//             fileName: Some(
//                 "this is alfa.org".to_string(),
//             ),
//         }]),
//         filtered_results: None,
//     };
//     // WHEN
//     &s.process_raw_results();
//     // THEN
//     assert_eq!(
//         "this is alfa.org".to_string(),
//         s.filtered_results.unwrap()[0],
//     )
// }

// #[test]
// fn basic_ready_test() {
//     let params =
//         vec!["path_placeholder".to_string()];
//     assert_eq!(
//         "Ready...",
//         process_term(params)
//     );
// }

// #[test]
// fn return_new_file_name() {
//     let params = vec![
//         "path_placeholder".to_string(),
//         "new file name.".to_string(),
//     ];
//     assert_eq!(
//         "new file name.".to_string(),
//         process_term(params)
//     );
// }

// #[test]
// fn get_basic_results() {
//     let params = vec![
//         "path_placeholder".to_string(),
//         "osa- example".to_string(),
//     ];
//     assert_eq!(
//         r#"osa- this is an example.org
// osa- anotehr example.org
// osa- it doesn't matter what's here.org"#
//             .to_string(),
//         process_term(params)
//     );
// }

// TODO: make new filename with whitespace
// after the dot.
//
// }
