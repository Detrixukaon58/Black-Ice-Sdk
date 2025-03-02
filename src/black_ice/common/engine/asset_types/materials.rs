#![allow(unused)]

use std::{any::*, collections::HashMap, option};
use components::component_system::Constructor;
use engine::asset_mgr::AssetManager;
use parking_lot::*;
use shaderc::ShaderKind;
use crate::black_ice::common::{filesystem::files::*, engine::gamesys::*, *};
use std::sync::Arc;

use super::{super::super::engine::pipeline::RenderPipelineSystem, shader_asset::*, AssetResource};


// pub struct ParamDescriptor {
//     pub name: String,
//     pub value: Box<MaybeUninit<&'static dyn Any>>,
//     pub data_type: TypeId,
//     pub size: isize,
// }

// impl ParamDescriptor {
//     fn new<T: Any>(name: String, value: &'static T) -> ParamDescriptor{
//         return ParamDescriptor {name: name, value: Box::new(MaybeUninit::new(value.as_any().clone())), data_type: value.type_id(), size: size_of_val(&value) as isize};
//     }
    
//     fn new_uninit(name: String) -> ParamDescriptor {
//         return ParamDescriptor { name: name, value: Box::new(MaybeUninit::uninit()), data_type: TypeId::of::<&dyn Any>(), size: 0 };
//     }

//     fn set_value<T: Any>(&mut self, value: &'static T){
//         self.value = Box::new(MaybeUninit::new(value.as_any().clone()));
//         self.data_type = value.type_id();
//         self.size = size_of_val(&value) as isize;
//     }
// }

pub struct Material {
    
    pub shader: Shader,
    pub shader_descriptor: HashMap<String, (Arc<Mutex<ShaderDataType>>, ShaderDataHint)>,

}


impl Clone for Material {
    fn clone(&self) -> Self {
        let mut mat = Material::new();
        mat.shader = self.shader.clone();
        mat.shader_descriptor = HashMap::new();
        for param in self.shader_descriptor.keys() {
            let value = self.shader_descriptor.get(param).unwrap().clone();
            let data_type = value.0.lock();
            
            mat.shader_descriptor.insert(param.to_string(), (Arc::new(Mutex::new(data_type.clone())), value.1.clone()));
        }
        return mat;
    }
}

impl Base for Material{}

impl New<Material> for Material {
    fn new() -> Material {
        let mut shader: Shader = AssetManager::load_asset("ASSET:assets/shaders/slim-shadey.shad".to_string());
        return Material {shader: shader, shader_descriptor: HashMap::new() };
    }
}

impl Reflection for Material{
    fn register_reflect(&'static self) -> Ptr<Register<>> {
        let _register = Box::new(Register::new(Box::new(self)));
        
        

        return Ptr {b: _register};
    } 
}