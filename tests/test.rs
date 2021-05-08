use concat_arrays::concat_arrays;

#[test]
fn concat_some_arrays() {
    let a0 = [];
    let a1 = [0];
    let a2 = [1, 2];
    let a3 = [3, 4, 5];

    let _b0: [u8; 0] = concat_arrays!();
    let _b0: [u8; 0] = concat_arrays!(a0);
    let _b0: [u8; 0] = concat_arrays!(a0,);

    let b1: [u8; 1] = concat_arrays!(a1);
    assert_eq!(b1, [0]);
    let b1: [u8; 1] = concat_arrays!(a1,);
    assert_eq!(b1, [0]);
    let b1: [u8; 1] = concat_arrays!(a0, a1);
    assert_eq!(b1, [0]);
    let b1: [u8; 1] = concat_arrays!(a0, a1,);
    assert_eq!(b1, [0]);

    let b3: [u8; 3] = concat_arrays!(a0, a1, a2);
    assert_eq!(b3, [0, 1, 2]);
    let b3: [u8; 3] = concat_arrays!(a0, a1, a2,);
    assert_eq!(b3, [0, 1, 2]);

    let b6: [u8; 6] = concat_arrays!(a0, a1, a2, a3);
    assert_eq!(b6, [0, 1, 2, 3, 4, 5]);
    let b6: [u8; 6] = concat_arrays!(a0, a1, a2, a3,);
    assert_eq!(b6, [0, 1, 2, 3, 4, 5]);
}
