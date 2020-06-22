use gl_renderer::{init, run, Model, Renderable};
use nalgebra::Vector3;
#[macro_use]
extern crate ecs;
mod planets {
    const G: f64 = 6.674e-11;
    use gl_renderer::Model;
    use nalgebra::Vector3;
    create_entity!(
        mass: f64,
        position: Vector3<f64>,
        velocity: Vector3<f64>,
        model: Model
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
                    let data_new = data.get_other(key.clone()).unwrap();
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
    pub struct Renderable{}
    impl BehaviorComponent for Renderable{
        fn update(&mut self, data: &mut DataGetter) {
            let pos = data.get_self().position.unwrap();
            let model = Model::cube(Vector3::new(pos.x as f32,pos.y as f32,pos.z as f32));
            data.model(model);
        
        
        }

    }
}
struct State {
    planet_system: planets::EntityManager,
}
impl State {
    pub fn new() -> Self {
        let mut s = State {
            planet_system: planets::EntityManager::new(),
        };
        s.planet_system.new_entity(
            planets::Data::new(
                || Some(1.0),
                || Some(Vector3::new(0.0, 0.0, 0.0)),
                || Some(Vector3::new(0.0, 0.0, 0.0)),
                || None,
            ),
            vec![
                Box::new(planets::PrintPosition {}),
                Box::new(planets::Gravity {}),
                Box::new(planets::Renderable{}),
            ],
        );
        s.planet_system.new_entity(
            planets::Data::new(
                || Some(1.0),
                || Some(Vector3::new(1.0, 0.0, 0.0)),
                || Some(Vector3::new(0.0, 0.0, 0.0)),
                || None,
            ),
            vec![
                Box::new(planets::PrintPosition {}),
                Box::new(planets::Gravity {}),
                Box::new(planets::Renderable{}),
            ],
        );
        s.planet_system.new_entity(
            planets::Data::new(
                || Some(1.0),
                || Some(Vector3::new(1.0, 10.0, 5.0)),
                || Some(Vector3::new(0.0, 0.0, 0.0)),
                || None,
            ),
            vec![
                Box::new(planets::PrintPosition {}),
                Box::new(planets::Gravity {}),
                Box::new(planets::Renderable{}),
            ],
        );
        return s;
    }
}

impl Renderable for State {
    fn render(&mut self) -> Vec<Model> {
        self.planet_system.process();
        let mut v = vec![];
        for (id,data) in self.planet_system.iter(){
            let model = data.model();
            if model.is_some(){
                v.push(model.unwrap());

            }
        }
        return v
    }
}
fn main() {
    println!("Hello, world!");
    run(|| State::new(), init());
}
