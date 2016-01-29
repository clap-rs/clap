extern crate clap;

use clap::{App, Arg, ArgSettings};

#[test]
fn opts_using_short() {
    let r = App::new("opts")
        .args(&[
            Arg::from_usage("-f [flag] 'some flag'"),
            Arg::from_usage("-c [color] 'some other flag'")
            ])
        .get_matches_from_safe(vec!["", "-f", "some", "-c", "other"]);
    assert!(r.is_ok());
    let m = r.unwrap();
    assert!(m.is_present("f"));
    assert_eq!(m.value_of("f").unwrap(), "some");
    assert!(m.is_present("c"));
    assert_eq!(m.value_of("c").unwrap(), "other");
}

#[test]
fn opts_using_long_space() {
    let r = App::new("opts")
        .args(&[
            Arg::from_usage("--flag [flag] 'some flag'"),
            Arg::from_usage("--color [color] 'some other flag'")
            ])
        .get_matches_from_safe(vec!["", "--flag", "some", "--color", "other"]);
    assert!(r.is_ok());
    let m = r.unwrap();
    assert!(m.is_present("flag"));
    assert_eq!(m.value_of("flag").unwrap(), "some");
    assert!(m.is_present("color"));
    assert_eq!(m.value_of("color").unwrap(), "other");
}

#[test]
fn opts_using_long_equals() {
    let r = App::new("opts")
        .args(&[
            Arg::from_usage("--flag [flag] 'some flag'"),
            Arg::from_usage("--color [color] 'some other flag'")
            ])
        .get_matches_from_safe(vec!["", "--flag=some", "--color=other"]);
    assert!(r.is_ok());
    let m = r.unwrap();
    assert!(m.is_present("flag"));
    assert_eq!(m.value_of("flag").unwrap(), "some");
    assert!(m.is_present("color"));
    assert_eq!(m.value_of("color").unwrap(), "other");
}

#[test]
fn opts_using_mixed() {
    let r = App::new("opts")
        .args(&[
            Arg::from_usage("-f, --flag [flag] 'some flag'"),
            Arg::from_usage("-c, --color [color] 'some other flag'")
            ])
        .get_matches_from_safe(vec!["", "-f", "some", "--color", "other"]);
    assert!(r.is_ok());
    let m = r.unwrap();
    assert!(m.is_present("flag"));
    assert_eq!(m.value_of("flag").unwrap(), "some");
    assert!(m.is_present("color"));
    assert_eq!(m.value_of("color").unwrap(), "other");
}

#[test]
fn opts_using_mixed2() {
    let r = App::new("opts")
        .args(&[
            Arg::from_usage("-f, --flag [flag] 'some flag'"),
            Arg::from_usage("-c, --color [color] 'some other flag'")
            ])
        .get_matches_from_safe(vec!["", "--flag=some", "-c", "other"]);
    assert!(r.is_ok());
    let m = r.unwrap();
    assert!(m.is_present("flag"));
    assert_eq!(m.value_of("flag").unwrap(), "some");
    assert!(m.is_present("color"));
    assert_eq!(m.value_of("color").unwrap(), "other");
}

#[test]
fn create_option_usage0() {
    // Short only
    let a = Arg::from_usage("[option] -o [opt] 'some help info'");
    assert_eq!(a.name, "option");
    assert_eq!(a.short.unwrap(), 'o');
    assert!(a.long.is_none());
    assert_eq!(a.help.unwrap(), "some help info");
    assert!(!a.is_set(ArgSettings::Multiple));
    assert!(a.is_set(ArgSettings::TakesValue));
    assert!(!a.is_set(ArgSettings::Required));
    assert_eq!(a.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), ["opt"]);
    assert!(a.num_vals.is_none());
}

#[test]
fn create_option_usage1() {
    let b = Arg::from_usage("-o [opt] 'some help info'");
    assert_eq!(b.name, "o");
    assert_eq!(b.short.unwrap(), 'o');
    assert!(b.long.is_none());
    assert_eq!(b.help.unwrap(), "some help info");
    assert!(!b.is_set(ArgSettings::Multiple));
    assert!(b.is_set(ArgSettings::TakesValue));
    assert!(!b.is_set(ArgSettings::Required));
    assert_eq!(b.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), ["opt"]);
    assert!(b.num_vals.is_none());
}

