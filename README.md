# **fast-wasm-scraper**

A fast alternative for JavaScript-based scraping tools, intended for both 
frontend and backend. **fast-wasm-scraper** is practically a wrapper for 
**[scraper](https://github.com/causal-agent/scraper)** (intended for parsing 
HTML and querying with CSS selectors) -- which compiles to WebAssembly. 

## **Installation**

```
$ yarn add fast-wasm-scraper
```

## **Examples**

### Loading

```
const { Document } = require('fast-wasm-scraper');
const doc = new Document('<html>Hello world!</html>');

doc.root.inner_html;
// => <html>Hello world!</html>
```

### Querying

```
const { Document } = require('fast-wasm-scraper');
const html = `
<html>
  <body>
    <div>
      <ul>
        <li>One</li>
        <li>Two</li>
        <li>Three</li>
      </ul>
    </div>
  </body>
</html>
`;
const doc = new Document(html);

doc.root.query('li');
// => [
//      Element { name: 'li', inner_html: 'One',   ... },
//      Element { name: 'li', inner_html: 'Two',   ... },
//      Element { name: 'li', inner_html: 'Three', ... },
//    ]
```

## **Types**

### Document

| property      | type                         |
| ------------- | ---------------------------- |
| `constructor` | `(html: string) => Document` |
| `root`        | `Element`                    |

### Element


| property      | type                                       |
| ------------- | ------------------------------------------ |
| `name`        | `string`                                   |
| `inner_html`  | `string`                                   |
| `attributes`  | `Map<string, string>`                      |
| `query`       | `(query_str: string) => Array<Element>`    |
| `text`        | `() => Array<string>`                      |


## **Benchmarks**

### Results

| implementation     | fast-wasm-scraper | cheerio 	  |
| ------------------ | ----------------- | ---------- |
| Backend	           | Rust -> WASM      | JavaScript |
| Time for 100 (sec) | X                 | Y          |
| Speedup (%)        | -                 | -          |

### Specifications

- **CPU**: intel ix-xxxxx (x cores @ x.xx GHz)
- **RAM**: xx GB DDRX @ xxxx MHz
- **Node**: version 12.xx
