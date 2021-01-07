const fs = require('fs');
const Benchmark = require('benchmark');
const cheerio = require('cheerio');
const fws = require('fast-wasm-scraper');
const jsdom = require('jsdom');

const html = fs.readFileSync('./testdata/sample.html').toString();
const suite = new Benchmark.Suite;

suite.add('cheerio', function() {
  const doc = cheerio.load(html);
  doc('*');
})
.add('jsdom', function() {
  const doc = new jsdom.JSDOM(html);
  doc.window.document.querySelectorAll('*');
})
.add('fast-wasm-scraper', function() {
  const doc = new fws.Document(html);
  doc.root.query('*').forEach(e => e.free());
  doc.free();
})
.on('cycle', function(event) {
  console.log(String(event.target));
})
.on('complete', function() {
  console.log('Fastest is ' + this.filter('fastest').map('name'));
})
.run();
