use ouroforge_core::{simulate_scene_physics_step, SceneDocument};
use serde_json::json;

#[test]
fn physics_step_resolves_contact_triggers_grounding_and_disabled_colliders() {
    let mut scene: SceneDocument = serde_json::from_value(json!({
        "schemaVersion": "1",
        "id": "physics-2d-core-step",
        "bounds": { "width": 96, "height": 64 },
        "collisionRules": {
            "version": "2",
            "defaultLayer": "world",
            "layers": [
                { "id": "world", "solid": true, "collidesWith": ["actors"] },
                { "id": "actors", "solid": true, "collidesWith": ["world", "triggers"] },
                { "id": "triggers", "solid": false, "triggerOnly": true, "collidesWith": ["actors"] }
            ]
        },
        "entities": [
            {
                "id": "player",
                "sprite": { "color": "#5eead4" },
                "components": {
                    "transform": { "x": 8, "y": 32 },
                    "velocity": { "x": 18, "y": 12 },
                    "size": { "width": 16, "height": 16 },
                    "controllable": true,
                    "collider": {
                        "shape": "aabb",
                        "body": "dynamic",
                        "size": { "width": 16, "height": 16 },
                        "collisionGroup": "actors",
                        "collisionMask": ["world", "triggers"]
                    }
                }
            },
            {
                "id": "floor",
                "sprite": { "color": "#475569" },
                "components": {
                    "transform": { "x": 0, "y": 48 },
                    "velocity": { "x": 0, "y": 0 },
                    "size": { "width": 96, "height": 16 },
                    "controllable": false,
                    "collider": {
                        "shape": "aabb",
                        "body": "static",
                        "size": { "width": 96, "height": 16 },
                        "collisionGroup": "world",
                        "collisionMask": ["actors"]
                    }
                }
            },
            {
                "id": "goal",
                "sprite": { "color": "#facc15" },
                "components": {
                    "transform": { "x": 24, "y": 32 },
                    "velocity": { "x": 0, "y": 0 },
                    "size": { "width": 16, "height": 16 },
                    "controllable": false,
                    "collider": {
                        "shape": "aabb",
                        "body": "static",
                        "size": { "width": 16, "height": 16 },
                        "sensor": true,
                        "collisionGroup": "triggers",
                        "collisionMask": ["actors"]
                    }
                }
            },
            {
                "id": "disabled-wall",
                "sprite": { "color": "#64748b" },
                "components": {
                    "transform": { "x": 28, "y": 32 },
                    "velocity": { "x": 0, "y": 0 },
                    "size": { "width": 16, "height": 16 },
                    "controllable": false,
                    "collider": {
                        "shape": "aabb",
                        "body": "static",
                        "disabled": true,
                        "size": { "width": 16, "height": 16 },
                        "collisionGroup": "world",
                        "collisionMask": ["actors"]
                    }
                }
            }
        ]
    }))
    .expect("scene parses");

    let evidence = simulate_scene_physics_step(&mut scene, 7).expect("physics step succeeds");
    let player = scene
        .entities
        .iter()
        .find(|entity| entity.id == "player")
        .expect("player exists");

    assert_eq!(player.components.transform.x, 26);
    assert_eq!(player.components.transform.y, 32);
    assert_eq!(player.components.velocity.y, 0);
    assert_eq!(evidence.schema_version, "physics-2d-step-evidence-v1");
    assert_eq!(evidence.contact_events.len(), 1);
    assert_eq!(evidence.contact_events[0].pair_id, "floor:player");
    assert_eq!(evidence.trigger_events.len(), 1);
    assert_eq!(evidence.trigger_events[0].pair_id, "goal:player");
    assert_eq!(evidence.grounded.get("player"), Some(&true));
    assert_eq!(evidence.blocked_movement.len(), 1);
    assert_eq!(evidence.blocked_movement[0].axis, "y");
}
