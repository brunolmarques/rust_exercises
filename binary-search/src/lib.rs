use core::cmp::Ordering;

pub fn find<T: Ord>(array: impl AsRef<[T]>, key: T) -> Option<usize> {
    find_tail(array.as_ref(), key, 0)
}

pub fn find_tail<T: Ord>(array: &[T], key: T, offset: usize) -> Option<usize> {
    let mid = array.len() / 2;

    // if array is empty, array[..0].get(_) -> None
    match key.cmp(array.get(mid)?) {
        Ordering::Equal => Some(mid + offset),
        Ordering::Less => find_tail(&array[..mid], key, offset),
        Ordering::Greater => find_tail(&array[mid + 1..], key, offset + mid + 1),
    }
}
