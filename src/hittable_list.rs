use crate::{
    hittable::{HitRecord, Hittable},
    point3::Point3,
    ray::Ray,
    vec3::Vec3,
};

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_tmin: f64, ray_tmax: f64, rec: &mut HitRecord) -> bool {
        let mut temp_rec: HitRecord = HitRecord {
            p: Point3::new([0.0, 0.0, 0.0]),
            normal: Vec3::new([0.0, 0.0, 0.0]),
            t: 0.0,
            front_face: false,
        };
        let mut hit_anything = false;
        let mut closest_so_far = ray_tmax;

        for object in &(self.objects) {
            if object.hit(r, ray_tmin, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec.clone();
            }
        }
        hit_anything
    }
}

impl HittableList {
    pub fn new() -> Self {
        HittableList {
            objects: Vec::<Box<dyn Hittable>>::new(),
        }
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }
}
