// Defines the FaultModel enum, representing the different fault models (stuck-at-0, stuck-at-1, transient bit-flip).
#[derive(Debug, Clone, Copy)]
pub enum FaultModel {
    StuckAt0,
    StuckAt1,
    TransientBitFlip,
}

impl std::fmt::Display for FaultModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FaultModel::StuckAt0 => write!(f, "Stuck-at-0"),
            FaultModel::StuckAt1 => write!(f, "Stuck-at-1"),
            FaultModel::TransientBitFlip => write!(f, "Transient bit-flip"),
        }
    }
}