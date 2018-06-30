use rand::RngCore;  // TODO: investigate why it dont compile without RngCore
use rand::rngs::OsRng;

use samp_sdk::amx::{AMX, AmxResult};
use samp_sdk::types::Cell;
use samp_sdk::consts::AMX_ERR_NONE;

define_native!(os_random);


pub struct MyPlugin {
    rng: OsRng
}

impl MyPlugin {
    pub fn load(&self) -> bool {
        return true;
    }

    pub fn unload(&self) {

    }

    pub fn amx_load(&mut self, amx: &mut AMX) -> Cell {
        let natives = natives!{
            "OSRandom" => os_random
        };

        match amx.register(&natives) {
            Ok(_) => log!("Natives are successful loaded"),
            Err(err) => log!("Whoops, there is an error {:?}", err),
        }

        AMX_ERR_NONE
    }

    pub fn amx_unload(&self, amx: &AMX) -> Cell {
        AMX_ERR_NONE
    }

    pub fn os_random(&mut self, _: &AMX) -> AmxResult<Cell> {
        Ok(self.rng.next_u32() as Cell)
    }
}

impl Default for MyPlugin {
    fn default() -> Self {
        MyPlugin {
            rng: OsRng::new().unwrap()
        }
    }
}
