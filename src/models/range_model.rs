pub trait Range<T: Sized + PartialEq + PartialOrd> {

    const MIN: T;
    const MAX: T;

    fn is_in_range(&self) -> bool;
}
