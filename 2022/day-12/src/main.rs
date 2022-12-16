use std::{
    fmt::{Debug, Display},
    fs, vec,
};

const X: usize = 173; //8; //173;
const Y: usize = 41; //5; //41;

#[derive(Debug, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Clone, Copy)]
struct ValueWithShortestPath {
    val: u8,
    shortest_path_to_point: Option<usize>,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Point { x, y }
    }
}

// impl Hash for Point {
//     fn hash<H: Hasher>(&self, state: &mut H) {
//         self.0.hash(state);
//         self.1.hash(state);
//     }
// }

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Point {}

#[derive(Debug, Clone)]
struct Field {
    elevations: [[ValueWithShortestPath; Y]; X],
    start: Point,
    end: Point,
    shortest_path: Option<Vec<Point>>,
}

impl Debug for ValueWithShortestPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} ({})",
            (self.val + b'a' - 1) as char,
            self.shortest_path_to_point.unwrap_or(0)
        )
    }
}

impl Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..Y {
            for x in 0..X {
                if self.elevations[x][y].val == 0 {
                    write!(f, "S")?;
                } else if self.elevations[x][y].val == 255 {
                    write!(f, "E")?;
                } else if let Some(v) = &self.shortest_path {
                    /*if v.contains(&Point { x, y }) {
                        write!(f, " {} ", (self.elevations[x][y].val + b'a' - 1) as char)?
                    } else*/
                    {
                        write!(
                            f,
                            "({})",
                            self.elevations[x][y].shortest_path_to_point.unwrap_or(0)
                        )?
                    }
                }
            }
            f.write_str("\r\n")?;
        }
        Ok(())
    }
}

impl Field {
    fn print_path(&self) {
        if let Some(v) = &self.shortest_path {
            for p in v {
                print!("{:#?}->", self.elevations[p.x][p.y]);
            }
        }
        println!();
    }
    fn parse_grid(lines: Vec<&str>) -> ([[ValueWithShortestPath; Y]; X], Point, Point) {
        let mut grid = [[ValueWithShortestPath {
            val: 0,
            shortest_path_to_point: None,
        }; Y]; X];
        let mut start_xy = Point::new(0, 0);
        let mut end_xy = Point::new(0, 0);
        for (y, line) in lines.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let mut v = match c {
                    'S' => 0,
                    'E' => b'z' - b'a' + 1,
                    _ => c as u8 - b'a' + 1,
                };
                if v == 0 {
                    start_xy = Point::new(x, y);
                    v = 1;
                } else if v == b'z' - b'a' + 1 {
                    end_xy = Point::new(x, y);
                }
                grid[x][y] = ValueWithShortestPath {
                    val: v,
                    shortest_path_to_point: None,
                };
            }
        }
        (grid, start_xy, end_xy)
    }
    fn parse(lines: Vec<&str>) -> Self {
        let (grid, start, end) = Self::parse_grid(lines);
        Field {
            elevations: grid,
            start,
            end,
            shortest_path: None,
        }
    }

    fn find_shortest_path(&mut self, point: &mut Point, path: &mut Vec<Point>) {
        path.push(*point);
        if let Some(dist) = self.elevations[point.x][point.y].shortest_path_to_point {
            if dist <= path.len() {
                return;
            }
        }

        self.elevations[point.x][point.y].shortest_path_to_point = Some(path.len());
        if let Some(s) = &self.shortest_path {
            if path.len() > s.len() {
                return;
            }
        }

        if *point == self.end {
            //println!("\nFound path... {:?}", path.len());
            match &self.shortest_path {
                None => self.shortest_path = Some(path.clone()),
                Some(v) if v.len() > path.len() => self.shortest_path = Some(path.clone()),
                _ => (), //println!("Found path but it's not shorter..."),
            }
            return;
        }
        let moves = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
        let mut new_points = Vec::new();
        for m in moves {
            let new_y = point.y as i32 + m.1;
            let new_x = point.x as i32 + m.0;
            if new_y >= 0 && new_y < Y as i32 && new_x >= 0 && new_x < X as i32 {
                let new_point = Point::new(new_x as usize, new_y as usize);
                //we can only climb +1, we can go down
                if self.elevations[new_point.x][new_point.y].val
                    > self.elevations[point.x][point.y].val + 1
                {
                    continue;
                }
                if !path.contains(&new_point) {
                    new_points.push(new_point);
                }
            }
        }

        if new_points.is_empty() {
            /*print!(
                "\rReached point: {:?} {:?}",
                point, self.elevations[point.x][point.y]
            );*/
            //return;
            path.pop();
        } else {
            for mut p in new_points {
                let mut new_path = path.clone();
                self.find_shortest_path(&mut p, &mut new_path);
            }
        }
    }

    fn find_shortest_path_reverse(&mut self, point: &mut Point, path: &mut Vec<Point>) {
        path.push(*point);
        if let Some(dist) = self.elevations[point.x][point.y].shortest_path_to_point {
            if dist <= path.len() {
                return;
            }
        }

        self.elevations[point.x][point.y].shortest_path_to_point = Some(path.len());
        if let Some(s) = &self.shortest_path {
            if path.len() > s.len() {
                return;
            }
        }

        if self.elevations[point.x][point.y].val == 1 {
            //println!("\nFound path... {:?}", path.len());
            match &self.shortest_path {
                None => self.shortest_path = Some(path.clone()),
                Some(v) if v.len() > path.len() => self.shortest_path = Some(path.clone()),
                _ => (), //println!("Found path but it's not shorter..."),
            }
            return;
        }
        let moves = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
        let mut new_points = Vec::new();
        for m in moves {
            let new_y = point.y as i32 + m.1;
            let new_x = point.x as i32 + m.0;
            if new_y >= 0 && new_y < Y as i32 && new_x >= 0 && new_x < X as i32 {
                let new_point = Point::new(new_x as usize, new_y as usize);
                //we can only go down +1, we can go down
                if (self.elevations[new_point.x][new_point.y].val + 1
                    == self.elevations[point.x][point.y].val
                    || self.elevations[new_point.x][new_point.y].val
                        == self.elevations[point.x][point.y].val
                    || self.elevations[new_point.x][new_point.y].val
                        > self.elevations[point.x][point.y].val)
                    && !path.contains(&new_point)
                {
                    new_points.push(new_point);
                }
            }
        }

        if new_points.is_empty() {
            /*print!(
                "\rReached point: {:?} {:?}",
                point, self.elevations[point.x][point.y]
            );*/
            //return;
            path.pop();
        } else {
            for mut p in new_points {
                let mut new_path = path.clone();
                self.find_shortest_path_reverse(&mut p, &mut new_path);
            }
        }
    }
}

