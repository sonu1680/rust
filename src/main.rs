fn main() {
println!("{}",fibonaic(4));

}


// fn is_even(num:i8)->bool{
//     if num%2==0 {
//         return true;
//     }
//     else{

//         return false;
//     }
// }

// fn fibonaic(num:i8)->i8{
 let mut first:i8=0;
 let mut second:i8=1;

 if num==0{
    return first;
 }

 if num==1{
    return second;
 }
 for _ in 0..(num-1){
     let  temp=second;
     //println!("{}",temp);
     second=second+first;
     first=temp;
}
return second;
 

}