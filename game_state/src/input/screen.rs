#[derive(Debug, Clone)]
pub struct ScreenPoint {
    pub x: i32,
    pub y: i32,
}

impl ScreenPoint {
    pub fn new(x: i32, y: i32) -> Self {
        ScreenPoint { x, y }
    }
}

#[derive(Debug, Clone)]
pub struct ScreenRect {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}

impl ScreenRect {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Self {
        ScreenRect { x, y, w, h }
    }
    pub fn intersects(&self, point: &ScreenPoint) -> bool {
        point.x >= self.x
            && point.y >= self.y
            && point.x < self.x + self.w
            && point.y < self.y + self.h
    }
}

#[derive(Debug, Clone)]
pub struct DeltaVector {
    pub delta_x: i32,
    pub delta_y: i32,
}

impl DeltaVector {
    pub fn new(dx: i32, dy: i32) -> Self {
        DeltaVector {
            delta_x: dx,
            delta_y: dy,
        }
    }
    pub fn from_points(old: &ScreenPoint, new: &ScreenPoint) -> Self {
        Self::new(new.x - old.x, new.y - old.y)
    }
}
