use rhai::plugin::*;

#[export_module]
mod {{crate_name}} {
    // Build your module here.
}

/// Export the {{crate_name}} module.
#[no_mangle]
extern "C" pub fn module_entrypoint() -> rhai::Shared<rhai::Module> {
    rhai::exported_module!({{crate_name}}).into()
}