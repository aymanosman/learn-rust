use std::io;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Debug, Eq, PartialEq)]
struct Key(i32);

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
    let (sweetness, cookies) = run2();

    // let v: Vec<Key> =
    //     vec![1, 2, 3, 9, 10, 12].into_iter().map(Key).collect();

    // TODO: make run2 return Vec Key
    let mut heap =
        BinaryHeap::from(cookies.into_iter().map(Key).collect::<Vec<Key>>());

    println!("sweetness: {}", sweetness);
    run(sweetness, &mut heap);
}

fn run(sweetness: i32, mut heap: &mut BinaryHeap<Key>) {
    let mut num_steps = 0;

    loop {
        match step(sweetness, &mut heap) {
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
    // println!("heap: {:?}", heap.into_vec());
}

// TODO: return False when done?
fn step(sweetness: i32, heap: &mut BinaryHeap<Key>) -> Result<bool,i32> {
    if let (Some(Key(a)), Some(Key(b))) = (heap.pop(), heap.pop()) {
        if a > sweetness {
            heap.push(Key(a));
            heap.push(Key(b));
            return Ok(true);
        } else {
            let c = a+2*b;
            // println!("CC {:?}", c);
            heap.push(Key(c));
            return Ok(false);
        }
    } else {
        return Err(42);
    }
}


fn run2() -> (i32, Vec<i32>){

    let h = io::stdin();
    let mut input = String::new();

    h.read_line(&mut input).expect("err1");
    let line1 = input.split_whitespace()
        .map(|x| { x.parse::<i32>().unwrap() });

    let mut v = Vec::new();
    v.extend(line1);
    // println!("v: {:?}", v);
    let num = v[0];
    let sweetness = v[1];
    let mut rest = String::new();

    h.read_line(&mut rest).expect("err2");
    let line2 = rest.split_whitespace()
        .map(|x| { x.parse::<i32>().unwrap() });

    let mut cookies: Vec<i32> = Vec::with_capacity(num as usize);
    cookies.extend(line2);
    // println!("len: {}", cookies.len());
    // println!("cookies: {:?}", cookies.into_iter().take(5).collect::<Vec<i32>>());
    (sweetness, cookies)
}