#[test]
fn create_option_usage2() {
    let c = Arg::from_usage("<option> -o <opt> 'some help info'");
    assert_eq!(c.name, "option");
    assert_eq!(c.short.unwrap(), 'o');
    assert!(c.long.is_none());
    assert_eq!(c.help.unwrap(), "some help info");
    assert!(!c.is_set(ArgSettings::Multiple));
    assert!(c.is_set(ArgSettings::TakesValue));
    assert!(c.is_set(ArgSettings::Required));
    assert_eq!(c.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), ["opt"]);
    assert!(c.num_vals.is_none());
}

#[test]
fn create_option_usage3() {
    let d = Arg::from_usage("-o <opt> 'some help info'");
    assert_eq!(d.name, "o");
    assert_eq!(d.short.unwrap(), 'o');
    assert!(d.long.is_none());
    assert_eq!(d.help.unwrap(), "some help info");
    assert!(!d.is_set(ArgSettings::Multiple));
    assert!(d.is_set(ArgSettings::TakesValue));
    assert!(d.is_set(ArgSettings::Required));
    assert_eq!(d.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), ["opt"]);
    assert!(d.num_vals.is_none());
}

#[test]
fn create_option_usage4() {
    let a = Arg::from_usage("[option] -o [opt]... 'some help info'");
    assert_eq!(a.name, "option");
    assert_eq!(a.short.unwrap(), 'o');
    assert!(a.long.is_none());
    assert_eq!(a.help.unwrap(), "some help info");
    assert!(a.is_set(ArgSettings::Multiple));
    assert!(a.is_set(ArgSettings::TakesValue));
    assert!(!a.is_set(ArgSettings::Required));
    assert_eq!(a.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), ["opt"]);
    assert!(a.num_vals.is_none());
}

#[test]
fn create_option_usage5() {
    let a = Arg::from_usage("[option]... -o [opt] 'some help info'");
    assert_eq!(a.name, "option");
    assert_eq!(a.short.unwrap(), 'o');
    assert!(a.long.is_none());
    assert_eq!(a.help.unwrap(), "some help info");
    assert!(a.is_set(ArgSettings::Multiple));
    assert!(a.is_set(ArgSettings::TakesValue));
    assert!(!a.is_set(ArgSettings::Required));
    assert_eq!(a.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), ["opt"]);
    assert!(a.num_vals.is_none());
}

#[test]
fn create_option_usage6() {
    let b = Arg::from_usage("-o [opt]... 'some help info'");
    assert_eq!(b.name, "o");
    assert_eq!(b.short.unwrap(), 'o');
    assert!(b.long.is_none());
    assert_eq!(b.help.unwrap(), "some help info");
    assert!(b.is_set(ArgSettings::Multiple));
    assert!(b.is_set(ArgSettings::TakesValue));
    assert!(!b.is_set(ArgSettings::Required));
    assert_eq!(b.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), ["opt"]);
    assert!(b.num_vals.is_none());
}

#[test]
fn create_option_usage7() {
    let c = Arg::from_usage("<option> -o <opt>... 'some help info'");
    assert_eq!(c.name, "option");
    assert_eq!(c.short.unwrap(), 'o');
    assert!(c.long.is_none());
    assert_eq!(c.help.unwrap(), "some help info");
    assert!(c.is_set(ArgSettings::Multiple));
    assert!(c.is_set(ArgSettings::TakesValue));
    assert!(c.is_set(ArgSettings::Required));
    assert_eq!(c.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), ["opt"]);
    assert!(c.num_vals.is_none());
}

#[test]
fn create_option_usage8() {
    let c = Arg::from_usage("<option>... -o <opt> 'some help info'");
    assert_eq!(c.name, "option");
    assert_eq!(c.short.unwrap(), 'o');
    assert!(c.long.is_none());
    assert_eq!(c.help.unwrap(), "some help info");
    assert!(c.is_set(ArgSettings::Multiple));
    assert!(c.is_set(ArgSettings::TakesValue));
    assert!(c.is_set(ArgSettings::Required));
    assert_eq!(c.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), ["opt"]);
    assert!(c.num_vals.is_none());
}

