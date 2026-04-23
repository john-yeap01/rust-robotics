use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

const DEFAULT_TARGET_ANGLE_RADIANS: f32 = 1.4;
const DEFAULT_MAX_TORQUE: f32 = 5_000.0;
const DEFAULT_MOTOR_STIFFNESS: f32 = 200.0;
const DEFAULT_MOTOR_DAMPING: f32 = 24.0;

pub struct ControlPlugin;

#[derive(Component, Debug, Default)]
pub struct ActuatedJoint;

#[derive(Resource, Debug)]
pub struct JointTarget {
    pub angle_radians: f32,
    pub max_torque: f32,
}

impl Default for JointTarget {
    fn default() -> Self {
        Self {
            angle_radians: DEFAULT_TARGET_ANGLE_RADIANS,
            max_torque: DEFAULT_MAX_TORQUE,
        }
    }
}

impl Plugin for ControlPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<JointTarget>()
            .add_systems(Update, drive_actuated_joints);
    }
}

fn drive_actuated_joints(
    target: Res<JointTarget>,
    mut joints: Query<&mut ImpulseJoint, With<ActuatedJoint>>,
) {
    for mut joint in &mut joints {
        let Some(revolute) = joint.data.as_mut().as_revolute_mut() else {
            continue;
        };

        revolute
            .set_motor_model(MotorModel::AccelerationBased)
            .set_motor_position(
                target.angle_radians,
                DEFAULT_MOTOR_STIFFNESS,
                DEFAULT_MOTOR_DAMPING,
            )
            .set_motor_max_force(target.max_torque);
    }
}
