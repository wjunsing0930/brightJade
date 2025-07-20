#![allow(dead_code)]

mod gap{
    use std::ops::Range;
    pub struct GapBuffer<T>{
        storage : Vec<T>, gap : Range<usize>
    }

    impl<T> GapBuffer<T>{
        pub fn new() -> GapBuffer<T>{
            GapBuffer{ storage: Vec::new(), gap : 0..0}
        }
        pub fn capacity(&self) -> usize{
            self.storage.capacity()
        }
        pub fn len(&self) ->usize{
            self.capacity() - self.gap.len()
        }
        pub fn position(&self) -> usize{
            self.gap.start
        }
        unsafe fn space(&self, index:usize)->*const T{
            self.storage.as_ptr().offset(index as isize)
        }
        unsafe fn space_mut(&mut self, index: usize) -> *mut T{
            self.storage.as_mut_ptr().offset(index as isize)
        }
        fn index_to_raw(&self, index:usize) ->usize{
            if index < self.gap.start{
                index
            } else {
                index + self.gap.len()
            }
        }
        pub fn get(&self, index:usize)->Option<&T>{
            let raw = self.index_to_raw(index);
            if raw < self.capacity(){
                unsafe{
                    Some(&*self.space(raw))
                }
            }else{
                None
            }
        }
        pub fn set_position(&mut self, pos:usize){
            if pos > self.len(){
                panic!("index {} out of range for GapBuffer", pos);
            }
            unsafe{
                let gap = self.gap.clone();
                if pos > gap.start{
                    let distance = pos - gap.start;
                    std::ptr::copy(self.space(gap.end),
                        self.space_mut(gap.start), distance );
                } else if pos < gap.start{
                    let distance = gap.start - pos;
                    std::ptr::copy(self.space(pos), self.space_mut(gap.end-distance),distance);
                }
                self.gap = pos .. pos + gap.len();
            }
        }


    }
}