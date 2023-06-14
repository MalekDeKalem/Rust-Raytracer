pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32
}

impl Color {
    pub fn new(red: f32, green: f32, blue: f32) -> Self {
        let new_col = Self {
            r: red,
            g: green,
            b: blue
        };
        new_col
    }
}
