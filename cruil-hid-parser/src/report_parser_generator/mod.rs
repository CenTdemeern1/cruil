use crate::descriptor_parser::{InputField, Report, ReportDescriptor, Usage, UsageSet};
use nom::{
    IResult, Parser,
    bits::{bits, complete::take},
    branch::alt,
    bytes::tag,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ParsedReportField<'u> {
    usages: &'u UsageSet,
    range: (i32, i32),
    values: Vec<i32>,
}

fn generate_parser(descriptor: &ReportDescriptor) -> impl Parser<&[u8]> {
    let parsers: Vec<_> = descriptor
        .reports
        .iter()
        .map(generate_report_parser)
        .collect();
    alt(parsers)
}

fn generate_report_parser(
    report: &Report,
) -> impl Parser<&[u8], Output = (&[u8], Vec<ParsedReportField>)> {
    let mut parsers: Vec<_> = report
        .fields
        .iter()
        .map(generate_report_field_parser)
        .collect();
    let combined_parsers = move |mut input| -> IResult<(&[u8], usize), Vec<ParsedReportField>> {
        let mut vec: Vec<ParsedReportField> = Vec::with_capacity(parsers.len());
        let mut output;
        for parser in &mut parsers {
            unsafe { (input, output) = parser.parse_complete(input).unwrap_unchecked() };
            vec.push(output);
        }
        Ok((input, vec))
    };
    (
        tag::<_, _, nom::error::Error<_>>(std::slice::from_ref(&report.report_id)),
        bits(combined_parsers),
    )
}

fn generate_report_field_parser(
    field: &InputField,
) -> impl Parser<(&[u8], usize), Output = ParsedReportField> {
    move |mut input| {
        let parsed_field = ParsedReportField {
            usages: &field.usages,
            range: field.logical_range,
            values: vec![],
        };
        if field.usages.is_padding() {
            return IResult::Ok((input, parsed_field));
        }
        let mut vec = Vec::with_capacity(field.report_count as usize);
        let mut output;
        for _ in 0..field.report_count {
            (input, output) = take::<_, _, _, nom::error::Error<_>>(field.report_size)(input)?;
            vec.push(output);
        }
        IResult::Ok((
            input,
            ParsedReportField {
                values: vec,
                ..parsed_field
            },
        ))
    }
}
