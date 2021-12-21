// Copyright (c) 2021 Shreepad Shukla
// SPDX-License-Identifier: MIT

struct Target {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}

#[derive(Debug)]
struct Position {
    x: i32,
    y: i32,
}

struct Velocity {
    vx: i32,
    vy: i32,
}


pub fn calculate_maxy_trajectory(x1: i32, x2: i32, y1: i32, y2: i32) -> i32 {
    println!(
        "Calculating trajectories for target: ({},{}) - ({},{})",
        x1, y1, x2, y2
    );

    let (maxy, _) = calculate_trajectory(
        Velocity{vx: 10, vy: -10},
        Target{x1: x1, y1: y1, x2: x2, y2: y2}
    );

    maxy
}

fn calculate_trajectory(init_velocity: Velocity, target: Target) -> (i32, Option<Position>) {
    // calculate max y and target intersect position (if any)

    let mut posn = Position { x: 0, y: 0};
    let mut velocity = init_velocity;
    let mut maxy = 0i32;

    while posn.x <= target.x2  &&  posn.y >= target.y1 {
        posn.x += velocity.vx;
        posn.y += velocity.vy;
        //println!("New position: ({},{})", posn.x, posn.y);

        if posn.y > maxy {
            maxy = posn.y;
        }

        if velocity.vx > 0 {
            velocity.vx -= 1;
        }

        velocity.vy -= 1;

        // check if posn intersects target
        if (target.x1..=target.x2).contains(&posn.x)  &&  (target.y1..=target.y2).contains(&posn.y) {
            return ( maxy, Some(posn) );
        }
    }

    (maxy, None)

}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn trajectory_1() {
        let v = Velocity{vx:7, vy:2};
        let t = Target{x1:20, x2:30, y1:-10, y2:-5};
        let (maxy, strike_posn) = calculate_trajectory(v, t);
        println!("Trajectory: maxy={}, strike={:?}", maxy, strike_posn);
        assert_eq!(maxy, 4);
    }


    #[test]
    fn trajectory_2() {
        let v = Velocity{vx:6, vy:3};
        let t = Target{x1:20, x2:30, y1:-10, y2:-5};
        let (maxy, strike_posn) = calculate_trajectory(v, t);
        println!("Trajectory: maxy={}, strike={:?}", maxy, strike_posn);
        assert_eq!(maxy, 4);
    }


    #[test]
    fn trajectory_3() {
        let v = Velocity{vx:17, vy:-4};
        let t = Target{x1:20, x2:30, y1:-10, y2:-5};
        let (maxy, strike_posn) = calculate_trajectory(v, t);
        println!("Trajectory: maxy={}, strike={:?}", maxy, strike_posn);
        assert_eq!(maxy, 4);
    }
}
