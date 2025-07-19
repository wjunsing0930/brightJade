pub mod my_ascii{
    #[derive(Debug, Eq, PartialEq)]
    pub struct Ascii( pub Vec<u8>);

    impl Ascii{
        pub fn from_bytes(bytes:Vec<u8>) -> Result<Ascii, NotAsciiError>{
            if bytes.iter().any(|&byte| !byte.is_ascii()) {
                return Err(NotAsciiError(bytes))
            }
            Ok(Ascii(bytes))
        }

        pub unsafe fn from_bytes_unchecked(bytes:Vec<u8>) ->Ascii{
            Ascii(bytes)
        }
    }

    #[derive(Debug, Eq, PartialEq)]
    pub struct NotAsciiError(pub Vec<u8>); //自定义错误类型  Resutl<Ok,Err>

    impl From<Ascii> for String{
        fn from(ascii:Ascii) ->String{
            //0 代表访问结构体的第一个字段也是唯一字段,多字段不适用
            //因为Ascii没有给成员起名字且只有唯一的成员,所以直接用0
            unsafe{String::from_utf8_unchecked(ascii.0)}
        }
    }
}

#[test]
fn good_ascii() {
    use my_ascii::Ascii;

    let bytes: Vec<u8> = b"ASCII and ye shall receive".to_vec();
    let ascii: Ascii = Ascii::from_bytes(bytes).unwrap(); 
    let string = String::from(ascii);
    assert_eq!(string, "ASCII and ye shall receive");
}

#[test]
fn bad_ascii(){
    use my_ascii::Ascii;
    let bytes = vec![0xf7, 0xbf, 0xbf, 0xbf];
    let ascii = unsafe{
        Ascii::from_bytes_unchecked(bytes)
    };
    let bogus : String = ascii.into();
    assert_eq!(bogus, "\u{FFFD}\u{FFFD}\u{FFFD}\u{FFFD}");
}
