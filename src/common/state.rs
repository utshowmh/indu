use super::{error::Error, object::Object};

pub(crate) enum State {
    Error(Error),
    Return(Object),
}
