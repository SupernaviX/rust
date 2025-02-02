use crate::spec::{Cc, LinkerFlavor, Lld, PanicStrategy, Target, TargetMetadata, TargetOptions};

pub fn target() -> Target {
    let base = opts();
    Target {
        llvm_target: "v810-unknown-vb".into(),
        metadata: TargetMetadata {
            description: None,
            tier: None,
            host_tools: None,
            std: Some(false)
        },
        pointer_width: 32,
        data_layout: "e-p:32:32-i32:32-i64:32-f32:32-a:0:32-n32:32-S32".into(),
        arch: "v810".into(),
        options: base,
    }
}

fn opts() -> TargetOptions {
    let mut options: TargetOptions = Default::default();
    options.abi_return_struct_as_int = true;
    options.cpu = "vb".into();
    options.crt_static_default = true;
    options.features = "+gprel".into();
    options.linker_flavor = LinkerFlavor::Gnu(Cc::No, Lld::Yes);
    options.linker = Some("rust-lld".into());
    options.panic_strategy = PanicStrategy::Abort;
    options
}
