#![allow(dead_code)]

mod ref_with_flag{
    use std::marker::PhantomData; //用于标记类型所有权/生命期的零大小类型
    use std::mem::align_of;

    pub struct RefWithFlag<'a, T>{
        ptr_and_bit: usize,
        behaves_like : PhantomData<&'a T>
    }

    impl<'a,T:'a> RefWithFlag<'a, T>{
        pub fn new(ptr: &'a T, flag: bool) -> RefWithFlag<T> {
            assert!(align_of::<T>() % 2 == 0);
            RefWithFlag {
                ptr_and_bit: ptr as *const T as usize | flag as usize,
                behaves_like: PhantomData
            }
        }
        pub fn get_ref(&self) -> &'a T {
            unsafe {
                let ptr = (self.ptr_and_bit & !1) as *const T;
                &*ptr
            }
        }
        pub fn get_flag(&self) ->bool{
            self.ptr_and_bit & 1 != 0
        }
    }
}

#[cfg(test)]  //被 #[cfg(test)] 标记的代码不会出现在 release 编译结果中。
mod ref_with_flag_tests {
    use super::ref_with_flag;

    #[test]
    fn use_ref_with_flag() {
        use ref_with_flag::RefWithFlag;

        let vec = vec![10, 20, 30];
        let flagged = RefWithFlag::new(&vec, true);
        assert_eq!(flagged.get_ref()[1], 20);
        assert_eq!(flagged.get_flag(), true);
    }
}


