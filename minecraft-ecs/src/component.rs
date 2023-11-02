use std::any::Any;



#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ComponentId(pub u32);

pub trait Component: Any + 'static {
    fn get_component_id(&self) -> ComponentId;
    fn as_any(&self) -> &dyn Any;
}

