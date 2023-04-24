use rand::Rng;
use super::mat::Material;
use super::hit::{Hittable, HitRecord};
use super::aabb::AABB;
use super::vec::{Vec3, Point3};
use super::ray::Ray;

#[derive(Clone)]
pub enum Plane {
    XY,
    XZ,
    YZ
}

#[derive(Clone)]
pub struct AARect<M: Material> {
    plane: Plane,
    a0: f64,
    a1: f64,
    b0: f64,
    b1: f64,
    k: f64,
    material: M
}

fn get_axis_index(plane: &Plane) -> (usize, usize, usize) {
    match plane {
        Plane::YZ => (0, 1, 2),
        Plane::XZ => (1, 0, 2),
        Plane::XY => (2, 0, 1)
    }
}

impl<M: Material> AARect<M> {
    pub fn new(plane: Plane, a0: f64, a1: f64, b0: f64, b1: f64, k: f64, material: M) -> AARect<M> {
        AARect {
            plane,
            a0,
            a1,
            b0,
            b1,
            k,
            material
        }
    }
}

impl<M: Material> Hittable for AARect<M> {
    fn hit(&self, r: &crate::ray::Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let (k_axis_index, a_axis_index, b_axis_index) = get_axis_index(&self.plane);

        let t = (self.k - r.origin()[k_axis_index]) / r.direction()[k_axis_index];
        if t < t_min || t > t_max {
            None
        } else {
            let a = r.origin()[a_axis_index] + t * r.direction()[a_axis_index];
            let b = r.origin()[b_axis_index] + t * r.direction()[b_axis_index];
            if a < self.a0 || a > self.a1 || b < self.b0 || b > self.b1 {
                None
            } else {
                let u = (a - self.a0) / (self.a1 - self.a0);
                let v = (b - self.b0) / (self.b1 - self.b0);
                let p = r.at(t);
                let mut normal = Vec3::new(0.0, 0.0, 0.0);
                normal[k_axis_index] = 1.0;

                let mut rec = HitRecord {
                    position: p,
                    normal,
                    t,
                    u,
                    v,
                    front_face: false,
                    material: &self.material
                };

                rec.set_face_normal(r, normal);

                Some(rec)
            }
        }
    }

    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB> {
        // the bounding box must have non-zero width in each dimension, so pad the Z dimension a small amount.
        let min = Vec3::new(self.a0, self.b0, self.k - 0.0001);
        let max = Vec3::new(self.a1, self.b1, self.k + 0.0001);

        Some(AABB::new(min, max))
    }
    fn pdf_value(&self, o: Point3, v: Vec3) -> f64 {
        if let Some(rec) = self.hit(&Ray::new(o, v, 0.0), 0.001, f64::INFINITY) {
            // integration by substitution
            let area = (self.a1 - self.a0) * (self.b1 - self.b0);
            let distance_squared = rec.t.powi(2) * v.length().powi(2);
            let cosine = v.dot(rec.normal).abs() / v.length();
            if cosine != 0.0 { distance_squared / (cosine * area) } else { 0.0 }
        } else {
            0.0
        }
    }

    fn random(&self, o: Vec3) -> Vec3 {
        let mut rng = rand::thread_rng();
        let (k_axis, a_axis, b_axis) = get_axis_index(&self.plane);
        let mut random_point = Vec3::new(0.0, 0.0, 0.0);
        random_point[a_axis] = rng.gen_range(self.a0..self.a1);
        random_point[b_axis] = rng.gen_range(self.b0..self.b1);
        random_point[k_axis] = self.k;
        random_point - o
    }
}