use std::io;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Debug, Eq, PartialEq)]
struct Key(usize);

impl Ord for Key {
  fn cmp(&self, other: &Key) -> Ordering {
      other.0.cmp(&self.0)
  }
}

impl PartialOrd for Key {
    fn partial_cmp(&self, other: &Key) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    run();
    run2()
}

fn run() {
    // let mut heap = BinaryHeap::from(vec![Key(1), Key(2), Key(3), Key(9), Key(10), Key(12)]);
    let v: Vec<Key> =
        vec![1, 2, 3, 9, 10, 12].into_iter().map(Key).collect();
    let mut heap = BinaryHeap::from(v);
    let mut num_steps = 0;

    loop {
        match step(&mut heap) {
            Ok(is_finished) => {
                if is_finished {
                    break;
                } else {
                    num_steps += 1;
                }
            },
            Err(_) => break
        }
    }

    println!("num_steps: {}", num_steps);
    println!("heap: {:?}", heap);
}

// TODO: return False when done?
fn step(heap: &mut BinaryHeap<Key>) -> Result<bool,i32> {
    let sweetness = 7;
    if let (Some(Key(a)), Some(Key(b))) = (heap.pop(), heap.pop()) {
        if a > sweetness {
            heap.push(Key(a));
            heap.push(Key(b));
            return Ok(true);
        } else {
            let c = a+2*b;
            println!("CC {:?}", c);
            heap.push(Key(c));
            return Ok(false);
        }
    } else {
        return Err(42);
    }
}


fn run2() {

    let h = io::stdin();
    let mut input = String::new();
    // let mut t = String::new();

    h.read_line(&mut input).expect("err1");
    let ww = input.split_whitespace()
        .map(|x| { x.parse::<i32>().unwrap() });

    let mut v = Vec::new();
    v.extend(ww.into_iter());
    println!("v: {:?}", v);
    let num = v[0];
    let sweetness = v[1];

    // for i in input.split_whitespace() {
    //     println!("i: {}", i.parse::<i32>().unwrap());
    // }
    println!("QQ: {}", input);
    // h.read_line(&mut input).expect("err1");
    // println!("QQ: {}", input);
    // println!("SS: {}", t);
}
