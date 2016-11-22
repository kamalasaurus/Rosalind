function fib(n, k, a) {
  while (a.length < n) {
    if (!!~[0,1].indexOf(a.length)) a.push(1)
    else a.push(a[a.length - 1] + (k * a[a.length - 2]))
  }
  return a
}

void function main() {
  var coll = fib(+process.argv[2], +process.argv[3], [])
  console.log(coll)
}()

//function fib(n) {
  //let a = [1,1]
  //for (let i=2; i < n; i++) a.push(a.shift(), a[0])
  //return a.pop()
//}

//void function main() {
  //console.log(fib(+process.argv[2]))
//}

