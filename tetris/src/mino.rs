use rand::Rng;

pub const MINO_WIDTH: usize = 4;
pub const MINO_HEIGHT: usize = 4;
const MINO_ANGLE_MAX: usize = 4;
const MINO_TYPE_MAX: usize = 7;

#[derive(Clone, Copy)]
pub struct Mino {
    pub typ: usize,
    pub angle: usize,
    pub x: usize,
    pub y: usize,
}

impl Mino {
    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        Self {
            typ: rng.gen_range(1, 101) % MINO_TYPE_MAX,
            angle: 0,
            x: 4,
            y: 0,
        }
    }

    pub fn value(&self, x: usize, y: usize) -> usize {
            SHAPES[self.typ][self.angle][y][x]
        }
    pub fn has_value(&self, x: usize, y: usize) -> bool { self.value(x, y) == 1 }

    pub fn left(&self) -> Self {
        Self {
            typ: self.typ,
            angle: self.angle,
            x: self.x - 1,
            y: self.y,
        }
    }
    pub fn right(&self) -> Self {
        Self {
            typ: self.typ,
            angle: self.angle,
            x: self.x + 1,
            y: self.y,
        }
    }
    pub fn down(&self) -> Self {
        Self {
            typ: self.typ,
            angle: self.angle,
            x: self.x,
            y: self.y + 1,
        }
    }
    pub fn rotate(&self) -> Self {
        Self {
            typ: self.typ,
            angle: (self.angle + 1) % MINO_ANGLE_MAX,
            x: self.x,
            y: self.y,
        }
    }
}


//ミノ構造定義
const SHAPES: [[[[usize; MINO_WIDTH]; MINO_HEIGHT]; MINO_ANGLE_MAX]; MINO_TYPE_MAX] =
    [
        [ //TYPE_I
            [
                [0, 1, 0, 0],
                [0, 1, 0, 0],
                [0, 1, 0, 0],
                [0, 1, 0, 0],
            ],
            [
                [0, 0, 0, 0],
                [0, 0, 0, 0],
                [1, 1, 1, 1],
                [0, 0, 0, 0],
            ],
            [
                [0, 1, 0, 0],
                [0, 1, 0, 0],
                [0, 1, 0, 0],
                [0, 1, 0, 0],
            ],
            [
                [0, 0, 0, 0],
                [1, 1, 1, 1],
                [0, 0, 0, 0],
                [0, 0, 0, 0],
            ],
        ],
        [ //TYPE_O
            [
                [0, 0, 0, 0],
                [0, 1, 1, 0],
                [0, 1, 1, 0],
                [0, 0, 0, 0],
            ],
            [
                [0, 0, 0, 0],
                [0, 1, 1, 0],
                [0, 1, 1, 0],
                [0, 0, 0, 0],
            ],
            [
                [0, 0, 0, 0],
                [0, 1, 1, 0],
                [0, 1, 1, 0],
                [0, 0, 0, 0],
            ],
            [
                [0, 0, 0, 0],
                [0, 1, 1, 0],
                [0, 1, 1, 0],
                [0, 0, 0, 0],
            ],
        ],
        [ //TYPE_S
            [
                [0, 0, 0, 0],
                [0, 1, 1, 0],
                [1, 1, 0, 0],
                [0, 0, 0, 0],
            ],
            [
                [0, 1, 0, 0],
                [0, 1, 1, 0],
                [0, 0, 1, 0],
                [0, 0, 0, 0],
            ],
            [
                [0, 0, 0, 0],
                [0, 1, 1, 0],
                [1, 1, 0, 0],
                [0, 0, 0, 0],
            ],
            [
                [0, 0, 0, 0],
                [0, 1, 0, 0],
                [0, 1, 1, 0],
                [0, 0, 1, 0],
            ],
        ],
        [ //TYPE_Z
            [
                [0, 0, 0, 0],
                [1, 1, 0, 0],
                [0, 1, 1, 0],
                [0, 0, 0, 0],
            ],
            [
                [0, 0, 0, 0],
                [0, 0, 1, 0],
                [0, 1, 1, 0],
                [0, 1, 0, 0],
            ],
            [
                [0, 0, 0, 0],
                [0, 1, 1, 0],
                [0, 0, 1, 1],
                [0, 0, 0, 0],
            ],
            [
                [0, 0, 1, 0],
                [0, 1, 1, 0],
                [0, 1, 0, 0],
                [0, 0, 0, 0],
            ],
        ],
        [ //TYPE_J
            [
                [0, 0, 1, 0],
                [0, 0, 1, 0],
                [0, 1, 1, 0],
                [0, 0, 0, 0],
            ],
            [
                [0, 0, 0, 0],
                [1, 1, 1, 0],
                [0, 0, 1, 0],
                [0, 0, 0, 0],
            ],
            [
                [0, 0, 0, 0],
                [0, 1, 1, 0],
                [0, 1, 0, 0],
                [0, 1, 0, 0],
            ],
            [
                [0, 0, 0, 0],
                [0, 1, 0, 0],
                [0, 1, 1, 1],
                [0, 0, 0, 0],
            ],
        ],
        [ //TYPE_L
            [
                [0, 1, 0, 0],
                [0, 1, 0, 0],
                [0, 1, 1, 0],
                [0, 0, 0, 0],
            ],
            [
                [0, 0, 0, 0],
                [0, 0, 1, 0],
                [1, 1, 1, 0],
                [0, 0, 0, 0],
            ],
            [
                [0, 0, 0, 0],
                [0, 1, 1, 0],
                [0, 0, 1, 0],
                [0, 0, 1, 0],
            ],
            [
                [0, 0, 0, 0],
                [0, 1, 1, 1],
                [0, 1, 0, 0],
                [0, 0, 0, 0],
            ],
        ],
        [ //TYPE_T
            [
                [0, 0, 0, 0],
                [1, 1, 1, 0],
                [0, 1, 0, 0],
                [0, 0, 0, 0],
            ],
            [
                [0, 0, 0, 0],
                [0, 1, 0, 0],
                [0, 1, 1, 0],
                [0, 1, 0, 0],
            ],
            [
                [0, 0, 0, 0],
                [0, 0, 1, 0],
                [0, 1, 1, 1],
                [0, 0, 0, 0],
            ],
            [
                [0, 0, 1, 0],
                [0, 1, 1, 0],
                [0, 0, 1, 0],
                [0, 0, 0, 0],
            ],
        ],
    ];
