
* 项目中组织 mod 的方式，在src中的cmd将内部mod提到顶级位置，还有在 Cargo.toml 中的配置
* rayon lib 的用法需要注意下，very cool feature.



* raw string literials
    * https://doc.rust-lang.org/reference/tokens.html#raw-string-literals
    
    ```
    let theme_str = r#
    [extra]
    hello = "foo"
    a_value = 10
    "#;
    ```

