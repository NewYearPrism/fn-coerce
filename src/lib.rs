#![no_std]

pub use stringz::string as lit_t;

pub struct FnCo<const N: usize, const UNSAFE: bool, Abi> {
    __p_abi: core::marker::PhantomData<Abi>,
}

trait Sealed {}

#[allow(private_bounds)]
pub trait Co<F>: Sealed {
    type FnPtr;

    fn co(i: Self::FnPtr) -> Self::FnPtr;
}

macro_rules! impl_extern {
    ($cc:literal, $n:expr,) => {
        impl_extern!(@impl_one $cc, $n,);
    };
    ($cc:literal, $n:expr, $first:ident, $($rest:ident,)*) => {
        impl_extern!(@impl_one $cc, $n, $first, $($rest,)*);
        impl_extern!($cc, $n-1, $($rest,)*);
    };
    (@impl_one $cc:literal, $n:expr, $($all:ident,)*) => {
        impl Sealed for FnCo<{$n}, {true}, stringz::string!($cc)> {}
        impl<R, $($all,)*> Co<unsafe extern $cc fn($($all,)*) -> R> for FnCo<{$n}, {true}, stringz::string!($cc)> {
            type FnPtr = unsafe extern $cc fn($($all,)*) -> R;
            fn co(i: unsafe extern $cc fn($($all,)*) -> R) -> unsafe extern $cc fn($($all,)*) -> R { i }
        }
        #[cfg(feature = "safe")]
        impl Sealed for FnCo<{$n}, {false}, stringz::string!($cc)> {}
        #[cfg(feature = "safe")]
        impl<R, $($all,)*> Co<extern $cc fn($($all,)*) -> R> for FnCo<{$n}, {false}, stringz::string!($cc)> {
            type FnPtr = extern $cc fn($($all,)*) -> R;
            fn co(i: extern $cc fn($($all,)*) -> R) -> extern $cc fn($($all,)*) -> R { i }
        }
    };
}

macro_rules! xxx_conventions {
    ($m:ident) => {
        //$m!("Rust");
        #[cfg(feature = "system-abi")]
        $m!("system");
        #[cfg(feature = "C-abi")]
        $m!("C");

        #[cfg(all(feature = "stdcall-abi", target_arch = "x86"))]
        $m!("stdcall");
        #[cfg(all(feature = "cdecl-abi", target_arch = "x86"))]
        $m!("cdecl");
        #[cfg(feature = "win64-abi")]
        $m!("win64");
        // $m!("sysv64");
        // $m!("aapcs");
        // $m!("fastcall");
        // $m!("thiscall");

        // $m!("efiapi");

        #[cfg(feature = "system-unwind-abi")]
        $m!("system-unwind");
        #[cfg(feature = "C-unwind-abi")]
        $m!("C-unwind");
        #[cfg(all(feature = "stdcall-unwind-abi", target_arch = "x86"))]
        $m!("stdcall-unwind");
        #[cfg(all(feature = "cdecl-unwind-abi", target_arch = "x86"))]
        $m!("cdecl-unwind");
        #[cfg(feature = "win64-unwind-abi")]
        $m!("win64-unwind");
        // $m!("sysv64-unwind");
        // $m!("aapcs-unwind");
        // $m!("fastcall-unwind");
        // $m!("thiscall-unwind");
    };
}

macro_rules! impl_22 {
    ($cc:literal) => {
        impl_extern!(
            $cc, 22, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17,
            T18, T19, T20, T21, T22,
        );
    };
}

xxx_conventions!(impl_22);

#[cfg(test)]
mod tests {
    use crate::{
        Co,
        FnCo,
        lit_t,
    };

    #[test]
    fn test() {
        unsafe extern "system" fn aaa() {}
        let _a = FnCo::<0, true, lit_t!("system")>::co(aaa);
    }
}
