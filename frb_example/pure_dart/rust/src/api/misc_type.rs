use crate::auxiliary::sample_types::MySize;
use log::info;

pub fn func_string_twin_normal(arg: String) -> String {
    arg
}

#[allow(clippy::unused_unit)]
pub fn func_return_unit_twin_normal() -> () {}

pub fn handle_list_of_struct_twin_normal(mut l: Vec<MySize>) -> Vec<MySize> {
    info!("handle_list_of_struct({:?})", &l);
    let mut ans = l.clone();
    ans.append(&mut l);
    ans
}

pub fn handle_string_list_twin_normal(names: Vec<String>) -> Vec<String> {
    for name in &names {
        info!("Hello, {}", name);
    }
    names
}

#[derive(Debug, Clone)]
pub struct Empty {}

pub fn empty_struct_twin_normal(empty: Empty) -> Empty {
    empty
}