#[test]
fn create_option_usage9() {
    let d = Arg::from_usage("-o <opt>... 'some help info'");
    assert_eq!(d.name, "o");
    assert_eq!(d.short.unwrap(), 'o');
    assert!(d.long.is_none());
    assert_eq!(d.help.unwrap(), "some help info");
    assert!(d.is_set(ArgSettings::Multiple));
    assert!(d.is_set(ArgSettings::TakesValue));
    assert!(d.is_set(ArgSettings::Required));
    assert_eq!(d.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), ["opt"]);
    assert!(d.num_vals.is_none());
}

#[test]
fn create_option_usage_long1() {
    let a = Arg::from_usage("[option] --opt [opt] 'some help info'");
    assert_eq!(a.name, "option");
    assert_eq!(a.long.unwrap(), "opt");
    assert!(a.short.is_none());
    assert_eq!(a.help.unwrap(), "some help info");
    assert!(!a.is_set(ArgSettings::Multiple));
    assert!(a.is_set(ArgSettings::TakesValue));
    assert!(!a.is_set(ArgSettings::Required));
    assert_eq!(a.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), ["opt"]);
    assert!(a.num_vals.is_none());
}

#[test]
fn create_option_usage_long2() {
    let b = Arg::from_usage("--opt [option] 'some help info'");
    assert_eq!(b.name, "opt");
    assert_eq!(b.long.unwrap(), "opt");
    assert!(b.short.is_none());
    assert_eq!(b.help.unwrap(), "some help info");
    assert!(!b.is_set(ArgSettings::Multiple));
    assert!(b.is_set(ArgSettings::TakesValue));
    assert!(!b.is_set(ArgSettings::Required));
    assert_eq!(b.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), ["option"]);
    assert!(b.num_vals.is_none());
}

#[test]
fn create_option_usage_long3() {
    let c = Arg::from_usage("<option> --opt <opt> 'some help info'");
    assert_eq!(c.name, "option");
    assert_eq!(c.long.unwrap(), "opt");
    assert!(c.short.is_none());
    assert_eq!(c.help.unwrap(), "some help info");
    assert!(!c.is_set(ArgSettings::Multiple));
    assert!(c.is_set(ArgSettings::TakesValue));
    assert!(c.is_set(ArgSettings::Required));
    assert_eq!(c.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), ["opt"]);
    assert!(c.num_vals.is_none());
}

#[test]
fn create_option_usage_long4() {
    let d = Arg::from_usage("--opt <option> 'some help info'");
    assert_eq!(d.name, "opt");
    assert_eq!(d.long.unwrap(), "opt");
    assert!(d.short.is_none());
    assert_eq!(d.help.unwrap(), "some help info");
    assert!(!d.is_set(ArgSettings::Multiple));
    assert!(d.is_set(ArgSettings::TakesValue));
    assert!(d.is_set(ArgSettings::Required));
    assert_eq!(d.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), ["option"]);
    assert!(d.num_vals.is_none());
}

#[test]
fn create_option_usage_long5() {
    let a = Arg::from_usage("[option] --opt [opt]... 'some help info'");
    assert_eq!(a.name, "option");
    assert_eq!(a.long.unwrap(), "opt");
    assert!(a.short.is_none());
    assert_eq!(a.help.unwrap(), "some help info");
    assert!(a.is_set(ArgSettings::Multiple));
    assert!(a.is_set(ArgSettings::TakesValue));
    assert!(!a.is_set(ArgSettings::Required));
    assert_eq!(a.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), ["opt"]);
    assert!(a.num_vals.is_none());
}

#[test]
fn create_option_usage_long6() {
    let a = Arg::from_usage("[option]... --opt [opt] 'some help info'");
    assert_eq!(a.name, "option");
    assert_eq!(a.long.unwrap(), "opt");
    assert!(a.short.is_none());
    assert_eq!(a.help.unwrap(), "some help info");
    assert!(a.is_set(ArgSettings::Multiple));
    assert!(a.is_set(ArgSettings::TakesValue));
    assert!(!a.is_set(ArgSettings::Required));
    assert_eq!(a.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), ["opt"]);
    assert!(a.num_vals.is_none());
}

