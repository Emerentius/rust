// revisions: full min
#![cfg_attr(full, feature(const_generics))]
#![cfg_attr(full, allow(incomplete_features))]
#![cfg_attr(min, feature(min_const_generics))]

use std::mem;

pub trait Trait {
    type Associated: Sized;

    fn associated_size(&self) -> usize {
        [0u8; mem::size_of::<Self::Associated>()];
        //[full]~^ ERROR constant expression depends on a generic parameter
        //[min]~^^ ERROR generic parameters may not be used in const operations
        0
    }
}

fn main() {}
