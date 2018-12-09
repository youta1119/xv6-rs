use spin::Once;
struct Lazy<T, F: Fn() -> T> {
    cell: Once<T>,
    init: F,
}
impl<T, F: Fn() -> T> Lazy<T, F> {
    const fn new(init: F) -> Lazy<T, F> {
        Lazy { cell: sync::OnceCell::new(), init }
    }
}
impl<T, F: Fn() -> T> ::std::ops::Deref for Lazy<T, F> {
    type Target = T;
    fn deref(&self) -> &T {
        self.cell.get_or_init(|| (self.init)())
    }
}