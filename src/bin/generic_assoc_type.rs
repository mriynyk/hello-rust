#![allow(unused)]

struct ArrayIter<T> {
    array: [T; 5],
    i: usize,
}

// Generic Type
trait GenericIterator<T> {
    fn get_next(&mut self) -> Option<T>;
}

impl GenericIterator<u32> for ArrayIter<u32> {
    fn get_next(&mut self) -> Option<u32> {
        match self.array.get(self.i) {
            Some(v) => {
                self.i += 1;
                Some(*v)
            }
            _ => None,
        }
    }
}

// Can be created one more impl
// impl GenericIterator<bool> for ArrayIter<u32> {
//     fn get_next(&mut self) -> Option<bool> {
//         Some(true)
//     }
// }

// Associated type
trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

impl Iterator for ArrayIter<u32> {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        match self.array.get(self.i) {
            Some(v) => {
                self.i += 1;
                Some(*v)
            }
            _ => None,
        }
    }
}

// Can not be created one more impl
// impl Iterator for ArrayIter<u32> {
//     type Item = bool;

//     fn next(&mut self) -> Option<Self::Item> {
//         Some(true)
//     }
// }
fn main() {
    let mut arr_iter: ArrayIter<u32> = ArrayIter {
        array: [1, 2, 3, 4, 5],
        i: 0,
    };

    while let Some(v) = arr_iter.get_next() {
        println!("{:?}", v);
    }

    let mut arr_iter: ArrayIter<u32> = ArrayIter {
        array: [1, 2, 3, 4, 5],
        i: 0,
    };

    while let Some(v) = arr_iter.next() {
        println!("{:?}", v);
    }
}
