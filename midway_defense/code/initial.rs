fn fill(maze: Maze, to_map: Vec<(Point, Point)>) -> Maze {


    let paths = for pair in to_map {
        find_path(maze, pair)
    };



 


    let (remap_paths, new_maze) = update_maze(maze, paths);


    // recursively call `fill` as necessary
}