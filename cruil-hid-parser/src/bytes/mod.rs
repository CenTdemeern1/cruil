use nom::{IResult, bytes::complete::take};

pub fn take1(bytes: &[u8]) -> IResult<&[u8], u8> {
    take(1usize)(bytes).map(|(r, o)| (r, o[0]))
}

pub fn take_const<const N: usize>(bytes: &[u8]) -> IResult<&[u8], [u8; N]> {
    // UNWRAP: `take()` returns `Err` if `o` is the wrong size so this is infallible
    take(N)(bytes).map(|(r, o)| (r, o.try_into().unwrap()))
}
