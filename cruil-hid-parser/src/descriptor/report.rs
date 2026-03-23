use crate::descriptor::{Item, ItemType, Usage, UsageSet};
use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
struct GlobalStateTable {
    usage_page: u16,
    logical_range: (i32, i32),
    report_size: u32,
    report_id: u8,
    report_count: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum DelimiterState {
    #[default]
    Closed,
    Open,
    ForceIgnore,
}

#[derive(Debug, Default)]
struct LocalStateTable {
    usages: UsageSet,
    delimiter_state: DelimiterState,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Hash)]
pub struct InputField {
    usages: UsageSet,
    logical_range: (i32, i32),
    report_size: u32,
    report_count: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Hash)]
pub struct Report {
    report_id: u8,
    fields: Vec<InputField>,
}

impl Report {
    pub(crate) fn parse(items: &[Item]) -> Vec<Report> {
        let mut reports: HashMap<u8, Report> = HashMap::new();
        let mut global_state_stack = vec![];
        let mut global_state = GlobalStateTable::default();
        let mut local_state = LocalStateTable::default();

        for item in items {
            // Spec does not define any long items
            if item.long {
                continue;
            }

            match item.b_type {
                ItemType::Main => {
                    match item.b_tag {
                        0b1000 => {
                            // Input
                            let report = reports.entry(global_state.report_id).or_insert_with_key(
                                |&report_id| Report {
                                    report_id,
                                    ..Default::default()
                                },
                            );
                            report.fields.push(InputField {
                                usages: local_state.usages,
                                logical_range: global_state.logical_range,
                                report_size: global_state.report_size,
                                report_count: global_state.report_count,
                            });
                        }
                        _ => {}
                    }
                    local_state = LocalStateTable::default();
                }
                ItemType::Global => match item.b_tag {
                    0b0000 => {
                        // Usage Page
                        global_state.usage_page = Self::number_from_data(&item) as u16;
                    }
                    0b0001 => {
                        // Logical Minimum
                        global_state.logical_range.0 = Self::signed_number_from_data(&item);
                    }
                    0b0010 => {
                        // Logical Maximum
                        global_state.logical_range.1 = Self::signed_number_from_data(&item);
                    }
                    0b0111 => {
                        // Report Size
                        global_state.report_size = Self::number_from_data(&item);
                    }
                    0b1000 => {
                        // Report ID
                        global_state.report_id = Self::number_from_data(&item) as u8;
                    }
                    0b1001 => {
                        // Report Count
                        global_state.report_count = Self::number_from_data(&item);
                    }
                    0b1010 => {
                        // Push
                        global_state_stack.push(global_state.clone());
                    }
                    0b1011 => {
                        // Pop
                        if let Some(state) = global_state_stack.pop() {
                            global_state = state;
                        }
                    }
                    _ => {}
                },
                ItemType::Local => {
                    let usage = match item.b_size {
                        0..3 => Usage::new(
                            global_state.usage_page,
                            Self::number_from_data(&item) as u16,
                        ),
                        3 => u32::from_le_bytes(item.data.try_into().unwrap()).into(),
                        _ => unreachable!(),
                    };

                    // Watch out for control flow hazards. This uses `continue` and match arm precedence
                    match item.b_tag {
                        0b1010 => {
                            // Delimiter
                            // Implemented by interpreting everything after the first item as padding.
                            local_state.delimiter_state = if Self::number_from_data(&item) != 0 {
                                DelimiterState::Open
                            } else {
                                DelimiterState::Closed
                            };
                            continue;
                        }
                        _ if local_state.delimiter_state == DelimiterState::ForceIgnore => continue,
                        0b0000 => {
                            // Usage
                            if let UsageSet::List(list) = &mut local_state.usages {
                                list.push(usage);
                            } else {
                                // What?
                                // If this actually ever happens it might be worth having multiple sets instead of a List variant.
                                // .nth() would need to be changed for that
                                // I don't know how it would handle [Input, Usage, Usage Maximum, Usage, Input] though
                                local_state.usages = UsageSet::List(vec![usage]);
                            }
                            // Fallthrough to forceignore state switch
                        }
                        0b0001 => {
                            // Usage Minimum
                            let usage = usage.into();
                            if let UsageSet::Range(min, _) = &mut local_state.usages {
                                *min = usage;
                                // Fallthrough to forceignore state switch
                            } else {
                                local_state.usages = UsageSet::Range(usage, None);
                                continue;
                            }
                        }
                        0b0010 => {
                            // Usage Maximum
                            let usage = Some(usage.into());
                            if let UsageSet::Range(_, max) = &mut local_state.usages {
                                *max = usage;
                                // Fallthrough to forceignore state switch
                            } else {
                                // Uh... pray that it gets assigned later?
                                local_state.usages = UsageSet::Range(0, usage);
                                continue;
                            }
                        }
                        _ => {}
                    }
                    // The forceignore state switch in question
                    if local_state.delimiter_state == DelimiterState::Open {
                        local_state.delimiter_state = DelimiterState::ForceIgnore;
                    }
                }
                ItemType::Reserved => {}
            }
        }
        reports.into_values().collect()
    }

    fn number_from_data(item: &Item) -> u32 {
        match item.b_size {
            0 => 0,
            1 => item.data[0] as u32,
            2 => u16::from_le_bytes(item.data.try_into().unwrap()) as u32,
            3 => u32::from_le_bytes(item.data.try_into().unwrap()),
            _ => unreachable!(),
        }
    }

    fn signed_number_from_data(item: &Item) -> i32 {
        match item.b_size {
            0 => 0,
            1 => item.data[0].cast_signed() as i32,
            2 => i16::from_le_bytes(item.data.try_into().unwrap()) as i32,
            3 => i32::from_le_bytes(item.data.try_into().unwrap()),
            _ => unreachable!(),
        }
    }
}
