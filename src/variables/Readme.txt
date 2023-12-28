1) Primitive Types
never	    The ! type, also called “never”.
array	    A fixed-size array, denoted [T; N], for the element type, T, and the non-negative compile-time constant size, N.
bool	    The boolean type.
char	    A character type.
f32	        A 32-bit floating point type (specifically, the “binary32” type defined in IEEE 754-2008).
f64	        A 64-bit floating point type (specifically, the “binary64” type defined in IEEE 754-2008).
fn	        Function pointers, like fn(usize) -> bool.
i8	        The 8-bit signed integer type.
i16	        The 16-bit signed integer type.
i32	        The 32-bit signed integer type.
i64	        The 64-bit signed integer type.
i128	    The 128-bit signed integer type.
isize	    The pointer-sized signed integer type.
pointer	    Raw, unsafe pointers, *const T, and *mut T.
reference	References, &T and &mut T.
slice	    A dynamically-sized view into a contiguous sequence, [T]. Contiguous here means that elements are laid out so that every element is the same distance from its neighbors.
str	        String slices.
tuple	    A finite heterogeneous sequence, (T, U, ..).
u8	        The 8-bit unsigned integer type.
u16         The 16-bit unsigned integer type.
u32         The 32-bit unsigned integer type.
u64         The 64-bit unsigned integer type.
u128	    The 128-bit unsigned integer type.
unit	    The () type, also called “unit”.
usize	    The pointer-sized unsigned integer type.

2) grammer
ex1) let num: u8 = 5;   // 8bit 정수 type (u8) / 변수명 num / 값 5
ex2) let isRust: bool = true;   //  bool 타입 (bool) / 변수명 isRust / 값 true
