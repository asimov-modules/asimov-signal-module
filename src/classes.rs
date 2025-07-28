// This is free and unencumbered software released into the public domain.

use alloc::string::String;

#[derive(Clone, Debug)]
pub struct SignalAccount {
    pub id: String,
}

#[derive(Clone, Debug)]
pub struct SignalChat {
    pub id: String,
}

#[derive(Clone, Debug)]
pub struct SignalGroupChat {
    pub id: String,
}
