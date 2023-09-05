fn main() {
    let v = [2, 11, 3, 4, 7];
    let res = cal_partial_sums(&v);

    println!("Input : {:?} => Result : {:?}", v,  res);
}


fn cal_partial_sums(v : &[i32]) -> Vec<i32> {
    let mut res : Vec<i32> = Vec::new();

    let mut summation = 0;
    for i in 0..v.len() {
        
        summation = summation + v[i];
        res.push(summation);

    }

    res
}


#[cfg(test)]

#[test]
fn cal_partial_sums_test() {
    let res = cal_partial_sums(&[0,0,0]);
    assert_eq!([0,0,0].to_vec() , res);

    let res = cal_partial_sums(&[1,2,3,10]);
    assert_eq!([1,3,6,16].to_vec(), res);
}
