#[macro_export]
macro_rules! export_handler {
    ($handler_ident: ident ($request_type: ty) as $export_ident: ident) => {
        #[no_mangle]
        pub unsafe extern "C" fn $export_ident(
            string_pointer: *const std::os::raw::c_char,
        ) -> *const std::os::raw::c_char {
            // Read and Deserialize the Passed String
            let payload = crate::utils::read_and_deserialize::<$request_type>(string_pointer);
            let payload = match payload {
                Ok(payload) => payload,
                Err(error) => return crate::serialize_to_ptr!(error),
            };

            // Request Validation
            match payload.validate() {
                Err(error) => return crate::serialize_to_ptr!(error),
                _ => {}
            };

            // Handler Dispatch
            let response = $handler_ident(payload);
            let response = match response {
                Ok(response) => response,
                Err(error) => return crate::serialize_to_ptr!(error),
            };

            // Response Validation
            match response.validate() {
                Err(error) => return crate::serialize_to_ptr!(error),
                _ => {}
            };

            crate::serialize_to_ptr!(response)
        }
    };
}

#[macro_export]
macro_rules! serialize_to_ptr {
    ($value: expr) => {
        std::ffi::CString::new(
            serde_json::to_string(&$value).expect("Unable to serialize a trusted payload"),
        )
        .expect("Unable to create a CString from a trusted string")
        .into_raw()
    };
}

#[macro_export]
macro_rules! make_request {
    ($function_ident: ident, $request: expr, $response_type: ty) => {{
        let request_pointer: *const std::os::raw::c_char = crate::serialize_to_ptr!($request);
        let response_pointer: *const std::os::raw::c_char = $function_ident(request_pointer);
        if let Ok(resp) = crate::utils::read_and_deserialize::<$response_type>(response_pointer) {
            Ok(resp)
        } else if let Ok(resp) =
            crate::utils::read_and_deserialize::<$crate::error::Error>(response_pointer)
        {
            Err(resp)
        } else {
            panic!("Neither a valid response nor a valid error")
        }
    }};
}
