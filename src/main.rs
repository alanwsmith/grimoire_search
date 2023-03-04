#![allow(unused_variables)]
use grimoire_search::search::Search;
use std::env;
// use futures::executor::block_on;
// use serde::{
//     Deserialize,
//     Serialize,
// };

// use get_search_results_v3::process_term;

fn main() {
    //
    //////////////////////////////////////////////
    // Setup, grab the arguments, and run
    //
    let args: Vec<String> = env::args().collect();
    let mut search = Search::new(args);
    search.query_search_engine();
    for line in search.filtered_results() {
        println!("{line}");
    }

    // if args.len() == 1 {
    //     println!("Ready...")
    // }
    // else if args.len() == 2 {
    //     block_on(async move {
    //         let my_stuff = client
    //             .index("grimoire")
    //             .search()
    //             .with_limit(14)
    //             .with_query(args[1].as_str())
    //             .execute::<Page>()
    //             .await
    //             .unwrap()
    //             .hits;
    //         for thing in my_stuff {
    //             println!(
    //                 "{}",
    //                 thing.result.fileName.unwrap()
    //             );
    //         }
    //     })
    // }
}
