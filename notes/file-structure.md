* src
```
    src
    ├── cli.rs                      # parse cli options by main.rs
    ├── cmd                         # subcommands goes here
    │   ├── build.rs                # build command
    │   ├── init.rs                 # init
    │   ├── livereload.js
    │   ├── mod.rs                  # 
    │   └── serve.rs                # create a webserver & websocket server, sending changes down to websocket server stream
    ├── console.rs                  # console utility
    ├── main.rs                     # main entry
    ├── prompt.rs                   # asking questions
    └── rebuild.rs                  
```
* components
```
    components
    ├── config                                      # provie functionality to parse config from string or file
    │   └── src
    ├── content                                     
    │   ├── benches
    │   └── src
    ├── errors                                      # create errors using error-chain, which i haven't quited figure it out how to use it yet
    │   └── src
    ├── front_matter
    │   └── src
    ├── highlighting
    │   └── src
    ├── pagination
    │   └── src
    ├── rendering
    │   ├── benches
    │   ├── examples
    │   ├── src
    │   └── tests
    ├── site
    │   ├── benches
    │   ├── src
    │   ├── test_site
    │   │   ├── content
    │   │   │   ├── paginated
    │   │   │   └── posts
    │   │   │       ├── no-section
    │   │   │       ├── tutorials
    │   │   │       │   ├── devops
    │   │   │       │   └── programming
    │   │   │       └── with-assets
    │   │   ├── sass
    │   │   ├── static
    │   │   │   └── scripts
    │   │   ├── templates
    │   │   │   └── shortcodes
    │   │   └── themes
    │   │       └── sample
    │   │           ├── sass
    │   │           ├── static
    │   │           └── templates
    │   └── tests
    ├── taxonomies
    │   └── src
    ├── templates
    │   └── src
    │       └── builtins
    │           ├── internal
    │           └── shortcodes
    └── utils
        ├── src
        └── templates

```