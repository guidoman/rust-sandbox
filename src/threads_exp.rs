use crossbeam_utils::sync::ShardedLock;

fn main() {
    let lock = ShardedLock::new(5);
    // Any number of read locks can be held at once.
    {
        println!("Lock r1");
        let r1 = lock.read().unwrap();
        println!("Lock r2");
        let r2 = lock.read().unwrap();
        assert_eq!(*r1, 5);
        assert_eq!(*r2, 5);
    } // Read locks are dropped at this point.
    // However, only one write lock may be held.
    {
        println!("Lock w1");
        let mut w = lock.write().unwrap();
        *w += 1;
        assert_eq!(*w, 6);
    } // Write lock is dropped here.
}
