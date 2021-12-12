use core::ptr;

pub trait AsRawPtr {
    type Pointee;
    fn as_raw_ptr(&self) -> *const Self::Pointee;
}

pub trait AsRawMutPtr {
    type Pointee;
    fn as_raw_mut_ptr(&mut self) -> *mut Self::Pointee;
}

impl<'a, T> AsRawPtr for Option<&'a T> {
    type Pointee = T;

    #[inline(always)]
    fn as_raw_ptr(&self) -> *const Self::Pointee {
        unsafe { ptr::read::<*const T>(self as *const _ as *const _) }
    }
}

impl<'a, T> AsRawPtr for Option<&'a mut T> {
    type Pointee = T;

    #[inline(always)]
    fn as_raw_ptr(&self) -> *const Self::Pointee {
        unsafe { ptr::read::<*const T>(self as *const _ as *const _) }
    }
}

impl<'a, T> AsRawMutPtr for Option<&'a mut T> {
    type Pointee = T;

    #[inline(always)]
    fn as_raw_mut_ptr(&mut self) -> *mut Self::Pointee {
        unsafe { ptr::read::<*mut T>(self as *const _ as *const _) }
    }
}

#[cfg(feature = "alloc")]
impl<T> AsRawPtr for Option<alloc::boxed::Box<T>> {
    type Pointee = T;

    #[inline(always)]
    fn as_raw_ptr(&self) -> *const Self::Pointee {
        unsafe { ptr::read::<*const T>(self as *const _ as *const _) }
    }
}

#[cfg(feature = "alloc")]
impl<T> AsRawMutPtr for Option<alloc::boxed::Box<T>> {
    type Pointee = T;

    #[inline(always)]
    fn as_raw_mut_ptr(&mut self) -> *mut Self::Pointee {
        unsafe { ptr::read::<*mut T>(self as *const _ as *const _) }
    }
}

impl<T> AsRawPtr for Option<ptr::NonNull<T>> {
    type Pointee = T;

    #[inline(always)]
    fn as_raw_ptr(&self) -> *const Self::Pointee {
        unsafe { ptr::read::<*const T>(self as *const _ as *const _) }
    }
}

impl<T> AsRawMutPtr for Option<ptr::NonNull<T>> {
    type Pointee = T;

    #[inline(always)]
    fn as_raw_mut_ptr(&mut self) -> *mut Self::Pointee {
        unsafe { ptr::read::<*mut T>(self as *const _ as *const _) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ref() {
        let num = 1usize;
        let opt = Some(&num);

        let ptr = opt.as_raw_ptr();
        unsafe {
            assert_eq!(*ptr, 1);
        }
    }

    #[test]
    fn test_ref_none() {
        let num = Option::<&usize>::None;

        let ptr = num.as_raw_ptr();

        assert!(ptr.is_null());
    }

    #[test]
    fn test_mut_ref() {
        let mut num = 1usize;
        let mut opt = Some(&mut num);

        let ptr = opt.as_raw_mut_ptr();
        unsafe {
            *ptr = 2;
        }

        assert_eq!(num, 2);
    }

    #[test]
    fn test_mut_ref_none() {
        let mut num = Option::<&mut usize>::None;

        let ptr = num.as_raw_mut_ptr();

        assert!(ptr.is_null());
    }

    #[test]
    fn test_box() {
        let mut num = Some(alloc::boxed::Box::new(1usize));

        let ptr = num.as_raw_mut_ptr();
        unsafe {
            *ptr = 2;
        }

        assert!(matches!(num, Some(b) if *b == 2));
    }

    #[test]
    fn test_box_none() {
        let mut num = Option::<alloc::boxed::Box<usize>>::None;

        let ptr = num.as_raw_mut_ptr();

        assert!(ptr.is_null());
    }
}
