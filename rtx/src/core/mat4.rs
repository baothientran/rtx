use crate::core::math;
use crate::core::vec3;
use crate::core::vec4;
use auto_ops::{impl_op_ex, impl_op_ex_commutative};

#[derive(Copy, Clone, Debug)]
pub struct Mat4 {
    cols: [vec4::Vec4; 4],
}

impl Mat4 {
    pub fn new() -> Mat4 {
        return Mat4 {
            cols: [
                vec4::Vec4::new(1.0, 0.0, 0.0, 0.0),
                vec4::Vec4::new(0.0, 1.0, 0.0, 0.0),
                vec4::Vec4::new(0.0, 0.0, 1.0, 0.0),
                vec4::Vec4::new(0.0, 0.0, 0.0, 1.0),
            ],
        };
    }

    pub fn from_scalars(
        x0: f32,
        y0: f32,
        z0: f32,
        w0: f32,
        x1: f32,
        y1: f32,
        z1: f32,
        w1: f32,
        x2: f32,
        y2: f32,
        z2: f32,
        w2: f32,
        x3: f32,
        y3: f32,
        z3: f32,
        w3: f32,
    ) -> Mat4 {
        return Mat4 {
            cols: [
                vec4::Vec4::new(x0, y0, z0, w0),
                vec4::Vec4::new(x1, y1, z1, w1),
                vec4::Vec4::new(x2, y2, z2, w2),
                vec4::Vec4::new(x3, y3, z3, w3),
            ],
        };
    }

    pub fn from_vec4s(c0: &vec4::Vec4, c1: &vec4::Vec4, c2: &vec4::Vec4, c3: &vec4::Vec4) -> Mat4 {
        return Mat4 {
            cols: [*c0, *c1, *c2, *c3],
        };
    }

    pub fn translate(&self, v: &vec3::Vec3) -> Mat4 {
        let mut cols = self.cols;
        cols[3] = self.cols[0] * v.x + self.cols[1] * v.y + self.cols[2] * v.z + self.cols[3];
        return Mat4 { cols };
    }

    pub fn scale(&self, v: &vec3::Vec3) -> Mat4 {
        let mut cols = self.cols;
        cols[0] *= v.x;
        cols[1] *= v.y;
        cols[2] *= v.z;
        return Mat4 { cols };
    }

    pub fn rotate(&self, angle: f32, axis: &vec3::Vec3) -> Mat4 {
        let cos = f32::cos(angle);
        let sin = f32::sin(angle);
        let one_minus_cos = 1.0 - cos;

        let x_sq = axis.x * axis.x;
        let y_sq = axis.y * axis.y;
        let z_sq = axis.z * axis.z;

        let xy = axis.x * axis.y;
        let xz = axis.x * axis.z;
        let yz = axis.y * axis.z;

        let x_sin = axis.x * sin;
        let y_sin = axis.y * sin;
        let z_sin = axis.z * sin;

        // construct rotation matrix
        let x0 = cos + x_sq * one_minus_cos;
        let y0 = xy * one_minus_cos + z_sin;
        let z0 = xz * one_minus_cos - y_sin;

        let x1 = xy * one_minus_cos - z_sin;
        let y1 = cos + y_sq * one_minus_cos;
        let z1 = yz * one_minus_cos + x_sin;

        let x2 = xz * one_minus_cos + y_sin;
        let y2 = yz * one_minus_cos - x_sin;
        let z2 = cos + z_sq * one_minus_cos;

        let mut cols = self.cols;
        cols[0] = self.cols[0] * x0 + self.cols[1] * y0 + self.cols[2] * z0;
        cols[1] = self.cols[0] * x1 + self.cols[1] * y1 + self.cols[2] * z1;
        cols[2] = self.cols[0] * x2 + self.cols[1] * y2 + self.cols[2] * z2;

        return Mat4 { cols };
    }

