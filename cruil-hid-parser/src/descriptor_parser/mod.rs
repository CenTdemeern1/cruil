use nom::{Err, Parser, error::Error, multi::many1};

mod item;
use item::*;

mod report;
pub use report::*;

mod usage;
pub use usage::*;

#[derive(Debug, Clone, PartialEq, Eq, Default, Hash)]
pub struct ReportDescriptor {
    pub reports: Vec<Report>,
}

impl ReportDescriptor {
    /// Parses a report descriptor from some data.
    /// This data will be cloned into a pinned box in the resulting struct.
    pub fn parse(data: &[u8]) -> Result<ReportDescriptor, Err<Error<&[u8]>>> {
        let (_, items) = many1(Item::parse).parse_complete(data)?;
        let reports = Report::parse(&items);
        Ok(ReportDescriptor { reports })
    }
}
