use crate::geometry::Vec2;

pub struct PhysicsConfig {
    pub gravity: Vec2,
    pub dt: f64,
    pub restitution: f64,
}

pub struct Ball {
    pub position: Vec2,
    pub velocity: Vec2,
    pub radius: f64,
}

pub fn integrate_ball(ball: &mut Ball, config: &PhysicsConfig) {
    ball.velocity += config.gravity * config.dt;
    ball.position += ball.velocity * config.dt;
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;

    #[test]
    fn ball_no_gravity_moves_with_constant_velocity() {
        let mut ball = Ball {
            position: Vec2::new(0.0, 0.0),
            velocity: Vec2::new(1.0, 0.0),
            radius: 0.5,
        };
        let config = PhysicsConfig {
            gravity: Vec2::new(0.0, 0.0),
            dt: 0.1,
            restitution: 0.0,
        };

        integrate_ball(&mut ball, &config);

        assert_eq!(ball.position, Vec2::new(0.1, 0.0));
        assert_eq!(ball.velocity, Vec2::new(1.0, 0.0));
    }

    #[test]
    fn ball_with_gravity_accelerates_downwards() {
        let mut ball = Ball {
            position: Vec2::new(0.0, 0.0),
            velocity: Vec2::new(0.0, 0.0),
            radius: 0.5,
        };
        let config = PhysicsConfig {
            gravity: Vec2::new(0.0, 9.8),
            dt: 0.1,
            restitution: 0.0,
        };

        integrate_ball(&mut ball, &config);

        assert_abs_diff_eq!(ball.position, Vec2::new(0.0, 0.098));
        assert_abs_diff_eq!(ball.velocity, Vec2::new(0.0, 0.98));
    }
}
