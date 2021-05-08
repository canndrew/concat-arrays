use concat_arrays::*;

#[test]
fn concat_const_arrays() {
    const A0: [u8; 0] = [];
    const A1: [u8; 1] = [0];
    const A2: [u8; 2] = [1, 2];
    const A3: [u8; 3] = [3, 4, 5];

    const S1: [&str; 1] = ["Hello"];
    const S2: [&str; 2] = ["World", "!"];

    let _s = concat_const_arrays!(&str, S1, S2);
    assert_eq!(_s, ["Hello", "World", "!"]);

    let _b0 = concat_const_arrays!(u8);
    let _b0 = concat_const_arrays!(u8, A0);
    let _b0 = concat_const_arrays!(u8, A0,);

    let b1 = concat_const_arrays!(u8, A1);
    assert_eq!(b1, [0]);
    let b1 = concat_const_arrays!(u8, A1,);
    assert_eq!(b1, [0]);
    let b1 = concat_const_arrays!(u8, A0, A1);
    assert_eq!(b1, [0]);
    let b1 = concat_const_arrays!(u8, A0, A1,);
    assert_eq!(b1, [0]);

    let b3 = concat_const_arrays!(u8, A0, A1, A2);
    assert_eq!(b3, [0, 1, 2]);
    let b3 = concat_const_arrays!(u8, A0, A1, A2,);
    assert_eq!(b3, [0, 1, 2]);

    let b6 = concat_const_arrays!(u8, A0, A1, A2, A3);
    assert_eq!(b6, [0, 1, 2, 3, 4, 5]);
    let b6 = concat_const_arrays!(u8, A0, A1, A2, A3,);
    assert_eq!(b6, [0, 1, 2, 3, 4, 5]);
}

#[test]
fn concat_byte_arrays() {
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

#[test]
fn concat_string_arrays() {
    let a0 = [];
    let a1 = [String::from("0")];
    let a2 = [String::from("1"), String::from("2")];
    let a3 = [String::from("3"), String::from("4"), String::from("5")];

    let _b0: [String; 0] = concat_arrays!();
    let _b0: [String; 0] = concat_arrays!(a0.clone());
    let _b0: [String; 0] = concat_arrays!(a0.clone(),);

    let b1: [String; 1] = concat_arrays!(a1.clone());
    assert_eq!(b1, [String::from("0")]);
    let b1: [String; 1] = concat_arrays!(a1.clone(),);
    assert_eq!(b1, [String::from("0")]);
    let b1: [String; 1] = concat_arrays!(a0.clone(), a1.clone());
    assert_eq!(b1, [String::from("0")]);
    let b1: [String; 1] = concat_arrays!(a0.clone(), a1.clone(),);
    assert_eq!(b1, [String::from("0")]);

    let b3: [String; 3] = concat_arrays!(a0.clone(), a1.clone(), a2.clone());
    assert_eq!(b3, [String::from("0"), String::from("1"), String::from("2")]);
    let b3: [String; 3] = concat_arrays!(a0.clone(), a1.clone(), a2.clone(),);
    assert_eq!(b3, [String::from("0"), String::from("1"), String::from("2")]);

    let b6: [String; 6] = concat_arrays!(a0.clone(), a1.clone(), a2.clone(), a3.clone());
    assert_eq!(b6, [
        String::from("0"),
        String::from("1"),
        String::from("2"),
        String::from("3"),
        String::from("4"),
        String::from("5"),
    ]);
    let b6: [String; 6] = concat_arrays!(a0.clone(), a1.clone(), a2.clone(), a3.clone(),);
    assert_eq!(b6, [
        String::from("0"),
        String::from("1"),
        String::from("2"),
        String::from("3"),
        String::from("4"),
        String::from("5"),
    ]);
}