#[test]
fn create_option_usage_long7() {
    let b = Arg::from_usage("--opt [option]... 'some help info'");
    assert_eq!(b.name, "opt");
    assert_eq!(b.long.unwrap(), "opt");
    assert!(b.short.is_none());
    assert_eq!(b.help.unwrap(), "some help info");
    assert!(b.is_set(ArgSettings::Multiple));
    assert!(b.is_set(ArgSettings::TakesValue));
    assert!(!b.is_set(ArgSettings::Required));
    assert_eq!(b.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), ["option"]);
    assert!(b.num_vals.is_none());
}

#[test]
fn create_option_usage_long8() {
    let c = Arg::from_usage("<option> --opt <opt>... 'some help info'");
    assert_eq!(c.name, "option");
    assert_eq!(c.long.unwrap(), "opt");
    assert!(c.short.is_none());
    assert_eq!(c.help.unwrap(), "some help info");
    assert!(c.is_set(ArgSettings::Multiple));
    assert!(c.is_set(ArgSettings::TakesValue));
    assert!(c.is_set(ArgSettings::Required));
    assert_eq!(c.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), ["opt"]);
    assert!(c.num_vals.is_none());
}

#[test]
fn create_option_usage_long9() {
    let c = Arg::from_usage("<option>... --opt <opt> 'some help info'");
    assert_eq!(c.name, "option");
    assert_eq!(c.long.unwrap(), "opt");
    assert!(c.short.is_none());
    assert_eq!(c.help.unwrap(), "some help info");
    assert!(c.is_set(ArgSettings::Multiple));
    assert!(c.is_set(ArgSettings::TakesValue));
    assert!(c.is_set(ArgSettings::Required));
    assert_eq!(c.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), ["opt"]);
    assert!(c.num_vals.is_none());
}

#[test]
fn create_option_usage_long10() {
    let d = Arg::from_usage("--opt <option>... 'some help info'");
    assert_eq!(d.name, "opt");
    assert_eq!(d.long.unwrap(), "opt");
    assert!(d.short.is_none());
    assert_eq!(d.help.unwrap(), "some help info");
    assert!(d.is_set(ArgSettings::Multiple));
    assert!(d.is_set(ArgSettings::TakesValue));
    assert!(d.is_set(ArgSettings::Required));
    assert_eq!(d.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), ["option"]);
    assert!(d.num_vals.is_none());
}

#[test]
fn create_option_usage_long_equals1() {
    let a = Arg::from_usage("[option] --opt=[opt] 'some help info'");
    assert_eq!(a.name, "option");
    assert_eq!(a.long.unwrap(), "opt");
    assert!(a.short.is_none());
    assert_eq!(a.help.unwrap(), "some help info");
    assert!(!a.is_set(ArgSettings::Multiple));
    assert!(a.is_set(ArgSettings::TakesValue));
    assert!(!a.is_set(ArgSettings::Required));
    assert_eq!(a.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), ["opt"]);
    assert!(a.num_vals.is_none());
}

#[test]
fn create_option_usage_long_equals2() {
    let b = Arg::from_usage("--opt=[option] 'some help info'");
    assert_eq!(b.name, "opt");
    assert_eq!(b.long.unwrap(), "opt");
    assert!(b.short.is_none());
    assert_eq!(b.help.unwrap(), "some help info");
    assert!(!b.is_set(ArgSettings::Multiple));
    assert!(b.is_set(ArgSettings::TakesValue));
    assert!(!b.is_set(ArgSettings::Required));
    assert_eq!(b.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), ["option"]);
    assert!(b.num_vals.is_none());
}

#[test]
fn create_option_usage_long_equals3() {
    let c = Arg::from_usage("<option> --opt=<opt> 'some help info'");
    assert_eq!(c.name, "option");
    assert_eq!(c.long.unwrap(), "opt");
    assert!(c.short.is_none());
    assert_eq!(c.help.unwrap(), "some help info");
    assert!(!c.is_set(ArgSettings::Multiple));
    assert!(c.is_set(ArgSettings::TakesValue));
    assert!(c.is_set(ArgSettings::Required));
    assert_eq!(c.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), ["opt"]);
    assert!(c.num_vals.is_none());
}

#[test]
fn create_option_usage_long_equals4() {
    let d = Arg::from_usage("--opt=<option> 'some help info'");
    assert_eq!(d.name, "opt");
    assert_eq!(d.long.unwrap(), "opt");
    assert!(d.short.is_none());
    assert_eq!(d.help.unwrap(), "some help info");
    assert!(!d.is_set(ArgSettings::Multiple));
    assert!(d.is_set(ArgSettings::TakesValue));
    assert!(d.is_set(ArgSettings::Required));
    assert_eq!(d.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), ["option"]);
    assert!(d.num_vals.is_none());
}

