fn fill(maze: Maze, to_map: Vec<(Point, Point)>) -> Maze {

    let (tm0, tm1) = splitup(to_map);
    let part0 = for pair in tm0 {
        find_path(maze, pair)
    };
    let part1 = for pair in tm1 {
        find_path(maze, pair)
    };

    let paths = join(part0, part1);

    let (remap_paths, new_maze) = update_maze(maze, paths);

    
    // recursively call `fill` as necessary
}