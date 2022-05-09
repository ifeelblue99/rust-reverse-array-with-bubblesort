fn main() {
  let mut data = vec!(111, 2, 33, 4, 5);
  println!("reversed arr. {:?}", reverse(data));
}

fn reverse(mut arr: Vec<usize>) -> Vec<usize> {
  let mut size = arr.len();
  
  'outer_loop: loop {
    for indx in 1..size {
      if arr[indx] >  arr[indx-1] {
        let temp = arr[indx];
        arr[indx] = arr[indx-1];
        arr[indx-1] = temp
      }
      println!("{}", indx)
    }
    size = size - 1;
    if size < 2 {
        break 'outer_loop
    }
  };
  arr
}
