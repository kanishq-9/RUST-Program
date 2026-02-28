fn firstname(fname:&str){
    println!("{:?}", fname);
}

fn sum(a:i32, b:i32)->i32{
    //return type does not need semicolon
    return a + b;
}

fn main(){
    firstname("Kanishq");
    let value:i32 = sum(2,3);
    println!("{:?}",value);
}