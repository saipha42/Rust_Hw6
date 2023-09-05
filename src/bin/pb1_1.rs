fn main(){

    let v = [1,2,3,4,0];
    let res = min_max_avg(&v);
    println!("Input vector : {:?} => Result : {:?}", v, res);
}


fn min_max_avg(v : &[i32])-> (i32, i32, f32) {

    let mut min_val = v[0];
    let mut max_val = v[0];
    let mut summation = 0;

    for i in 0..v.len() {
        if v[i] < min_val {
            min_val = v[i];
        }else {
            max_val = v[i];
        }
        summation += v[i];
    }

    let avg = summation as f32 / (v.len() as f32);

    return  (min_val, max_val, avg);
}


#[cfg(test)]

#[test]
fn min_max_avg_test() {
    let res = min_max_avg(&[0,0,0]);
    assert_eq!((0,0,0.0) , res);

    let res = min_max_avg(&[1,2,3,10]);
    assert_eq!((1,10,4.0), res);
}