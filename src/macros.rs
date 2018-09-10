
/// Create a CSI-introduced sequence.
macro_rules! csi {
    ($( $l:expr ),*) => { concat!("\x1B[", $( $l ),*) };
}

/// Derive a CSI sequence struct.
macro_rules! derive_csi_sequence {
    ($doc:expr, $name:ident, $value:expr) => {
        #[doc = $doc]
        #[derive(Copy, Clone)]
        pub struct $name;

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, csi!($value))
            }
        }

        impl AsRef<[u8]> for $name {
            fn as_ref(&self) -> &'static [u8] { csi!($value).as_bytes() }
        }

        impl AsRef<str> for $name {
            fn as_ref(&self) -> &'static str { csi!($value) }
        }

        impl $name {
            pub fn as_my_vec(&self) -> Vec<u8> {csi!($value).as_bytes().to_vec() }
        }
    };
}



// Farbe
// derive_csi_sequence!("Farbe Gelb", Blau, "48;5;136m");

// Reset
//derive_csi_sequence!("Reset",Reset, "0m");

// Löschen
//derive_csi_sequence!("Alles löschen",DeleteAll, "2J");
//derive_csi_sequence!("Zeile löschen",DeleteLine, "0K");

