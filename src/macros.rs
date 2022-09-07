#[macro_export]
macro_rules! link_handler {
    ($($method_name: ident => $handler: ident),*) => {
        $(
            #[no_mangle]
            pub unsafe extern "C" fn $method_name(
                string_pointer: *const std::os::raw::c_char,
            ) -> *const std::os::raw::c_char {
                // Read and Deserialize the Passed String
                let payload = crate::utils::prepare_request(string_pointer);

                let response = match payload {
                    Ok(payload) => {
                        // Validate the request
                        let validation_response = crate::validation::validate_request(&payload);
                        match validation_response {
                            Ok(()) => $handler(payload),
                            Err(error) => Err(error),
                        }
                    }
                    Err(error) => Err(error),
                };

                // Validate the response
                let response = match response {
                    Ok(response) => {
                        let validation_response = crate::validation::validate_response(&response);
                        match validation_response {
                            Ok(()) => Ok(response),
                            Err(error) => Err(error),
                        }
                    }
                    Err(error) => Err(error),
                };

                let response_string = match response {
                    Ok(response) => serde_json::to_string(&response).unwrap(),
                    Err(error) => serde_json::to_string(&error).unwrap(),
                };

                std::ffi::CString::new(response_string).unwrap().into_raw()
            }
        )*
    };
}
