use scraper::{ Html, ElementRef, Selector };
use ego_tree::NodeId;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub struct Document(Box<Html>);

#[wasm_bindgen]
impl Document {
    
    /// **Takes** the raw HTML as a string, and **returns** a new `Document` 
    ///
    /// # Example
    /// ```
    /// const doc = new Document('<html><h1>Hello world!</h1></html>');
    /// // => Document { root: Element }
    /// ```
    #[wasm_bindgen(constructor)]
    pub fn load(html: &str) -> Self {
        Self(Box::new(Html::parse_document(html)))
    }

    /// **Returns** the root `Element`
    #[wasm_bindgen(getter)]
    pub fn root(&self) -> Element {
        Element(self.0.root_element().id(), &*self.0)
    }

}

#[wasm_bindgen]
pub struct Element(NodeId, *const Html);

#[wasm_bindgen]
impl Element {

    fn element(&self) -> ElementRef {
        unsafe {
            let html = &*self.1;
            let node = html.tree.get_unchecked(self.0);
            match ElementRef::wrap(node) {
                Some(res) => return res,
                None      => panic!()
            }
        }
    }

    /// **Returns** the name as a `string` for this `Element`
    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self.element().value().name().to_string()
    }

    /// **Returns** a `Map<string, string>` containing the attributes for this 
    /// `Element`
    #[wasm_bindgen(getter)]
    pub fn attributes(&self) -> Map {
        let result = Map::new();
        for (k, v) in self.element().value().attrs() {
            result.set(k, v);
        }
        result
    }
   
    /// **Returns** a `string` representation of the `Element`, with it's 
    /// descendants
    #[wasm_bindgen(getter)]
    pub fn html(&self) -> String {
        self.element().html()
    }

    /// **Returns** a `string` of the inner content of this `Element`
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

    /// **Takes** a query `string` and **returns** an `Array<Element>` 
    /// containing all descending elements matching the selector
    ///
    /// # Examples
    /// ```
    /// const doc = new Document(...);
    /// doc.root.query('li');
    /// // => [ 
    /// //      Element { name: 'li', ... },
    ///         Element { name: 'li', ... },
    ///         ... 
    ///       ]
    /// ```
    #[wasm_bindgen(method)]
    pub fn query(&self, query_str: &str) -> ElementArray {
        let result = ElementArray::new();
        match Selector::parse(query_str) {
            Ok(sel) => {
                for elem in self.element().select(&sel) {
                    result.push_el(Element(elem.id(), self.1));
                }
            }
            Err(_)  => return result
        };
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
