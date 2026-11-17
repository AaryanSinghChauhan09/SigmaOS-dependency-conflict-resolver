//! Integration tests for SigmaOS-dependency-conflict-resolver.

use sigmaos_dependency_conflict_resolver_lib::Component;

#[test]
fn component_initialises_and_runs() {
    let mut c = Component::new();
    c.init().expect("init should succeed");
    c.run().expect("run should succeed after init");
}
