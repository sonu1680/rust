struct User{
   user_name:String,
   age:i8,
   is_active:bool,

}

fn main() {

let user=User{
   user_name:String::from("Sonu pandit"),
   age:12,
   is_active:true
};

println!("{},{},{}",user.age,user.user_name,user.is_active);

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