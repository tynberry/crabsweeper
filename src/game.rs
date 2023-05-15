
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GameCellType {
    Number(i32),
    Crab,
}

#[derive(Clone, Copy, Debug)]
pub struct GameCell {
    pub uncovered: bool,
    pub cell_type: GameCellType
}

impl GameCell {
    pub fn is_number(&self) -> bool {
        matches!(self.cell_type, GameCellType::Number(_))
    }

    pub fn is_crab(&self) -> bool {
        matches!(self.cell_type, GameCellType::Crab)
    }
}

pub struct Game {
    sx: usize,
    sy: usize,
    cells: Vec<GameCell>
}

impl Game {
    pub fn new(sx: usize, sy: usize, crabs: usize) -> Self {
        //vytvoř prázdnou mapu
        let mut cells = vec![GameCell{
            uncovered: false, cell_type: GameCellType::Number(0)
        };sx*sy];
        //vyplň několika kraby
        if crabs >= sx * sy {
            panic!("Should be handled by the UI!");
        }

        for i in 0..crabs {
            cells[i].cell_type = GameCellType::Crab;
        }
        //random
        fastrand::shuffle(&mut cells);


        Self {
            sx,
            sy,
            cells,
        }
    }

    pub fn get_index(&self, x: isize, y: isize) -> Option<usize> {
        if x < 0 || y < 0 {
            return None;
        } 
        let (x, y) = (x as usize, y as usize);
        if x >= self.sx || y >= self.sy {
            return None;
        }
        Some(x + y * self.sx)
    }

    #[inline]
    pub fn get_pos(&self, ind: usize) -> (isize, isize) {
        ((ind % self.sx) as isize, (ind / self.sx) as isize)
    }

    pub fn fix_numbers(&mut self) {
        for index in 0..self.cells.len() {
            if self.cells[index].is_number() {
                //extrahuj pozici
                let (cell_x,cell_y) = self.get_pos(index);
                //podívej se okolo
                let mut new_number = 0;
                const OFFSETS: [(isize, isize); 8] = [
                    ( 1, 1),( 1,-1),( 0,-1),(-1,-1),
                    ( 1, 0),( 0, 1),(-1, 1),(-1, 0),
                ];
                for (off_x, off_y) in OFFSETS {
                    let index = self.get_index(cell_x + off_x, cell_y + off_y);
                    if let Some(index) = index {
                        new_number += self.cells[index].is_crab() as i32;
                    }
                }
                //nastav číslo
                let GameCellType::Number(ref mut cell_number) = self.cells[index].cell_type else {
                    panic!("This should already be handled!");
                };
                *cell_number = new_number;
            }
        }
    }
}
