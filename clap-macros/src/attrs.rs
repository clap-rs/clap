use std::cell::RefCell;
use std::collections::{BTreeMap, HashMap};

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
        Attributes {
            summary: "".into(),
            docs: "".into(),
            map: BTreeMap::new(),
        }
    }
}

impl Attributes {
    pub fn new() -> Self { Default::default() }

    pub fn from_attrs(attrs: &[syn::Attribute]) -> Self {
        use syn::NestedMetaItem as N;
        use syn::MetaItem as M;
        let mut claps = BTreeMap::new();
        for attr in attrs {
            if let syn::MetaItem::List(ref ident, ref values) = attr.value {
                if ident != "clap" {
                    panic!("Attribute other than #[clap(..)] found");
                }
                for value in values {
                    match *value {
                        // #[foo = "bar"]
                        N::MetaItem(M::NameValue(ref name, ref value)) => {
                            let &mut (_, ref mut attr) = claps.entry(name.to_string())
                                .or_insert((RefCell::new(0), Attribute::new(name.to_string())));
                            attr.push(value.clone());
                        }
                        // #[foo]
                        N::MetaItem(M::Word(ref name)) => {
                            let &mut (_, ref mut attr) = claps.entry(name.to_string())
                                .or_insert((RefCell::new(0), Attribute::new(name.to_string())));
                            attr.push(syn::Lit::Bool(true));
                        }
                        // #[derive(..)]
                        N::MetaItem(M::List(ref ident, ref values)) => {
                            let &mut (_, ref mut attr) = claps.entry(ident.as_ref().to_string())
                                .or_insert((RefCell::new(0),
                                            Attribute::new(ident.as_ref().to_string())));
                            for value in values {
                                match *value {
                                    N::MetaItem(M::Word(ref name)) => attr.push(name.as_ref().into()),
                                    _ => {
                                        panic!("Invalid clap attribute {} literal value not supported",
                                               quote!(#attr).to_string().replace(" ", ""))
                                    }
                                }
                            }
                        }
                        _ => {
                            panic!("Invalid clap attribute {} literal value not supported",
                                   quote!(#attr).to_string().replace(" ", ""))
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
            .fold(String::new(),
                  |docs, line| docs + line.trim_left_matches('/').trim() + "\n");

        let index = docs.find("\n\n");
        let (summary, docs) = if let Some(index) = index {
            let (summary, docs) = docs.split_at(index);
            let (_, docs) = docs.split_at(2);
            (summary.into(), docs.into())
        } else {
            (docs, "".into())
        };

        Attributes {
            summary: summary,
            docs: docs,
            map: claps,
        }
    }

    pub fn check_used(&self, name: &str, field: Option<&str>) {
        for (ref attr, &(ref counter, _)) in &self.map {
            if *counter.borrow() == 0 {
                match field {
                    Some(field) => {
                        println!("clap-macros: unexpected attribute '{}' on field '{}' of struct '{}'",
                                 attr,
                                 field,
                                 name)
                    }
                    None => {
                        println!("clap-macros: unexpected attribute '{}' on struct '{}'",
                                 attr,
                                 name)
                    }
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

    pub fn get_bool(&self, key: &str) -> bool { self.get(key).map(|a| a.into()).unwrap_or(false) }
}

impl FieldAttributes {
    pub fn check_used(&self, name: &str) {
        for (ref field, &(ref counter, ref attrs)) in &self.map {
            if *counter.borrow() == 0 {
                panic!("clap-macros: didn't access attributes for field '{}' on struct '{}' for some reason",
                       field,
                       name);
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

/// Extracts all clap attributes of the form #[clap(i = V)]
pub fn extract_attrs(ast: &syn::MacroInput) -> (Attributes, FieldAttributes) {
    use syn::Body as B;
    use syn::VariantData as V;
    let empty = Attributes::new();
    let root_attrs = Attributes::from_attrs(&ast.attrs);
    let field_attrs = match ast.body {
        B::Struct(V::Struct(ref fields)) => {
            fields.iter()
                .map(|field| {
                    (field.ident.clone().unwrap(),
                     (RefCell::new(0), Attributes::from_attrs(&field.attrs)))
                })
                .collect()
        }
        B::Struct(V::Tuple(_)) => panic!("TODO: tuple struct unsupported msg"),
        B::Struct(V::Unit) | B::Enum(_) => HashMap::new(),
    };
    (root_attrs,
     FieldAttributes {
         empty: empty,
         map: field_attrs,
     })
}
