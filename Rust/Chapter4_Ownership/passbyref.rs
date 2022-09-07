fn main(){
    //for ints we dont need passbyref
    let v1: i32 = 4;
    let v2: i32 = 6;
    
    let result: i32 = add(v1, v2);

    println!("{} plus {} is: {}", v1, v2, result);
    
    //passbyref example
    let s1 = String::from("hello");
    let len : usize = calc_len(&s1);
    
    println!("The length of '{}' is {}", s1, len);
}

fn add(x : i32, y : i32) -> i32 {
    x+y
}

fn calc_len(s: &String) -> usize {
    s.len()
}