    pub fn inverse(&self) -> Option<Mat4> {
        let mut inv = Mat4::new();
        inv.cols[0].x = self.cols[1].y * self.cols[2].z * self.cols[3].w
            - self.cols[1].y * self.cols[3].z * self.cols[2].w
            - self.cols[1].z * self.cols[2].y * self.cols[3].w
            + self.cols[1].z * self.cols[3].y * self.cols[2].w
            + self.cols[1].w * self.cols[2].y * self.cols[3].z
            - self.cols[1].w * self.cols[3].y * self.cols[2].z;

        inv.cols[0].y = -self.cols[0].y * self.cols[2].z * self.cols[3].w
            + self.cols[0].y * self.cols[3].z * self.cols[2].w
            + self.cols[0].z * self.cols[2].y * self.cols[3].w
            - self.cols[0].z * self.cols[3].y * self.cols[2].w
            - self.cols[0].w * self.cols[2].y * self.cols[3].z
            + self.cols[0].w * self.cols[3].y * self.cols[2].z;

        inv.cols[0].z = self.cols[0].y * self.cols[1].z * self.cols[3].w
            - self.cols[0].y * self.cols[3].z * self.cols[1].w
            - self.cols[0].z * self.cols[1].y * self.cols[3].w
            + self.cols[0].z * self.cols[3].y * self.cols[1].w
            + self.cols[0].w * self.cols[1].y * self.cols[3].z
            - self.cols[0].w * self.cols[3].y * self.cols[1].z;

        inv.cols[0].w = -self.cols[0].y * self.cols[1].z * self.cols[2].w
            + self.cols[0].y * self.cols[2].z * self.cols[1].w
            + self.cols[0].z * self.cols[1].y * self.cols[2].w
            - self.cols[0].z * self.cols[2].y * self.cols[1].w
            - self.cols[0].w * self.cols[1].y * self.cols[2].z
            + self.cols[0].w * self.cols[2].y * self.cols[1].z;

        inv.cols[1].x = -self.cols[1].x * self.cols[2].z * self.cols[3].w
            + self.cols[1].x * self.cols[3].z * self.cols[2].w
            + self.cols[1].z * self.cols[2].x * self.cols[3].w
            - self.cols[1].z * self.cols[3].x * self.cols[2].w
            - self.cols[1].w * self.cols[2].x * self.cols[3].z
            + self.cols[1].w * self.cols[3].x * self.cols[2].z;

        inv.cols[1].y = self.cols[0].x * self.cols[2].z * self.cols[3].w
            - self.cols[0].x * self.cols[3].z * self.cols[2].w
            - self.cols[0].z * self.cols[2].x * self.cols[3].w
            + self.cols[0].z * self.cols[3].x * self.cols[2].w
            + self.cols[0].w * self.cols[2].x * self.cols[3].z
            - self.cols[0].w * self.cols[3].x * self.cols[2].z;

        inv.cols[1].z = -self.cols[0].x * self.cols[1].z * self.cols[3].w
            + self.cols[0].x * self.cols[3].z * self.cols[1].w
            + self.cols[0].z * self.cols[1].x * self.cols[3].w
            - self.cols[0].z * self.cols[3].x * self.cols[1].w
            - self.cols[0].w * self.cols[1].x * self.cols[3].z
            + self.cols[0].w * self.cols[3].x * self.cols[1].z;

        inv.cols[1].w = self.cols[0].x * self.cols[1].z * self.cols[2].w
            - self.cols[0].x * self.cols[2].z * self.cols[1].w
            - self.cols[0].z * self.cols[1].x * self.cols[2].w
            + self.cols[0].z * self.cols[2].x * self.cols[1].w
            + self.cols[0].w * self.cols[1].x * self.cols[2].z
            - self.cols[0].w * self.cols[2].x * self.cols[1].z;

        inv.cols[2].x = self.cols[1].x * self.cols[2].y * self.cols[3].w
            - self.cols[1].x * self.cols[3].y * self.cols[2].w
            - self.cols[1].y * self.cols[2].x * self.cols[3].w
            + self.cols[1].y * self.cols[3].x * self.cols[2].w
            + self.cols[1].w * self.cols[2].x * self.cols[3].y
            - self.cols[1].w * self.cols[3].x * self.cols[2].y;

        inv.cols[2].y = -self.cols[0].x * self.cols[2].y * self.cols[3].w
            + self.cols[0].x * self.cols[3].y * self.cols[2].w
            + self.cols[0].y * self.cols[2].x * self.cols[3].w
            - self.cols[0].y * self.cols[3].x * self.cols[2].w
            - self.cols[0].w * self.cols[2].x * self.cols[3].y
            + self.cols[0].w * self.cols[3].x * self.cols[2].y;

        inv.cols[2].z = self.cols[0].x * self.cols[1].y * self.cols[3].w
            - self.cols[0].x * self.cols[3].y * self.cols[1].w
            - self.cols[0].y * self.cols[1].x * self.cols[3].w
            + self.cols[0].y * self.cols[3].x * self.cols[1].w
            + self.cols[0].w * self.cols[1].x * self.cols[3].y
            - self.cols[0].w * self.cols[3].x * self.cols[1].y;

        inv.cols[2].w = -self.cols[0].x * self.cols[1].y * self.cols[2].w
            + self.cols[0].x * self.cols[2].y * self.cols[1].w
            + self.cols[0].y * self.cols[1].x * self.cols[2].w
            - self.cols[0].y * self.cols[2].x * self.cols[1].w
            - self.cols[0].w * self.cols[1].x * self.cols[2].y
            + self.cols[0].w * self.cols[2].x * self.cols[1].y;

        inv.cols[3].x = -self.cols[1].x * self.cols[2].y * self.cols[3].z
            + self.cols[1].x * self.cols[3].y * self.cols[2].z
            + self.cols[1].y * self.cols[2].x * self.cols[3].z
            - self.cols[1].y * self.cols[3].x * self.cols[2].z
            - self.cols[1].z * self.cols[2].x * self.cols[3].y
            + self.cols[1].z * self.cols[3].x * self.cols[2].y;

        inv.cols[3].y = self.cols[0].x * self.cols[2].y * self.cols[3].z
            - self.cols[0].x * self.cols[3].y * self.cols[2].z
            - self.cols[0].y * self.cols[2].x * self.cols[3].z
            + self.cols[0].y * self.cols[3].x * self.cols[2].z
            + self.cols[0].z * self.cols[2].x * self.cols[3].y
            - self.cols[0].z * self.cols[3].x * self.cols[2].y;

        inv.cols[3].z = -self.cols[0].x * self.cols[1].y * self.cols[3].z
            + self.cols[0].x * self.cols[3].y * self.cols[1].z
            + self.cols[0].y * self.cols[1].x * self.cols[3].z
            - self.cols[0].y * self.cols[3].x * self.cols[1].z
            - self.cols[0].z * self.cols[1].x * self.cols[3].y
            + self.cols[0].z * self.cols[3].x * self.cols[1].y;

        inv.cols[3].w = self.cols[0].x * self.cols[1].y * self.cols[2].z
            - self.cols[0].x * self.cols[2].y * self.cols[1].z
            - self.cols[0].y * self.cols[1].x * self.cols[2].z
            + self.cols[0].y * self.cols[2].x * self.cols[1].z
            + self.cols[0].z * self.cols[1].x * self.cols[2].y
            - self.cols[0].z * self.cols[2].x * self.cols[1].y;

        let mut det = self.cols[0].x * inv.cols[0].x
            + self.cols[1].x * inv.cols[0].y
            + self.cols[2].x * inv.cols[0].z
            + self.cols[3].x * inv.cols[0].w;

        if math::equal_epsilon_f32(det, 0.0, math::EPSILON_F32_6) {
            return None;
        }

        det = 1.0 / det;
        return Some(inv * det);
    }

