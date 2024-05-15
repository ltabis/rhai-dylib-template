use rhai::plugin::*;

#[export_module]
mod {{crate_name}} {
    // Build your module here.
}

/// Export the {{crate_name}} module.
#[no_mangle]
pub extern "C" fn module_entrypoint() -> rhai::Shared<rhai::Module> {
    // The seed must be the same as the one used in the program that will
    // load this module.
    rhai::config::hashing::set_hashing_seed(Some({{ahash-key}})).unwrap();

    rhai::exported_module!({{crate_name}}).into()
}