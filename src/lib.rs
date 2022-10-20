pub mod {{crate_name}} {
    // build your module here.
}

#[no_mangle]
pub fn module_entrypoint() -> rhai::Shared<rhai::Module> {
    rhai::exported_module!({{crate_name}}).into()
}