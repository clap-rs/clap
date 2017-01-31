use syn;

use attrs::Attributes;

pub enum Field<'a> {
    Arg(Arg<'a>),
    Subcommand(Subcommand<'a>),
}

pub struct Arg<'a> {
    pub ident: &'a syn::Ident,
    pub name: &'a str,
    pub ty: &'a syn::Ty,
    pub short: Option<String>,
    pub long: Option<&'a str>,
    pub value_name: Option<&'a str>,
    pub index: Option<u64>,
    pub summary: &'a str,
    pub docs: &'a str,
    pub takes_value: bool,
    pub is_counter: bool,
    pub multiple: bool,
    pub is_optional: bool,
    pub required: bool,
    pub default_value: Option<&'a str>,
    pub min_values: Option<u64>,
    pub max_values: Option<u64>,
}

pub struct Subcommand<'a> {
    pub ident: &'a syn::Ident,
    pub ty: &'a syn::Ty,
    pub is_optional: bool,
}

impl<'a> Field<'a> {
    pub fn arg(&self) -> Option<&Arg> {
        if let Field::Arg(ref arg) = *self {
            Some(arg)
        } else {
            None
        }
    }

    pub fn subcommand(&self) -> Option<&Subcommand> {
        if let Field::Subcommand(ref subcommand) = *self {
            Some(subcommand)
        } else {
            None
        }
    }
}

impl<'a> From<(&'a syn::Field, &'a Attributes)> for Field<'a> {
    fn from((field, attrs): (&'a syn::Field, &'a Attributes)) -> Field<'a> {
        if attrs.get_bool("subcommand") {
            Field::Subcommand(Subcommand::from(field))
        } else {
            Field::Arg(Arg::from((field, attrs)))
        }
    }
}

impl<'a> From<(&'a syn::Field, &'a Attributes)> for Arg<'a> {
    fn from((field, attrs): (&'a syn::Field, &'a Attributes)) -> Arg<'a> {
        let name = attrs.get("name")
            .map(|a| a.into())
            .unwrap_or_else(|| field.ident.as_ref().unwrap().as_ref());

        let index = attrs.get("index").map(|a| a.into());

        // Unlike clap we default to a flag option unless there's a attribute given
        // telling us to not do so
        let is_flag = !index.is_some() && !attrs.get_bool("arg");

        let long = attrs.get("long")
            .map(|a| a.into())
            .or_else(|| if is_flag { Some(name) } else { None });

        let short = attrs.get("short").map(|s| Into::<char>::into(s).to_string());
        let value_name = attrs.get("value_name").map(|a| a.into());

        let is_counter = attrs.get_bool("counted");

        let (is_bool, is_optional, is_vec, ty);
        match field.ty {
            syn::Ty::Path(None, ref path) => {
                is_bool = path.segments[0].ident == "bool";
                is_optional = path.segments[0].ident == "Option";
                is_vec = path.segments[0].ident == "Vec";
                if is_optional || is_vec {
                    if let syn::PathParameters::AngleBracketed(ref params) =
                        path.segments[0].parameters {
                        ty = &params.types[0];
                    } else {
                        panic!();
                    }
                } else {
                    ty = &field.ty;
                }
            }
            _ => panic!("unsupported field type {:?}", field.ty),
        };

        let multiple = is_counter || is_vec;
        let default_value = attrs.get("default_value").map(|a| a.into());
        let min_values = attrs.get("min_values").map(|a| a.into());
        let max_values = attrs.get("max_values").map(|a| a.into());

        let required = !is_bool && !is_optional;

        Arg {
            ident: field.ident.as_ref().unwrap(),
            ty: ty,
            name: name,
            short: short,
            long: long,
            index: index,
            value_name: value_name,
            summary: &attrs.summary,
            docs: &attrs.docs,
            is_counter: is_counter,
            multiple: multiple,
            takes_value: !is_counter && !is_bool,
            is_optional: is_optional,
            required: required,
            default_value: default_value,
            min_values: min_values,
            max_values: max_values,
        }
    }
}

impl<'a> From<&'a syn::Field> for Subcommand<'a> {
    fn from(field: &'a syn::Field) -> Subcommand<'a> {
        let (is_optional, ty);
        match field.ty {
            syn::Ty::Path(None, ref path) => {
                is_optional = path.segments[0].ident == "Option";
                if is_optional {
                    if let syn::PathParameters::AngleBracketed(ref params) =
                        path.segments[0].parameters {
                        ty = &params.types[0];
                    } else {
                        panic!();
                    }
                } else {
                    ty = &field.ty;
                }
            }
            _ => panic!("unsupported field type {:?}", field.ty),
        };

        Subcommand {
            ident: field.ident.as_ref().unwrap(),
            ty: ty,
            is_optional: is_optional,
        }
    }
}
