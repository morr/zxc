use ctor::ctor;
use zxc::config::{apply_global_config, load_config};

#[ctor]
fn setup() {
    apply_global_config(load_config());
}
