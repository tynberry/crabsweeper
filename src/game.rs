use macroquad::prelude::*;

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
    
    pub fn render(&self, center_x: f32, center_y: f32, font: &Font) {
        const CELL_MARGIN: f32 = 5.0;
        const CELL_SIZE: f32 = 30.0;
        //vykresli buňky postupně
        for (index, cell) in self.cells.iter().enumerate() {
            //extrakce pozice
            let (grid_x, grid_y) = self.get_pos(index);
            let (cell_x, cell_y) = (
                CELL_MARGIN + (CELL_SIZE + CELL_MARGIN) * grid_x as f32,
                CELL_MARGIN + (CELL_SIZE + CELL_MARGIN) * grid_y as f32,
            );
            //pokud neotočeno tak šedá
            let mut color = WHITE;
            let mut show_text: Option<String> = None;
            if !cell.uncovered {
                color = GRAY;
            }
            //barva podle obsahu
            if cell.uncovered {
                match cell.cell_type {
                    GameCellType::Number(numero) => {
                        color = match numero {
                            0 => WHITE,
                            1 => BLUE,
                            2 => GREEN,
                            3 => RED,
                            4 => MAGENTA,
                            5 => YELLOW,
                            6 => BROWN,
                            7 => color_u8!(0, 255, 255, 255),
                            8 => BLACK,
                            _ => panic!("Mathematically impossible")
                        };
                        show_text = Some(format!("{}", numero));
                    },
                    GameCellType::Crab => {
                        color = MAROON;
                        show_text = Some("CRAB".to_owned());
                    },
                }
            }
            //vykresli barvu
            draw_rectangle(cell_x, cell_y, CELL_SIZE, CELL_SIZE, color);
            //vykresli text
            if let Some(text) = show_text {
                let text_dim = measure_text(&text, Some(font.clone()), 32, 1.0);
                draw_text(&text, cell_x + (CELL_SIZE - text_dim.width) / 2.0, cell_y + (CELL_SIZE - text_dim.height) / 2.0, 32.0, color);
            }

        }
    }
}