fn main() -> std::io::Result<()> {
    println!("🎄 Ostra Piła - Advent of Code 2022 - Day 12 🎄");
    let content = fs::read_to_string("input-12.txt")?;
    let lines = content.split("\r\n").collect::<Vec<_>>();
    let mut field = Field::parse(lines);
    let mut field2 = field.clone();
    let mut path = vec![];
    println!("Start: {:?}, end: {:?}", field.start, field.end);
    let mut start = field.start;
    field.find_shortest_path(&mut start, &mut path);
    println!();
    let shortest_path = field.shortest_path.unwrap().len();
    println!("Part1 - {:?}", shortest_path - 1);

    /*let mut points_with_a = vec![];
    for y in 0..Y {
        for x in 0..X {
            if field.elevations[x][y].val == 1 {
                points_with_a.push((field.elevations[x][y], Point { x, y }));
            }
        }
    }
    points_with_a.sort_by_key(|(p, _)| p.shortest_path_to_point);
    points_with_a.reverse();

    println!("Potential points to check: {}", points_with_a.len());
    let mut shortest_len = shortest_path;
    for mut p in points_with_a {
        let mut f = field2.clone();
        let mut path = vec![];
        f.find_shortest_path(&mut p.1, &mut path, Some(shortest_len));

        if let Some(path) = f.shortest_path {
            if path.len() < shortest_len {
                shortest_len = path.len();
                println!("\rShortest path is {} from {:?}", shortest_len, p);
            }
        }
    }*/
    let mut path = vec![];
    let mut start = field.end;
    field2.find_shortest_path_reverse(&mut start, &mut path);
    let shortest_path = field2.shortest_path.unwrap().len();
    println!("Part2 - {:?}", shortest_path - 1);
    Ok(())
}
