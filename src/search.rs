use futures::executor::block_on;
use meilisearch_sdk::client::*;
use std::fs;

////////////////////////////////////
// TODO: Rename params to args
// to match main
//

use crate::get_credentials::get_credentials;
use crate::page::Page;

#[derive(Debug)]
pub struct Search {
    params: Vec<String>,
    pub search_engine_response: Option<Vec<Page>>,
}

impl Search {
    pub fn query_search_engine(&mut self) {
        ////////////////////////////////////////////////
        //// this looks for `env_name` for a credentials
        //// to make dev easier. If it doesn't find it,
        //// it uses the `cred_user` and `cred_key`
        //// fields to call out to the system keyring.
        let ms_url = "http://127.0.0.1:7700/";
        let env_key = "MEILISEARCH_TEST";
        let cred_key =
            "alan--meilisearch--scratchpad--admin-key";
        let cred_user = "alan";
        let ms_key =
            get_credentials(env_key, cred_key, cred_user);
        let client = Client::new(ms_url, ms_key.unwrap());
        if self.params.len() == 2 {
            block_on(async move {
                let my_stuff = client
                    .index("grimoire")
                    .search()
                    .with_limit(10)
                    .with_query(self.params[1].as_str())
                    .execute::<Page>()
                    .await
                    .unwrap()
                    .hits;
                self.search_engine_response = Some(
                    my_stuff
                        .iter()
                        .map(|ya| ya.result.clone())
                        .collect(),
                );
            });
        }
    }
}

impl Search {
    pub fn new(params: Vec<String>) -> Search {
        Search {
            params,
            search_engine_response: None,
        }
    }
}

///////////////////////////////////
// This could maybe do with some
// refactoring to reduce the nesting
//
impl Search {
    pub fn filtered_results(&self) -> Vec<String> {
        ///////////////////////////////////
        // Return ready if no search term was
        // passed in.
        //
        if self.params.len() == 1 {
            vec!["Ready...".to_string()]
        }
        ///////////////////////////////////
        // Handle empty strings
        //
        else if self.params[1].is_empty() == true {
            // NOTE: Skipping creating of this file
            // since it'll already be there and if not
            // can be created. TODO: Move this into
            // a config at some point
            let history_file_config_path = "/Users/alan/.config/grimoire-mode/search-history.txt";

            let history_text = fs::read_to_string(
                history_file_config_path,
            )
            .unwrap();

            history_text
                .lines()
                .take(10)
                .map(|line| line.to_string())
                .collect::<Vec<String>>()

            // let raw = fs::read_to_string(
            //     "~/Desktop/tmp.txt",
            // )
            // .unwrap();

            // let lines: Vec<&str> =
            //     raw.lines().collect();

            // vec!["Ready...".to_string()]
        }
        //////////////////////////////////
        // A search term exists so process it.
        //
        else {
            /////////////////////////////////
            // return directly if the search term
            // ends with a dot in order to tell
            // emacs to create a new file
            //
            if self.params[1].chars().last().unwrap()
                == '.'
            {
                vec![self.params[1].to_string()]
            }
            /////////////////////////////////
            // otherwise process the results
            // from the search engine
            //
            else {
                let mut results: Vec<String> = vec![];
                'page_loop: for page in self
                    .search_engine_response
                    .as_ref()
                    .unwrap()
                    .iter()
                {
                    let file_name =
                        page.fileName.to_string();
                    ///////////////////////////////////
                    // Filter stuff that shouldn't be
                    // in the results unless explicitly
                    // called. Opportunity to move this
                    // into a Vec is high
                    //
                    let filter_keys = vec![
                        String::from("biz-"),
                        String::from("msync-"),
                        String::from("neob-"),
                        String::from("private-"),
                        String::from("self-"),
                        String::from("work-"),
                        String::from("wrk-"),
                        String::from("alans-"),
                        String::from("alan-"),
                        String::from("tour-"),
                        String::from("data-"),
                    ];
                    for filter_key in filter_keys.iter() {
                        /////////////////////////////////
                        // TODO: Look up how to do multiple
                        // conditionals
                        //
                        if file_name.find(filter_key)
                            == Some(0)
                        {
                            if self.params[1]
                                .to_string()
                                .find(filter_key)
                                != Some(0)
                            {
                                continue 'page_loop;
                            }
                        }
                    }
                    /////////////////////////////////
                    // Add the filename to the result
                    // set if it hasn't been filtered out
                    //
                    results
                        .push(page.fileName.to_string());
                }
                results
            }
        }
    }
}
