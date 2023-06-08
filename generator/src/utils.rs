use heck::ToSnakeCase;

pub fn snake_case_type_name<T>() -> String {
    std::any::type_name::<T>()
        .split("::")
        .last()
        .unwrap()
        .to_owned()
        .to_snake_case()
}
