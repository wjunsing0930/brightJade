macro_rules! define_complex{
    () => {
        #[derive(Clone, Copy, Debug)]
        struct Complex<T>{
            re : T, im : T
        }
    }
}

mod first_cut{
    #[derive(Clone, Copy, Debug)]
    struct Complex<T>{
        re : T, im : T,
    }

    use std::ops::Add;
    impl<T> Add for Complex<T>
        where T : Add<Output = T>
    {
        type Output = Self;
        fn add(self, rhs:Self) -> Self{
            Complex{
                re : self.re + rhs.re,
                im : self.im + rhs.im,
            }
        }
    }

    use std::ops::Sub;
    impl<T> Sub for Complex<T>
        where T : Sub<Output = T>
    {
        type Output = Self;
        fn sub(self, rhs : Self) ->Self{
            Complex{
                re : self.re - rhs.re,
                im : self.im - rhs.im,
            }
        }
    }
    use std::ops::Mul;

    impl<T> Mul for Complex<T>
    where
        T: Clone + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
    {
        type Output = Self;
        fn mul(self, rhs: Self) -> Self {
            Complex {
                re: self.re.clone() * rhs.re.clone()
                    - (self.im.clone() * rhs.im.clone()),
                im: self.im * rhs.re + self.re * rhs.im,
            }
        }
    }  

    #[test]
    fn try_it_out() {
        let mut z = Complex { re: 1, im: 2 };
        let c = Complex { re: 3, im: 4 };
        z = z * z + c;
        let _ = std::mem::forget(z);//防止一个值在离开作用域时自动调用其析构函数
    }  
    impl<T: PartialEq> PartialEq for Complex<T> {
        fn eq(&self, other: &Complex<T>) -> bool {
            self.re == other.re && self.im == other.im
        }
    }
    #[test]
    fn test_complex_eq() {
        let x = Complex { re: 5, im: 2 };
        let y = Complex { re: 2, im: 5 };
        assert_eq!(x * y, Complex { re: 0, im: 29 });
    }

    impl<T: Eq> Eq for Complex<T> {}
}

mod non_generic_add{
    define_complex!();
    use std::ops::Add;
    impl Add for Complex<i32>{
        type Output = Complex<i32>;
        fn add(self, rhs:Self) ->Self{
            Complex{
                re : self.re + rhs.re,
                im : self.im + rhs.im,
            }
        }
    }

    #[test]
    fn test_add_method(){
        let mut z = Complex { re: 1, im: 2 };
        let c = Complex { re: 3, im: 4 };
        z = z + c;
        assert_eq!(z.re, 4);
        assert_eq!(z.im, 6);
    }
}