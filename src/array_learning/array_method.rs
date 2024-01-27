#[test]
fn arr_test(){
    let mut sum_1 = vec![3,2,4,1];
    let mut sum_2 = vec![6];
    sum_1.append(&mut sum_2);
    sum_1.sort_by(|a,b|{
        a.partial_cmp(b).unwrap()
    });
    fn is_even(input:usize)->bool{
        input%2!=0
    }
    fn get_mean(arr:&Vec<i32>)->f64{
        if is_even(arr.len()) {
            let mean_pos = arr.len()/2;
            arr.get(mean_pos).unwrap().clone() as f64
        }else{
            let start_pos = (arr.len()+1)/2;
            let end_pos = (arr.len()-1)/2;
           let start =  arr.get(start_pos).unwrap().clone() as f64;
           let end =  arr.get(end_pos).unwrap().clone() as f64;
            (start+end)/2.0
        }
    }
    println!("{:?},{:?}",sum_1,get_mean(&sum_1));
}