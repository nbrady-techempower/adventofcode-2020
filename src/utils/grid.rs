pub enum DIR {
    N, S, E, W, NW, NE, SW, SE
}

#[derive(Debug, Clone, Copy)]
pub struct Point {
    x: i32,
    y: i32
}

#[derive(Debug, Clone)]
pub struct Grid {
    map: Vec<Vec<char>>,
    pub pos: Point,
    iter_pos: Point
}

impl Grid {
    pub fn new(map: Vec<String>) -> Self {
        let mut v: Vec<Vec<char>> = vec!();
        for i in map {
            v.push(i.chars().collect());
        }
        Self {
            map: v,
            pos: Point { x: 0, y: 0 },
            iter_pos: Point { x: 0, y: 0 }
        }
    }

    // Moves in a DIR and returns the char on the map if move was successful
    pub fn moves(&mut self, dir: &DIR) -> Option<char> {
        let mut tmp = self.pos;
        match dir {
            DIR::N => { tmp.y -= 1; },
            DIR::S => { tmp.y += 1; },
            DIR::E => { tmp.x += 1; },
            DIR::W => { tmp.x -= 1; },
            DIR::NW => { tmp.x -= 1; tmp.y -= 1; },
            DIR::SW => { tmp.x -= 1; tmp.y += 1; },
            DIR::NE => { tmp.x += 1; tmp.y -= 1; },
            DIR::SE => { tmp.x += 1; tmp.y += 1; }
        }

        self.move_to(tmp)
    }

    // Looks in a DIR and returns the char on the map if there is one
    pub fn look(&self, dir: &DIR) -> Option<char> {
        let mut tmp = self.pos;
        match dir {
            DIR::N => { tmp.y -= 1; },
            DIR::S => { tmp.y += 1; },
            DIR::E => { tmp.x += 1; },
            DIR::W => { tmp.x -= 1; },
            DIR::NW => { tmp.x -= 1; tmp.y -= 1; },
            DIR::SW => { tmp.x -= 1; tmp.y += 1; },
            DIR::NE => { tmp.x += 1; tmp.y -= 1; },
            DIR::SE => { tmp.x += 1; tmp.y += 1; }
        }

        self.get_point(tmp)
    }

    pub fn move_to(&mut self, point: Point) -> Option<char> {
        let to_ret = self.get_point(point);
        if let Some(_) = to_ret {
            self.pos = point;
        }
        to_ret
    }

    pub fn get_point(&self, point: Point) -> Option<char> {
        if point.x < 0 || point.y < 0 || point.y >= self.map.len() as i32 || point.x >= self.map[0].len() as i32 {
            return None;
        }
        Some(self.map[point.y as usize][point.x as usize])
    }

    pub fn set_point(&mut self, point: Point, c: char) {
        if point.x < 0 || point.y < 0 || point.y >= self.map.len() as i32 || point.x >= self.map[0].len() as i32 {
            panic!("Trying to set a point on Grid that doesn't exist.");
        }
        self.map[point.y as usize][point.x as usize] = c;
    }

    // counts the number of c: char on the map
    pub fn count_char(&self, c: char) -> i32 {
        let mut to_ret = 0;
        for i in &self.map {
            for ch in i {
                if *ch == c {
                    to_ret += 1;
                }
            }
        }
        to_ret
    }
}

impl Iterator for Grid {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        let mut to_ret = Some(self.iter_pos);
        if self.get_point(self.iter_pos).is_none() {
            to_ret = None;
        }

        if self.iter_pos.y < self.map.len() as i32 {
            self.iter_pos.x += 1;
            if self.iter_pos.x >= self.map[self.iter_pos.y as usize].len() as i32 {
                self.iter_pos.y += 1;
                self.iter_pos.x = 0;
            }
        }
        to_ret
    }
}

impl PartialEq for Grid {
    fn eq(&self, other: &Grid) -> bool {
        if self.map.len() != other.map.len() {
            return false;
        }

        for p in self.clone() {
            if self.get_point(p) != other.get_point(p)  {
                return false;
            }
        }
        true
    }
}
