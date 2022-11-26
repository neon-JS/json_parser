use crate::definitions::parser::Parser;
use crate::value_containers::container_bool::ContainerBool;
use crate::value_containers::container_null::ContainerNull;
use crate::value_containers::container_string::ContainerString;
use crate::value_containers::container_number::ContainerNumber;

mod value_containers {
    pub mod container_array;
    pub mod container_bool;
    pub mod container_null;
    pub mod container_string;
    pub mod container_number;
}
mod definitions {
    pub mod property;
    pub mod parser;
}

fn main() {
    let mut cs = ContainerString { value: String::new() };
    let ps = cs.parse("\"HALLO WELT\",,");
  //  println!("{}", &ps.unwrap().0.string_value.unwrap());
   // println!("{}", cs.value);
    println!("{}", &ps.unwrap().1);

    let mut cb = ContainerBool { value: false };
    let pb = cb.parse("true,,");
  //  println!("{}", if pb.unwrap().0.bool_value.unwrap() { "true" } else { "false " });
 //   println!("{}", if cb.value { "true" } else { "false " });
    println!("{}", &pb.unwrap().1);


    let mut cn = ContainerNull { };
    let pn = cn.parse("null,,");
  //  println!("{}", if pn.unwrap().0.is_null_value { "true" } else { "false " });
    println!("{}", &pn.unwrap().1);


    let mut cf = ContainerNumber { value: 0.0 };
    let pf = cf.parse("12222323232,,");
   // println!("{}", &pf.unwrap().0.numeric_value.unwrap());
    println!("{}", &pf.unwrap().1);
  //  println!("{}", cf.value);
}
