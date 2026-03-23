use crate::DeviceKind;
use hidparser::{Report, ReportDescriptor, ReportField, VariableField};

pub struct FilteredDescriptor {
    /// The (optional) report id associated with the report.
    pub report_id: Option<ReportId>,
    /// The size in bits of the report.
    pub size_in_bits: usize,
    /// The list of fields in the report.
    pub fields: Vec<VariableField>,
}

impl FilteredDescriptor {
    pub fn from_descriptor(
        report_descriptor: ReportDescriptor,
        device_kind: DeviceKind,
    ) -> Vec<Self> {
        let device_usage = device_kind.to_hid_usage();
        report_descriptor
            .input_reports
            .into_iter()
            .filter_map(
                |Report {
                     report_id,
                     size_in_bits,
                     fields,
                 }| {
                    // Filter to fields that are related to the devicekind
                    let fields = fields
                        .into_iter()
                        .filter_map(|field| match field {
                            ReportField::Variable(field) => field
                                .member_of
                                .iter()
                                .any(|collection| {
                                    device_usage == (collection.usage.page(), collection.usage.id())
                                })
                                .then_some(field),
                            // TODO: Implement
                            _ => None,
                        })
                        .collect();
                    if fields.is_empty() {
                        None
                    } else {
                        Some(FilteredDescriptor {
                            report_id,
                            size_in_bits,
                            fields,
                        })
                    }
                },
            )
            .collect()
    }
}
