#!/usr/bin/env node

var fs = require('fs');

var bases = fs.readFileSync('./rosalind_dna.txt', 'utf8');

var counts = bases.trim().split('').reduce(function(counts, base) {
  counts[base] += 1;
  return counts;
}, {A: 0, C: 0, G: 0, T: 0});

var str = Object.keys(counts).map(function(key) {
  return counts[key];
}).join(' ');

console.log(str);

