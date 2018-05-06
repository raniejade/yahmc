mod proxy;
mod registry;

pub use self::registry::ClassRegistry;

pub(crate) mod constants {
    pub mod metamethod {
        pub static INDEX: &str = "__index";
    }
}