    pub fn transpose(&self) -> Mat4 {
        return Mat4::from_scalars(
            self.cols[0].x,
            self.cols[1].x,
            self.cols[2].x,
            self.cols[3].x,
            self.cols[0].y,
            self.cols[1].y,
            self.cols[2].y,
            self.cols[3].y,
            self.cols[0].z,
            self.cols[1].z,
            self.cols[2].z,
            self.cols[3].z,
            self.cols[0].w,
            self.cols[1].w,
            self.cols[2].w,
            self.cols[3].w,
        );
    }

    pub fn equal_epsilon(&self, rhs: &Mat4, epsilon: f32) -> bool {
        return vec4::Vec4::equal_epsilon(&self.cols[0], &rhs.cols[0], epsilon)
            && vec4::Vec4::equal_epsilon(&self.cols[1], &rhs.cols[1], epsilon)
            && vec4::Vec4::equal_epsilon(&self.cols[2], &rhs.cols[2], epsilon)
            && vec4::Vec4::equal_epsilon(&self.cols[3], &rhs.cols[3], epsilon);
    }
}

impl_op_ex!(+ |lhs: &Mat4, rhs: &Mat4| -> Mat4 {
    return Mat4 {
        cols: [
            lhs.cols[0] + rhs.cols[0],
            lhs.cols[1] + rhs.cols[1],
            lhs.cols[2] + rhs.cols[2],
            lhs.cols[3] + rhs.cols[3],
        ],
    };
});

impl_op_ex!(-|lhs: &Mat4, rhs: &Mat4| -> Mat4 {
    return Mat4 {
        cols: [
            lhs.cols[0] - rhs.cols[0],
            lhs.cols[1] - rhs.cols[1],
            lhs.cols[2] - rhs.cols[2],
            lhs.cols[3] - rhs.cols[3],
        ],
    };
});

impl_op_ex!(*|lhs: &Mat4, rhs: &Mat4| -> Mat4 {
    let c0 = lhs.cols[0] * rhs.cols[0].x
        + lhs.cols[1] * rhs.cols[0].y
        + lhs.cols[2] * rhs.cols[0].z
        + lhs.cols[3] * rhs.cols[0].w;
    let c1 = lhs.cols[0] * rhs.cols[1].x
        + lhs.cols[1] * rhs.cols[1].y
        + lhs.cols[2] * rhs.cols[1].z
        + lhs.cols[3] * rhs.cols[1].w;
    let c2 = lhs.cols[0] * rhs.cols[2].x
        + lhs.cols[1] * rhs.cols[2].y
        + lhs.cols[2] * rhs.cols[2].z
        + lhs.cols[3] * rhs.cols[2].w;
    let c3 = lhs.cols[0] * rhs.cols[3].x
        + lhs.cols[1] * rhs.cols[3].y
        + lhs.cols[2] * rhs.cols[3].z
        + lhs.cols[3] * rhs.cols[3].w;
    return Mat4 {
        cols: [c0, c1, c2, c3],
    };
});

impl_op_ex!(*|lhs: &Mat4, rhs: &vec4::Vec4| -> vec4::Vec4 {
    return lhs.cols[0] * rhs.x + lhs.cols[1] * rhs.y + lhs.cols[2] * rhs.z + lhs.cols[3] * rhs.w;
});

impl_op_ex!(*|lhs: &vec4::Vec4, rhs: &Mat4| -> vec4::Vec4 {
    return vec4::Vec4::new(
        vec4::Vec4::dot(&lhs, &rhs.cols[0]),
        vec4::Vec4::dot(&lhs, &rhs.cols[1]),
        vec4::Vec4::dot(&lhs, &rhs.cols[2]),
        vec4::Vec4::dot(&lhs, &rhs.cols[3]),
    );
});

impl_op_ex_commutative!(*|lhs: &Mat4, rhs: &f32| -> Mat4 {
    return Mat4 {
        cols: [
            lhs.cols[0] * rhs,
            lhs.cols[1] * rhs,
            lhs.cols[2] * rhs,
            lhs.cols[3] * rhs,
        ],
    };
});

impl_op_ex!(/ |lhs: &Mat4, rhs: &f32| -> Mat4 {
    return Mat4 {
        cols: [
            lhs.cols[0] / rhs,
            lhs.cols[1] / rhs,
            lhs.cols[2] / rhs,
            lhs.cols[3] / rhs,
        ],
    };
});

impl_op_ex!(+= |lhs: &mut Mat4, rhs: &Mat4| {
    *lhs = lhs as &Mat4 + rhs;
});

impl_op_ex!(-= |lhs: &mut Mat4, rhs: &Mat4| {
    *lhs = lhs as &Mat4 - rhs;
});

impl_op_ex!(*= |lhs: &mut Mat4, rhs: &Mat4| {
    *lhs = lhs as &Mat4 * rhs;
});

impl_op_ex!(*= |lhs: &mut Mat4, rhs: &f32| {
    *lhs = lhs as &Mat4 * rhs;
});

impl_op_ex!(/= |lhs: &mut Mat4, rhs: &f32| {
    *lhs = lhs as &Mat4 / rhs;
});

