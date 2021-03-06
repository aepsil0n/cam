#![allow(missing_docs)]
#![allow(dead_code)]

//! A first person camera.

use std::num::{Float, FloatMath};
use event::GenericEvent;
use input::{Button, Keyboard};
use input::keyboard;
use {
    input,
    Camera,
};

bitflags!(flags Keys: u8 {
    const MOVE_FORWARD  = 0b00000001,
    const MOVE_BACKWARD = 0b00000010,
    const STRAFE_LEFT   = 0b00000100,
    const STRAFE_RIGHT  = 0b00001000,
    const FLY_UP        = 0b00010000,
    const FLY_DOWN      = 0b00100000
})

/// First person camera settings.
pub struct FirstPersonSettings<T=f32> {
    /// Which button to press to move forward.
    pub move_forward_button: input::Button,
    /// Which button to press to move backward.
    pub move_backward_button: input::Button,
    /// Which button to press to strafe left.
    pub strafe_left_button: input::Button,
    /// Which button to press to strafe right.
    pub strafe_right_button: input::Button,
    /// Which button to press to fly up.
    pub fly_up_button: input::Button,
    /// Which button to press to fly down.
    pub fly_down_button: input::Button,
    /// Which button to press to move faster.
    pub move_faster_button: input::Button,
    /// The horizontal movement speed.
    ///
    /// This is measured in units per second.
    pub speed_horizontal: T,
    /// The vertical movement speed.
    ///
    /// This is measured in units per second.
    pub speed_vertical: T,
}

impl<T: Float> FirstPersonSettings<T> {
    /// Creates new first person camera settings with wasd defaults.
    pub fn keyboard_wasd() -> FirstPersonSettings<T> {
        FirstPersonSettings {
            move_forward_button: Keyboard(keyboard::W),
            move_backward_button: Keyboard(keyboard::S),
            strafe_left_button: Keyboard(keyboard::A),
            strafe_right_button: Keyboard(keyboard::D),
            fly_up_button: Keyboard(keyboard::Space),
            fly_down_button: Keyboard(keyboard::LShift),
            move_faster_button: Keyboard(keyboard::LCtrl),
            speed_horizontal: Float::one(),
            speed_vertical: Float::one(),
        }
    }

    /// Creates a new first person camera settings with esdf defaults.
    pub fn keyboard_esdf() -> FirstPersonSettings<T> {
        FirstPersonSettings {
            move_forward_button: Keyboard(keyboard::E),
            move_backward_button: Keyboard(keyboard::D),
            strafe_left_button: Keyboard(keyboard::S),
            strafe_right_button: Keyboard(keyboard::F),
            fly_up_button: Keyboard(keyboard::Space),
            fly_down_button: Keyboard(keyboard::Z),
            move_faster_button: Keyboard(keyboard::LShift),
            speed_horizontal: Float::one(),
            speed_vertical: Float::one(),
        }
    }
}

/// Models a flying first person camera.
pub struct FirstPerson<T=f32> {
    /// The first person camera settings.
    pub settings: FirstPersonSettings<T>,
    /// The yaw angle (in radians).
    pub yaw: T,
    /// The pitch angle (in radians).
    pub pitch: T,
    /// The direction we are heading.
    pub direction: [T, ..3],
    /// The position of the camera.
    pub position: [T, ..3],
    /// The velocity we are moving in the direction.
    pub velocity: T,
    /// The keys that are pressed.
    keys: Keys,
}

impl<T: Float + FromPrimitive + Copy + FloatMath> FirstPerson<T> {
    /// Creates a new first person camera.
    pub fn new(
        position: [T, ..3], 
        settings: FirstPersonSettings<T>
    ) -> FirstPerson<T> {
        let _0: T = Float::zero();
        FirstPerson {
            settings: settings,
            yaw: _0,
            pitch: _0,
            keys: Keys::empty(),
            direction: [_0, _0, _0],
            position: position,
            velocity: Float::one(),
        }
    }

    /// Computes camera.
    pub fn camera(&self, dt: f64) -> Camera<T> {
        let dt: T = FromPrimitive::from_f64(dt).unwrap();
        let dh = dt * self.velocity * self.settings.speed_horizontal;
        let [dx, dy, dz] = self.direction;
        let (s, c) = (self.yaw.sin(), self.yaw.cos());
        let mut camera = Camera::new(
            self.position[0] + (s * dx - c * dz) * dh,
            self.position[1] + dy * dt * self.settings.speed_vertical,
            self.position[2] + (s * dz + c * dx) * dh
        );
        camera.set_yaw_pitch(self.yaw, self.pitch);
        camera
    }

