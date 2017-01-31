use std::cell::RefCell;
use std::collections::{ BTreeMap, HashMap };

use syn;

use attr::Attribute;

pub struct Attributes {
    pub summary: String,
    pub docs: String,
    map: BTreeMap<String, (RefCell<usize>, Attribute)>,
}

pub struct FieldAttributes {
    empty: Attributes,
    map: HashMap<syn::Ident, (RefCell<usize>, Attributes)>,
}

impl Default for Attributes {
    fn default() -> Self {
        Attributes { summary: "".into(), docs: "".into(), map: BTreeMap::new() }
    }
}

impl Attributes {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn check_used(&self, name: &str, field: Option<&str>) {
        for (ref attr, &(ref counter, _)) in &self.map {
            if *counter.borrow() == 0 {
                match field {
                    Some(field) =>
                        println!("clap-macros: unexpected attribute '{}' on field '{}' of struct '{}'", attr, field, name),
                    None =>
                        println!("clap-macros: unexpected attribute '{}' on struct '{}'", attr, name),
                }
            }
        }
    }

    pub fn get(&self, key: &str) -> Option<&Attribute> {
        if let Some(&(ref counter, ref attr)) = self.map.get(key) {
            *counter.borrow_mut() += 1;
            Some(attr)
        } else {
            None
        }
    }

    pub fn get_bool(&self, key: &str) -> bool {
        self.get(key).map(|a| a.into()).unwrap_or(false)
    }
}

impl FieldAttributes {
    pub fn check_used(&self, name: &str) {
        for (ref field, &(ref counter, ref attrs)) in &self.map {
            if *counter.borrow() == 0 {
                panic!("clap-macros: didn't access attributes for field '{}' on struct '{}' for some reason", field, name);
            }
            attrs.check_used(name, Some(field.as_ref()));
        }
    }

    pub fn get(&self, field: &syn::Field) -> &Attributes {
        if let Some(&(ref counter, ref attrs)) = self.map.get(field.ident.as_ref().unwrap()) {
            *counter.borrow_mut() += 1;
            attrs
        } else {
            &self.empty
        }
    }
}

fn extract_attrs_inner(attrs: &Vec<syn::Attribute>) -> Attributes {
    let mut claps = BTreeMap::new();
    for attr in attrs {
        if let syn::MetaItem::List(ref ident, ref values) = attr.value {
            if ident == "clap" {
                for value in values {
                    match *value {
                        syn::NestedMetaItem::MetaItem(ref item) => match *item {
                            syn::MetaItem::NameValue(ref name, ref value) => {
                                let &mut (_, ref mut attr) = claps.entry(name.to_string()).or_insert((RefCell::new(0), Attribute::new(name.to_string())));
                                attr.push(value.clone());
                            }
                            syn::MetaItem::Word(ref name) => {
                                let &mut (_, ref mut attr) = claps.entry(name.to_string()).or_insert((RefCell::new(0), Attribute::new(name.to_string())));
                                attr.push(syn::Lit::Bool(true));
                            }
                            syn::MetaItem::List(ref ident, ref values) => {
                                let &mut (_, ref mut attr) = claps.entry(ident.as_ref().to_string()).or_insert((RefCell::new(0), Attribute::new(ident.as_ref().to_string())));
                                for value in values {
                                    match *value {
                                        syn::NestedMetaItem::MetaItem(ref item) => match *item {
                                            syn::MetaItem::Word(ref name) => {
                                                attr.push(name.as_ref().into());
                                            }
                                            syn::MetaItem::NameValue(..) => {
                                                panic!("Invalid clap attribute {} named value in sublist not supported", quote!(#attr).to_string().replace(" ", ""));
                                            }
                                            syn::MetaItem::List(..) => {
                                                panic!("Invalid clap attribute {} sublist in sublist not supported", quote!(#attr).to_string().replace(" ", ""));
                                            }
                                        },
                                        syn::NestedMetaItem::Literal(_) => {
                                            panic!("Invalid clap attribute {} literal value not supported", quote!(#attr).to_string().replace(" ", ""));
                                        },
                                    }
                                }
                            }
                        },
                        syn::NestedMetaItem::Literal(_) => {
                            panic!("Invalid clap attribute {} literal value not supported", quote!(#attr).to_string().replace(" ", ""));
                        },
                    }
                }
            }
        }
    }

    let docs = attrs.iter()
        .filter(|a| a.is_sugared_doc)
        .map(|a| match a.value {
            syn::MetaItem::NameValue(_, syn::Lit::Str(ref doc, _)) => doc,
            _ => unreachable!(),
        })
        .fold(String::new(), |docs, line| docs + line.trim_left_matches('/').trim() + "\n");

    let index = docs.find("\n\n");
    let (summary, docs) = if let Some(index) = index {
        let (summary, docs) = docs.split_at(index);
        let (_, docs) = docs.split_at(2);
        (summary.into(), docs.into())
    } else {
        (docs, "".into())
    };

    Attributes { summary: summary, docs: docs, map: claps }
}

/// Extracts all clap attributes of the form #[clap(i = V)]
pub fn extract_attrs(ast: &syn::MacroInput) -> (Attributes, FieldAttributes) {
    let empty = Attributes::new();
    let root_attrs = extract_attrs_inner(&ast.attrs);
    let field_attrs = match ast.body {
        syn::Body::Struct(syn::VariantData::Struct(ref fields)) => {
            fields
                .iter()
                .map(|field| (field.ident.clone().unwrap(), (RefCell::new(0), extract_attrs_inner(&field.attrs))))
                .collect()
        }
        syn::Body::Struct(syn::VariantData::Tuple(_)) => {
            panic!("TODO: tuple struct unsupported msg")
        }
        syn::Body::Struct(syn::VariantData::Unit) | syn::Body::Enum(_) => {
            HashMap::new()
        }
    };
    (root_attrs, FieldAttributes { empty: empty, map: field_attrs })
}
