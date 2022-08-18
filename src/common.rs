use std::collections::HashMap;

#[derive(Default)]
pub struct RunningApps(pub HashMap<String, String>);
// TODO impl PartialEq
