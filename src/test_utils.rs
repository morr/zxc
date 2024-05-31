#[cfg(not(test))]
pub fn with_config<T>(_f: impl FnOnce(&mut RootConfig) -> T) -> T {
    panic!("with_config should not be used in non-test code");
}

#[cfg(test)]
pub fn with_config<T>(f: impl FnOnce(&mut RootConfig) -> T) -> T {
    let mut config = CONFIG.write().unwrap();
    f(&mut config)
}
