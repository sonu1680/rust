struct Rect{
   height:u8,
   width:u8

}


impl Rect {
    fn area(&self)->u8{
      return self. height*self.width;
    }
    fn peri(&self)->u8{
      return 2*(self.height+self.width);
    }
}
fn main() {

let rect=Rect{
   height:2,
   width:2
};
println!("{}",rect.area());
println!("{}",rect.peri());




}


// fn get_string_length(str:String)->usize{
//    return str.chars().count();
   
// }

// fn is_even(num:i8)->bool{
//     if num%2==0 {
//         return true;
//     }
//     else{
//         return false;
//     }
// }



//  fn fibonaic(num:i8)->i8{
//  let mut first:i8=0;
//  let mut second:i8=1;

//  if num==0{
//     return first;
//  }

//  if num==1{
//     return second;
//  }
//  for _ in 0..(num-1){
//      let  temp=second;
//      second=second+first;
//      first=temp;
// }
// return second;
 

// }