#[cfg(test)]
mod test {
    use super::*;
    use crate::core::math;

    #[test]
    fn test_create() {
        let mat = Mat4::new();
        let col0 = mat.cols[0];
        let expected_col0 = vec4::Vec4::new(1.0, 0.0, 0.0, 0.0);
        assert!(vec4::Vec4::equal_epsilon(
            &col0,
            &expected_col0,
            math::EPSILON_F32_5
        ));

        let col1 = mat.cols[1];
        let expected_col1 = vec4::Vec4::new(0.0, 1.0, 0.0, 0.0);
        assert!(vec4::Vec4::equal_epsilon(
            &col1,
            &expected_col1,
            math::EPSILON_F32_5
        ));

        let col2 = mat.cols[2];
        let expected_col2 = vec4::Vec4::new(0.0, 0.0, 1.0, 0.0);
        assert!(vec4::Vec4::equal_epsilon(
            &col2,
            &expected_col2,
            math::EPSILON_F32_5
        ));

        let col3 = mat.cols[3];
        let expected_col3 = vec4::Vec4::new(0.0, 0.0, 0.0, 1.0);
        assert!(vec4::Vec4::equal_epsilon(
            &col3,
            &expected_col3,
            math::EPSILON_F32_5
        ));
    }

    #[test]
    fn test_from_scalars() {
        let mat = Mat4::from_scalars(
            12.0, 2.0, 3.0, 1.0, 2.0, 3.0, 4.0, 1.0, 2.0, 1.0, 3.0, 1.2, 3.0, 1.2, 2.0, 3.0,
        );

        let col0 = mat.cols[0];
        let expected_col0 = vec4::Vec4::new(12.0, 2.0, 3.0, 1.0);
        assert!(vec4::Vec4::equal_epsilon(
            &col0,
            &expected_col0,
            math::EPSILON_F32_5
        ));

        let col1 = mat.cols[1];
        let expected_col1 = vec4::Vec4::new(2.0, 3.0, 4.0, 1.0);
        assert!(vec4::Vec4::equal_epsilon(
            &col1,
            &expected_col1,
            math::EPSILON_F32_5
        ));

        let col2 = mat.cols[2];
        let expected_col2 = vec4::Vec4::new(2.0, 1.0, 3.0, 1.2);
        assert!(vec4::Vec4::equal_epsilon(
            &col2,
            &expected_col2,
            math::EPSILON_F32_5
        ));

        let col3 = mat.cols[3];
        let expected_col3 = vec4::Vec4::new(3.0, 1.2, 2.0, 3.0);
        assert!(vec4::Vec4::equal_epsilon(
            &col3,
            &expected_col3,
            math::EPSILON_F32_5
        ));
    }

    #[test]
    fn test_from_vec4s() {
        let c0 = vec4::Vec4::new(1.0, 2.0, 3.0, 4.0);
        let c1 = vec4::Vec4::new(1.0, 2.0, 2.0, 1.0);
        let c2 = vec4::Vec4::new(1.0, 2.0, 4.0, 1.0);
        let c3 = vec4::Vec4::new(1.0, 2.0, 2.0, 1.2);
        let mat = Mat4::from_vec4s(&c0, &c1, &c2, &c3);

        let col0 = mat.cols[0];
        assert!(vec4::Vec4::equal_epsilon(&col0, &c0, math::EPSILON_F32_5));

        let col1 = mat.cols[1];
        assert!(vec4::Vec4::equal_epsilon(&col1, &c1, math::EPSILON_F32_5));

        let col2 = mat.cols[2];
        assert!(vec4::Vec4::equal_epsilon(&col2, &c2, math::EPSILON_F32_5));

        let col3 = mat.cols[3];
        assert!(vec4::Vec4::equal_epsilon(&col3, &c3, math::EPSILON_F32_5));
    }

    #[test]
    fn test_translate_identity() {
        let mut mat = Mat4::new();
        let v = vec3::Vec3::new(2.0, 1.2, 3.0);
        mat = Mat4::translate(&mat, &v);

        let expected_col0 = vec4::Vec4::new(1.0, 0.0, 0.0, 0.0);
        let expected_col1 = vec4::Vec4::new(0.0, 1.0, 0.0, 0.0);
        let expected_col2 = vec4::Vec4::new(0.0, 0.0, 1.0, 0.0);
        let expected_col3 = vec4::Vec4::from_vec3(&v, 1.0);
        let expected = Mat4::from_vec4s(
            &expected_col0,
            &expected_col1,
            &expected_col2,
            &expected_col3,
        );
        assert!(Mat4::equal_epsilon(&mat, &expected, math::EPSILON_F32_5));
    }

    #[test]
    fn test_translate_non_identity() {
        let c0 = vec4::Vec4::new(1.0, 2.0, 3.0, 4.0);
        let c1 = vec4::Vec4::new(1.0, 2.0, 2.0, 1.0);
        let c2 = vec4::Vec4::new(1.0, 2.0, 4.0, 1.0);
        let c3 = vec4::Vec4::new(1.0, 2.0, 2.0, 1.2);
        let mut mat = Mat4::from_vec4s(&c0, &c1, &c2, &c3);
        let v = vec3::Vec3::new(2.0, 1.2, 3.0);
        mat = Mat4::translate(&mat, &v);

        let expected_col0 = vec4::Vec4::new(1.0, 2.0, 3.0, 4.0);
        let expected_col1 = vec4::Vec4::new(1.0, 2.0, 2.0, 1.0);
        let expected_col2 = vec4::Vec4::new(1.0, 2.0, 4.0, 1.0);
        let expected_col3 = vec4::Vec4::new(7.2, 14.4, 22.4, 13.4);
        let expected = Mat4::from_vec4s(
            &expected_col0,
            &expected_col1,
            &expected_col2,
            &expected_col3,
        );
        assert!(Mat4::equal_epsilon(&mat, &expected, math::EPSILON_F32_5));
    }

