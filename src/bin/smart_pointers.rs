use std::rc::Rc;

fn main() {
    {
        let mut boxed: Box<u32> = Box::new(5);
        assert_eq!(5, *boxed);
        assert_eq!(Box::new(5), boxed);
        *boxed = 15;
        assert_eq!(15, *boxed);
    }
    {
        let rc: Rc<u32> = Rc::new(10);
        {
            let rc_clone: Rc<u32> = Rc::clone(&rc);
            assert_eq!(10, *rc);
            assert_eq!(10, *rc_clone);
            assert_eq!(2, Rc::strong_count(&rc));
            assert_eq!(rc, rc_clone);
            // Rc does not allow mutable access directly; we need to use Rc<RefCell<T>> for that.
            // So the following line would not compile even if we declared rc as mutable.
            //*rc = 22;
        }
        assert_eq!(1, Rc::strong_count(&rc));
    }
}
