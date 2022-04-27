use glam::{Vec3, EulerRot, Quat};

use super::settings::*;


pub enum AXIS {
    X,
    Y,
    Z,
}

const VEC_ERROR : &str = "THE TWO ORIENTATION WORLD VECTORS NEED TO BE DIFFERENT";


const WORLD_SIDE_VECTOR: AXIS =
    match WORLD_FOWARD_VECTOR {
        AXIS::X => match WORLD_UP_VECTOR {
            AXIS::Y => AXIS::Z,
            AXIS::Z => AXIS::Y, _ => { panic!("{}", VEC_ERROR); },
        },

        AXIS::Y => match WORLD_UP_VECTOR {
            AXIS::Z => AXIS::X,
            AXIS::X => AXIS::Z, _ => { panic!("{}", VEC_ERROR); },
        },

        AXIS::Z => match WORLD_UP_VECTOR {
            AXIS::X => AXIS::Y,
            AXIS::Y => AXIS::X, _ => { panic!("{}", VEC_ERROR); },
        }
    };

const WORLD_EULERROT: EulerRot =
    match WORLD_UP_VECTOR {
        AXIS::X => match WORLD_SIDE_VECTOR {
            AXIS::Y => EulerRot::XYZ,
            AXIS::Z => EulerRot::XZY, _ => { panic!("{}", VEC_ERROR); },
        },

        AXIS::Y => match WORLD_SIDE_VECTOR {
            AXIS::Z => EulerRot::YZX,
            AXIS::X => EulerRot::YXZ, _ => { panic!("{}", VEC_ERROR); },
        },

        AXIS::Z => match WORLD_SIDE_VECTOR {
            AXIS::X => EulerRot::ZXY,
            AXIS::Y => EulerRot::ZYX, _ => { panic!("{}", VEC_ERROR); },
        }
    };


//rotation is in reverse order of effect at "from_euler", so "a" is the last rotation axis
pub fn get_quat(rotation: Vec3) -> Quat {

    let a = match WORLD_UP_VECTOR {
        AXIS::X => rotation.x,
        AXIS::Y => rotation.y,
        AXIS::Z => rotation.z,
    };

    let b = match WORLD_SIDE_VECTOR {
        AXIS::X => rotation.x,
        AXIS::Y => rotation.y,
        AXIS::Z => rotation.z,
    };

    let c = match WORLD_FOWARD_VECTOR {
        AXIS::X => rotation.x,
        AXIS::Y => rotation.y,
        AXIS::Z => rotation.z,
    };

    Quat::from_euler(WORLD_EULERROT, a, b, c)
}