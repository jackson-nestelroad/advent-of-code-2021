use crate::common::{iAoc, AocResult, IntoAocResult};
use std::collections::{HashMap, HashSet};

struct Cave<'a> {
    name: &'a str,
    small: bool,
    adjacent: Vec<&'a str>,
}

impl<'a> Cave<'a> {
    pub fn new(name: &'a str) -> Self {
        Cave {
            name,
            small: name.chars().all(|ch| ch.is_ascii_lowercase()),
            adjacent: Vec::new(),
        }
    }

    pub fn is_small(&self) -> bool {
        self.small
    }

    pub fn is_start(&self) -> bool {
        self.name == "start"
    }

    pub fn is_end(&self) -> bool {
        self.name == "end"
    }
}

struct CaveSystem<'a> {
    caves: HashMap<&'a str, Cave<'a>>,
}

impl<'a> CaveSystem<'a> {
    pub fn new() -> Self {
        CaveSystem {
            caves: HashMap::new(),
        }
    }

    pub fn from_str(input: &'a str) -> AocResult<Self> {
        let mut system = CaveSystem::new();
        let caves = &mut system.caves;
        for line in input.lines() {
            let (from, to) = line.split_once('-').into_aoc_result()?;
            caves
                .entry(from)
                .or_insert(Cave::new(from))
                .adjacent
                .push(to);
            caves.entry(to).or_insert(Cave::new(to)).adjacent.push(from);
        }
        Ok(system)
    }

    fn count_paths_dfs(
        &self,
        location: &'a str,
        visited: &mut HashSet<&'a str>,
        mut allow_extra_cave: bool,
    ) -> AocResult<iAoc> {
        let cave = self
            .caves
            .get(location)
            .into_aoc_result_msg("cave not found")?;

        if cave.is_end() {
            return Ok(1);
        }

        let mut cave_is_visited_extra = false;
        if cave.is_small() {
            if visited.contains(location) {
                if allow_extra_cave && !cave.is_start() {
                    allow_extra_cave = false;
                    cave_is_visited_extra = true;
                } else {
                    return Ok(0);
                }
            } else {
                visited.insert(location);
            }
        }

        let mut count = 0;
        for adj in &cave.adjacent {
            count += self.count_paths_dfs(adj, visited, allow_extra_cave)?;
        }
        if cave.is_small() && !cave_is_visited_extra {
            visited.remove(location);
        }
        Ok(count)
    }

    pub fn count_paths(&self, allow_extra_cave: bool) -> AocResult<iAoc> {
        let mut visited = HashSet::new();
        self.count_paths_dfs("start", &mut visited, allow_extra_cave)
    }
}

pub fn solve_a(input: &str) -> AocResult<iAoc> {
    let system = CaveSystem::from_str(input)?;
    let result = system.count_paths(false)?;
    Ok(result)
}

pub fn solve_b(input: &str) -> AocResult<iAoc> {
    let system = CaveSystem::from_str(input)?;
    let result = system.count_paths(true)?;
    Ok(result)
}
