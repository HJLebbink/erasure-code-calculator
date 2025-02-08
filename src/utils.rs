
const BYTE: u128 = 1;
const KIBIBYTE: u128 = 1024 * BYTE;
const MEBIBYTE: u128 = 1024 * KIBIBYTE;
const GIBIBYTE: u128 = 1024 * MEBIBYTE;
const TEBIBYTE: u128 = 1024 * GIBIBYTE;
const PEBIBYTE: u128 = 1024 * TEBIBYTE;
const EXBIBYTE: u128 = 1024 * PEBIBYTE;
const ZEBIBYTE: u128 = 1024 * EXBIBYTE;

#[allow(dead_code)]
#[derive(Debug)]
pub enum StorageUnit {
    Bytes,
    KiB,
    MiB,
    GiB,
    TiB,
    PiB,
    EiB,
    ZiB
    // you want Yotta bytes???
}

impl StorageUnit {
    pub(crate) fn to_bytes(&self) -> u128 {
        match self {
            StorageUnit::Bytes => BYTE,
            StorageUnit::KiB => KIBIBYTE,
            StorageUnit::MiB => MEBIBYTE,
            StorageUnit::GiB => GIBIBYTE,
            StorageUnit::TiB => TEBIBYTE,
            StorageUnit::PiB => PEBIBYTE,
            StorageUnit::EiB => EXBIBYTE,
            StorageUnit::ZiB => ZEBIBYTE,
        }
    }
}

/// Returns a formatted string for bytes, automatically choosing the best unit
pub fn format_bytes(bytes: u128) -> String {
    if bytes >= ZEBIBYTE {
        format!("{:.2} ZiB", bytes as f64 / ZEBIBYTE as f64)
    } else if bytes >= EXBIBYTE {
        format!("{:.2} EiB", bytes as f64 / EXBIBYTE as f64)
    } else if bytes >= PEBIBYTE {
        format!("{:.2} PiB", bytes as f64 / PEBIBYTE as f64)
    } else if bytes >= TEBIBYTE {
        format!("{:.2} TiB", bytes as f64 / TEBIBYTE as f64)
    } else if bytes >= GIBIBYTE {
        format!("{:.2} GiB", bytes as f64 / GIBIBYTE as f64)
    } else if bytes >= MEBIBYTE {
        format!("{:.2} MiB", bytes as f64 / MEBIBYTE as f64)
    } else if bytes >= KIBIBYTE {
        format!("{:.2} KiB", bytes as f64 / KIBIBYTE as f64)
    } else {
        format!("{} B", bytes)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::format_bytes;

    #[test]
    fn test_format_bytes() {
        assert_eq!(format_bytes(1024), "1.00 KiB");
        assert_eq!(format_bytes(1024 * 1024), "1.00 MiB");
        assert_eq!(format_bytes(1024 * 1024 * 1024), "1.00 GiB");
        assert_eq!(format_bytes(1024 * 1024 * 1024 * 1024), "1.00 TiB");
        assert_eq!(format_bytes(1024 * 1024 * 1024 * 1024 * 1024), "1.00 PiB");
        assert_eq!(format_bytes(1024 * 1024 * 1024 * 1024 * 1024 * 1024), "1.00 EiB");
        assert_eq!(format_bytes(1024 * 1024 * 1024 * 1024 * 1024 * 1024 * 1024), "1.00 ZiB");
    }
}