    #[test]
    fn test_scale_identity() {
        let mut mat = Mat4::new();
        let s = vec3::Vec3::new(2.0, 1.2, 3.0);
        mat = Mat4::scale(&mat, &s);

        let expected_col0 = vec4::Vec4::new(2.0, 0.0, 0.0, 0.0);
        let expected_col1 = vec4::Vec4::new(0.0, 1.2, 0.0, 0.0);
        let expected_col2 = vec4::Vec4::new(0.0, 0.0, 3.0, 0.0);
        let expected_col3 = vec4::Vec4::new(0.0, 0.0, 0.0, 1.0);
        let expected = Mat4::from_vec4s(
            &expected_col0,
            &expected_col1,
            &expected_col2,
            &expected_col3,
        );
        assert!(Mat4::equal_epsilon(&mat, &expected, math::EPSILON_F32_5));
    }

    #[test]
    fn test_scale_no_identity() {
        let c0 = vec4::Vec4::new(1.0, 2.0, 3.0, 4.0);
        let c1 = vec4::Vec4::new(1.0, 2.0, 2.0, 1.0);
        let c2 = vec4::Vec4::new(1.0, 2.0, 4.0, 1.0);
        let c3 = vec4::Vec4::new(1.0, 2.0, 2.0, 1.2);
        let mut mat = Mat4::from_vec4s(&c0, &c1, &c2, &c3);

        let s = vec3::Vec3::new(2.5, 3.6, 1.5);
        mat = Mat4::scale(&mat, &s);

        let expected_col0 = vec4::Vec4::new(2.5, 5.0, 7.5, 10.0);
        let expected_col1 = vec4::Vec4::new(3.6, 7.2, 7.2, 3.6);
        let expected_col2 = vec4::Vec4::new(1.5, 3.0, 6.0, 1.5);
        let expected_col3 = vec4::Vec4::new(1.0, 2.0, 2.0, 1.2);
        let expected = Mat4::from_vec4s(
            &expected_col0,
            &expected_col1,
            &expected_col2,
            &expected_col3,
        );
        assert!(Mat4::equal_epsilon(&mat, &expected, math::EPSILON_F32_5));
    }

    #[test]
    fn test_generic_rotate_x() {
        let mut mat = Mat4::new();
        let axis = vec3::Vec3::new(1.0, 0.0, 0.0);
        let angle = math::degree_to_radian(30.0);
        mat = Mat4::rotate(&mat, angle, &axis);

        let expected_col0 = vec4::Vec4::new(1.0, 0.0, 0.0, 0.0);
        let expected_col1 = vec4::Vec4::new(0.0, f32::cos(angle), f32::sin(angle), 0.0);
        let expected_col2 = vec4::Vec4::new(0.0, -f32::sin(angle), f32::cos(angle), 0.0);
        let expected_col3 = vec4::Vec4::new(0.0, 0.0, 0.0, 1.0);
        let expected = Mat4::from_vec4s(
            &expected_col0,
            &expected_col1,
            &expected_col2,
            &expected_col3,
        );
        assert!(Mat4::equal_epsilon(&mat, &expected, math::EPSILON_F32_5));
    }

    #[test]
    fn test_generic_rotate_y() {
        let mut mat = Mat4::new();
        let axis = vec3::Vec3::new(0.0, 1.0, 0.0);
        let angle = math::degree_to_radian(30.0);
        mat = Mat4::rotate(&mat, angle, &axis);

        let expected_col0 = vec4::Vec4::new(f32::cos(angle), 0.0, -f32::sin(angle), 0.0);
        let expected_col1 = vec4::Vec4::new(0.0, 1.0, 0.0, 0.0);
        let expected_col2 = vec4::Vec4::new(f32::sin(angle), 0.0, f32::cos(angle), 0.0);
        let expected_col3 = vec4::Vec4::new(0.0, 0.0, 0.0, 1.0);
        let expected = Mat4::from_vec4s(
            &expected_col0,
            &expected_col1,
            &expected_col2,
            &expected_col3,
        );
        assert!(Mat4::equal_epsilon(&mat, &expected, math::EPSILON_F32_5));
    }

    #[test]
    fn test_generic_rotate_z() {
        let mut mat = Mat4::new();
        let axis = vec3::Vec3::new(0.0, 0.0, 1.0);
        let angle = math::degree_to_radian(30.0);
        mat = Mat4::rotate(&mat, angle, &axis);

        let expected_col0 = vec4::Vec4::new(f32::cos(angle), f32::sin(angle), 0.0, 0.0);
        let expected_col1 = vec4::Vec4::new(-f32::sin(angle), f32::cos(angle), 0.0, 0.0);
        let expected_col2 = vec4::Vec4::new(0.0, 0.0, 1.0, 0.0);
        let expected_col3 = vec4::Vec4::new(0.0, 0.0, 0.0, 1.0);
        let expected = Mat4::from_vec4s(
            &expected_col0,
            &expected_col1,
            &expected_col2,
            &expected_col3,
        );
        assert!(Mat4::equal_epsilon(&mat, &expected, math::EPSILON_F32_5));
    }

