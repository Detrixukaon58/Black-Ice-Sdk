
use parking_lot::*;

use std::sync::Arc;

pub mod shader_asset;

pub enum InputData {
    OBJECT(String, InputData),
    ARRAY(Vec<InputData>),
    BYTEARRAY(i32),// This is a pointer to the loaded byte data so as to ensure that shared
    // references do not have multiple copies of the same data throughout memory
}

pub enum OutputData {
    BYTEARRAY(Vec<i8>),
    INT(i32),
    FLOAT(f32),
    STRING(String),
    NONE
}

pub trait AssetResource {

    pub fn load(&mut self); // loads the asset's data. This needs to be defined in order for the
    // asset manager to be able to process your custom asset resource

    pub fn update(&mut self) -> Result<OutputData, std::error::Error>{
        return Ok(OutputData::NONE);
    }// possible update function
    // for any asset that mey need to stream data instead of loading just the once

    pub fn unload(&mut self);// This must be done in order for the asset data that has been loaded
    // to be reset on the case of the asset no longer being used or for the application to be
    // closed. You must remember that data that has been previously loaded will be stored in a
    // shared memory in order to be memory efficient. So when loading and unloading, you must make
    // sure that you only drop any data that you have created in the load function and not data
    // that has been passed into the load function as a parameter

    
}
