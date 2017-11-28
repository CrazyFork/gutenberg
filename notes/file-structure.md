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
    │   │   ├── benches
    │   │   └── all.rs
    │   └── src
    │       ├── file_info.rs                        # FileInfo struct, various info perserved in this struct
    │       ├── lib.rs
    │       ├── page.rs                             #
    │       ├── section.rs                          # 
    │       └── sorting.rs                          # on how to sort pages
    ├── errors                                      # create errors using error-chain, which i haven't quited figure it out how to use it yet
    │   └── src
    ├── front_matter
    │   └── src                         
    │       ├── lib.rs
    │       ├── page.rs                             # .md file 中的 page info, 在 .md 文件中的 header 定义
    │       └── section.rs                          # .md file 中的 section info. 在_index.md文件中的header定义， 定义了如何从str中去parse
    ├── highlighting                                # provide syntax highlight.
    │   └── src
    │       └── lib.rs                      
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
    ├── templates                                   # tera templates 相关的代码, 注册了一些通用的 templates
    │   └── src
    │       ├── builtins
    │       │   ├── anchor-link.html
    │       │   ├── internal
    │       │   │   └── alias.html                  # 这重定向写在html页面中的比较6
    │       │   ├── rss.xml
    │       │   ├── shortcodes
    │       │   └── sitemap.xml                     # 这 sitemap 比我预想的要简单的多了
    │       ├── filters.rs                          # markdown -> html transpile, and base64 encoding, decoding, 在 tera template engine 中注册的 filter
    │       ├── global_fns.rs                       # 3 个工具函数, 2个通过 path 字符串找对应的 Page, Section实例, 一个拼接 url
    │       └── lib.rs                              # 
    └── utils
        ├── src
        │   ├── fs.rs                               # file utils, create & read file or dirs
        │   ├── lib.rs
        │   ├── site.rs                             # get_reading_analytics, resolve_internal_link, 两个函数, 第一个返回大致的阅读时间, 第二个用于 resolve relative link to absolute one in markdown file.
        │   └── templates.rs                        # 用于操作 Tera template, 没怎么看懂
        └── templates
            ├── child.html
            ├── included.html
            ├── index.html
            ├── macros.html
            └── using-macros.html

```