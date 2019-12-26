extern crate cgmath;

use cgmath::prelude::InnerSpace;

type Vector = cgmath::Vector2<f64>;

pub struct Fabrik {
    joints: Vec<Vector>,
    tolerance: f64,
    lengths: Vec<f64>,
    max_length: f64,
}

impl Fabrik {
    pub fn new(joints: Vec<Vector>, tolerance: f64) -> Fabrik {
        if tolerance <= 0.0 {
            panic!("tolerance must be > 0");
        }

        let mut lengths: Vec<f64> = Vec::new();

        let mut joint_a = joints[0];
        for joint_b in joints[1..].iter() {
            let link = joint_a - joint_b;
            lengths.push(link.magnitude());
            joint_a = *joint_b;
        }

        if lengths.iter().any(|l| *l <= 0.0) {
            panic!("link lenghts must be > 0");
        }

        let max_length = lengths.iter().sum();

        Fabrik {
            joints: joints,
            tolerance: tolerance,
            lengths: lengths,
            max_length: max_length,
        }
    }

    pub fn solvable(&self, target: Vector) -> bool {
        return self.max_length >= target.magnitude();
    }

    pub fn move_to(&mut self, target: Vector, try_to_reach: bool) -> usize {
        if !self.solvable(target) {
            if !try_to_reach {
                return 0;
            }

            return self.iterate(target.normalize_to(self.max_length));
        }

        self.iterate(target)
    }

    pub fn angles(&self) -> Vec<f64> {
        // TODO: Only recompute when the chain has moved
        let mut angles = vec![self.joints[1].y.atan2(self.joints[1].x)];

        let mut prev_angle = angles[0];
        for i in 2..self.joints.len() {
            let p = self.joints[i] - self.joints[i - 1];
            let abs_angle = p.y.atan2(p.x);
            angles.push(abs_angle - prev_angle);
            prev_angle = abs_angle;
        }

        angles
    }

    pub fn angles_deg(&self) -> Vec<f64> {
        self.angles().iter().map(|a| a.to_degrees()).collect()
    }

    fn iterate(&mut self, target: Vector) -> usize {
        let mut iteration: usize = 0;
        let initial_position: Vector = self.joints[0];
        let last = self.joints.len() - 1;

        while (self.joints[last] - target).magnitude() > self.tolerance {
            iteration += 1;

            self.joints[last] = target;
            for i in (0..last).rev() {
                let (next, current) = (self.joints[i + 1], self.joints[i]);
                let len_share = self.lengths[i] / (next - current).magnitude();
                self.joints[i] = (1.0 - len_share) * next + len_share * current;
            }

            self.joints[0] = initial_position;
            for i in 0..last {
                let (next, current) = (self.joints[i + 1], self.joints[i]);
                let len_share = self.lengths[i] / (next - current).magnitude();
                self.joints[i + 1] = (1.0 - len_share) * current + len_share * next;
            }
        }

        iteration
    }
}

#[cfg(test)]
mod tests {
    use self::super::*;

    #[test]
    fn test_integrated_fabrik_test() {
        let poss = vec![
            Vector::new(0.0, 0.0),
            Vector::new(10.0, 0.0),
            Vector::new(20.0, 0.0),
        ];
        let mut fab = Fabrik::new(poss, 0.01);

        assert_eq!(fab.move_to(Vector::new(20.0, 0.0), true), 0);
        assert_eq!(fab.angles_deg(), vec![0.0, 0.0]);

        assert_eq!(fab.move_to(Vector::new(60.0, 60.0), true), 249);
        assert_eq!(fab.angles_deg(), vec![43.187653094161064, 3.622882738369357]);

        assert_eq!(fab.move_to(Vector::new(0.0, 20.0), true), 250);
        assert_eq!(fab.angles_deg(), vec![88.19119752090381, 3.6158044811401675]);

        assert_eq!(fab.move_to(Vector::new(0.0, 10.0), true), 5);
        assert_eq!(fab.angles_deg(), vec![30.05682734132901, 119.97158632933548]);
    }
}
