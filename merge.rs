fn merge_vectors<T:Clone + Ord>(a: &[T], b: &[T]) -> ~[T] {
    let mut result: ~[T] = ~[];
    let a_len = a.len();
    let b_len = b.len();
    let mut i = 0;
    let mut j = 0;

    loop {
        if i >= a_len && j >= b_len {
            break;
        }

        let (i_delta, j_delta) =
          if i >= a_len || (j < b_len && b[j] < a[i]) {
              result.push(b[j].clone());
              (0, 1)
          } else {
              result.push(a[i].clone());
              (1, 0)
          };
        
          i += i_delta; 
          j += j_delta; 
    }

    result
}


pub fn merge_sort<T:Clone + Ord>(a: &[T]) -> ~[T] {
    match a.len() {
        0 => ~[],
        1 => ~[a[0].clone()],
        _ => merge_vectors(merge_sort(a.slice_to(a.len() / 2)),
                           merge_sort(a.slice_from(a.len() / 2)))
    }
}


#[test]
fn merge_empty() {
    let a: ~[int] = ~[];
    assert!(merge_vectors(a, a) == a);
}


#[test]
fn merge_nonempty_to_empty_right() {
    let a: ~[int] = ~[];
    let b: ~[int] = ~[1, 2];
    assert!(merge_vectors(b, a) == b);
}


#[test]
fn merge_nonempty_to_empty_left() {
    let a: ~[int] = ~[];
    let b: ~[int] = ~[1, 2];
    assert!(merge_vectors(a, b) == b);
}


#[test]
fn merge_equal() {
    let a: ~[int] = ~[1, 2];
    let b: ~[int] = ~[1, 1, 2, 2];
    assert!(merge_vectors(a, a) == b);
}


#[test]
fn merge_inorder() {
    let a: ~[int] = ~[1, 2];
    let b: ~[int] = ~[3, 4];
    let c: ~[int] = ~[1, 2, 3, 4];
    assert!(merge_vectors(a, b) == c);
}


#[test]
fn merge_reverse_order() {
    let a: ~[int] = ~[3, 4];
    let b: ~[int] = ~[1, 2];
    let c: ~[int] = ~[1, 2, 3, 4];
    assert!(merge_vectors(a, b) == c);
}


#[test]
fn merge_mixed() {
    let a: ~[int] = ~[1, 3];
    let b: ~[int] = ~[2, 4];
    let c: ~[int] = ~[1, 2, 3, 4];
    assert!(merge_vectors(a, b) == c);
}


#[test]
fn test_empty_merge_sort() {
    let a: ~[int] = ~[];
    assert!(merge_sort(a) == a);
}


#[test]
fn test_one_element() {
    let a: ~[int] = ~[1];
    assert!(merge_sort(a) == a);
}


#[test]
fn test_two_already_ok() {
    let a: ~[int] = ~[1, 2];
    assert!(merge_sort(a) == a);
}


#[test]
fn test_two_not_ok() {
    let a: ~[int] = ~[3, 2];
    let b: ~[int] = ~[2, 3];
    assert!(merge_sort(a) == b);
}


#[test]
fn test_odd_len() {
    let a: ~[int] = ~[3, 2, 1];
    let b: ~[int] = ~[1, 2, 3];
    assert!(merge_sort(a) == b);
}


#[test]
fn test_bad_case() {
    let a: ~[int] = ~[10, 9, 8, 7, 6, 5, 4, 3, 2, 1];
    let b: ~[int] = ~[1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    assert!(merge_sort(a) == b);
}


#[test]
fn test_mixed() {
    let a: ~[int] = ~[1, 9, 10, 4, 3, 7, 2, 5, 6, 8];
    let b: ~[int] = ~[1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    assert!(merge_sort(a) == b);
}
