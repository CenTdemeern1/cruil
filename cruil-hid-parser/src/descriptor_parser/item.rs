use crate::bytes::take1;
use ItemType::*;
use nom::{IResult, Parser, bytes::complete::take};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum ItemType {
    Main = 0,
    Global = 1,
    Local = 2,
    Reserved = 3,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Item<'d> {
    pub b_size: u8,
    pub b_type: ItemType,
    pub b_tag: u8,
    /// Whether this is a long item. [`b_type`](Self::b_type) is always [`Reserved`]
    pub long: bool,
    pub data: &'d [u8],
}

impl<'d> Item<'d> {
    pub fn parse(bytes: &'d [u8]) -> IResult<&'d [u8], Self> {
        let (bytes, header) = take1().parse_complete(bytes)?;
        let mut b_size = match header & 0b11 {
            n @ 0..3 => n,
            3 => 4,
            _ => unreachable!(),
        };
        let b_type = match (header >> 2) & 0b11 {
            0 => Main,
            1 => Global,
            2 => Local,
            3 => Reserved,
            _ => unreachable!(),
        };
        let mut b_tag = header >> 4;

        let (mut bytes, mut data) = take(b_size)(bytes)?;

        let long = if b_size == 2 && b_type == Reserved && b_tag == 0b1111 {
            // This is a long item!
            // UNWRAP: Infallible because we know b_size == 2 and take succeeded
            [b_size, b_tag] = data.try_into().unwrap();

            (bytes, data) = take(b_size)(bytes)?;

            true
        } else {
            false
        };

        Ok((
            bytes,
            Item {
                b_size,
                b_type,
                b_tag,
                long,
                data,
            },
        ))
    }
}
