use std::time::Duration;

use anyhow::anyhow;

pub(crate) const SANE_FRAME_LENGTH: i32 = 5 * 1024 * 1024;
pub(crate) const SANITY_DURATION: Duration = Duration::from_micros(10);

pub(crate) fn checked_payload_buffer_size(total_size: i64) -> Result<usize, anyhow::Error> {
    if total_size < 0 || total_size > i64::from(SANE_FRAME_LENGTH) {
        return Err(anyhow!(
            "Invalid byte payload size: {total_size}; expected 0..={SANE_FRAME_LENGTH}"
        ));
    }

    usize::try_from(total_size).map_err(|_| anyhow!("Payload size cannot fit in memory index"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sane_frame_length_is_exactly_five_mib() {
        assert_eq!(SANE_FRAME_LENGTH, 5_242_880);
    }

    #[test]
    fn payload_buffer_size_accepts_only_sane_non_negative_values() {
        assert_eq!(checked_payload_buffer_size(0).unwrap(), 0);
        assert_eq!(
            checked_payload_buffer_size(i64::from(SANE_FRAME_LENGTH)).unwrap(),
            SANE_FRAME_LENGTH as usize
        );
        assert!(checked_payload_buffer_size(-1).is_err());
        assert!(checked_payload_buffer_size(i64::from(SANE_FRAME_LENGTH) + 1).is_err());
    }
}
