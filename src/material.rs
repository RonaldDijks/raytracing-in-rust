use crate::{hittable::HitRecord, ray::Ray, vec3::Vec3};

pub struct ScatterResult {
    pub scattered: Ray,
    pub attenuation: Vec3,
}

pub trait Material {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<ScatterResult>;
}

pub struct Lambertian {
    pub albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, rec: &HitRecord) -> Option<ScatterResult> {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        Some(ScatterResult {
            scattered: Ray::new(rec.position, scatter_direction),
            attenuation: self.albedo,
        })
    }
}

fn reflect(a: &Vec3, b: &Vec3) -> Vec3 {
    a - 2.0 * a.dot(b) * b
}

pub struct Fuzz(f64);

impl Fuzz {
    pub fn new(value: f64) -> Fuzz {
        if value > 1.0 {
            Fuzz(1.0)
        } else {
            Fuzz(value)
        }
    }
}

pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: Fuzz,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: Fuzz) -> Self {
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<ScatterResult> {
        let reflected = reflect(&ray.direction.normalized(), &rec.normal);
        let scattered = Ray::new(
            rec.position,
            reflected + self.fuzz.0 * Vec3::random_in_unit_sphere(),
        );
        let attenuation = self.albedo;
        if scattered.direction.dot(&rec.normal) > 0.0 {
            Some(ScatterResult {
                scattered,
                attenuation,
            })
        } else {
            None
        }
    }
}
