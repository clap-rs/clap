use crate::util::fnv::{Key, EMPTY_HASH, HELP_HASH, VERSION_HASH};
use std::fmt::{Debug, Formatter, Result};
use std::hash::{Hash, Hasher};
use std::ops::Deref;

#[derive(Clone, Eq, Default)]
#[cfg_attr(not(debug_assertions), derive(Copy), repr(transparent))]
pub(crate) struct Id {
    #[cfg(debug_assertions)]
    name: String,
    id: u64,
}

macro_rules! precomputed_hashes {
    ($($fn_name:ident, $const_name:ident, $name:expr;)*) => {
        impl Id {
            $(
                pub(crate) fn $fn_name() -> Self {
                    Id {
                        #[cfg(debug_assertions)]
                        name: $name.into(),
                        id: $const_name,
                    }
                }
            )*
        }
    };
}

precomputed_hashes! {
    empty_hash, EMPTY_HASH, "";
    help_hash, HELP_HASH, "help";
    version_hash, VERSION_HASH, "version";
}

impl Id {
    pub(crate) fn from_ref<T: Key>(val: T) -> Self {
        Id {
            #[cfg(debug_assertions)]
            name: val.to_string(),
            id: val.key(),
        }
    }
}

impl Debug for Id {
    fn fmt(&self, f: &mut Formatter) -> Result {
        #[cfg(debug_assertions)]
        write!(f, "{:?} ", self.name)?;
        write!(f, "[hash: {}]", self.id)
    }
}

impl Deref for Id {
    type Target = u64;

    fn deref(&self) -> &Self::Target {
        &self.id
    }
}

impl<T: Key> From<T> for Id {
    fn from(val: T) -> Self {
        Id {
            #[cfg(debug_assertions)]
            name: val.to_string(),
            id: val.key(),
        }
    }
}

impl Hash for Id {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.id.hash(state)
    }
}

impl PartialEq for Id {
    fn eq(&self, other: &Id) -> bool {
        self.id == other.id
    }
}
