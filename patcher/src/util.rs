use {log::error, macros::fail};

/// Align JSON Key-Values for readability
/// Can't find a decent library for this, so we're doing it manually
pub(crate) fn align_json_values(json: &mut String) {
    const KEY_ALIGNMENT: usize = 56;
    let mut index_colon = 0;
    while index_colon < json.len() {
        let index_colon_opt = json[index_colon..].find(":");
        if index_colon_opt.is_none() {
            break;
        }
        index_colon += index_colon_opt.unwrap();
        if ['{', '['].contains(&json[index_colon..].chars().nth(2).unwrap()) {
            index_colon += 1;
            continue;
        }

        let index_prev_new_line = json[..index_colon].rfind("\n").unwrap_or_else(|| {
            fail!("Couldn't fine new line character before index: {}", index_colon);
        });
        let line_length_up_to_value = index_colon - index_prev_new_line;

        if KEY_ALIGNMENT < line_length_up_to_value {
            error!("Failed to write Spoiler Log");
            error!(
                "JSON Key Alignment value smaller than line length up to that point: {} < {}",
                KEY_ALIGNMENT, line_length_up_to_value
            );
            fail!("Problem line: {}", &json[index_prev_new_line..index_colon]);
        }

        let spaces_to_add = KEY_ALIGNMENT - line_length_up_to_value;

        json.insert_str(
            index_colon + 1,
            (0..spaces_to_add).map(|_| " ").collect::<String>().as_str(),
        );
        index_colon += 1;
    }
}
