pub fn search_files(
    locations: &[String],
    pattern: lazy_regex::regex::Regex,
) -> anyhow::Result<Vec<String>> {
    // Validate and process the first location.
    let location = &locations[0];
    let more_locations = &locations[1..locations.len()];
    let more_locations: Vec<String> = more_locations.to_vec();

    // Search for files matching the given pattern in the specified locations.
    let files: Vec<String> = rust_search::SearchBuilder::default()
        .location(location)
        .more_locations(more_locations)
        .build()
        .filter(|entry| {
            std::fs::metadata(entry)
                .is_ok_and(|metadata| metadata.is_file() && pattern.is_match(entry))
        })
        .collect();

    Ok(files)
}
