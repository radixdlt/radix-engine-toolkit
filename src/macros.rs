#[macro_export]
macro_rules! link_handler {
    ($($method_name: ident => $handler: ident),*) => {
        $(
            #[no_mangle]
            pub unsafe extern "C" fn $method_name(
                string_pointer: *const std::os::raw::c_char
            ) -> *const std::os::raw::c_char {
                let payload = crate::utils::prepare_request(string_pointer);
                let response = match payload {
                    Ok(payload) => $handler(payload),
                    Err(error) => Err(error),
                };

                let response_string = match response {
                    Ok(response) => serde_json::to_string(&response)
                        .expect("Serialization of a trusted response failed"),
                    Err(error) => serde_json::to_string(&error)
                        .expect("Serialization of a trusted response failed")
                };

                std::ffi::CString::new(response_string)
                    .expect("Creation of a C String from a trusted string failed")
                    .into_raw()
            }
        )*
    };
}
