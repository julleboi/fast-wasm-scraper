# **fast-wasm-scraper**

![Continuous integration](https://github.com/julleboi/fast-wasm-scraper/workflows/Continuous%20integration/badge.svg?branch=main)

A fast alternative for JavaScript-based scraping tools, intended for both 
frontend and backend. **fast-wasm-scraper** is practically a wrapper for 
**[scraper](https://github.com/causal-agent/scraper)** (intended for parsing 
HTML and querying with CSS selectors) -- which compiles to **WebAssembly**. 

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

| property      | type                         | Description                                                        |
| ------------- | :--------------------------: | :----------------------------------------------------------------: |
| `constructor` | `(html: string) => Document` | `Takes the raw html as a string and returns a new Document object` |
| `root`        | `Element`                    | `Returns the root element of the Document`                         |

### Element


| property      | type                                       | Description                                                            |
| ------------- | :----------------------------------------: | :--------------------------------------------------------------------: |
| `name`        | `string`                                   | `Returns the name of the element as a string, ex: 'div'`               |
| `html`        | `string`                                   | `Returns a string representation of this Element and it's descendants` |
| `inner_html`  | `string`                                   | `Returns the inner content of this Element as a string`                |
| `attributes`  | `Map<string, string>`                      | `Returns the attributes as a Map<string, string>`                      |
| `query`       | `(query_str: string) => Array<Element>`    | `Returns an array of Elements from the resulting query`                |
| `text`        | `() => Array<string>`                      | `Returns an array of strings from descending text nodes`               |


## **Benchmark**

|                                                              | fast-wasm-scraper       | cheerio           | JsDOM             |
| ------------------------------------------------------------ | :---------------------: | :---------------: | :---------------: |
| **Runtime**	                                                 | WebAssembly (from Rust) | JavaScript        | JavaScript        |
|                                                              |                         |                   |                   |
| Parsing, and querying with `*`, for a ~200 KB HTML document  |                         |                   |                   |
|                                                              |                         |                   |                   |
| Sample size (#)                                              | 45                      | 32                | 10                |
| Speed (ops/s)                                                | 17.27 (+/- 1.35%)       | 11.47 (+/- 7.90%) | 2.14 (+/- 13.32%) |
| Speedup (%)                                                  | +50.57 compared to cheerio, and +807.00 to JsDOM              |||
*This benchmark was conducted on a rather modest dual core CPU and Node.js 
v.12.20.0. It should be noted that the expected performance gains are even 
greater with larger documents. You can also run the benchmarks locally by 
cloning the GitHub repository.*
