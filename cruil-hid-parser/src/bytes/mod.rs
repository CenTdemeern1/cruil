use nom::{IResult, Input, Parser, bytes::take, combinator::map, error::ParseError};

// pub fn take1(bytes: &[u8]) -> IResult<&[u8], u8> {
//     take(1usize)(bytes).map(|(r, o)| (r, o[0]))
// }

pub fn take1<I, Error>() -> impl Parser<I, Output = I::Item, Error = Error>
where
    I: Input,
    Error: ParseError<I>,
{
    // UNWRAP: `take()` returns `Err` if `o` is the wrong size so this is infallible
    map(take(1usize), |o: I| o.iter_elements().next().unwrap())
}

pub fn take_const<const N: usize>(bytes: &[u8]) -> IResult<&[u8], [u8; N]> {
    // UNWRAP: `take()` returns `Err` if `o` is the wrong size so this is infallible
    take(N)
        .parse_complete(bytes)
        .map(|(r, o)| (r, o.try_into().unwrap()))
}
