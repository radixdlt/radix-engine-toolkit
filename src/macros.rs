#[macro_export]
macro_rules! export_request {
    ($request_type: ident as $export_ident: ident) => {
        /// This function exports a request function and builds a wrapper for it.
        ///
        /// # Safety
        ///
        /// This function makes use of pointers which is an unsafe feature.
        #[no_mangle]
        pub unsafe extern "C" fn $export_ident(
            string_pointer: $crate::memory::Pointer,
        ) -> $crate::memory::Pointer {
            // Loading the request from a string pointer into a request object
            let request: Result<$request_type, _> = $request_type::new_from_pointer(string_pointer);
            let request: $request_type = match request {
                Ok(request) => request,
                Err(error) => {
                    return $crate::memory::toolkit_serialize_and_write_to_memory(&error)
                        .expect("Failed to write a trusted string to memory")
                }
            };

            // Fulfilling the request and either getting back an error or a valid response
            let response: Result<_, _> = request.fulfill_request();
            match response {
                Ok(response) => $crate::memory::toolkit_serialize_and_write_to_memory(&response)
                    .expect("Failed to write a trusted string to memory"),
                Err(error) => $crate::memory::toolkit_serialize_and_write_to_memory(&error)
                    .expect("Failed to write a trusted string to memory"),
            }
        }
    };
}

#[macro_export]
macro_rules! make_request {
    ($function_ident: ident, $request: expr, $response_type: ty) => {{
        let request_pointer: $crate::memory::Pointer =
            $crate::memory::toolkit_serialize_and_write_to_memory(&$request)
                .expect("Failed to write a trusted string to memory");
        let response_pointer: $crate::memory::Pointer = $function_ident(request_pointer);
        if let Ok(resp) = $crate::memory::toolkit_read_and_deserialize_string_from_memory::<
            $response_type,
        >(response_pointer)
        {
            Ok(resp)
        } else if let Ok(resp) = $crate::memory::toolkit_read_and_deserialize_string_from_memory::<
            $crate::error::Error,
        >(response_pointer)
        {
            Err(resp)
        } else {
            panic!("Neither a valid response nor a valid error")
        }
    }};
}
