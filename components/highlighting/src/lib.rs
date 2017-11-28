#[macro_use]
extern crate lazy_static;
extern crate syntect;

use syntect::dumps::from_binary;
use syntect::parsing::SyntaxSet;
use syntect::highlighting::ThemeSet;

// :note, thread local variables
// for more info, see: https://doc.rust-lang.org/std/thread/struct.LocalKey.html
thread_local!{              
    pub static SYNTAX_SET: SyntaxSet = {
        // :note, std::include_bytes
        let mut ss: SyntaxSet = from_binary(include_bytes!("../../../sublime_syntaxes/newlines.packdump"));
        ss.link_syntaxes();
        ss
    };
}

lazy_static!{
    pub static ref THEME_SET: ThemeSet = from_binary(include_bytes!("../../../sublime_themes/all.themedump"));
}