    #[test]
    fn test_generic_rotate_arbitrary_axis() {
        let mut mat = Mat4::new();
        let mut axis = vec3::Vec3::new(1.0, 0.0, 1.0);
        axis = vec3::Vec3::normalize(&axis).unwrap();
        let angle = math::degree_to_radian(30.0);
        mat = Mat4::rotate(&mat, angle, &axis);

        let expected_col0 = vec4::Vec4::new(0.9330127, 0.3535535, 0.0669874, 0.0);
        let expected_col1 = vec4::Vec4::new(-0.3535535, 0.8660253, 0.3535535, 0.0);
        let expected_col2 = vec4::Vec4::new(0.0669874, -0.3535535, 0.9330127, 0.0);
        let expected_col3 = vec4::Vec4::new(0.0, 0.0, 0.0, 1.0);
        let expected = Mat4::from_vec4s(
            &expected_col0,
            &expected_col1,
            &expected_col2,
            &expected_col3,
        );
        assert!(Mat4::equal_epsilon(&mat, &expected, math::EPSILON_F32_5));
    }

    #[test]
    fn test_generic_rotate_axis_non_identity() {
        let c0 = vec4::Vec4::new(2.0, 2.0, 3.0, 4.0);
        let c1 = vec4::Vec4::new(1.0, 2.0, 2.0, 1.0);
        let c2 = vec4::Vec4::new(1.0, 2.0, 4.0, 1.0);
        let c3 = vec4::Vec4::new(1.0, 2.0, 2.0, 1.2);
        let mut mat = Mat4::from_vec4s(&c0, &c1, &c2, &c3);

        let mut axis = vec3::Vec3::new(1.0, 0.0, 1.0);
        axis = vec3::Vec3::normalize(&axis).unwrap();
        let angle = math::degree_to_radian(30.0);
        mat = Mat4::rotate(&mat, angle, &axis);

        let expected_col0 = vec4::Vec4::new(2.2865663, 2.7071072, 3.7740947, 4.1525917);
        let expected_col1 = vec4::Vec4::new(0.5124718, 1.7320506, 2.0856041, -0.1946352);
        let expected_col2 = vec4::Vec4::new(0.713434, 1.2928932, 3.225906, 0.8474088);
        let expected_col3 = vec4::Vec4::new(1.0, 2.0, 2.0, 1.2);
        let expected = Mat4::from_vec4s(
            &expected_col0,
            &expected_col1,
            &expected_col2,
            &expected_col3,
        );
        assert!(Mat4::equal_epsilon(&mat, &expected, math::EPSILON_F32_5));
    }

    #[test]
    fn test_add() {
        let lhs = Mat4::new();
        let rhs = Mat4::from_scalars(
            1.0, 2.0, 1.0, 4.0, 1.0, 3.0, 4.0, 12.1, 1.0, 23.0, 14.0, 12.1, 3.4, 2.3, 4.4, 5.4,
        );
        let result = lhs + rhs;
        let expected = Mat4::from_scalars(
            2.0, 2.0, 1.0, 4.0, 1.0, 4.0, 4.0, 12.1, 1.0, 23.0, 15.0, 12.1, 3.4, 2.3, 4.4, 6.4,
        );
        assert!(Mat4::equal_epsilon(&result, &expected, math::EPSILON_F32_5));
    }

    #[test]
    fn test_sub() {
        let lhs = Mat4::from_scalars(
            1.0, 2.0, 1.0, 4.0, 1.0, 3.0, 4.0, 12.1, 1.0, 23.0, 14.0, 12.1, 3.4, 2.3, 4.4, 5.4,
        );
        let rhs = Mat4::new();
        let result = lhs - rhs;
        let expected = Mat4::from_scalars(
            0.0, 2.0, 1.0, 4.0, 1.0, 2.0, 4.0, 12.1, 1.0, 23.0, 13.0, 12.1, 3.4, 2.3, 4.4, 4.4,
        );
        assert!(Mat4::equal_epsilon(&result, &expected, math::EPSILON_F32_5));
    }

    #[test]
    fn test_mul_mat4_mat4() {
        let c0 = vec4::Vec4::new(2.0, 2.0, 3.0, 4.0);
        let c1 = vec4::Vec4::new(1.0, 2.0, 2.0, 1.0);
        let c2 = vec4::Vec4::new(1.0, 2.0, 4.0, 1.0);
        let c3 = vec4::Vec4::new(1.0, 2.0, 2.0, 1.2);
        let lhs = Mat4::from_vec4s(&c0, &c1, &c2, &c3);

        let mut axis = vec3::Vec3::new(1.0, 0.0, 1.0);
        axis = vec3::Vec3::normalize(&axis).unwrap();
        let angle = math::degree_to_radian(30.0);
        let mut rhs = Mat4::new();
        rhs = Mat4::rotate(&rhs, angle, &axis);

        let result = lhs * rhs;
        let expected_col0 = vec4::Vec4::new(2.2865663, 2.7071072, 3.7740947, 4.1525917);
        let expected_col1 = vec4::Vec4::new(0.5124718, 1.7320506, 2.0856041, -0.1946352);
        let expected_col2 = vec4::Vec4::new(0.713434, 1.2928932, 3.225906, 0.8474088);
        let expected_col3 = vec4::Vec4::new(1.0, 2.0, 2.0, 1.2);
        let expected = Mat4::from_vec4s(
            &expected_col0,
            &expected_col1,
            &expected_col2,
            &expected_col3,
        );
        assert!(Mat4::equal_epsilon(&result, &expected, math::EPSILON_F32_5));
    }

