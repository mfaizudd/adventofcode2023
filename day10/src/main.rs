use std::{
    collections::{BTreeMap, HashMap},
    fs,
    ops::{Add, AddAssign},
};

struct Row<'a> {
    id: usize,
    value: &'a str,
}

#[derive(Clone, Debug)]
struct Cursor {
    steps: u64,
    pos: Vector2,
    next: Vector2,
}

struct Cell {
    value: char,
    pos: Vector2,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Vector2 {
    x: i64,
    y: i64,
}

impl Add<Vector2> for Vector2 {
    type Output = Vector2;

    fn add(self, rhs: Vector2) -> Self::Output {
        point(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Add<&Vector2> for &Vector2 {
    type Output = Vector2;

    fn add(self, rhs: &Vector2) -> Self::Output {
        point(self.x + rhs.x, self.y + rhs.y)
    }
}

impl AddAssign for Vector2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Cursor {
    fn new(pos: Vector2) -> Self {
        Cursor {
            steps: 0,
            pos,
            next: point(0, 0),
        }
    }

    fn mov(&mut self, map: &Vec<Row>, target: Vector2) -> Result<(), &str> {
        let Some(n) = try_move(map, &target, &self.pos) else {
            // println!("Invalid {:?}", &target + &self.pos);
            return Err("Invalid move")
        };
        self.pos = &target + &self.pos;
        self.next = n;
        self.steps += 1;
        Ok(())
    }

    fn move_next(&mut self, map: &Vec<Row>) -> Result<(), &str> {
        let Some(n) = try_move(map, &self.next, &self.pos) else {
            // println!("Invalid {:?}", &self.next + &self.pos);
            return Err("Invalid move")
        };
        self.pos = &self.next + &self.pos;
        self.next = n;
        self.steps += 1;
        Ok(())
    }
}

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();
    let map: Vec<_> = content
        .lines()
        .enumerate()
        .map(|(i, l)| Row { id: i, value: l })
        .collect();
    let mut lines: Vec<Vec<char>> = content.lines().map(|l| l.chars().collect()).collect();
    let starting_row = map.iter().find(|&r| r.value.contains('S')).unwrap();
    let starting_col = starting_row.value.find('S').unwrap();
    let starting_cell = Cell {
        value: 'S',
        pos: Vector2 {
            x: starting_col as i64,
            y: starting_row.id as i64,
        },
    };
    let starting_cursor = Cursor::new(starting_cell.pos);
    let mut history: BTreeMap<i64, Vec<i64>> = BTreeMap::new();
    history.insert(starting_row.id as i64, vec![starting_col as i64]);
    let mut cursors = Vec::new();
    let mut cursor = starting_cursor.clone();
    // println!("Starting({}, {})", cursor.pos.x, cursor.pos.y);
    let mut dirs = vec![false, false, false, false];
    if let Ok(_) = cursor.mov(&map, point(0, -1)) {
        cursors.push(cursor);
        dirs[0] = true;
    }
    cursor = starting_cursor.clone();
    if let Ok(_) = cursor.mov(&map, point(1, 0)) {
        cursors.push(cursor);
        dirs[1] = true;
    }
    cursor = starting_cursor.clone();
    if let Ok(_) = cursor.mov(&map, point(0, 1)) {
        cursors.push(cursor);
        dirs[2] = true;
    }
    cursor = starting_cursor.clone();
    if let Ok(_) = cursor.mov(&map, point(-1, 0)) {
        cursors.push(cursor);
        dirs[3] = true;
    }
    let mut true_s = 'S';
    if dirs[0] && dirs[1] {
        true_s = 'L';
    } else if dirs[1] && dirs[2] {
        true_s = 'F';
    } else if dirs[2] && dirs[3] {
        true_s = '7';
    } else if dirs[3] && dirs[0] {
        true_s = 'J';
    } else if dirs[0] && dirs[2] {
        true_s = '|';
    } else if dirs[1] && dirs[3] {
        true_s = '-';
    }
    // println!(
    //     "Cursor1({},{}), Cursor2({},{})",
    //     cursors[0].pos.x, cursors[0].pos.y, cursors[1].pos.x, cursors[1].pos.y
    // );
    history
        .entry(cursors[0].pos.y)
        .and_modify(|v| v.push(cursors[0].pos.x))
        .or_insert(vec![cursors[0].pos.x]);
    history
        .entry(cursors[1].pos.y)
        .and_modify(|v| v.push(cursors[1].pos.x))
        .or_insert(vec![cursors[1].pos.x]);
    while cursors[0].pos != cursors[1].pos {
        cursors[0].move_next(&map).unwrap();
        if cursors[0].pos == cursors[1].pos {
            break;
        }
        cursors[1].move_next(&map).unwrap();
        if cursors[0].pos == cursors[1].pos {
            break;
        }
        history
            .entry(cursors[0].pos.y)
            .and_modify(|v| v.push(cursors[0].pos.x))
            .or_insert(vec![cursors[0].pos.x]);
        history
            .entry(cursors[1].pos.y)
            .and_modify(|v| v.push(cursors[1].pos.x))
            .or_insert(vec![cursors[1].pos.x]);
        // println!(
        //     "Cursor1({},{}), Cursor2({},{})",
        //     cursors[0].pos.x, cursors[0].pos.y, cursors[1].pos.x, cursors[1].pos.y
        // );
    }
    let mut result = 0;
    lines[starting_row.id][starting_col] = true_s;
    for (y, xes) in history {
        let y = y as usize;
        let min = xes.iter().min().unwrap();
        let max = xes.iter().max().unwrap();
        // println!("min({min}) - max({max})");
        let mut inside = false;
        let mut pos = *min;
        while pos < *max {
            let x = pos as usize;
            if is_pipe(lines[y][x]) && !xes.contains(&pos) {
                lines[y][x] = '.'
            }
            let ch = lines[y][x];
            match ch {
                '|' => {
                    inside = !inside;
                    pos += 1;
                }
                'L' | 'F' => {
                    inside = read_horizontal(&lines[y], &mut pos, inside);
                }
                '.' if inside => {
                    lines[y][x] = 'I';
                    result += 1;
                }
                _ => {
                    pos += 1;
                    continue;
                }
            }
            // if inside && lines[y][x] == '.' {
            //     lines[y][x] = 'I';
            //     result += 1;
            // }
        }
    }
    // for line in lines {
    //     for ch in line {
    //         print!("{ch}");
    //     }
    //     println!("");
    // }
    println!("{}, {}", cursors[0].steps, cursors[1].steps);
    println!("{result}");
}

fn read_horizontal(chars: &Vec<char>, pos: &mut i64, inside: bool) -> bool {
    let first = chars[*pos as usize];
    let verts = HashMap::from([('L', '7'), ('F', 'J')]);
    *pos += 1;
    while let Some(ch) = chars.get(*pos as usize) {
        match ch {
            '7' | 'J' if verts[&first] == *ch => return !inside,
            '-' => *pos += 1,
            _ => return inside,
        };
    }
    inside
}

fn point(x: i64, y: i64) -> Vector2 {
    Vector2 { x, y }
}

fn is_pipe(input: char) -> bool {
    matches!(input, '|' | '-' | 'L' | 'J' | '7' | 'F' | 'S')
}
//
// fn is_horizontal_pipe(input: char) -> bool {
//     matches!(input, '-' | 'L' | 'J' | '7' | 'F')
// }

fn try_move(map: &Vec<Row>, pos: &Vector2, ori: &Vector2) -> Option<Vector2> {
    let cell = get_cell(map, &(pos + ori))?;
    get_movement(cell, ori)
}

fn get_cell(map: &Vec<Row>, pos: &Vector2) -> Option<Cell> {
    if pos.x < 0 || pos.y < 0 {
        return None;
    }
    if pos.y as usize >= map.len() {
        return None;
    }
    Some(Cell {
        value: map[pos.y as usize]
            .value
            .chars()
            .nth(pos.x as usize)
            .unwrap(),
        pos: pos.clone(),
    })
}

fn get_movement(input: Cell, origin: &Vector2) -> Option<Vector2> {
    match input.value {
        '-' => {
            if input.pos.y != origin.y {
                return None;
            }
            if input.pos.x == origin.x {
                return None;
            }
            if input.pos.x > origin.x {
                return Some(Vector2 { x: 1, y: 0 });
            } else {
                return Some(Vector2 { x: -1, y: 0 });
            }
        }
        '|' => {
            if input.pos.x != origin.x {
                return None;
            }
            if input.pos.y == origin.y {
                return None;
            }
            if input.pos.y > origin.y {
                return Some(Vector2 { x: 0, y: 1 });
            } else {
                return Some(Vector2 { x: 0, y: -1 });
            }
        }
        'L' => {
            if input.pos.x == origin.x && input.pos.y - 1 == origin.y {
                return Some(Vector2 { x: 1, y: 0 });
            } else if input.pos.y == origin.y && input.pos.x + 1 == origin.x {
                return Some(Vector2 { x: 0, y: -1 });
            } else {
                return None;
            }
        }
        'J' => {
            if input.pos.x == origin.x && input.pos.y - 1 == origin.y {
                return Some(Vector2 { x: -1, y: 0 });
            } else if input.pos.y == origin.y && input.pos.x - 1 == origin.x {
                return Some(Vector2 { x: 0, y: -1 });
            } else {
                return None;
            }
        }
        '7' => {
            if input.pos.x == origin.x && input.pos.y + 1 == origin.y {
                return Some(Vector2 { x: -1, y: 0 });
            } else if input.pos.y == origin.y && input.pos.x - 1 == origin.x {
                return Some(Vector2 { x: 0, y: 1 });
            } else {
                return None;
            }
        }
        'F' => {
            if input.pos.x == origin.x && input.pos.y + 1 == origin.y {
                return Some(Vector2 { x: 1, y: 0 });
            } else if input.pos.y == origin.y && input.pos.x + 1 == origin.x {
                return Some(Vector2 { x: 0, y: 1 });
            } else {
                return None;
            }
        }
        _ => None,
    }
}