#[test]
fn create_option_usage_long_equals5() {
    let a = Arg::from_usage("[option] --opt=[opt]... 'some help info'");
    assert_eq!(a.name, "option");
    assert_eq!(a.long.unwrap(), "opt");
    assert!(a.short.is_none());
    assert_eq!(a.help.unwrap(), "some help info");
    assert!(a.is_set(ArgSettings::Multiple));
    assert!(a.is_set(ArgSettings::TakesValue));
    assert!(!a.is_set(ArgSettings::Required));
    assert_eq!(a.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), ["opt"]);
    assert!(a.num_vals.is_none());
}

#[test]
fn create_option_usage_long_equals6() {
    let a = Arg::from_usage("[option]... --opt=[opt] 'some help info'");
    assert_eq!(a.name, "option");
    assert_eq!(a.long.unwrap(), "opt");
    assert!(a.short.is_none());
    assert_eq!(a.help.unwrap(), "some help info");
    assert!(a.is_set(ArgSettings::Multiple));
    assert!(a.is_set(ArgSettings::TakesValue));
    assert!(!a.is_set(ArgSettings::Required));
    assert_eq!(a.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), ["opt"]);
    assert!(a.num_vals.is_none());
}

#[test]
fn create_option_usage_long_equals7() {
    let b = Arg::from_usage("--opt=[option]... 'some help info'");
    assert_eq!(b.name, "opt");
    assert_eq!(b.long.unwrap(), "opt");
    assert!(b.short.is_none());
    assert_eq!(b.help.unwrap(), "some help info");
    assert!(b.is_set(ArgSettings::Multiple));
    assert!(b.is_set(ArgSettings::TakesValue));
    assert!(!b.is_set(ArgSettings::Required));
    assert_eq!(b.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), ["option"]);
    assert!(b.num_vals.is_none());
}

#[test]
fn create_option_usage_long_equals8() {
    let c = Arg::from_usage("<option> --opt=<opt>... 'some help info'");
    assert_eq!(c.name, "option");
    assert_eq!(c.long.unwrap(), "opt");
    assert!(c.short.is_none());
    assert_eq!(c.help.unwrap(), "some help info");
    assert!(c.is_set(ArgSettings::Multiple));
    assert!(c.is_set(ArgSettings::TakesValue));
    assert!(c.is_set(ArgSettings::Required));
    assert_eq!(c.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), ["opt"]);
    assert!(c.num_vals.is_none());
}

#[test]
fn create_option_usage_long_equals9() {
    let c = Arg::from_usage("<option>... --opt=<opt> 'some help info'");
    assert_eq!(c.name, "option");
    assert_eq!(c.long.unwrap(), "opt");
    assert!(c.short.is_none());
    assert_eq!(c.help.unwrap(), "some help info");
    assert!(c.is_set(ArgSettings::Multiple));
    assert!(c.is_set(ArgSettings::TakesValue));
    assert!(c.is_set(ArgSettings::Required));
    assert_eq!(c.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), ["opt"]);
    assert!(c.num_vals.is_none());
}

#[test]
fn create_option_usage_long_equals10() {
    let d = Arg::from_usage("--opt=<option>... 'some help info'");
    assert_eq!(d.name, "opt");
    assert_eq!(d.long.unwrap(), "opt");
    assert!(d.short.is_none());
    assert_eq!(d.help.unwrap(), "some help info");
    assert!(d.is_set(ArgSettings::Multiple));
    assert!(d.is_set(ArgSettings::TakesValue));
    assert!(d.is_set(ArgSettings::Required));
    assert_eq!(d.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), ["option"]);
    assert!(d.num_vals.is_none());
}

#[test]
fn create_option_usage_both1() {
    let a = Arg::from_usage("[option] -o --opt [option] 'some help info'");
    assert_eq!(a.name, "option");
    assert_eq!(a.long.unwrap(), "opt");
    assert_eq!(a.short.unwrap(), 'o');
    assert_eq!(a.help.unwrap(), "some help info");
    assert!(!a.is_set(ArgSettings::Multiple));
    assert!(a.is_set(ArgSettings::TakesValue));
    assert!(!a.is_set(ArgSettings::Required));
    assert_eq!(a.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), ["option"]);
    assert!(a.num_vals.is_none());
}

