// TODO: see how to dedup / extract this to app-local test utils
// not to framework, because that would require a runtime dep on insta
#[allow(unused_macros)]
macro_rules! configure_insta {
    ($($expr:expr),*) => {
        let mut settings = insta::Settings::clone_current();
        settings.set_prepend_module_to_snapshot(false);
        settings.set_snapshot_suffix("auth_request");
        let _guard = settings.bind_to_scope();
    };
}
