use std;

pub fn return_type_of<T>(_: &T) -> String {
    format!("{}", unsafe { std::intrinsics::type_name::<T>() })
}