    #[test]
    fn test_mul_vec4_mat4() {
        let lhs = vec4::Vec4::new(2.2, 1.0, 2.4, 1.0);
        let rhs = Mat4::from_scalars(
            1.0, 2.0, 1.0, 4.0, 1.0, 3.0, 4.0, 12.1, 1.0, 23.0, 14.0, 12.1, 3.4, 2.3, 4.4, 5.4,
        );
        let result = lhs * rhs;
        let expected = vec4::Vec4::new(10.6, 26.9, 70.9, 25.74);
        assert!(vec4::Vec4::equal_epsilon(
            &result,
            &expected,
            math::EPSILON_F32_5
        ));
    }

    #[test]
    fn test_mul_mat4_vec4() {
        let lhs = Mat4::from_scalars(
            1.0, 2.0, 1.0, 4.0, 1.0, 3.0, 4.0, 12.1, 1.0, 23.0, 14.0, 12.1, 3.4, 2.3, 4.4, 5.4,
        );
        let rhs = vec4::Vec4::new(2.2, 1.0, 2.4, 1.0);
        let result = lhs * rhs;
        let expected = vec4::Vec4::new(9.0, 64.9, 44.2, 55.34);
        assert!(vec4::Vec4::equal_epsilon(
            &result,
            &expected,
            math::EPSILON_F32_5
        ));
    }

    #[test]
    fn test_mul_f32_mat4() {
        let lhs = 2.0;
        let rhs = Mat4::from_scalars(
            1.0, 2.0, 1.0, 4.0, 1.0, 3.0, 4.0, 12.1, 1.0, 23.0, 14.0, 12.1, 3.4, 2.3, 4.4, 5.4,
        );
        let result = lhs * rhs;
        let expected = Mat4::from_scalars(
            2.0, 4.0, 2.0, 8.0, 2.0, 6.0, 8.0, 24.2, 2.0, 46.0, 28.0, 24.2, 6.8, 4.6, 8.8, 10.8,
        );
        assert!(Mat4::equal_epsilon(&result, &expected, math::EPSILON_F32_5));
    }

    #[test]
    fn test_mul_mat4_f32() {
        let lhs = Mat4::from_scalars(
            1.0, 2.0, 1.0, 4.0, 1.0, 3.0, 4.0, 12.1, 1.0, 23.0, 14.0, 12.1, 3.4, 2.3, 4.4, 5.4,
        );
        let rhs = 2.0;
        let result = lhs * rhs;
        let expected = Mat4::from_scalars(
            2.0, 4.0, 2.0, 8.0, 2.0, 6.0, 8.0, 24.2, 2.0, 46.0, 28.0, 24.2, 6.8, 4.6, 8.8, 10.8,
        );
        assert!(Mat4::equal_epsilon(&result, &expected, math::EPSILON_F32_5));
    }

    #[test]
    fn test_div_mat4_f32() {
        let lhs = Mat4::from_scalars(
            1.0, 2.0, 1.0, 4.0, 1.0, 3.0, 4.0, 12.1, 1.0, 23.0, 14.0, 12.1, 3.4, 2.3, 4.4, 5.4,
        );
        let rhs = 2.0;
        let result = lhs / rhs;
        let expected = Mat4::from_scalars(
            0.5, 1.0, 0.5, 2.0, 0.5, 1.5, 2.0, 6.05, 0.5, 11.5, 7.0, 6.05, 1.7, 1.15, 2.2, 2.7,
        );
        assert!(Mat4::equal_epsilon(&result, &expected, math::EPSILON_F32_5));
    }

    #[test]
    fn test_add_assign() {
        let mut result = Mat4::new();
        let rhs = Mat4::from_scalars(
            1.0, 2.0, 1.0, 4.0, 1.0, 3.0, 4.0, 12.1, 1.0, 23.0, 14.0, 12.1, 3.4, 2.3, 4.4, 5.4,
        );
        result += rhs;
        let expected = Mat4::from_scalars(
            2.0, 2.0, 1.0, 4.0, 1.0, 4.0, 4.0, 12.1, 1.0, 23.0, 15.0, 12.1, 3.4, 2.3, 4.4, 6.4,
        );
        assert!(Mat4::equal_epsilon(&result, &expected, math::EPSILON_F32_5));
    }

    #[test]
    fn test_sub_assign() {
        let mut result = Mat4::from_scalars(
            1.0, 2.0, 1.0, 4.0, 1.0, 3.0, 4.0, 12.1, 1.0, 23.0, 14.0, 12.1, 3.4, 2.3, 4.4, 5.4,
        );
        let rhs = Mat4::new();
        result -= rhs;
        let expected = Mat4::from_scalars(
            0.0, 2.0, 1.0, 4.0, 1.0, 2.0, 4.0, 12.1, 1.0, 23.0, 13.0, 12.1, 3.4, 2.3, 4.4, 4.4,
        );
        assert!(Mat4::equal_epsilon(&result, &expected, math::EPSILON_F32_5));
    }

    #[test]
    fn test_mul_assign_mat4_mat4() {
        let c0 = vec4::Vec4::new(2.0, 2.0, 3.0, 4.0);
        let c1 = vec4::Vec4::new(1.0, 2.0, 2.0, 1.0);
        let c2 = vec4::Vec4::new(1.0, 2.0, 4.0, 1.0);
        let c3 = vec4::Vec4::new(1.0, 2.0, 2.0, 1.2);
        let mut result = Mat4::from_vec4s(&c0, &c1, &c2, &c3);

        let mut axis = vec3::Vec3::new(1.0, 0.0, 1.0);
        axis = vec3::Vec3::normalize(&axis).unwrap();
        let angle = math::degree_to_radian(30.0);
        let mut rhs = Mat4::new();
        rhs = Mat4::rotate(&rhs, angle, &axis);

        result *= rhs;
        let expected_col0 = vec4::Vec4::new(2.2865663, 2.7071072, 3.7740947, 4.1525917);
        let expected_col1 = vec4::Vec4::new(0.5124718, 1.7320506, 2.0856041, -0.1946352);
        let expected_col2 = vec4::Vec4::new(0.713434, 1.2928932, 3.225906, 0.8474088);
        let expected_col3 = vec4::Vec4::new(1.0, 2.0, 2.0, 1.2);
        let expected = Mat4::from_vec4s(
            &expected_col0,
            &expected_col1,
            &expected_col2,
            &expected_col3,
        );
        assert!(Mat4::equal_epsilon(&result, &expected, math::EPSILON_F32_5));
    }

