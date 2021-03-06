use super::*;
use std::io::Cursor;

#[test]
fn basic_8() {
    let mut out = Cursor::new(Vec::new());

    assert_eq!(0u8.pack(&mut out).unwrap(), 1);
    assert_eq!(0xaau8.pack(&mut out).unwrap(), 1);
    assert_eq!(0x34u8.pack(&mut out).unwrap(), 1);

    let v = out.into_inner();

    assert_eq!(v.len(), 3);
    assert_eq!(v, vec![0, 0xaa, 0x34]);

    let mut input = Cursor::new(v);
    assert_eq!(Unpack::unpack(&mut input).unwrap(), (0u8, 1));
    assert_eq!(Unpack::unpack(&mut input).unwrap(), (0xaau8, 1));
    assert_eq!(Unpack::unpack(&mut input).unwrap(), (0x34u8, 1));
}

#[test]
fn basic_32() {
    {
        let mut out = Cursor::new(Vec::new());

        assert_eq!(0u32.pack(&mut out).unwrap(), 4);
        assert_eq!(1000u32.pack(&mut out).unwrap(), 4);
        assert_eq!(823987423u32.pack(&mut out).unwrap(), 4);

        let v = out.into_inner();

        assert_eq!(v.len(), 12);
        assert_eq!(v, vec![0x00, 0x00, 0x00, 0x00,
                           0x00, 0x00, 0x03, 0xe8,
                           0x31, 0x1d, 0x0c, 0xdf,  ]);

        let mut input = Cursor::new(v);
        assert_eq!(Unpack::unpack(&mut input).unwrap(), (0u32, 4));
        assert_eq!(Unpack::unpack(&mut input).unwrap(), (1000u32, 4));
        assert_eq!(Unpack::unpack(&mut input).unwrap(), (823987423u32, 4));
    }

    {
        let mut out = Cursor::new(Vec::new());

        assert_eq!(0i32.pack(&mut out).unwrap(), 4);
        assert_eq!((-1238i32).pack(&mut out).unwrap(), 4);
        assert_eq!(((1i32<<31) as i32).pack(&mut out).unwrap(), 4);

        let v = out.into_inner();

        assert_eq!(v.len(), 12);
        assert_eq!(v, vec![0x00, 0x00, 0x00, 0x00,
                           0xff, 0xff, 0xfb, 0x2a,
                           0x80, 0x00, 0x00, 0x00  ]);

        let mut input = Cursor::new(v);
        assert_eq!(Unpack::unpack(&mut input).unwrap(), (0i32, 4));
        assert_eq!(Unpack::unpack(&mut input).unwrap(), (-1238i32, 4));
        assert_eq!(Unpack::unpack(&mut input).unwrap(), ((1<<31) as i32, 4));
    }
}

#[test]
fn basic_64() {
    {
        let mut out = Cursor::new(Vec::new());

        assert_eq!(0u64.pack(&mut out).unwrap(), 8);
        assert_eq!(0x0011223344556677u64.pack(&mut out).unwrap(), 8);
        assert_eq!(0xff00ff00ff00ff00u64.pack(&mut out).unwrap(), 8);

        let v = out.into_inner();

        assert_eq!(v.len(), 24);
        assert_eq!(v, vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                           0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77,
                           0xff, 0x00, 0xff, 0x00, 0xff, 0x00, 0xff, 0x00  ]);

        let mut input = Cursor::new(v);
        assert_eq!(Unpack::unpack(&mut input).unwrap(), (0u64, 8));
        assert_eq!(Unpack::unpack(&mut input).unwrap(), (4822678189205111u64, 8));
        assert_eq!(Unpack::unpack(&mut input).unwrap(), (18374966859414961920u64, 8));
    }

    {
        let mut out = Cursor::new(Vec::new());

        assert_eq!(0i64.pack(&mut out).unwrap(), 8);
        assert_eq!((-2938928374982749237i64).pack(&mut out).unwrap(), 8);
        assert_eq!(((1i64<<63) as i64).pack(&mut out).unwrap(), 8);

        let v = out.into_inner();

        assert_eq!(v.len(), 24);
        assert_eq!(v, vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                           0xd7, 0x36, 0xd4, 0x36, 0xcc, 0xd6, 0x53, 0xcb,
                           0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00  ]);

        let mut input = Cursor::new(v);
        assert_eq!(Unpack::unpack(&mut input).unwrap(), (0i64, 8));
        assert_eq!(Unpack::unpack(&mut input).unwrap(), (-2938928374982749237i64, 8));
        assert_eq!(Unpack::unpack(&mut input).unwrap(), ((1i64<<63) as i64, 8));
    }
}

