use std::{borrow::Cow, path::Path};

pub struct VdirConfig<'a> {
    pub root: Cow<'a, Path>,
}
