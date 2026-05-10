#![allow(unused)]

trait List<T> {
    fn count(&self) -> usize;
    fn first(&self) -> &T;
}

impl List<u32> for (u32, u32) {
    fn count(&self) -> usize {
        2
    }

    fn first(&self) -> &u32 {
        &self.0
    }
}

impl<T> List<T> for Vec<T> {
    fn count(&self) -> usize {
        self.len()
    }

    fn first(&self) -> &T {
        &self[0]
    }
}

impl<X, Y> List<(X, Y)> for [(X, Y); 3] {
    fn count(&self) -> usize {
        self.len()
    }

    fn first(&self) -> &(X, Y) {
        &self[0]
    }
}

fn main() {
    let xy: (u32, u32) = (12, 13);
    println!("xy count: {:?}", xy.count());
    println!("xy first: {:?}", xy.first());

    let arr: [(u32, u32); 3] = [(3, 2), (8, 7), (8, 7)];
    println!("arr count: {:?}", arr.count());
    println!("arr first: {:?}", arr.first());
}
