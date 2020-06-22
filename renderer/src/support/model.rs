use nalgebra::Vector3;
#[derive(Clone)]
pub struct Model{
    pub verticies:Vec<Vector3<f32>>,
    pub indicies:Vec<u32>,
    pub position:Vector3<f32>,
}
impl Model{
    pub fn cube(position:Vector3<f32>)->Self{
        Model{
            verticies:
                vec![
                    Vector3::new(-0.5, 0.5, -0.5),
                    Vector3::new( 0.5, 0.5, -0.5),
                    Vector3::new( 0.5, 0.5,  0.5),
                    Vector3::new(-0.5, 0.5,  0.5),

                    Vector3::new(-0.5,-0.5, -0.5),
                    Vector3::new( 0.5,-0.5, -0.5),
                    Vector3::new( 0.5,-0.5,  0.5),
                    Vector3::new(-0.5,-0.5,  0.5),
                ],
            indicies: vec![
                0,1,2,  0,2,3,
                0,4,5,  0,1,5,
                5,1,2,  5,6,2,
                6,7,3,  6,2,3,
                4,0,3,  4,7,3,
                4,5,6,  4,7,6,

            ],
            position:position,

        }

    }

}
