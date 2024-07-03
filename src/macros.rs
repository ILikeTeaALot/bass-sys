/// Macro to write public binding functions with a convenient Rust-y syntax.
///
/// To mark a function unsafe, it is necessary to write it as `most unsafe` to avoid local ambiguity.
#[macro_export]
macro_rules! generate_bindings {
    {
	    $library_name:ident;
		$doc_root:expr;
	    $(
			$(#[$meta:meta])*
			$visibility:vis
			$(most $safety:ident)?
			fn $binding_name:ident$(<$($generic_type:ident /*$(: $($generic_trait:ty)+)?*/ $(= $generic_default: ty)?),*>)?($($parameter_name:ident$(.$method: ident())*: $parameter_type:ty $(as $parameter_cast: ty)*),*$(,)?) $(-> $return_type:ty)?;
	    )*
    } => {
        $(
			$(#[$meta])*
			// #[doc = concat!("The following documentation was directly `include_str!()`'d from the original CHM. Links will not work, and some formatting issues will be present.", include_str!(concat!(env!("OUT_DIR"), $doc_root, stringify!($binding_name), ".html")))]
			///
			/// <br>
			///
			/// *The original BASS documentation follows...*
			///
			#[doc = include_str!(concat!(env!("OUT_DIR"), $doc_root, stringify!($binding_name), ".md"))]
			#[allow(non_snake_case)]
			$visibility
			// pub
			$($safety)?
			fn $binding_name $(< $($generic_type /*$(: $($generic_trait)+)?*/ $(= $generic_default)?),* >)?($($parameter_name: $parameter_type),*) $(-> $return_type)? {
				unsafe { $library_name.$binding_name($($parameter_name$(.$method())* $(as $parameter_cast)*),*) }.into()
				// [<STATIC_ $binding_name:upper>]($($parameter_name),*)
			}
        )*
    };
}


#[macro_export]
macro_rules! include_doc {
	($file:expr $(,)?) => {
		concat!("The following documentation was directly `include_str!`'d from the original CHM. Links will not work, and some formatting issues will be present.", include_str!($file))
	};
}