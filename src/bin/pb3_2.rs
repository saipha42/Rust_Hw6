fn main() {
    let v = [(1, 4, 5), (2, 2, 1)];
    let res = unpack_number_tuples3(&v);
    println!("Input : {:?}", v);
    println!("Result => {:?}", res);
}

fn unpack_number_tuples3(v1: &[(i32, i32, i32)]) -> (Vec<i32>, Vec<i32>, Vec<i32>) {
    let mut res1: Vec<i32> = Vec::new();
    let mut res2 : Vec<i32> = Vec::new();
    let mut res3 : Vec<i32> = Vec::new();

    for i in 0..3 {

        if i == 0 {
            for k in 0..v1.len() {
                res1.push(v1[k].0);
            }
        }else if i == 1 {
            for k in 0..v1.len() {
                res2.push(v1[k].1);
            }
        }else if i == 2 {
            for k in 0..v1.len() {
                res3.push(v1[k].2);
            }
        }
    }

    (res1, res2,  res3)
}

#[cfg(test)]

#[test]
fn unpack_number_tuples3_three_test() {
    
    let res = unpack_number_tuples3(&[(1, 2, 3), (4,5,6)]);
    assert_eq!((vec![1,4], vec![2,5], vec![3,6]), res);

    let res = unpack_number_tuples3(&[]);
    assert_eq!((vec![], vec![], vec![]), res);

}