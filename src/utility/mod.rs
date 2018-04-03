use std;

pub fn print_type_of<T>(_: &T) {
    println!("{}", unsafe { std::intrinsics::type_name::<T>() });
}

pub fn return_type_of<T>(_: &T) -> String {
    format!("{}", unsafe { std::intrinsics::type_name::<T>() })
}