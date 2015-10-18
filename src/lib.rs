#![feature(core_intrinsics)]

#[macro_export]
macro_rules! t {
  ($from:expr) => (self::print_type_of($from));
}


fn print_type_of<'a, T>(_: T) -> &'a str {
  let type_name = unsafe { std::intrinsics::type_name::<T>() };
  type_name
}
