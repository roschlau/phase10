use crate::phases::PhaseGoal::{RunOfX, XOfColor, XOfNumber};

pub enum PhaseGoal {
    XOfNumber(u8),
    XOfColor(u8),
    RunOfX(u8),
}

const PHASE_GOALS: [&[PhaseGoal]; 10] = [
    &[XOfNumber(3), XOfNumber(3)],
    &[XOfNumber(3), RunOfX(4)],
    &[XOfNumber(4), RunOfX(4)],
    &[RunOfX(7)],
    &[RunOfX(8)],
    &[RunOfX(9)],
    &[XOfNumber(4), XOfNumber(4)],
    &[XOfColor(7)],
    &[XOfNumber(5), XOfNumber(2)],
    &[XOfNumber(5), XOfNumber(3)],
];
