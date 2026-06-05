use crate::geometry::Vec2;
use crate::physics::Ball;

pub struct Peg {
    pub center: Vec2,
    pub radius: f64,
}

pub struct Contact {
    pub normal: Vec2,
    pub penetration_depth: f64,
}

pub fn detect_ball_collision(ball: &Ball, peg: &Peg) -> Option<Contact> {
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

        assert!(detect_ball_collision(&ball, &peg).is_none());
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

        assert!(detect_ball_collision(&ball, &peg).is_none());
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

        let contact = detect_ball_collision(&ball, &peg).unwrap();
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

        let contact = detect_ball_collision(&ball, &peg).unwrap();
        assert_abs_diff_eq!(
            contact.normal,
            Vec2::new(-1.0 / 2f64.sqrt(), -1.0 / 2f64.sqrt())
        );
        assert_abs_diff_eq!(contact.penetration_depth, 2.0 - 2f64.sqrt());
    }
}
