#[macro_export]
macro_rules! clap_app {
    (@as_expr $expr:expr) => { $expr };

    (@app $builder:expr;) => { $builder };
    (@app $builder:expr; (meta => $($name:ident: $($value:expr),+)*) $($tail:tt)*) => {
        clap_app!{ @app $builder $( .$name($($value),*) )*; $($tail)* }
    };
    (@app $builder:expr; (args => $($tt:tt)*) $($tail:tt)*) => {
        clap_app!{ @app $builder.args(clap_app!{ @rules $($tt)* }); $($tail)* }
    };

    (@rules $($name:ident: $tt:tt $desc:tt)*) => {
        vec![
            $( clap_app!{ @rule0
                $crate::Rule::with_name(stringify!($name))
                    .description(clap_app!{ @as_expr $desc });
                $tt
            } ),*
        ]
    };
    (@rule0 $rule:expr; ($($tt:tt)*)) => {
        clap_app!{ @rule $rule .required(); $($tt)* };
    };
    (@rule0 $rule:expr; [$($tt:tt)*]) => {
        clap_app!{ @rule $rule; $($tt)* };
    };

    (@rule $rule:expr;) => { $rule };
    (@rule $rule:expr; --$long:ident $($tail:tt)*) => {
        clap_app!{ @rule $rule.long(stringify!($long)); $($tail)* }
    };
    (@rule $rule:expr; -$short:ident $($tail:tt)*) => {
        clap_app!{ @rule $rule.short(stringify!($short).chars().next().unwrap()); $($tail)* }
    };
    (@rule $rule:expr; ... $($tail:tt)*) => {
        clap_app!{ @rule $rule.max_occurrences(0); $($tail)* }
    };
    (@rule $rule:expr; .. $n:tt $($tail:tt)*) => {
        clap_app!{ @rule $rule.max_occurrences(clap_app!{ @as_expr $n }); $($tail)* }
    };
    (@rule $rule:expr; {$val:expr} $($tail:tt)*) => {
        clap_app!{ @rule $rule /* .val(val) */; $($tail)* }
    };
    (@rule $rule:expr; $arg:ident $($tail:tt)*) => {
        clap_app!{ @rule $rule.takes_value(stringify!($arg)); $($tail)* }
    };

    ($app_name:ident; $($tt:tt)*) => {
//      struct $app_name;

//      impl $app_name {
//          fn new() -> Self {
                clap_app!{ @app $crate::AppBuilder::new(); $($tt)* }
//          }
//      }
    };

}

#[test]
fn testapp() {
    use {App, CollectionMatcher};

    let ac: App = clap_app!{ TestApp;
        (meta => author: "James McGlashan"
                 about:  "Testing application")
        (args => config:  (--config conf {exists}) "Configuration file"
                 verbose: [-v --verbose..3]        "Verbosity level"
                 input:   (input_file)             "Input file"
        )
//      (@subcommands => TestApp) // recursion
    }.into();

    let ref mut sample = vec!["-vvv", "--config", "config.conf", "input"].into_iter();
    assert!(ac.get_matches(sample).is_ok());
}
