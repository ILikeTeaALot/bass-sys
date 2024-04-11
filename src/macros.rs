#[macro_export]
macro_rules! generate_bindings {
    {
	    $library_name:ident;
		$doc_root:expr;
	    $(
	        $(#[$meta:meta])*
	        fn $binding_name:ident$(<$($generic_type:ident /*$(: $($generic_trait:ty)+)?*/ $(= $generic_default: ty)?),*>)?($($parameter_name:ident$(.$method: ident())*: $parameter_type:ty $(as $parameter_cast: ty)?),*) $(-> $return_type:ty)?;
	    )*
    } => {
        $(
            static $static_name: once_cell::sync::Lazy<libloading::Symbol<'static, extern "system" fn($($parameter_name: $parameter_type),*) $(-> $return_type)?>> = once_cell::sync::Lazy::new(|| {
                if let Ok(function) = unsafe { BASS_LIBRARY.get(stringify!($binding_name).as_bytes()) } {
                    return function;
                } else {
                    panic!("Failed to load the function.");
                }
            });

            #[allow(non_snake_case)]
            pub fn $binding_name($($parameter_name: $parameter_type),*) $(-> $return_type)? {
                $static_name($($parameter_name),*)
            }
        )*
    };
				$(#[$meta])*
				// #[doc = concat!("The following documentation was directly `include_str!()`'d from the original CHM. Links will not work, and some formatting issues will be present.", include_str!(concat!(env!("OUT_DIR"), $doc_root, stringify!($binding_name), ".html")))]
				#[doc = include_str!(concat!(env!("OUT_DIR"), $doc_root, stringify!($binding_name), ".md"))]
				#[allow(non_snake_case)]
				pub fn $binding_name $(< $($generic_type /*$(: $($generic_trait)+)?*/ $(= $generic_default)?),* >)? ($($parameter_name: $parameter_type),*) $(-> $return_type)? {
					unsafe { $library_name.$binding_name($($parameter_name$(.$method())* $(as $parameter_cast)?),*) }
					// [<STATIC_ $binding_name:upper>]($($parameter_name),*)
				}
        )*
    }
}


#[macro_export]
macro_rules! include_doc {
	($file:expr $(,)?) => {
		concat!("The following documentation was directly `include_str!`'d from the original CHM. Links will not work, and some formatting issues will be present.", include_str!($file))
	};
}