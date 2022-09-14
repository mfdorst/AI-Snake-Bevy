use std::cmp::Ordering;
use std::collections::BinaryHeap;

use super::components::*;
use super::consts::*;

const BOARD_SIZE: usize = ARENA_WIDTH as usize * ARENA_HEIGHT as usize;
const BOARD_MAX_DIST: usize = ARENA_WIDTH as usize + ARENA_HEIGHT as usize;

#[derive(Debug, Copy, Clone, Eq)]
struct HeapElem {
    node: Node,
    pos: Pos,
}

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
struct Node {
    kind: NodeKind,
    cost: Cost,
    dir_back: Option<Dir>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
enum NodeKind {
    Food,
    Empty,
    Snake,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
struct Cost {
    f_cost: u32,
    g_cost: u32,
    h_cost: u32,
}

struct Board {
    nodes: [Node; BOARD_SIZE],
}

/// Uses A* pathfinding to find the shortest path from the snake head to `food`. The snake head is
/// assumed to be the first element of `snake`.
pub fn find_path(snake: Vec<Pos>, food: Pos) -> Vec<Dir> {
    let head = snake[0];
    let mut board = Board::new(snake, food);
    // Compute the H cost from the head to the food and store it in `board`. G cost starts at 0.
    board.get_mut(head).cost = Cost::compute(0, head, food);
    // Create a min-heap (priority queue) containing only the snake head node.
    let mut heap = BinaryHeap::new();
    heap.push(HeapElem {
        node: *board.get(head),
        pos: head,
    });
    // Keep searching until the heap is empty, lowest-cost nodes first.
    while let Some(HeapElem { node, pos }) = heap.pop() {
        // Look in each direction
        for dir in [Dir::Left, Dir::Right, Dir::Up, Dir::Down] {
            let next_pos = pos.in_direction(dir);
            // Skip the node if it is out of bounds
            if !next_pos.in_bounds() {
                continue;
            }
            let next_node = board.get_mut(next_pos);
            // Skip the node if is part of the snake's body
            if next_node.kind == NodeKind::Snake {
                continue;
            }
            // Skip the node if a shorter path to it has already been found
            if next_node.cost.g_cost <= node.cost.g_cost + 1 {
                continue;
            }
            // Update the direction back to the snake head
            next_node.dir_back = Some(dir.opposite());
            // Check if we have found the food yet
            if next_node.kind == NodeKind::Food {
                // Path found!
                // Trace the path back to the snake head and record the directions needed to get
                // there, in reverse, so that the final result is a path from the snake head to the
                // food.
                let mut path = Vec::with_capacity(BOARD_MAX_DIST);
                let mut pos = next_pos;
                while let Some(dir) = board.get(pos).dir_back {
                    path.push(dir.opposite());
                    pos = pos.in_direction(dir);
                }
                return path;
            }
            // Update the node's cost
            next_node.cost = Cost::compute(node.cost.g_cost + 1, next_pos, food);
            // Add it to the queue
            heap.push(HeapElem {
                node: *next_node,
                pos: next_pos,
            });
        }
    }

    vec![]
}

impl Board {
    fn new(snake: Vec<Pos>, food: Pos) -> Self {
        let mut board = Self {
            nodes: [Node::default(); BOARD_SIZE],
        };
        for segment in snake {
            board.get_mut(segment).kind = NodeKind::Snake;
        }
        board.get_mut(food).kind = NodeKind::Food;
        board
    }

    fn get(&self, pos: Pos) -> &Node {
        &self.nodes[pos.y as usize * ARENA_WIDTH as usize + pos.x as usize]
    }

    fn get_mut(&mut self, pos: Pos) -> &mut Node {
        &mut self.nodes[pos.y as usize * ARENA_WIDTH as usize + pos.x as usize]
    }
}

impl Cost {
    fn compute(g_cost: u32, node: Pos, goal: Pos) -> Self {
        // Compute the "Manhattan distance" between node and goal; that is, the minimum distance
        // between two points on a square grid where only orthogonal moves are allowed.
        let h_cost = (node.x - goal.x).unsigned_abs() + (node.y - goal.y).unsigned_abs();
        Self {
            f_cost: g_cost + h_cost,
            g_cost,
            h_cost,
        }
    }
}

impl Ord for HeapElem {
    // Ord is implemented as a reverse cost comparison, so that when inserted into a BinaryHeap,
    // HeapElem's will be sorted in min-heap order, and yield the lowest cost element first.
    fn cmp(&self, other: &Self) -> Ordering {
        other.node.cost.cmp(&self.node.cost)
    }
}

impl PartialOrd for HeapElem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for HeapElem {
    fn eq(&self, other: &Self) -> bool {
        self.node.cost == other.node.cost
    }
}

impl Default for NodeKind {
    fn default() -> Self {
        NodeKind::Empty
    }
}

impl Default for Cost {
    fn default() -> Self {
        Self {
            f_cost: u32::MAX,
            g_cost: u32::MAX,
            h_cost: u32::MAX,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn straight_path_right() {
        let head = Pos::new(3, 3);
        let food = Pos::new(6, 3);
        let path = find_path(vec![head], food);
        let expected = vec![Dir::Right, Dir::Right, Dir::Right];
        assert_eq!(path, expected);
    }

    #[test]
    fn straight_path_left() {
        let head = Pos::new(3, 0);
        let food = Pos::new(0, 0);
        let path = find_path(vec![head], food);
        let expected = vec![Dir::Left, Dir::Left, Dir::Left];
        assert_eq!(path, expected);
    }

    #[test]
    fn straight_path_up() {
        let head = Pos::new(0, 0);
        let food = Pos::new(0, 3);
        let path = find_path(vec![head], food);
        let expected = vec![Dir::Up, Dir::Up, Dir::Up];
        assert_eq!(path, expected);
    }

    #[test]
    fn straight_path_down() {
        let head = Pos::new(0, 3);
        let food = Pos::new(0, 0);
        let path = find_path(vec![head], food);
        let expected = vec![Dir::Down, Dir::Down, Dir::Down];
        assert_eq!(path, expected);
    }

    #[test]
    fn path_across_board() {
        let head = Pos::new(0, 0);
        let food = Pos::new(19, 19);
        let path = find_path(vec![head], food);
        // Count the number of Dir::Right's
        let right_count = path.iter().filter(|&&d| d == Dir::Right).count();
        let up_count = path.iter().filter(|&&d| d == Dir::Up).count();
        assert_eq!(right_count, 19);
        assert_eq!(up_count, 19);
    }
}