#[test]
fn create_option_usage_both2() {
    let b = Arg::from_usage("-o --opt [option] 'some help info'");
    assert_eq!(b.name, "opt");
    assert_eq!(b.long.unwrap(), "opt");
    assert_eq!(b.short.unwrap(), 'o');
    assert_eq!(b.help.unwrap(), "some help info");
    assert!(!b.is_set(ArgSettings::Multiple));
    assert!(b.is_set(ArgSettings::TakesValue));
    assert!(!b.is_set(ArgSettings::Required));
    assert_eq!(b.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), ["option"]);
    assert!(b.num_vals.is_none());
}

#[test]
fn create_option_usage_both3() {
    let c = Arg::from_usage("<option> -o --opt <opt> 'some help info'");
    assert_eq!(c.name, "option");
    assert_eq!(c.long.unwrap(), "opt");
    assert_eq!(c.short.unwrap(), 'o');
    assert_eq!(c.help.unwrap(), "some help info");
    assert!(!c.is_set(ArgSettings::Multiple));
    assert!(c.is_set(ArgSettings::TakesValue));
    assert!(c.is_set(ArgSettings::Required));
    assert_eq!(c.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), ["opt"]);
    assert!(c.num_vals.is_none());
}

#[test]
fn create_option_usage_both4() {
    let d = Arg::from_usage("-o --opt <option> 'some help info'");
    assert_eq!(d.name, "opt");
    assert_eq!(d.long.unwrap(), "opt");
    assert_eq!(d.short.unwrap(), 'o');
    assert_eq!(d.help.unwrap(), "some help info");
    assert!(!d.is_set(ArgSettings::Multiple));
    assert!(d.is_set(ArgSettings::TakesValue));
    assert!(d.is_set(ArgSettings::Required));
    assert_eq!(d.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), ["option"]);
    assert!(d.num_vals.is_none());
}

#[test]
fn create_option_usage_both5() {
    let a = Arg::from_usage("[option]... -o --opt [option] 'some help info'");
    assert_eq!(a.name, "option");
    assert_eq!(a.long.unwrap(), "opt");
    assert_eq!(a.short.unwrap(), 'o');
    assert_eq!(a.help.unwrap(), "some help info");
    assert!(a.is_set(ArgSettings::Multiple));
    assert!(a.is_set(ArgSettings::TakesValue));
    assert!(!a.is_set(ArgSettings::Required));
    assert_eq!(a.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), ["option"]);
    assert!(a.num_vals.is_none());
}

#[test]
fn create_option_usage_both6() {
    let b = Arg::from_usage("-o --opt [option]... 'some help info'");
    assert_eq!(b.name, "opt");
    assert_eq!(b.long.unwrap(), "opt");
    assert_eq!(b.short.unwrap(), 'o');
    assert_eq!(b.help.unwrap(), "some help info");
    assert!(b.is_set(ArgSettings::Multiple));
    assert!(b.is_set(ArgSettings::TakesValue));
    assert!(!b.is_set(ArgSettings::Required));
    assert_eq!(b.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), ["option"]);
    assert!(b.num_vals.is_none());
}

#[test]
fn create_option_usage_both7() {
    let c = Arg::from_usage("<option>... -o --opt <opt> 'some help info'");
    assert_eq!(c.name, "option");
    assert_eq!(c.long.unwrap(), "opt");
    assert_eq!(c.short.unwrap(), 'o');
    assert_eq!(c.help.unwrap(), "some help info");
    assert!(c.is_set(ArgSettings::Multiple));
    assert!(c.is_set(ArgSettings::TakesValue));
    assert!(c.is_set(ArgSettings::Required));
    assert_eq!(c.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), ["opt"]);
    assert!(c.num_vals.is_none());
}

