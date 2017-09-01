
trait Log {
     fn show(&self);
}

impl Log for i32{
    fn show(&self){
        println!("{}",*self);
    }
}


fn main() {
    println!("Hello, world!");
    32.show();
}
