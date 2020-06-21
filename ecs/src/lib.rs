/// What is Left:
/// InterEntity Communication
///Usage:
/// create entity with create_entity.
#[macro_export]
macro_rules! create_entity {
    ($($element: ident: $ty: ty),*) => {
    
        #[allow(dead_code)]
        pub struct Data{
            $($element: Option<$ty>),*
        }
        impl Data{
            #[allow(dead_code)]
            pub fn new($($element:fn()->Option<$ty>),*)->Self{
                return Self{
                    $(
                        $element: $element()
                    ),*
                }
            }
            $(
                #[allow(dead_code)]
                pub fn $element (&self) ->Option<$ty> {
                self.$element.clone()
            }
            )*
        }

        #[allow(dead_code)]
        trait BehaviorComponent{
            /// For now the user promises not to update id's other then the provided id
            fn update(&mut self,id:ID,entity_namager: &mut std::collections::HashMap<ID,Data>);
        }
        type ID = u32;
        #[allow(dead_code)]
        pub struct Entity{
            data: Data,
            behavior: Vec<Box<dyn BehaviorComponent>>,
        }
        //impl Entity{
        //    #[allow(dead_code)] 
        //    pub fn new($($element:fn()->Option<$ty>),*,behavior:Vec<Box<dyn BehaviorComponent>>)->Self{
        //        Self{
        //            data:Data::new($($element),*),
        //            behavior: behavior
        //        }
        //    }
        //    pub fn search(&mut self,manager: &EntityManager){
        //        for b in self.behavior.iter_mut(){
        //            b.search(&self.data,manager)
        //        }

        //    }
        //    pub fn update(&mut self){
        //        for b in self.behavior.iter(){
        //            b.update(&mut self.data)
        //        }
        //    }
        //}
        #[allow(dead_code)]
        pub struct EntityManager{
            data_elements: std::collections::HashMap<ID,Data>,
            behavior: std::collections::HashMap<ID,Vec<Box<dyn BehaviorComponent>>>
        }
        use rand::Rng;
        impl EntityManager{
            #[allow(dead_code)]
           pub fn new()->Self{
                EntityManager{
                    data_elements:std::collections::HashMap::new(),
                    behavior: std::collections::HashMap::new(),
                }
            }
            #[allow(dead_code)]
            fn get_entity(&self,id:ID)->Option<&Data>{
                self.data_elements.get(&id)
            }
            ///Function to get elements in Entity With id
            $(
                #[allow(dead_code)]
                pub fn $element(&self,id:ID)->Option<$ty>{
                let entity = self.get_entity(id);
                if entity.is_some(){
                    entity.unwrap().$element()
                }else{
                    None
                }
                
            })*
            
            #[allow(dead_code)]
            fn generate_id(&self)->ID{
                let mut rng = rand::thread_rng();
                let val = rng.gen();
                if self.data_elements.contains_key(&val){
                    return self.generate_id();
                }else{
                    return val
                }

            }
            #[allow(dead_code)]
            pub fn new_entity(&mut self,data:Data,behavior:Vec<Box<dyn BehaviorComponent>>)->ID{
                let id = self.generate_id();
                self.data_elements.insert(id,data);
                self.behavior.insert(id,behavior);
                return id;
            }
            #[allow(dead_code)]
            pub fn process(&mut self){
                for (id,b_vec) in self.behavior.iter_mut(){
                    for b in b_vec.iter_mut(){
                        b.update(id.clone(),&mut self.data_elements);
                    }
                }
            }
        }
    }
}
mod test{
    create_entity!(a:u32);
    mod t2{
      create_entity!(a:u32,b:f32);
    }
    #[test]
    fn create_system(){
        let s = EntityManager::new();
    }
    #[test]
    fn create_entity(){
        let mut s = EntityManager::new();
        s.new_entity(Data::new(||Some(0)),vec![]);

    }
}
