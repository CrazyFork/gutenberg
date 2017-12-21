
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

    // customize serde's serilize result
    impl ser::Serialize for Section {
    fn serialize<S>(&self, serializer: S) -> StdResult<S::Ok, S::Error> where S: ser::Serializer {
        let mut state = serializer.serialize_struct("section", 13)?;
    }
    ```


    * serde_derive
        * a package bundled with dependencies.


* lazy-static.rs, A small macro for defining lazy evaluated static variables in Rust.
    * https://github.com/rust-lang-nursery/lazy-static.rs

* pulldown-cmark: a mark down parser

* slug, A small library for generating ASCII slugs from unicode strings
    https://github.com/Stebalien/slug-rs

* sass_rs, Rust library wrapper for libsass (sass-sys).
    https://github.com/compass-rs/sass-rs
