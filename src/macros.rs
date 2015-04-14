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

/// Convenience macro getting a typed value
#[macro_export]
macro_rules! value_t {
	($m:ident.value_of($v:expr), $t:ty) => {
		match $m.value_of($v) {
			Some(v) => {
				match v.parse::<$t>() {
					Ok(val) => Ok(val),
					Err(_)  => Err(format!("{} isn't a valid {}",v,stringify!($t))),
				}
			},
			None => Err(format!("Argument not found"))
		}
	};
}

/// Convenience macro getting a typed value or exiting on failure
#[macro_export]
macro_rules! value_t_or_exit {
	($m:ident.value_of($v:expr), $t:ty) => {
		match $m.value_of($v) {
			Some(v) => {
				match v.parse::<$t>() {
					Ok(val) => val,
					Err(_)  => {
						println!("{} isn't a valid {}\n{}\nPlease re-run with --help for more information",v,stringify!($t), $m.usage());
						::std::process::exit(1);
					}
				}
			},
			None => {
				println!("Argument not found\n{}\nPlease re-run with --help for more information", $m.usage());
				::std::process::exit(1);
			}
		}
	};
}