#[test]
fn create_option_usage_both8() {
    let d = Arg::from_usage("-o --opt <option>... 'some help info'");
    assert_eq!(d.name, "opt");
    assert_eq!(d.long.unwrap(), "opt");
    assert_eq!(d.short.unwrap(), 'o');
    assert_eq!(d.help.unwrap(), "some help info");
    assert!(d.is_set(ArgSettings::Multiple));
    assert!(d.is_set(ArgSettings::TakesValue));
    assert!(d.is_set(ArgSettings::Required));
    assert_eq!(d.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), ["option"]);
    assert!(d.num_vals.is_none());
}

#[test]
fn create_option_usage_both_equals1() {
    let a = Arg::from_usage("[option] -o --opt=[option] 'some help info'");
    assert_eq!(a.name, "option");
    assert_eq!(a.long.unwrap(), "opt");
    assert_eq!(a.short.unwrap(), 'o');
    assert_eq!(a.help.unwrap(), "some help info");
    assert!(!a.is_set(ArgSettings::Multiple));
    assert!(a.is_set(ArgSettings::TakesValue));
    assert!(!a.is_set(ArgSettings::Required));
    assert_eq!(a.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), ["option"]);
    assert!(a.num_vals.is_none());
}

#[test]
fn create_option_usage_both_equals2() {
    let b = Arg::from_usage("-o --opt=[option] 'some help info'");
    assert_eq!(b.name, "opt");
    assert_eq!(b.long.unwrap(), "opt");
    assert_eq!(b.short.unwrap(), 'o');
    assert_eq!(b.help.unwrap(), "some help info");
    assert!(!b.is_set(ArgSettings::Multiple));
    assert!(b.is_set(ArgSettings::TakesValue));
    assert!(!b.is_set(ArgSettings::Required));
    assert_eq!(b.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), ["option"]);
    assert!(b.num_vals.is_none());
}

#[test]
fn create_option_usage_both_equals3() {
    let c = Arg::from_usage("<option> -o --opt=<opt> 'some help info'");
    assert_eq!(c.name, "option");
    assert_eq!(c.long.unwrap(), "opt");
    assert_eq!(c.short.unwrap(), 'o');
    assert_eq!(c.help.unwrap(), "some help info");
    assert!(!c.is_set(ArgSettings::Multiple));
    assert!(c.is_set(ArgSettings::TakesValue));
    assert!(c.is_set(ArgSettings::Required));
    assert_eq!(c.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), ["opt"]);
    assert!(c.num_vals.is_none());
}

#[test]
fn create_option_usage_both_equals4() {
    let d = Arg::from_usage("-o --opt=<option> 'some help info'");
    assert_eq!(d.name, "opt");
    assert_eq!(d.long.unwrap(), "opt");
    assert_eq!(d.short.unwrap(), 'o');
    assert_eq!(d.help.unwrap(), "some help info");
    assert!(!d.is_set(ArgSettings::Multiple));
    assert!(d.is_set(ArgSettings::TakesValue));
    assert!(d.is_set(ArgSettings::Required));
    assert_eq!(d.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), ["option"]);
    assert!(d.num_vals.is_none());
}

#[test]
fn create_option_usage_both_equals5() {
    let a = Arg::from_usage("[option]... -o --opt=[option] 'some help info'");
    assert_eq!(a.name, "option");
    assert_eq!(a.long.unwrap(), "opt");
    assert_eq!(a.short.unwrap(), 'o');
    assert_eq!(a.help.unwrap(), "some help info");
    assert!(a.is_set(ArgSettings::Multiple));
    assert!(a.is_set(ArgSettings::TakesValue));
    assert!(!a.is_set(ArgSettings::Required));
    assert_eq!(a.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), ["option"]);
    assert!(a.num_vals.is_none());
}

#[test]
fn create_option_usage_both_equals6() {
    let b = Arg::from_usage("-o --opt=[option]... 'some help info'");
    assert_eq!(b.name, "opt");
    assert_eq!(b.long.unwrap(), "opt");
    assert_eq!(b.short.unwrap(), 'o');
    assert_eq!(b.help.unwrap(), "some help info");
    assert!(b.is_set(ArgSettings::Multiple));
    assert!(b.is_set(ArgSettings::TakesValue));
    assert!(!b.is_set(ArgSettings::Required));
    assert_eq!(b.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), ["option"]);
    assert!(b.num_vals.is_none());
}

