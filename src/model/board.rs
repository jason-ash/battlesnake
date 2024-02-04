use super::Position;

/// board struct
///
/// |   0 |   1 |   2 | ... |  10 |
/// |  11 |  12 |  13 | ... |  21 |
/// |  22 |  23 |  24 | ... |  32 |
/// ...
/// | 110 | 111 | 112 | ... | 120 |
pub struct Board {
    cells: [usize; 121],
}
