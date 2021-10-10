use crate::{ray::Ray, vec3::Vec3};

pub struct HitNormal {
    pub normal: Vec3,
    pub front_face: bool,
}

impl HitNormal {
    fn create(ray: &Ray, outward_normal: &Vec3) -> Self {
        let front_face = ray.direction.dot(outward_normal) < 0.0;
        let normal = if front_face {
            *outward_normal
        } else {
            -outward_normal
        };
        Self { front_face, normal }
    }
}

pub struct HitRecord {
    pub position: Vec3,
    pub normal: HitNormal,
    pub t: f64,
}

impl HitRecord {
    pub fn create(normal: HitNormal, position: Vec3, t: f64) -> Self {
        Self {
            normal,
            position,
            t,
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64) -> Self {
        Self { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = oc.dot(&ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        };
        let discriminant_sqrt = discriminant.sqrt();

        let root = (-half_b - discriminant_sqrt) / a;
        if root < t_min || t_max < root {
            let root = (-half_b + discriminant_sqrt) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let t = root;
        let position = ray.at(t);
        let outward_normal = (position - self.center) / self.radius;
        let normal = HitNormal::create(ray, &outward_normal);
        let record = HitRecord::create(normal, position, t);
        Some(record)
    }
}

pub struct HittableList {
    pub items: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new(items: Vec<Box<dyn Hittable>>) -> Self {
        Self { items }
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut result = None;
        let mut closest_so_far = t_max;

        for item in self.items.iter() {
            if let Some(rec) = item.hit(ray, t_min, closest_so_far) {
                closest_so_far = rec.t;
                result = Some(rec);
            }
        }

        result
    }
}
