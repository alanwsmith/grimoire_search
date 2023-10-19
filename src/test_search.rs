use crate::page::Page;
use crate::search::Search;

/////////////////////////////////////////
// Phase 1
//
// - [x] Return "Ready..." if no argumets are
// passed in
//
// - [x] Return verbatum if there's a dot
// (i.e. `.`) at the end of the search term
// to create a new file.
//
// - [x] Run a search using the argument as
// a parameter when only one is passed in
//
// - [x] Limit to 14 results - NOTE: This will
// be done directly in the query. Testing
// here is not required.
//
// - [x] Don't return files that begin with:
//          biz-
//          msync-
//          private-
//          self-
//          tour-
//          work-
//
// - [x] Allow these keywords if called explictly:
//          biz-
//          msync-
//          private-
//          self-
//          tour-
//          work-
//

/////////////////////////////////////////
// Phase 2
//
// - [] Return a search history of the
// last 10 items if there's no search query
// argument
//

/////////////////////////////////////////
// Skipped for now until they become a problem.
//
// - SKIP: Handle more than one param being passed in.
// REASON: For now, the first one is used and any
// additional ones are ignored.
//
// - SKIP: If a term comes in with a dot followed
// by white space, remove the whitespace and
// return the name with just the dot. REASON: For now
// just make sure not to do that. Worst case is
// if you do you get search results instead
// of making the new file.
//
// - SKIP: Was thinking about preventing resutls until
// the first dash. That would limit the ability
// to do generic searches if you don't know
// what the keyword token is. So, not doing
// that.
//
// - SKIP: Setup so filter for `work-`, `msync-` etc.
// is case insensitive. REASON: This hasn't been
// a problem. I default to typing everything lower
// case for the keywords.
//
// - SKIP: Filter out `work-` etc even it's not
// the start of the string. REASON: I haven't
// had a problem makeing sure the filtered keywords
// are used at the start of the file names
//
// - SKIP: Move the filtered out keywords to a config
// file. REASON: the words haven't chaned in a long
// time and if they do it's easy to add
// a new harded coded one.

#[test]
fn return_ready_when_no_search_term_is_sent() {
    ////////////////////////////////
    // Seand back "Ready..." if no
    // search term was provided
    //
    ////////////////////////////////
    // GIVEN
    //
    let params: Vec<String> =
        vec!["placeholder/path".to_string()];
    ////////////////////////////////
    // WHEN
    //
    let s = Search::new(params);
    ////////////////////////////////
    // THEN
    //
    assert_eq!(
        "Ready...".to_string(),
        s.filtered_results()[0],
    );
}

#[test]
fn return_string_directly_if_there_is_a_dot_at_the_end(
) {
    ////////////////////////////////
    // Append `org` to anything that
    // ends with a dot and send
    // just it back for making new
    // files. 
    //
    ////////////////////////////////
    // GIVEN
    //
    let params: Vec<String> = vec![
        "placeholder/path".to_string(),
        "file name with ending dot.".to_string(),
    ];
    ////////////////////////////////
    // WHEN
    //
    let s = Search::new(params);
    ////////////////////////////////
    // THEN
    //
    assert_eq!(
        "file name with ending dot.org".to_string(),
        s.filtered_results()[0],
    );
}

#[test]
fn return_basic_search_results() {
    ////////////////////////////////
    // This is the baseline test for
    // getting a result set back
    //
    ////////////////////////////////
    // GIVEN
    //
    let params: Vec<String> = vec![
        "placeholder/path".to_string(),
        "placeholder search pattern".to_string(),
    ];
    ///////////////////////////////
    // WHEN
    //
    let mut s = Search::new(params);
    s.search_engine_response = Some(vec![Page {
        id: "alfa".to_string(),
        fileName: "file name alfa".to_string(),
    }]);
    /////////////////////////////////
    // THEN
    //
    assert_eq!(
        "file name alfa".to_string(),
        s.filtered_results()[0],
    );
}

#[test]
fn allow_explict_keywords() {
    ////////////////////////////////
    // Let stuff in like `work-` if you
    // search for it explicitly.
    //
    ////////////////////////////////
    // GIVEN
    //
    let params: Vec<String> = vec![
        "placeholder/path".to_string(),
        "work- test thing".to_string(),
    ];
    ///////////////////////////////
    // WHEN
    //
    let mut s = Search::new(params);
    s.search_engine_response = Some(vec![
        Page {
            id: "alfa".to_string(),
            fileName: "self- example 1.org"
                .to_string(),
        },
        Page {
            id: "alfa".to_string(),
            fileName: "work- example 1.org"
                .to_string(),
        },
    ]);
    /////////////////////////////////
    // THEN
    //
    assert_eq!(
        "work- example 1.org".to_string(),
        s.filtered_results()[0],
    );
    assert_eq!(1, s.filtered_results().len(),);
}