#[test]
fn create_option_usage_both_equals7() {
    let c = Arg::from_usage("<option>... -o --opt=<opt> 'some help info'");
    assert_eq!(c.name, "option");
    assert_eq!(c.long.unwrap(), "opt");
    assert_eq!(c.short.unwrap(), 'o');
    assert_eq!(c.help.unwrap(), "some help info");
    assert!(c.is_set(ArgSettings::Multiple));
    assert!(c.is_set(ArgSettings::TakesValue));
    assert!(c.is_set(ArgSettings::Required));
    assert_eq!(c.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), ["opt"]);
    assert!(c.num_vals.is_none());
}

#[test]
fn create_option_usage_both_equals8() {
    let d = Arg::from_usage("-o --opt=<option>... 'some help info'");
    assert_eq!(d.name, "opt");
    assert_eq!(d.long.unwrap(), "opt");
    assert_eq!(d.short.unwrap(), 'o');
    assert_eq!(d.help.unwrap(), "some help info");
    assert!(d.is_set(ArgSettings::Multiple));
    assert!(d.is_set(ArgSettings::TakesValue));
    assert!(d.is_set(ArgSettings::Required));
    assert_eq!(d.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), ["option"]);
    assert!(d.num_vals.is_none());
}

#[test]
fn create_option_with_vals1() {
    let d = Arg::from_usage("-o <file> <mode> 'some help info'");
    assert_eq!(d.name, "o");
    assert!(d.long.is_none());
    assert_eq!(d.short.unwrap(), 'o');
    assert_eq!(d.help.unwrap(), "some help info");
    assert!(!d.is_set(ArgSettings::Multiple));
    assert!(d.is_set(ArgSettings::TakesValue));
    assert!(d.is_set(ArgSettings::Required));
    assert_eq!(d.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), ["file", "mode"]);
    assert_eq!(d.num_vals.unwrap(), 2);
}

#[test]
fn create_option_with_vals2() {
    let d = Arg::from_usage("-o <file> <mode>... 'some help info'");
    assert_eq!(d.name, "o");
    assert!(d.long.is_none());
    assert_eq!(d.short.unwrap(), 'o');
    assert_eq!(d.help.unwrap(), "some help info");
    assert!(d.is_set(ArgSettings::Multiple));
    assert!(d.is_set(ArgSettings::TakesValue));
    assert!(d.is_set(ArgSettings::Required));
    assert_eq!(d.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), ["file", "mode"]);
    assert_eq!(d.num_vals.unwrap(), 2);
}

#[test]
fn create_option_with_vals3() {
    let d = Arg::from_usage("--opt <file> <mode>... 'some help info'");
    assert_eq!(d.name, "opt");
    assert!(d.short.is_none());
    assert_eq!(d.long.unwrap(), "opt");
    assert_eq!(d.help.unwrap(), "some help info");
    assert!(d.is_set(ArgSettings::Multiple));
    assert!(d.is_set(ArgSettings::TakesValue));
    assert!(d.is_set(ArgSettings::Required));
    assert_eq!(d.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), ["file", "mode"]);
    assert_eq!(d.num_vals.unwrap(), 2);
}

#[test]
fn create_option_with_vals4() {
    let d = Arg::from_usage("[myopt] --opt <file> <mode> 'some help info'");
    assert_eq!(d.name, "myopt");
    assert!(d.short.is_none());
    assert_eq!(d.long.unwrap(), "opt");
    assert_eq!(d.help.unwrap(), "some help info");
    assert!(!d.is_set(ArgSettings::Multiple));
    assert!(d.is_set(ArgSettings::TakesValue));
    assert!(!d.is_set(ArgSettings::Required));
    assert_eq!(d.val_names.unwrap().iter().map(|(_, &v)| v).collect::<Vec<_>>(), ["file", "mode"]);
    assert_eq!(d.num_vals.unwrap(), 2);
}

#[test]
fn create_option_with_vals5() {
    let d = Arg::from_usage("--opt <file> <mode> 'some help info'");
    assert_eq!(d.name, "opt");
    assert!(d.short.is_none());
    assert_eq!(d.long.unwrap(), "opt");
    assert_eq!(d.help.unwrap(), "some help info");
    assert!(!d.is_set(ArgSettings::Multiple));
    assert!(d.is_set(ArgSettings::TakesValue));
    assert!(d.is_set(ArgSettings::Required));
    assert_eq!(d.num_vals.unwrap(), 2);
}
