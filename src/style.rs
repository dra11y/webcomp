use indicatif::ProgressStyle;
use std::sync::LazyLock;

// Defines progress bar and spinner styles for user feedback.

pub static STYLE_BAR: LazyLock<ProgressStyle> = LazyLock::new(|| {
    ProgressStyle::with_template("{wide_bar:.cyan/blue} {human_pos}/{human_len}").unwrap()
});

pub static STYLE_SPINNER: LazyLock<ProgressStyle> =
    LazyLock::new(|| ProgressStyle::with_template("{spinner:.blue} {msg}").unwrap());
