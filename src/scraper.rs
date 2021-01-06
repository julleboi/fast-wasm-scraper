use scraper::*;
use ego_tree::*;
use wasm_bindgen::prelude::*;
use std::pin::*;
use std::boxed::*;

#[wasm_bindgen]
pub struct Document(Pin<Box<Html>>);

#[wasm_bindgen]
impl Document {

    /// **Returns** a new `Document` object
    /// 
    /// # Arguments
    /// * `html` - A raw HTML string slice
    ///
    /// # Examples
    /// ```
    /// let doc = Document::load("<html><h1>Hello world!</h1></html>");
    /// let h1 = doc.root.query("h1");
    /// ```
    #[wasm_bindgen(constructor)]
    pub fn load(html: &str) -> Self {
        Self(Box::pin(Html::parse_document(html)))
    }

    #[wasm_bindgen(getter)]
    pub fn root(&self) -> Element {
        let html = self.0.as_ref();
        let node_id = html.root_element().id();
        Element(node_id, &*html)
    }

}

#[wasm_bindgen]
pub struct Element(NodeId, *const Html);

#[wasm_bindgen]
impl Element {

    fn element(&self) -> ElementRef {
        // Use of `unsafe` is justified, as the pointer should never be null. 
        // Also, we will not be mutating the elements anyhow. However, if the 
        // pointer is null somehow, we will panic.
        if let Some(html) = unsafe { self.1.as_ref() } {
            let node = html.tree.get(self.0).unwrap();
            return ElementRef::wrap(node).unwrap();
        }
        panic!("Bad pointer")
    }

    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self.element().value().name().to_string()
    }

    /// **Returns** a js map of type `Map<string, string>`, which contains the 
    /// attributes of this element
    #[wasm_bindgen(getter)]
    pub fn attributes(&self) -> Map {
        let result = Map::new();
        for (k, v) in self.element().value().attrs() {
            result.set(k, v);
        }
        result
    }

    #[wasm_bindgen(getter)]
    pub fn inner_html(&self) -> String {
        self.element().inner_html()
    }

    /// **Returns** a js array of type `Array<string>`, which contains all of 
    /// the descending text nodes
    #[wasm_bindgen(method)]
    pub fn text(&self) -> StringArray {
        let result = StringArray::new();
        for txt in self.element().text() {
            result.push_str(txt);
        }
        result
    }

    /// **Returns** a js array of type `Array<Element>`, which contains all of 
    /// the descending elements matching the selector
    ///
    /// # Arguments
    /// * `query_str` - string slice representation of the selector
    ///
    /// # Examples
    /// ```
    /// let element = Document::load(...).root();
    /// element.query("li"); // returns all elements with name `li`
    /// element.query("div.aClass"); // returns all `div`s with class `aClass`
    /// ```
    #[wasm_bindgen(method)]
    pub fn query(&self, query_str: &str) -> ElementArray {
        let result = ElementArray::new();
        let selector = match Selector::parse(query_str) {
            Ok(sel) => sel,
            _       => return result
        };
        for el in self.element().select(&selector) {
            result.push_el(Element(el.id(), self.1));
        }
        result
    }

}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "Array<string>")]
    pub type StringArray;
    #[wasm_bindgen(constructor, js_class = "Array")]
    pub fn new() -> StringArray;
    #[wasm_bindgen(method, js_class = "Array", js_name = "push")]
    pub fn push_str(this: &StringArray, item: &str);
    #[wasm_bindgen(method, js_class = "Array", js_name = "shift")]
    pub fn first_str(this: &StringArray) -> Option<String>;
    #[wasm_bindgen(method, getter, js_class = "Array", js_name = "length")]
    pub fn len_str(this: &StringArray) -> u32;

    #[wasm_bindgen(typescript_type = "Array<Element>")]
    pub type ElementArray;
    #[wasm_bindgen(constructor, js_class = "Array")]
    pub fn new() -> ElementArray;
    #[wasm_bindgen(method, js_class = "Array", js_name = "push")]
    pub fn push_el(this: &ElementArray, item: Element);
    #[wasm_bindgen(method, js_class = "Array", js_name = "shift")]
    pub fn first_el(this: &ElementArray) -> Option<Element>;
    #[wasm_bindgen(method, getter, js_class = "Array", js_name = "length")]
    pub fn len_el(this: &ElementArray) -> u32;

    #[wasm_bindgen(typescript_type = "Map<string, string>")]
    pub type Map;
    #[wasm_bindgen(constructor, js_class = "Map")]
    pub fn new() -> Map;
    #[wasm_bindgen(method, js_class = "Map")]
    pub fn set(this: &Map, key: &str, value: &str);
    #[wasm_bindgen(method, js_class = "Map")]
    pub fn get(this: &Map, key: &str) -> Option<String>;
    #[wasm_bindgen(method, js_class = "Map")]
    pub fn has(this: &Map, key: &str) -> bool;
}
