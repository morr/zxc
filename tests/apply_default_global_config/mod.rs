use ctor::ctor;
use zxc::*;

#[ctor]
fn setup() {
    apply_global_config(load_config());
}
