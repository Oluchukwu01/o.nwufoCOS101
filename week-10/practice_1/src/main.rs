fn main() {
    let v = vec![101, 250, 330, 400];
    //vector v own the object in the heap
    //only a single vatriable owns the heap memory at any given time

    let v2 = v.clone();
    //here 2 variables owns a heap value,
    //2 pointers to the same  content is not allowed in rust

    //Rust is veryy smart in terms of memory access so it detects a race condition
    //as two variables point at the same heap

    print!("{:?}",v);
}