    /// Handles game event and updates camera.
    pub fn event<E: GenericEvent>(&mut self, e: &E) {
        use event::{ MouseRelativeEvent, PressEvent, ReleaseEvent, UpdateEvent };

        e.update(|args| {
            let cam = self.camera(args.dt);
            self.position = cam.position;
        });

        let &FirstPerson {
            ref mut yaw,
            ref mut pitch,
            ref mut keys,
            ref mut direction,
            ref mut velocity,
            ref settings,
            ..
        } = self;

        let pi: T = Float::pi();
        let sqrt2: T = Float::sqrt2();
        let _0: T = Float::zero();
        let _1: T = Float::one();
        let _2: T = FromPrimitive::from_int(2).unwrap();
        let _3: T = FromPrimitive::from_int(3).unwrap();
        let _4: T = FromPrimitive::from_int(4).unwrap();
        let _360: T = FromPrimitive::from_int(360).unwrap();
        e.mouse_relative(|dx, dy| {
            let dx: T = FromPrimitive::from_f64(dx).unwrap();
            let dy: T = FromPrimitive::from_f64(dy).unwrap();
            *yaw = (*yaw - dx / _360 * pi / _4) % (_2 * pi);
            *pitch = *pitch + dy / _360 * pi / _4;
            *pitch = (*pitch).min(pi / _2).max(-pi / _2);
        });
        e.press(|button| {
            let [dx, dy, dz] = *direction;
            let sgn = |x: T| if x == _0 { _0 } else { x.signum() };
            let set = |k, x: T, y: T, z: T| {
                let (x, z) = (sgn(x), sgn(z));
                let (x, z) = if x != _0 && z != _0 {
                    (x / sqrt2, z / sqrt2)
                } else {
                    (x, z)
                };
                *direction = [x, y, z];
                keys.insert(k);
            };
            match button {
                x if x == settings.move_forward_button => 
                    set(MOVE_FORWARD, -_1, dy, dz),
                x if x == settings.move_backward_button => 
                    set(MOVE_BACKWARD, _1, dy, dz),
                x if x == settings.strafe_left_button => 
                    set(STRAFE_LEFT, dx, dy, _1),
                x if x == settings.strafe_right_button => 
                    set(STRAFE_RIGHT, dx, dy, -_1),
                x if x == settings.fly_up_button => 
                    set(FLY_UP, dx, _1, dz),
                x if x == settings.fly_down_button => 
                    set(FLY_DOWN, dx, -_1, dz),
                x if x == settings.move_faster_button => *velocity = _2,
                _ => {}
            }
        });
        e.release(|button| {
            let [dx, dy, dz] = *direction;
            let sgn = |x: T| if x == _0 { _0 } else { x.signum() };
            let set = |x: T, y: T, z: T| {
                let (x, z) = (sgn(x), sgn(z));
                let (x, z) = if x != _0 && z != _0 {
                    (x / sqrt2, z / sqrt2)
                } else {
                    (x, z)
                };
                *direction = [x, y, z];
            };
            let release = |key, rev_key, rev_val| {
                keys.remove(key);
                if keys.contains(rev_key) { rev_val } else { _0 }
            };
            match button {
                x if x == settings.move_forward_button => 
                    set(release(MOVE_FORWARD, MOVE_BACKWARD, _1), dy, dz),
                x if x == settings.move_backward_button => 
                    set(release(MOVE_BACKWARD, MOVE_FORWARD, -_1), dy, dz),
                x if x == settings.strafe_left_button => 
                    set(dx, dy, release(STRAFE_LEFT, STRAFE_RIGHT, -_1)),
                x if x == settings.strafe_right_button => 
                    set(dx, dy, release(STRAFE_RIGHT, STRAFE_LEFT, _1)),
                x if x == settings.fly_up_button => 
                    set(dx, release(FLY_UP, FLY_DOWN, -_1), dz),
                x if x == settings.fly_down_button => 
                    set(dx, release(FLY_DOWN, FLY_UP, _1), dz),
                x if x == settings.move_faster_button => *velocity = _1,
                _ => {}
            }
        });
    }
}


