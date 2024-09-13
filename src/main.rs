fn main() {
   // println!("{}",fib(5));
  // let name = String::from ("harkirat");
  // print!("{}",get_length(name));
  let rect1 = Rect{
    width:10,
    height:20
  };
  println!("the area ->>{}",rect1.area());
  print!("the perimeter-->{}",rect1.perimeter());
}
struct  Rect {
    width:i32,
    height:i32,
}
impl Rect {
    fn area(&self)->i32{
        return (self.width*self.height);
    }
    fn perimeter (&self)->i32{
        return 2*(self.width+self.height); 
    }
    
}

// fn fib (num:i32)->i32 {
//     let mut first = 0 ;
//     let mut second  = 1 ;
//     if num==0 {
//         return first ;

//     }
//     if num==1{
//        return 1 ;
//     }
//     for _ in 0..(num-1){
//         let temp = second ;
//         second = second + first ;
//         first  = temp ;
//     }

//     return  second ;

// }
// fn get_length (str:String)->usize{
//     str.chars().count()
// }