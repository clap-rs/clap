// Copyright (c) 2017 Guillaume Pinot <texitoi(a)texitoi.eu>
//
// This work is free. You can redistribute it and/or modify it under
// the terms of the Do What The Fuck You Want To Public License,
// Version 2, as published by Sam Hocevar. See the COPYING file for
// more details.

extern crate clap;

pub trait StructOpt {
    fn clap<'a, 'b>() -> clap::App<'a, 'b>;
    fn from_clap(clap::App) -> Self;
    fn from_args() -> Self where Self: Sized {
        Self::from_clap(Self::clap())
    }
}
