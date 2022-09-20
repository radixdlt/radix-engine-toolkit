#[macro_export]
macro_rules! export_request {
    ($request_type: ident as $export_ident: ident) => {
        #[no_mangle]
        pub unsafe extern "C" fn $export_ident(
            string_pointer: *const std::os::raw::c_char,
        ) -> *const std::os::raw::c_char {
            // Loading the request from a string pointer into a request object
            let request: Result<$request_type, _> = $request_type::new_from_pointer(string_pointer);
            let request: $request_type = match request {
                Ok(request) => request,
                Err(error) => return crate::serialize_to_ptr!(error),
            };

            // Fulfilling the request and either getting back an error or a valid response
            let response: Result<_, _> = request.fulfill_request();
            match response {
                Ok(response) => crate::serialize_to_ptr!(response),
                Err(error) => crate::serialize_to_ptr!(error),
            }
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
