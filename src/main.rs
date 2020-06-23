use gl_renderer::{init, run, Camera, Model, Renderable};
use nalgebra::Vector3;
#[macro_use]
extern crate ecs;
mod planets {
    #[derive(Clone)]
    pub struct Child {
        pub parent: ID,
        pub relative_position: Vector3<f64>,
    }
    const G: f64 = 6.674e-11;
    use gl_renderer::{Camera, Model};
    use nalgebra::Vector3;
    create_entity!(
        mass: f64,
        position: Vector3<f64>,
        velocity: Vector3<f64>,
        parent: Child,
        model: Model,
        camera: Camera
    );
    pub struct PrintPosition {}
    impl BehaviorComponent for PrintPosition {
        fn update(&mut self, data: &mut DataGetter) {
            let entity = data.get_self();
            println!("position: {}", entity.position().unwrap());
        }
    }
    pub struct Gravity {}
    impl BehaviorComponent for Gravity {
        fn update(&mut self, data: &mut DataGetter) {
            let delta_time = 10.0;
            let mut force = Vector3::new(0.0, 0.0, 0.0);
            let s_data = data.get_self();

            for key in data.keys() {
                if key.clone() != data.self_id() {
                    let data_new = data.get(key.clone()).unwrap();
                    let force_mag = (s_data.mass().unwrap() * data_new.mass.unwrap() * G)
                        / (s_data.position().unwrap() - data_new.position.unwrap()).norm();
                    force += ((data_new.position.unwrap() - s_data.position().unwrap())
                        * force_mag)
                        / (data_new.position.unwrap() - s_data.position().unwrap()).norm();
                }
            }

            let mut vel = data.get_self().velocity.unwrap().clone();
            vel += force * delta_time;
            data.velocity(vel);
            let mut pos = data.get_self().position.unwrap();
            pos += vel * delta_time;
            data.position(pos);
        }
    }
    pub struct Renderable {}
    impl BehaviorComponent for Renderable {
        fn update(&mut self, data: &mut DataGetter) {
            let pos = data.get_self().position.unwrap();
            let model = Model::cube(Vector3::new(pos.x as f32, pos.y as f32, pos.z as f32));
            data.model(model);
        }
    }
    pub struct ChildComponent {}

    impl BehaviorComponent for ChildComponent {
        fn update(&mut self, data: &mut DataGetter) {
            let p = data.get_self().parent.clone().unwrap();
            let parent = data.get(p.parent).unwrap().position.unwrap();
            let pos = parent + p.relative_position;
            data.position(pos);
        }
    }
    pub struct CameraComponent {}
    impl BehaviorComponent for CameraComponent {
        fn update(&mut self, data: &mut DataGetter) {
            let pos = data.get_self().position.unwrap();
            data.camera(Camera {
                position: Vector3::new(pos.x as f32, pos.y as f32, pos.z as f32),
                fov: 3.14 / 2.0,
            });
        }
    }
}
struct State {
    planet_system: planets::EntityManager,
}
fn new_planet(
    pos: Vector3<f64>,
) -> (
    planets::Data,
    Vec<std::boxed::Box<dyn planets::BehaviorComponent>>,
) {
    (
        planets::Data {
            mass: Some(1.0),
            position: Some(pos),
            velocity: Some(Vector3::new(0.0, 0.0, 0.0)),
            parent: None,
            model: None,
            camera: None,
        },
        vec![
            Box::new(planets::Gravity {}),
            Box::new(planets::Renderable {}),
        ],
    )
}
impl State {
    pub fn new() -> Self {
        let mut s = State {
            planet_system: planets::EntityManager::new(),
        };
        let (d, p1) = new_planet(Vector3::new(0.0, 0.0, 10.0));
        s.planet_system.new_entity(d, p1);

        let (d, p) = new_planet(Vector3::new(0.0, 0.0, -10.0));
        let id = s.planet_system.new_entity(d, p);
        let (d, p) = new_planet(Vector3::new(1.0, 10.0, 5.0));
        s.planet_system.new_entity(d, p);
        s.planet_system.new_entity(
            planets::Data {
                mass: Some(1.0),
                position: Some(Vector3::new(0.0, 0.0, -1.0)),
                velocity: None,
                parent: Some(planets::Child {
                    parent: id.clone(),
                    relative_position: Vector3::new(0.0, 0.0, 5.0),
                }),
                model: None,
                camera: None,
            },
            vec![
                Box::new(planets::CameraComponent {}),
                Box::new(planets::ChildComponent {}),
            ],
        );
        return s;
    }
}

impl Renderable for State {
    fn render(&mut self) -> (Vec<Model>, Camera) {
        self.planet_system.process();
        let mut v = vec![];
        let mut camera = Camera {
            position: Vector3::new(0.0, 0.0, 0.0),
            fov: 3.14 / 4.0,
        };
        for (id, data) in self.planet_system.iter() {
            let model = data.model();
            if model.is_some() {
                v.push(model.unwrap());
            }
            let c = data.camera();
            if c.is_some() {
                camera = c.unwrap();
            }
        }
        return (v, camera);
    }
}
fn main() {
    println!("Hello, world!");
    run(|| State::new(), init());
}
