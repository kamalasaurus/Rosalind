use std::env;

fn fib(n: u32, k: u32) -> u32 {
    let mut a: Vec<u32> = vec![1, 1];
    for _ in 2..n {
        let t: u32 = a.drain(0..1).next().unwrap();
        let v: u32 = a.clone()[0];
        a.push((k * t) + v);
    }
    return a.pop().unwrap();
}

fn main() {

    let mut args = env::args();
    let (_, n, k) = (
      args.next(),
      args.next()
        .unwrap()
        .parse::<u32>()
        .unwrap_or(1),
      args.next()
        .unwrap()
        .parse::<u32>()
        .unwrap_or(1)
    );
    let gen: u32 = fib(n, k);
    println!("{:?}", gen)
}