#[test]
fn basic_bool() {
    let mut out = Cursor::new(Vec::new());

    assert_eq!(true.pack(&mut out).unwrap(), 4);
    assert_eq!(false.pack(&mut out).unwrap(), 4);

    let v = out.into_inner();

    assert_eq!(v.len(), 8);
    assert_eq!(v, vec![0, 0, 0, 1,  0, 0, 0, 0]);

    let mut input = Cursor::new(v);
    assert_eq!(Unpack::unpack(&mut input).unwrap(), (true, 4));
    assert_eq!(Unpack::unpack(&mut input).unwrap(), (false, 4));

    let bad = vec![0, 0, 0, 2];
    let mut input = Cursor::new(bad);
    match bool::unpack(&mut input) {
        Err(Error::InvalidEnum) => (),
        res => panic!("bad result {:?}", res),
    }
}

#[test]
fn basic_string() {
    {
        let mut out = Cursor::new(Vec::new());

        assert_eq!("foo!".pack(&mut out).unwrap(), 8);

        let v = out.into_inner();

        assert_eq!(v.len(), 8);
        assert_eq!(v, vec![0x00, 0x00, 0x00, 0x04, 0x66, 0x6f, 0x6f, 0x21]);

        let mut input = Cursor::new(v);
        assert_eq!(Unpack::unpack(&mut input).unwrap(), (String::from("foo!"), 8));
    }

    {
        let mut out = Cursor::new(Vec::new());

        assert_eq!("foo".pack(&mut out).unwrap(), 8);

        let v = out.into_inner();

        assert_eq!(v.len(), 8);
        assert_eq!(v, vec![0x00, 0x00, 0x00, 0x03, 0x66, 0x6f, 0x6f, 0x00]);

        let mut input = Cursor::new(v);
        assert_eq!(Unpack::unpack(&mut input).unwrap(), (String::from("foo"), 8));
    }

    {
        let mut out = Cursor::new(Vec::new());

        assert_eq!("foobar".pack(&mut out).unwrap(), 12);
        assert_eq!("piff".pack(&mut out).unwrap(), 8);

        let v = out.into_inner();

        assert_eq!(v.len(), 20);
        assert_eq!(v, vec![0x00, 0x00, 0x00, 0x06,  0x66, 0x6f, 0x6f, 0x62,  0x61, 0x72, 0x00, 0x00,
                           0x00, 0x00, 0x00, 0x04,  0x70, 0x69, 0x66, 0x66]);

        let mut input = Cursor::new(v);
        assert_eq!(Unpack::unpack(&mut input).unwrap(), (String::from("foobar"), 12));
        assert_eq!(Unpack::unpack(&mut input).unwrap(), (String::from("piff"), 8));
    }
}

#[test]
fn basic_opaque_flex() {
    {
        let mut out = Cursor::new(Vec::new());

        assert_eq!(vec![0x11u8, 0x22, 0x33, 0x44].pack(&mut out).unwrap(), 8);

        let v = out.into_inner();

        assert_eq!(v.len(), 8);
        assert_eq!(v, vec![0x00, 0x00, 0x00, 0x04, 0x11, 0x22, 0x33, 0x44]);

        let mut input = Cursor::new(v);
        assert_eq!(Unpack::unpack(&mut input).unwrap(), (vec![0x11u8, 0x22, 0x33, 0x44], 8));
    }

    {
        let mut out = Cursor::new(Vec::new());

        assert_eq!(vec![0x11u8, 0x22, 0x33].pack(&mut out).unwrap(), 8);

        let v = out.into_inner();

        assert_eq!(v.len(), 8);
        assert_eq!(v, vec![0x00, 0x00, 0x00, 0x03, 0x11, 0x22, 0x33, 0x00]);

        let mut input = Cursor::new(v);
        assert_eq!(Unpack::unpack(&mut input).unwrap(), (vec![0x11u8, 0x22, 0x33], 8));
    }

    {
        let mut out = Cursor::new(Vec::new());

        assert_eq!(vec![0x11u8, 0x22, 0x33, 0x44, 0x55].pack(&mut out).unwrap(), 12);

        let v = out.into_inner();

        assert_eq!(v.len(), 12);
        assert_eq!(v, vec![0x00, 0x00, 0x00, 0x05, 0x11, 0x22, 0x33, 0x44, 0x55, 0x00, 0x00, 0x00]);

        let mut input = Cursor::new(v);
        assert_eq!(Unpack::unpack(&mut input).unwrap(), (vec![0x11u8, 0x22, 0x33, 0x44, 0x55], 12));
    }
}

