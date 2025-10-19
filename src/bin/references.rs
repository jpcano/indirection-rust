fn main() {
    {
        let num: u32 = 10;
        let num_ref: &u32 = &num;
        let num_deref: u32 = *num_ref;

        assert_eq!(&num, num_ref);
        assert_eq!(num, num_deref);
        assert_eq!(num_deref, 10);
    }

    {
        let mut num: u32 = 10;

        let num_mutref: &mut u32 = &mut num;
        *num_mutref += 2;

        // The following line is commented out because it would cause a compile-time error
        // because `num` is mutably borrowed by `num_mutref` while the immutable reference is active.
        //let num_ref: &u32 = &num;

        // This line would cause a compile-time error because `num` is
        // mutably borrowed by `num_mutref` after accessing num
        // assert!(num == *num_mutref);
        assert!(*num_mutref == num);

        let num_ref: &u32 = &num;
        assert_eq!(num, 12);
        assert_eq!(*num_ref, 12);
    }
}
