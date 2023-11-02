use std::any::Any;



#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ComponentId(pub u64);

pub trait Component: Any + 'static {
    fn get_component_id(&self) -> ComponentId;
    fn as_any(&self) -> &dyn Any;
}

