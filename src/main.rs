#![allow(unused_variables)]
use grimoire_search::search::Search;
use std::env;

fn main() {
    //////////////////////////////////////////////
    // Setup, grab the arguments, and run
    //
    let args: Vec<String> = env::args().collect();
    let mut search = Search::new(args);
    search.query_search_engine();
    for line in search.filtered_results() {
        println!("{line}");
    }
}
