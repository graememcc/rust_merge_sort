use std::rand;
use std::rand::Rng;

mod merge;

fn make_random_vec() -> ~[int] {
    let mut randgen = rand::task_rng();
    let mut result = ~[];
    do 10.times {
      let b: int = randgen.gen_integer_range(1, 100);
      result.push(b);
    }

    result
}


fn main() {
    let random_vec = make_random_vec();
    let sorted_vec = merge::merge_sort(random_vec);

    println(fmt!("%?", random_vec));
    println(fmt!("%?", sorted_vec));
}
