use crate::geometry::Vec2;
use crate::physics::{integrate_ball, Ball, PhysicsConfig};

#[derive(Debug, Clone, PartialEq)]
pub struct Peg {
    pub center: Vec2,
    pub radius: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Board {
    pub width: f64,
    pub height: f64,
    pub pegs: Vec<Peg>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Contact {
    pub normal: Vec2,
    pub penetration_depth: f64,
}

pub fn detect_ball_peg_collision(ball: &Ball, peg: &Peg) -> Option<Contact> {
    let to_ball = ball.position - peg.center;
    let distance = to_ball.len();
    let penetration_depth = ball.radius + peg.radius - distance;

    if penetration_depth > 0.0 {
        Some(Contact {
            normal: to_ball / distance,
            penetration_depth,
        })
    } else {
        None
    }
}

pub fn detect_ball_wall_collision(ball: &Ball, board: &Board) -> Option<Contact> {
    let left_penetration = ball.radius - ball.position.x;
    if left_penetration > 0.0 {
        return Some(Contact {
            normal: Vec2::new(1.0, 0.0),
            penetration_depth: left_penetration,
        });
    }

    let right_penetration = ball.radius - (board.width - ball.position.x);
    if right_penetration > 0.0 {
        return Some(Contact {
            normal: Vec2::new(-1.0, 0.0),
            penetration_depth: right_penetration,
        });
    }

    None
}

pub fn resolve_ball_collision(ball: &mut Ball, contact: &Contact, restitution: f64) {
    ball.position += contact.normal * contact.penetration_depth;
    let velocity_along_normal = ball.velocity.dot(contact.normal);
    if velocity_along_normal < 0.0 {
        ball.velocity =
            ball.velocity - (1.0 + restitution) * velocity_along_normal * contact.normal;
    }
}

pub fn step_ball(ball: &mut Ball, board: &Board, config: &PhysicsConfig) {
    integrate_ball(ball, config);

    if let Some(contact) = detect_ball_wall_collision(ball, board) {
        resolve_ball_collision(ball, &contact, config.restitution);
    }

    for peg in &board.pegs {
        if let Some(contact) = detect_ball_peg_collision(ball, peg) {
            resolve_ball_collision(ball, &contact, config.restitution);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;

    #[test]
    fn detect_no_collision() {
        let ball = Ball {
            position: Vec2::new(0.0, 0.0),
            velocity: Vec2::new(0.0, 0.0),
            radius: 1.0,
        };
        let peg = Peg {
            center: Vec2::new(3.0, 0.0),
            radius: 1.0,
        };

        assert!(detect_ball_peg_collision(&ball, &peg).is_none());
    }

    #[test]
    fn detect_collision_tangential() {
        let ball = Ball {
            position: Vec2::new(0.0, 0.0),
            velocity: Vec2::new(0.0, 0.0),
            radius: 1.0,
        };
        let peg = Peg {
            center: Vec2::new(2.0, 0.0),
            radius: 1.0,
        };

        assert!(detect_ball_peg_collision(&ball, &peg).is_none());
    }

    #[test]
    fn detect_collision_overlapping() {
        let ball = Ball {
            position: Vec2::new(0.0, 0.0),
            velocity: Vec2::new(0.0, 0.0),
            radius: 1.0,
        };
        let peg = Peg {
            center: Vec2::new(1.5, 0.0),
            radius: 1.0,
        };

        let contact = detect_ball_peg_collision(&ball, &peg).unwrap();
        assert_abs_diff_eq!(contact.normal, Vec2::new(-1.0, 0.0));
        assert_abs_diff_eq!(contact.penetration_depth, 0.5);
    }

    #[test]
    fn detect_collision_diagonal() {
        let ball = Ball {
            position: Vec2::new(0.0, 0.0),
            velocity: Vec2::new(0.0, 0.0),
            radius: 1.0,
        };
        let peg = Peg {
            center: Vec2::new(1.0, 1.0),
            radius: 1.0,
        };

        let contact = detect_ball_peg_collision(&ball, &peg).unwrap();
        assert_abs_diff_eq!(
            contact.normal,
            Vec2::new(-1.0 / 2f64.sqrt(), -1.0 / 2f64.sqrt())
        );
        assert_abs_diff_eq!(contact.penetration_depth, 2.0 - 2f64.sqrt());
    }

    #[test]
    fn resolve_horizontal_collision() {
        let mut ball = Ball {
            position: Vec2::new(1.4, 0.0),
            velocity: Vec2::new(-1.0, 0.0),
            radius: 0.5,
        };
        let contact = Contact {
            normal: Vec2::new(1.0, 0.0),
            penetration_depth: 0.5,
        };

        resolve_ball_collision(&mut ball, &contact, 1.0);

        assert_abs_diff_eq!(ball.position, Vec2::new(1.9, 0.0));
        assert_abs_diff_eq!(ball.velocity, Vec2::new(1.0, 0.0));
    }

    #[test]
    fn resolve_does_not_bounce_when_moving_away() {
        let mut ball = Ball {
            position: Vec2::new(1.4, 0.0),
            velocity: Vec2::new(1.0, 0.0),
            radius: 0.5,
        };

        let contact = Contact {
            normal: Vec2::new(1.0, 0.0),
            penetration_depth: 0.1,
        };

        resolve_ball_collision(&mut ball, &contact, 1.0);

        assert_abs_diff_eq!(ball.position, Vec2::new(1.5, 0.0));
        assert_abs_diff_eq!(ball.velocity, Vec2::new(1.0, 0.0));
    }

    #[test]
    fn resolve_no_wall_collision() {
        let ball = Ball {
            position: Vec2::new(5.0, 5.0),
            velocity: Vec2::new(1.0, 0.0),
            radius: 0.5,
        };
        let board = Board {
            width: 10.0,
            height: 10.0,
            pegs: vec![],
        };

        assert!(detect_ball_wall_collision(&ball, &board).is_none());
    }

    #[test]
    fn resolve_left_wall_collision() {
        let mut ball = Ball {
            position: Vec2::new(0.4, 0.0),
            velocity: Vec2::new(-1.0, 0.0),
            radius: 0.5,
        };
        let board = Board {
            width: 10.0,
            height: 10.0,
            pegs: vec![],
        };

        let contact = detect_ball_wall_collision(&ball, &board).unwrap();
        resolve_ball_collision(&mut ball, &contact, 1.0);

        assert_abs_diff_eq!(ball.position, Vec2::new(0.5, 0.0));
        assert_abs_diff_eq!(ball.velocity, Vec2::new(1.0, 0.0));
    }

    #[test]
    fn resolve_right_wall_collision() {
        let mut ball = Ball {
            position: Vec2::new(9.6, 0.0),
            velocity: Vec2::new(1.0, 0.0),
            radius: 0.5,
        };
        let board = Board {
            width: 10.0,
            height: 10.0,
            pegs: vec![],
        };

        let contact = detect_ball_wall_collision(&ball, &board).unwrap();
        resolve_ball_collision(&mut ball, &contact, 1.0);

        assert_abs_diff_eq!(ball.position, Vec2::new(9.5, 0.0));
        assert_abs_diff_eq!(ball.velocity, Vec2::new(-1.0, 0.0));
    }

    #[test]
    fn resolve_left_wall_collision_diagonal() {
        let mut ball = Ball {
            position: Vec2::new(0.4, 0.4),
            velocity: Vec2::new(-1.0, -1.0),
            radius: 0.5,
        };
        let board = Board {
            width: 10.0,
            height: 10.0,
            pegs: vec![],
        };

        let contact = detect_ball_wall_collision(&ball, &board).unwrap();
        resolve_ball_collision(&mut ball, &contact, 1.0);

        assert_abs_diff_eq!(ball.position, Vec2::new(0.5, 0.4));
        assert_abs_diff_eq!(ball.velocity, Vec2::new(1.0, -1.0));
    }

    #[test]
    fn resolve_left_wall_collision_with_restitution() {
        let mut ball = Ball {
            position: Vec2::new(0.4, 0.0),
            velocity: Vec2::new(-1.0, 0.0),
            radius: 0.5,
        };
        let board = Board {
            width: 10.0,
            height: 10.0,
            pegs: vec![],
        };

        let contact = detect_ball_wall_collision(&ball, &board).unwrap();
        resolve_ball_collision(&mut ball, &contact, 0.5);

        assert_abs_diff_eq!(ball.position, Vec2::new(0.5, 0.0));
        assert_abs_diff_eq!(ball.velocity, Vec2::new(0.5, 0.0));
    }

    #[test]
    fn step_ball_no_collisions() {
        let mut ball = Ball {
            position: Vec2::new(5.0, 5.0),
            velocity: Vec2::new(1.0, 0.0),
            radius: 0.5,
        };
        let board = Board {
            width: 10.0,
            height: 10.0,
            pegs: vec![],
        };
        let config = PhysicsConfig {
            gravity: Vec2::new(0.0, 0.0),
            dt: 1.0,
            restitution: 1.0,
        };
        step_ball(&mut ball, &board, &config);
        assert_abs_diff_eq!(ball.position, Vec2::new(6.0, 5.0));
        assert_abs_diff_eq!(ball.velocity, Vec2::new(1.0, 0.0));
    }

    #[test]
    fn step_ball_with_wall_collision() {
        let mut ball = Ball {
            position: Vec2::new(9.6, 5.0),
            velocity: Vec2::new(1.0, 0.0),
            radius: 0.5,
        };
        let board = Board {
            width: 10.0,
            height: 10.0,
            pegs: vec![],
        };
        let config = PhysicsConfig {
            gravity: Vec2::new(0.0, 0.0),
            dt: 1.0,
            restitution: 1.0,
        };
        step_ball(&mut ball, &board, &config);
        assert_abs_diff_eq!(ball.position, Vec2::new(9.5, 5.0));
        assert_abs_diff_eq!(ball.velocity, Vec2::new(-1.0, 0.0));
    }
}
