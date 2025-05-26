use crate::lua_codec::LuaCodec;

pub struct FITU16 {
    codec: LuaCodec
}

impl FITU16 {
    pub fn new() -> Self {
        FITU16 { 
            codec: LuaCodec::new()
        }
    }

    pub fn update(&mut self) {
        
    }

    pub fn draw(&mut self) {
        self.codec.draw();
    }
}