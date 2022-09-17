#[macro_export]
macro_rules! export_handler {
    ($handler_ident: ident as $export_ident: ident) => {
        #[no_mangle]
        pub unsafe extern "C" fn $export_ident(
            string_pointer: *const std::os::raw::c_char,
        ) -> *const std::os::raw::c_char {
            // Read and Deserialize the Passed String
            let payload = crate::utils::prepare_request(string_pointer);

            let response = match payload {
                Ok(payload) => {
                    // Validate the request
                    // let validation_response = payload.validate();
                    let validation_response = Ok(());
                    match validation_response {
                        Ok(()) => $handler_ident(payload),
                        Err(error) => Err(error),
                    }
                }
                Err(error) => Err(error),
            };

            // Validate the response
            let response = match response {
                Ok(response) => {
                    // let validation_response = response.validate();
                    let validation_response = Ok(());
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
    };
}
