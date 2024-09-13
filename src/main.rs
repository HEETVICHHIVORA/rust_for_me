enum Shape {
    Rectangle (f64,f64),
    Circle(f64),
}
fn main() {
// println!("{}",fib(5));
// let name = String::from ("harkirat");
// print!("{}",get_length(name));
//   let rect1 = Rect{
//     width:10,
//     height:20
//   };
// println!("the area ->>{}",rect1.area());
// print!("the perimeter-->{}",rect1.perimeter());
// let rect = Shape::Rectangle(1.0, 2.0);
// println!("the area of this rectangle is {}",calculate(rect));
// let circle = Shape::Circle(5.0);
// println!("the area of circle is {}",calculate(circle));
let index = find_a(String::from("Harkirat"));
    match index{
        Some(value)=> println!("hey we found it"),
        None => println!("sadly didn't make it"),
    }

}
fn find_a(s:String)->Option<i32>{
    for(index,char) in s.chars().enumerate(){
        if char=='a' {
            return Some(index as i32);
        }
    }
    return None;
}
// fn calculate (shape:Shape)->f64{
//     match shape{
//         Shape::Rectangle(a, b)=> a*b,
//         Shape::Circle(r)=>3.14*r*r,
//     }
// }
// struct  Rect {
//     width:i32,
//     height:i32,
// }
// impl Rect {
//     fn area(&self)->i32{
//         return self.width*self.height;
//     }
//     fn perimeter (&self)->i32{
//         return 2*(self.width+self.height); 
//     }
// }

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