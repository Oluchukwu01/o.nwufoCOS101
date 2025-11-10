fn main() {
    let mut num:i32 = 5;
    mutate_num_to_zero(&mut num);
    println!("The value of num is:{}",num);
}

fn mutate_num_to_zero(paran_num:&mut i32){
    *paran_num = *paran_num*0; //de reference
    println!("paran_num value is:{}",paran_num);
}