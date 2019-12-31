#[derive(Default)]
pub struct Resources<T: Default> {
    pub metal: T,
    pub crystal: T,
    pub deuterium: T,
}
