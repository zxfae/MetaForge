//@Dev : used to check type_of for debug
#[cfg(debug_assertions)]
pub fn print_type_of<T>(_: &T) {
    println!("Type : {}\n", std::any::type_name::<T>());
}
