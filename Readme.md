`actix_header` is a shortcut to implement actix-web `Header`, you only need to implement `From<String>` and `Into<String>` for your type, and then you can use your type in handlers directly.

example:

```rust
    use actix_web::web::Header;
    use actix_header::actix_header;

    #[actix_header("X-CUSTOMIZED-HEADER")]
    struct MyCustomizedHeader(String);

    impl From<String> for MyCustomizedHeader {
        fn from(s: String) -> Self {
            Self(s)
        }
    }

    impl From<MyCustomizedHeader> for String {
        fn from(s: MyCustomizedHeader) -> Self {
            s.0
        }
    }

    async fn index(Header(MyCustomizedHeader(content))) -> AnyResponse {
        ...
    }

```
