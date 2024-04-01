# Components

すべてのコンポーネントは、Component トレイトを実装します。

[src/frontend/src/components/mod.rs](/src/frontend/src/components/mod.rs)

```rust
pub trait Component {
    fn write(&self, props: &PageProps, buf: &mut String) -> Result<()>;
}
```

[src/frontend/src/pages/page.rs](/src/frontend/src/pages/page.rs)

```rust
pub struct PageProps {
    pub session_id: String,
    pub request_id: String,
    pub user_currency: String,
    pub currency_codes: Option<Vec<String>>,
    pub product_id: Option<String>,
    pub cart: Option<model::cart::Cart>,
    pub hot_products: Option<model::hot_product::HotProducts>,
    pub product: Option<model::product::Product>,
    pub recommendations: Option<model::recommendation::RecommendationList>,
    pub ad: Option<model::ad::AdItem>,
    pub order: Option<model::order::Order>,
}
```

画面の表示で使用されるデータを、PageProps 構造体に保持します。Option 属性になっている項目は、その画面で使われる場合のみ、Some(T)がセットされ、使われない場合は None になります。  
Component の write メソッドには、PageProps への参照が渡されます。

[src/frontend/src/pages/page.rs](/src/frontend/src/pages/page.rs)

```rust
// --snip--

// buffer size for outputting HTML content.
// specify a sufficient size according to the characteristics of the system.
const PAGE_BUFFER_SIZE: usize = 20_000;

pub struct Page {
    pub head: Box<dyn Component + Send>,
    pub body: Option<Box<dyn Component + Send>>,
}

impl Page {
    pub fn new() -> Page {
        Page {
            head: Box::new(Head {}),
            body: None,
        }
    }

    // output HTML content to buffer.
    pub fn write(&mut self, props: &crate::pages::page::PageProps) -> Result<String> {
        // buffer for outputting HTML content.
        let mut buf = String::with_capacity(PAGE_BUFFER_SIZE);

        buf.push_str(r#"<!DOCTYPE html>"#);
        buf.push_str(r#"<html lang="en">"#);
        {
            if let Err(e) = self.head.write(props, &mut buf) {
                return Err(e);
            }

            if let Some(body) = &self.body {
                if let Err(e) = body.write(props, &mut buf) {
                    return Err(e);
                }
            }
        }
        buf.push_str(r#"</html>"#);

        Ok(buf)
    }
}
```

「buf」という String に、HTML の出力を行います。

```rust
const PAGE_BUFFER_SIZE: usize = 20_000;
// --snip--
let mut buf = String::with_capacity(PAGE_BUFFER_SIZE);
```

となっているので、20,000 バイトのキャパシティでバッファを確保し、そこに、buf.push_str(...) で HTML タグを順次出力していきます。  
Page 構造体は、head コンポーネントと body コンポーネントを持っていて、それぞれの write 関数が、&lt;head&gt;、&lt;body&gt;タグを出力します。
