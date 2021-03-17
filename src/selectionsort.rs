use super::Sorter;
pub struct SelectionSort;

impl Sorter for SelectionSort {
    fn sort<T>(&self, slice: &mut [T]) where T: Ord {
        for unsorted in 0..slice.len() {
            let mut smallets_in_rest = unsorted;
            for i in (unsorted + 1)..slice.len() {
                if slice[i] < slice[smallets_in_rest] {
                    smallets_in_rest = i;
                }
            }

            if unsorted != smallets_in_rest {
                slice.swap(unsorted, smallets_in_rest);
            }
        }
    }
}

#[test]
fn selection_works() {
    let mut things = vec![4,2,3,1];
    SelectionSort.sort(&mut things);
    assert_eq!(things, &[1,2,3,4]);
}