use alef_compiler::module::AlefComponentModule;
use alef_compiler::resolve::Resolver; 
use std::{cell::RefCell, rc::Rc};

pub fn t(specifer: &str, source: &str) -> (String, Rc<RefCell<Resolver>>) {
    let module = AlefComponentModule::parse(specifer, source).expect("could not parse module");
    let resolver = Rc::new(RefCell::new(Resolver::default()));
    let (code, _) = module
        .transpile(resolver.clone())
        .expect("could not transpile module");
    println!("{}", code);
    (code, resolver)
}

pub fn t_custom_runtime_module(
    specifer: &str,
    source: &str,
    runtime_module: &str,
) -> (String, Rc<RefCell<Resolver>>) {
    let module = AlefComponentModule::parse(specifer, source).expect("could not parse module");
    let resolver = Rc::new(RefCell::new(Resolver::new(specifer, runtime_module)));
    let (code, _) = module
        .transpile(resolver.clone())
        .expect("could not transpile module");
    println!("{}", code);
    (code, resolver)
}
