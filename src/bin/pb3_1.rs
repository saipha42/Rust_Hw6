fn main() {

    let res = unpack_number_tuples(&[(1, 4), (3, 2), (2,1)]);
    println!("{:?}", res);
}

fn unpack_number_tuples( v1:  &[(i32, i32)] ) -> (Vec<i32>, Vec<i32>) {

    let mut res1 : Vec<i32> = Vec::new();
    let mut res2 : Vec<i32> = Vec::new();

    for i in 0..v1.len() {
        res1.push(v1[i].0);
        res2.push(v1[i].1);
    }

    (res1, res2)
}

#[cfg(test)]

#[test]
fn unpack_number_tuples_two_test() {
    let res = unpack_number_tuples(&[(1, 2), (3, 4), (5,6)]);
    assert_eq!((vec![1,3,5], vec![2,4,6]), res);

    let res = unpack_number_tuples(&[]);
    assert_eq!((vec![], vec![]), res);

}