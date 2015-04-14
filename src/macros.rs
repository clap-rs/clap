// De-duplication macro used in src/app.rs
macro_rules! get_help {
	($opt:ident) => {
		if let Some(h) = $opt.help {
	        format!("{}{}", h,
	            if let Some(ref pv) = $opt.possible_vals {
	                let mut pv_s = pv.iter().fold(String::with_capacity(50), |acc, name| acc + &format!(" {}",name)[..]);
	                pv_s.shrink_to_fit();
	                format!(" [values:{}]", &pv_s[..])
	            }else{"".to_owned()})
	    } else {
	        "    ".to_owned()
	    } 
	};
}

// Thanks to bluss and flan3002 in #rust IRC
//
// Helps with rightward drift when iterating over something and matching each item.
macro_rules! for_match {
	($it:ident, $($p:pat => $($e:expr);+),*) => {
		for i in $it {
			match i {
			$(
			    $p => { $($e)+ }
			)*
			}
		}
	};
}