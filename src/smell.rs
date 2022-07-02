use bevy::prelude::*;

#[derive(Component)]
pub struct EmitsSmell {
    pub smell: f32,
}

#[derive(Component)]
pub struct CanSmell {
    pub smell_radius: f32,
    pub current_smell: f32,
    pub previous_smell: f32,
}

impl CanSmell {
    pub fn update(&mut self, smell: f32) {
        self.previous_smell = self.current_smell;
        self.current_smell = smell;
    }

    pub fn get_signal(&self) -> f32 {
        let strongest = self.current_smell.max(self.previous_smell);
        
        (self.current_smell - self.previous_smell) / strongest
    }
}

pub fn smell_system(
    emitters: Query<(&EmitsSmell, &Transform)>,
    mut receivers: Query<(&mut CanSmell, &Transform)>,
) {
    for (mut receiver_smell, receiver_transform) in receivers.iter_mut() {
        receiver_smell.current_smell = 0.;

        for (emitter_smell, emitter_transform) in emitters.iter() {
            let distance = receiver_transform.translation.distance(emitter_transform.translation);
            
            if distance < receiver_smell.smell_radius {
                let smell_strength = emitter_smell.smell * (1. - distance / receiver_smell.smell_radius);
            
                receiver_smell.current_smell += smell_strength;
            }
        }
    }
}