use super::object::Object;

pub(crate) enum State {
    Normal,
    Return(Object),
}
