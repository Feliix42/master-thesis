fn fill(maze: Maze, to_map: Vec<(Point, Point)>) -> Maze {
    let (points, still_to_map) = take_n(to_map, frequency);
    let (tm0, tm1) = splitup(points);
    let part0 = for pair in tm0 {
        find_path(maze, pair)
    };
    let part1 = for pair in tm1 {
        find_path(maze, pair)
    };

    let paths = join(part0, part1);

    let (remap_paths, new_maze) = update_maze(maze, paths);
    let to_remap = join(remap_paths, still_to_map);

    // recursively call `fill` as necessary
}