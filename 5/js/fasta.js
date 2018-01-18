var fs = require('fs');

function percentGC(str) {
  let {total, gc} = str
    .toLowerCase()
    .split('')
    .reduce((coll, ch)=> {
      if (/g|c/.test(ch)) coll['gc'] += 1
      coll['total'] += 1
      return coll;
    }, {total: 0, gc: 0});

  return (gc/total) * 100;
}

void function main() {
  var lines = fs
    .readFileSync('./rosalind_gc.txt', 'utf8')
    .trim()
    .split('>')
    .map((l)=> l.replace(/\n/g, ''))
    .slice(1);

  var highest = lines
    .map((l)=> {
      let matcher = /^\w+_\d+/y;
      let [key] = matcher.exec(l);
      let val = l.substr(matcher.lastIndex);
      let perc = percentGC(val);
      return [key, perc];
    })
    .sort((a, b)=> b[1] - a[1])[0];

  console.log(highest);
}();

