use std::mem;

// & -> this is reference

fn analyze_array(reference_array: &[i8])  {
  println!("item array {0}", reference_array[0]);
  println!("item length {0}", reference_array.len());
  println!("byte allocated {0}", mem::size_of_val(reference_array));
  println!("all array {0:?}", reference_array);
}

fn main() {
  let array: [bool; 2] = [ true, false ];
  let array_list: [i8; 9] = [0; 9]; // update type and view mem size
  println!("all array {0:?}", array);
  analyze_array(&array_list);
  println!("all array final {0:?}", &array_list[0 .. 3]);

  let empty_array: [bool; 0] = [];
  assert_eq!(&empty_array, &[]);
}