    #[test]
    fn test_mul_assign_mat4_f32() {
        let mut result = Mat4::from_scalars(
            1.0, 2.0, 1.0, 4.0, 1.0, 3.0, 4.0, 12.1, 1.0, 23.0, 14.0, 12.1, 3.4, 2.3, 4.4, 5.4,
        );
        let rhs = 2.0;
        result *= rhs;
        let expected = Mat4::from_scalars(
            2.0, 4.0, 2.0, 8.0, 2.0, 6.0, 8.0, 24.2, 2.0, 46.0, 28.0, 24.2, 6.8, 4.6, 8.8, 10.8,
        );
        assert!(Mat4::equal_epsilon(&result, &expected, math::EPSILON_F32_5));
    }

    #[test]
    fn test_div_assign_mat4_f32() {
        let mut result = Mat4::from_scalars(
            1.0, 2.0, 1.0, 4.0, 1.0, 3.0, 4.0, 12.1, 1.0, 23.0, 14.0, 12.1, 3.4, 2.3, 4.4, 5.4,
        );
        let rhs = 2.0;
        result /= rhs;
        let expected = Mat4::from_scalars(
            0.5, 1.0, 0.5, 2.0, 0.5, 1.5, 2.0, 6.05, 0.5, 11.5, 7.0, 6.05, 1.7, 1.15, 2.2, 2.7,
        );
        assert!(Mat4::equal_epsilon(&result, &expected, math::EPSILON_F32_5));
    }

    #[test]
    fn test_inverse_generic() {
        let result = Mat4::inverse(&Mat4::from_vec4s(
            &vec4::Vec4::new(1.0, 2.0, 3.0, 4.0),
            &vec4::Vec4::new(0.0, 1.0, 2.0, 1.0),
            &vec4::Vec4::new(0.3, 4.2, 1.2, 1.0),
            &vec4::Vec4::new(0.2, 1.0, 3.0, 4.0),
        ))
        .unwrap();
        assert!(Mat4::equal_epsilon(
            &result,
            &Mat4::from_vec4s(
                &vec4::Vec4::new(1.3751868, 0.1345291, -0.3736920, -1.315396),
                &vec4::Vec4::new(-0.1001494, -0.1076233, 0.2989536, 0.0523168),
                &vec4::Vec4::new(0.115097, 0.8699551, -0.1943198, -0.2840059),
                &vec4::Vec4::new(-0.1300448, -0.6322868, 0.0896860, 0.5156950)
            ),
            math::EPSILON_F32_5
        ));
    }

    #[test]
    fn test_inverse_translate() {
        let result = Mat4::inverse(&Mat4::from_vec4s(
            &vec4::Vec4::new(1.4, 0.0, 0.0, 0.0),
            &vec4::Vec4::new(0.0, 4.5, 0.0, 0.0),
            &vec4::Vec4::new(0.0, 0.0, 2.2, 0.0),
            &vec4::Vec4::new(2.4, 2.3, 1.0, 1.0),
        ))
        .unwrap();

        assert!(Mat4::equal_epsilon(
            &result,
            &Mat4::from_vec4s(
                &vec4::Vec4::new(1.0 / 1.4, 0.0, 0.0, 0.0),
                &vec4::Vec4::new(0.0, 1.0 / 4.5, 0.0, 0.0),
                &vec4::Vec4::new(0.0, 0.0, 1.0 / 2.2, 0.0),
                &vec4::Vec4::new(-1.7142856, -0.5111111, -0.4545454, 1.0)
            ),
            math::EPSILON_F32_5
        ));
    }

    #[test]
    fn test_inverse_singular() {
        let result = Mat4::inverse(&Mat4::from_vec4s(
            &vec4::Vec4::from(0.0),
            &vec4::Vec4::from(0.0),
            &vec4::Vec4::from(0.0),
            &vec4::Vec4::from(0.0),
        ));

        assert!(result.is_none());
    }

    #[test]
    fn test_transpose() {
        let result = Mat4::transpose(&Mat4::from_vec4s(
            &vec4::Vec4::new(1.4, 0.0, 0.0, 0.0),
            &vec4::Vec4::new(2.0, 4.5, 0.0, 0.0),
            &vec4::Vec4::new(4.0, 0.0, 2.2, 0.0),
            &vec4::Vec4::new(2.4, 2.3, 1.0, 1.0),
        ));

        assert!(Mat4::equal_epsilon(
            &result,
            &Mat4::from_vec4s(
                &vec4::Vec4::new(1.4, 2.0, 4.0, 2.4),
                &vec4::Vec4::new(0.0, 4.5, 0.0, 2.3),
                &vec4::Vec4::new(0.0, 0.0, 2.2, 1.0),
                &vec4::Vec4::new(0.0, 0.0, 0.0, 1.0)
            ),
            math::EPSILON_F32_5
        ));
    }
}
