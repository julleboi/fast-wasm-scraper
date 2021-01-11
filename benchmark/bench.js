const fs = require('fs');
const path = require('path');
const Benchmark = require('benchmark');
const cheerio = require('cheerio');
const { Document } = require('fast-wasm-scraper');
const { JSDOM } = require('jsdom');

const html = fs.readFileSync(path.join(__dirname, './testdata/sample.html')).toString();
const suite = new Benchmark.Suite;

suite.add('cheerio', function() {
  const doc = cheerio.load(html);
  doc('*');
})
.add('jsdom', function() {
  const doc = new JSDOM(html);
  doc.window.document.querySelectorAll('li');
})
.add('fast-wasm-scraper', function() {
  const doc = new Document(html);
  doc.root.query('li').forEach(e => e.free());
  doc.free();
})
.on('cycle', function(event) {
  console.log(String(event.target));
})
.on('complete', function() {
  console.log('Fastest is ' + this.filter('fastest').map('name'));
})
.run();