#[test]
fn basic_opaque_array() {
    {
        let mut out = Cursor::new(Vec::new());
        let a = [0x11u8, 0x22, 0x33];

        assert_eq!(pack_array(&a, &mut out).unwrap(), 4);

        let v = out.into_inner();

        assert_eq!(v.len(), 4);
        assert_eq!(v, vec![0x11, 0x22, 0x33, 0x00]);

        let mut input = Cursor::new(v);
        let b: [u8; 3] = [Unpack::unpack(&mut input).unwrap().0,
                          Unpack::unpack(&mut input).unwrap().0,
                          Unpack::unpack(&mut input).unwrap().0,];
        assert_eq!(a, b);
    }

    {
        let mut out = Cursor::new(Vec::new());
        let a = [0x11u8, 0x22, 0x33, 0x44];

        assert_eq!(pack_array(&a, &mut out).unwrap(), 4);

        let v = out.into_inner();

        assert_eq!(v.len(), 4);
        assert_eq!(v, vec![0x11, 0x22, 0x33, 0x44]);

        let mut input = Cursor::new(v);
        let b: [u8; 4] = [Unpack::unpack(&mut input).unwrap().0,
                          Unpack::unpack(&mut input).unwrap().0,
                          Unpack::unpack(&mut input).unwrap().0,
                          Unpack::unpack(&mut input).unwrap().0,];
        assert_eq!(a, b);
    }

    {
        let mut out = Cursor::new(Vec::new());
        let a = [0x11u8, 0x22, 0x33, 0x44, 0x55];

        assert_eq!(pack_array(&a, &mut out).unwrap(), 8);

        let v = out.into_inner();

        assert_eq!(v.len(), 8);
        assert_eq!(v, vec![0x11, 0x22, 0x33, 0x44, 0x55, 0x00, 0x00, 0x00]);

        let mut input = Cursor::new(v);
        let b: [u8; 5] = [Unpack::unpack(&mut input).unwrap().0,
                          Unpack::unpack(&mut input).unwrap().0,
                          Unpack::unpack(&mut input).unwrap().0,
                          Unpack::unpack(&mut input).unwrap().0,
                          Unpack::unpack(&mut input).unwrap().0,
                          ];
        assert_eq!(a, b);
    }
}

#[test]
fn basic_option() {
    let mut out = Cursor::new(Vec::new());
    let none: Option<u32> = None;
    let some: Option<u32> = Some(0x11223344_u32);

    assert_eq!(none.pack(&mut out).unwrap(), 4);
    assert_eq!(some.pack(&mut out).unwrap(), 8);

    let v = out.into_inner();

    assert_eq!(v.len(), 12);
    assert_eq!(v, vec![0x00, 0x00, 0x00, 0x00,
                       0x00, 0x00, 0x00, 0x01,  0x11, 0x22, 0x33, 0x44,]);

    let mut input = Cursor::new(v);
    assert_eq!(Option::<u32>::unpack(&mut input).unwrap(), (None, 4));
    assert_eq!(Unpack::unpack(&mut input).unwrap(), (Some(0x11223344_u32), 8));

    let bad = vec![0, 0, 0, 2];
    let mut input = Cursor::new(bad);

    match Option::<u32>::unpack(&mut input) {
        Err(Error::InvalidEnum) => (),
        res => panic!("bad result {:?}", res),
    }
}
