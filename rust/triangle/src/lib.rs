#[derive(Debug)]
pub struct Triangle {
    a: usize,
    b: usize,
    c: usize,
}

impl Triangle {
    pub fn build(sides: [usize; 3]) -> Result<Triangle, &'static str> {
        let (a, b ,c) = (sides[0], sides[1], sides[2]);
        if a + b < c || a + c < b || c + b < a || sides.iter().any(|&s| s == 0) {
            Err("not a triangle")
        } else {
            Ok(Triangle{a: a, b: b, c: c})
        }
    }

    pub fn is_equilateral(&self) -> bool {
        self.a == self.b && self.b == self.c
    }

    pub fn is_isosceles(&self) -> bool {
        self.a == self.b && self.b != self.c ||
        self.a == self.c && self.c != self.b ||
        self.b == self.c && self.c != self.a
    }

    pub fn is_scalene(&self) -> bool {
        self.a != self.b && self.a != self.c && self.b != self.c
    }

}