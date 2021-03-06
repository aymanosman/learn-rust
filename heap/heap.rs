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
    //     vec![7].into_iter().map(Key).collect();

    // TODO: make run2 return Vec Key
    let v: Vec<Key> =
        cookies.into_iter().map(Key).collect();
    let mut heap =
        BinaryHeap::from(v);

    // println!("sweetness: {}", sweetness);
    run(sweetness, &mut heap);
}

fn run(sweetness: i32, mut heap: &mut BinaryHeap<Key>) {
    let mut num_steps = 0;
    let mut succeded = false;

    loop {
        match step(sweetness, &mut heap) {
            Ok(is_finished) => {
                if is_finished {
                    succeded = true;
                    break;
                } else {
                    num_steps += 1;
                }
            },
            Err(_) => break
        }
    }

    if succeded {
        println!("{}", num_steps);
    } else {
        println!("{}", -1);
    }
    // println!("heap: {:?}", heap);
}

// TODO: return False when done?
fn step(sweetness: i32, heap: &mut BinaryHeap<Key>) -> Result<bool,()> {
    // println!("step heap: {:?}", heap);
    match (heap.pop(), heap.pop()) {
        (Some(Key(a)), Some(Key(b))) => {
            if a >= sweetness {
                heap.push(Key(a));
                heap.push(Key(b));
                return Ok(true);
            } else {
                let c = a+2*b;
                heap.push(Key(c));
                return Ok(false);
            }
        },
        (Some(Key(a)), None) => {
            heap.push(Key(a));
            if a >= sweetness {
                Ok(true)
            } else {
                Err(())
            }
        },
        (None, _) => Err(())
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
