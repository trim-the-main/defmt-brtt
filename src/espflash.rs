#[cfg(not(any(
    feature = "esp32",
    feature = "esp32c2",
    feature = "esp32c3",
    feature = "esp32c6",
    feature = "esp32h2",
    feature = "esp32s2",
    feature = "esp32s3",
)))]
compile_error!(
    "The `espflash` feature requires a chip feature (e.g., `esp32`, `esp32c3`)."
);

/// Write the espflash framing header that marks the start of a defmt frame.
/// Must be called once per frame, before `start_frame`.
pub(crate) fn write_frame_header() {
    esp_println::Printer::write_bytes(&[0xFF, 0x00]);
}

/// Write encoded defmt bytes through esp-println.
pub(crate) fn do_write(bytes: &[u8]) {
    esp_println::Printer::write_bytes(bytes);
}
