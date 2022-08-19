use std::collections::HashMap;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct RunningApps(pub HashMap<String, String>);
