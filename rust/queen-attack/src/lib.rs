pub struct Queen(i32, i32);

impl Queen {
    pub fn new((x, y): (i32, i32)) -> Result<Queen, &'static str> {
        if x >= 0 && y >= 0 && x < 8 && y < 8 {
            Ok(Queen(x, y))
        } else {
            Err("out of bounds")
        }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        self.0 == other.0 || self.1 == other.1 ||
        (self.0 - other.0).abs() == (self.1 - other.1).abs()
    }
}
