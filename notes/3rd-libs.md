
* rayon, a parallesim data computed lib
* tera, rust template language engine


* chrono:
    * NaiveDateTime,  simple datetime without local or timezone info.
* syntect, Rust library for syntax highlighting using Sublime Text syntax definitions.
    * https://github.com/trishume/syntect
* serde

    ```
    #[serde(skip_serializing)] 


    // rename all fields
    #[serde(rename_all = "lowercase")]
    pub enum SortBy {
        /// Most recent to oldest
        Date,
        /// Lower order comes last
        Order,
        /// Lower weight comes first
        Weight,
        /// No sorting
        None,
    }
    ```

* lazy-static.rs, A small macro for defining lazy evaluated static variables in Rust.
    * https://github.com/rust-lang-nursery/lazy-static.rs

* pulldown-cmark: a mark down parser