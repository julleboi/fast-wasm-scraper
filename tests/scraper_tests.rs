extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;
use fast_wasm_scraper::scraper::*;

#[wasm_bindgen_test]
fn simple_text() {
    let doc = Document::load(SAMPLE_1);
    let sel = doc.root().query("h1");
    let el = sel.first_el();
    assert!(el.is_some());

    let el = el.unwrap();
    let txt = el.text().first_str();
    assert!(txt.is_some());

    let txt = txt.unwrap();
    assert!(txt.contains("Hello world!"));
}

#[wasm_bindgen_test]
fn simple_attribute() {
    let doc = Document::load(SAMPLE_1);
    let sel = doc.root().query("h1");
    let el = sel.first_el();
    assert!(el.is_some());

    let el = el.unwrap();
    let attr = el.attributes().get("foo");
    assert!(attr.is_some());

    let attr = attr.unwrap();
    assert_eq!(attr, "bar");
}

#[wasm_bindgen_test]
fn multiple_attributes() {
    let doc = Document::load(SAMPLE_3);
    let sel = doc.root().query("li[one][two][three]");
    let el = sel.first_el();
    assert!(el.is_some());
    
    let el = el.unwrap();
    let attrs = el.attributes();
    assert!(attrs.has("one") && attrs.has("two") && attrs.has("three"));
}

#[wasm_bindgen_test]
fn simple_selector() {
    let doc = Document::load(SAMPLE_1);
    let sel = doc.root().query("*");
    let len = sel.len_el();
    assert_eq!(len, 6);

    let doc = Document::load(SAMPLE_2);
    let sel = doc.root().query("li");
    let len = sel.len_el();
    assert_eq!(len, 5);
}

#[wasm_bindgen_test]
fn advanced_selector() {
    let doc = Document::load(SAMPLE_1);
    let sel = doc.root().query("div.container>h1[foo]");
    let len = sel.len_el();
    assert_eq!(len, 1);

    let sel = doc.root().query("div#nonExistingId");
    let len = sel.len_el();
    assert_eq!(len, 0);
}

const SAMPLE_1: &'static str = r#"
<html>
    <body>
        <div class="container">
            <h1 foo="bar">Hello world!</h1>
            <h1>Something</h1>
            <h1 bar="foo">Another</h1>
        </div>
    </body>
</html>
"#;

const SAMPLE_2: &'static str = r#"
<html>
    <body>
        <div>
            <ul>
                <li>1</li>
                <li>2</li>
                <li>3</li>
                <li>4</li>
                <li>5</li>
            </ul>
        </div>
    </body>
</html>
"#;

const SAMPLE_3: &'static str = "<li one=\"1\" two=\"2\" three=\"3\">abc</li>";
