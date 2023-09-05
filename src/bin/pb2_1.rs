fn main() {

    let v1 = [1,2];
    let v2 = [1];
    let v3 = [1,2,3];

    let res = pack_number_tuples3(&v1, &v2, &v3);
    println!("Input => v1 : {:?} and v2 : {:?} and v3 : {:?}", v1, v2,v3);
    println!("result : {:?}", res);
}


fn pack_number_tuples3(v1: &[i32], v2: &[i32], v3: &[i32]) -> Vec<(i32, i32, i32)> {

     let mut res:Vec<(i32, i32, i32)> = Vec::new();

    let mut max_limit = v1.len();
    if v2.len() > max_limit {
        max_limit = v2.len();
    }
    if v3.len() > max_limit {
        max_limit = v3.len();
    }

    for i in 0..max_limit {
        let mut val_1 = 0 ;
        let mut val_2 = 0;
        let mut val_3 = 0;

        if i < v1.len() {
            val_1 = v1[i];
        }
        if i < v2.len() {
            val_2 = v2[i];
        }
        if i < v3.len() {
            val_3 = v3[i];
        }

        res.push( (val_1, val_2, val_3));
    }

     return res;
}

#[cfg(test)]

#[test]
fn pack_number_tuples3_test() {
    let res = pack_number_tuples3(&[1], &[1], &[1]);
    assert_eq!([(1,1,1)].to_vec() , res);

    let res = pack_number_tuples3(&[1], &[1,2], &[1,2,3]);
    assert_eq!([(1,1,1), (0,2,2), (0,0,3)].to_vec() , res);
}