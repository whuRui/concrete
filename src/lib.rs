use std::collections::{HashSet, VecDeque};

use ndarray::{Array2, Array3};
use pyo3::prelude::*;
use rand::seq::SliceRandom;
use rand::thread_rng;

static AGGREGATE: u8 = 0;
static CERAMSITE: u8 = 7;

#[pyfunction]
fn make_some_ceramsite(mut cube: Vec<Vec<Vec<u8>>>, n: f64)
                       -> PyResult<(usize, usize, usize, Vec<Vec<Vec<u8>>>)> {
    let mut coords: Vec<(u16, u16, u16, usize)> = Vec::new();
    let pages = cube.len() as u16;
    let rows = cube[0].len() as u16;
    let columns = cube[0][0].len() as u16;
    println!("模型尺寸:{}x{}x{}", columns, rows, pages);
    let maximum = 47000;
    println!("骨料体积:{}", maximum);

    let mut visited = Array3::<u8>::zeros((pages as usize, rows as usize, columns as usize));
    for p in 0..pages {
        for r in 0..rows {
            for c in 0..columns {
                if cube[p as usize][r as usize][c as usize] == 0 && visited[[p as usize, r as usize, c as usize]] != 1 {
                    visited[[p as usize, r as usize, c as usize]] = 1;
                    bfs(&mut cube, (p, r, c), Some(&mut coords),
                        pages, rows, columns, &mut visited, None, maximum)
                }
            }
        }
    }
    let num = coords.len();
    println!("过滤前骨料数量:{}", num);
    let threshold = 100;
    println!("过滤阈值:{}", threshold);


    let mut coords: Vec<(u16, u16, u16, usize)> = coords.iter().filter_map(|&v| {
        if v.3 < threshold {
            return None;
        }
        Some(v)
    }).collect();
    let num = coords.len();
    println!("过滤后骨料数量:{}", num);

    let total = coords.iter().fold(0, |mut total, v| {
        total += v.3;
        total
    });
    let mut rng = thread_rng();
    coords.shuffle(&mut rng);

    let mut converted = 0;
    let mut convert_set: HashSet<(u16, u16, u16)> = HashSet::new();
    for i in 0..(num as f64 / n) as usize {
        convert_set.insert((coords[i].0, coords[i].1, coords[i].2));
        converted += coords[i].3;
    }

    let mut visited = Array3::<u8>::zeros((pages as usize, rows as usize, columns as usize));
    for p in 0..pages {
        for r in 0..rows {
            for c in 0..columns {
                if cube[p as usize][r as usize][c as usize] == 0 && visited[[p as usize, r as usize, c as usize]] != 1 {
                    visited[[p as usize, r as usize, c as usize]] = 1;
                    bfs(&mut cube, (p, r, c), None,
                        pages, rows, columns, &mut visited, Some(&mut convert_set), maximum)
                }
            }
        }
    }

    Ok((num, total, converted, cube))
}

fn bfs(cube: &mut Vec<Vec<Vec<u8>>>, start: (u16, u16, u16),
       mut coords: Option<&mut Vec<(u16, u16, u16, usize)>>,
       pages: u16, rows: u16, columns: u16, visited: &mut Array3<u8>,
       convert_set: Option<&mut HashSet<(u16, u16, u16)>>, maximum: usize) {
    let (p, r, c) = start;
    let convert = if let Some(set) = convert_set {
        set.contains(&(p, r, c))
    } else { false };

    let mut neighbors = VecDeque::from(vec![(p, r, c)]);
    let mut num = 1;
    'outer: while !neighbors.is_empty() {
        let (p, r, c) = neighbors.pop_front().unwrap();
        for (z, y, x) in [
            (p.overflowing_sub(1).0, r, c),
            (p, r.overflowing_sub(1).0, c),
            (p, r, c.overflowing_sub(1).0),
            (p + 1, r, c),
            (p, r + 1, c),
            (p, r, c + 1)] {
            if x < columns &&
                y < rows &&
                z < pages &&
                cube[z as usize][y as usize][x as usize] == AGGREGATE &&
                visited[[z as usize, y as usize, x as usize]] != 1 {
                neighbors.push_back((z, y, x));
                visited[[z as usize, y as usize, x as usize]] = 1;
                num += 1;
                if convert {
                    cube[z as usize][y as usize][x as usize] = CERAMSITE;
                }
                if num > maximum {
                    break 'outer;
                }
            }
        }
    }
    if let Some(ref mut m) = coords {
        m.push((p, r, c, num));
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn concrete(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(make_some_ceramsite, m)?)?;
    Ok(())
}

