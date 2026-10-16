use std::ffi::OsString;

#[derive(Debug, Clone)]
pub struct ProcessArguments {
    values: Vec<OsString>,
}

impl ProcessArguments {
    pub fn values(&self) -> &[OsString] { &self.values }
    pub fn len(&self) -> usize { self.values.len() }
    pub fn is_empty(&self) -> bool { self.values.is_empty() }
    pub fn to_lossy_strings(&self) -> Vec<String> {
        self.values.iter().map(|v| v.to_string_lossy().into_owned()).collect()
    }
}

pub fn current() -> ProcessArguments {
    ProcessArguments { values: std::env::args_os().collect() }
}
