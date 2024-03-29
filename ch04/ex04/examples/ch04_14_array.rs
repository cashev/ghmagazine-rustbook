fn main() {
    let a1 = [false, true, false];
    let a2 = [0.1, -1.0, 1.0, 0.5];
    assert!(a1[1]);
    assert_eq!(a2.len(), 4);

    let a3 = [0; 100];
    assert_eq!(a3.len(), 100);

    let a4 = [['a', 'b'], ['c', 'd']];
    assert_eq!(a4.len(), 2);
    assert_eq!(a4[0].len(), 2);

    // error
    // let a5 = [false, 'a'];

    let size = 100;
    // error
    // let a1 = [0; size];

    let mut v1 = vec![0; size];
    assert_eq!(v1.len(), 100);

    v1.push(1);
    assert_eq!(v1.len(), 101);
    assert_eq!(v1.pop(), Some(1));
    assert_eq!(v1.len(), 100);

    let array1 = ['H', 'e', 'l', 'l', 'o'];
    assert_eq!(array1[1], 'e');

    let mut array2 = [0, 1, 2];
    array2[1] = 10;
    assert_eq!(array2, [0, 10, 2]);

    let mut index = 0;
    assert_eq!(array2[index], 0);
    index += 1;
    assert_eq!(array2[index], 10);

    let array3 = [0, 1];
    // array3[2]; // error
    let index = 3;
    // array3[index]; // error
    assert_eq!(array3.get(1), Some(&1));
    assert_eq!(array3.get(2), None);
    assert_eq!(array3.get(index), None);

    let array4 = ['a'; 50];
    for ch in array4.iter() {
        print!("{},", *ch);
    }
    println!("");

    let mut array5 = [1; 50];
    for n in array5.iter_mut() {
        *n *= 2;
    }
    assert_eq!(array5, [2; 50]);
}
 