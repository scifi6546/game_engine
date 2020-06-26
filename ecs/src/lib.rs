/// What is Left:
/// InterEntity Communication
///Usage:
/// create entity with create_entity.
#[macro_export]
macro_rules! create_entity {
    ($GlobalState: ty, $($element: ident: $ty: ty),*) => {
        #[allow(dead_code)]
        #[derive(Clone)]
        pub struct DataReturn<'a>{
            $(pub $element: Option<&'a $ty>),*
        }
        #[allow(dead_code)]
        #[derive(Clone)]
        pub struct Data{
            $(pub $element: Option<$ty>),*
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
        pub trait BehaviorComponent{
            /// For now the user promises not to update id's other then the provided id
            fn update(&mut self,state: &mut $GlobalState,data:&mut DataGetter);
        }
        type ID = u32;
        #[allow(dead_code)]
        pub struct Entity{
            data: Data,
            behavior: Vec<Box<dyn BehaviorComponent>>,
        }
        pub struct DataGetter<'a>{
            $(
                $element:&'a mut std::collections::HashMap<ID,$ty>,

            )*
            self_id: ID,
            entities: &'a std::collections::HashMap<ID,()>
        }
        impl<'a> DataGetter<'a>{
            pub fn get_self(&self)->DataReturn<'_>{
                self.get(self.self_id)
            }
            pub fn get(&self,id:ID)->DataReturn<'_>{
                DataReturn{
                    $(
                        $element: self.$element.get(&id),

                    )*
                }
            }
            pub fn self_id(&self)->ID{
                self.self_id

            }

            pub fn keys(&self)->std::collections::hash_map::Keys<'_, u32, ()>{
                self.entities.keys()

            }
            $(
                #[allow(dead_code)]
                pub fn $element (&mut self,e: $ty){
                    self.$element.insert(self.self_id,e);
                }
            )*

        }
        pub struct EntityIter<'a>{
            keys:std::collections::hash_map::Iter<'a, u32,()>,
            data:&'a EntityManager,
        }
        impl<'a> std::iter::Iterator for EntityIter<'a>{
            type Item = (ID,DataReturn<'a>);
            fn next(&mut self)->Option<Self::Item>{

                if let Some((id,_)) = self.keys.next(){
                    return Some((id.clone(),self.data.get_entity(id.clone())))

                }else{
                    return None

                }
                
            }

        }
        #[allow(dead_code)]
        pub struct EntityManager{
            $(
                #[allow(dead_code)]
                $element: std::collections::HashMap<ID,$ty>,
            )*
            entities:std::collections::HashMap<ID,()>,
            behavior: std::collections::HashMap<ID,Vec<Box<dyn BehaviorComponent>>>,
            global_state: $GlobalState,
        }
        use rand::Rng;
        impl EntityManager{
            #[allow(dead_code)]
           pub fn new(state:&$GlobalState)->Self{
                EntityManager{
                $(
                    $element: std::collections::HashMap::new(),

                )*
                    entities: std::collections::HashMap::new(),
                    behavior: std::collections::HashMap::new(),
                    global_state:state.clone(),

                }
            }
            #[allow(dead_code)]
            fn get_entity(&self,id:ID)->DataReturn<'_>{
                DataReturn{
                    $(
                        #[allow(dead_code)]
                        $element: self.$element.get(&id)

                    ),*
                }
            }
            pub fn iter(&self)->EntityIter<'_>{
                EntityIter{
                    keys:self.entities.iter(),
                    data: self,

                }

            }
            ///Function to get elements in Entity With id
            $(
                #[allow(dead_code)]
                pub fn $element(&self,id:ID)->Option<&'_ $ty>{
                    self.$element.get(&id)

                }

            )*

            #[allow(dead_code)]
            fn generate_id(&self)->ID{
                let mut rng = rand::thread_rng();
                let val = rng.gen();
                if self.entities.contains_key(&val){
                    return self.generate_id();
                }else{
                    return val
                }

            }
            #[allow(dead_code)]
            pub fn new_entity(&mut self,data:Data,behavior:Vec<Box<dyn BehaviorComponent>>)->ID{
                let id = self.generate_id();
                self.entities.insert(id,());
                $(
                    if let Some(d) = data.$element{
                        self.$element.insert(id,d);
                    }

                )*
                self.behavior.insert(id,behavior);
                return id;
            }
            #[allow(dead_code)]
            pub fn process(&mut self){
                for (id,b_vec) in self.behavior.iter_mut(){
                    for b in b_vec.iter_mut(){
                        b.update(&mut self.global_state,
                            &mut DataGetter{$($element:&mut self.$element,)*self_id:id.clone(),entities:&self.entities}
                            );
                    }
                }
            }
        }
    }
}
mod test {
    create_entity!(u32,a: u32, b: f32);
    #[test]
    fn create_system() {
        let s = EntityManager::new(&0);
    }
    #[test]
    fn create_entity() {
        let mut s = EntityManager::new(&0);
        s.new_entity(Data::new(|| Some(0), || None), vec![]);
    }
}
