use std::fmt;

// Reset
derive_csi_sequence!("Reset", Reset, "0m");

// Löschen
derive_csi_sequence!("Alles löschen", DeleteAll, "2J");
derive_csi_sequence!("Zeile löschen", DeleteLine, "0K");

derive_csi_sequence!("Save current Cursor Position ", Save, "s");
derive_csi_sequence!("Restore saved Cursor Position ", Restore, "r");
