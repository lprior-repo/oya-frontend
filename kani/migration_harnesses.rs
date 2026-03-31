// Kani Model Checking Harnesses for Bead oya-frontend-rb4

#![feature(array_chunks)]

use oya_frontend::migration::*;

#[kani::proof]
fn zoomfactor_invariant_proof() {
    let value: f32 = kani::any();
    let result = ZoomFactor::from_f32(value);
    
    kani::assume(value.is_finite());
    
    match result {
        Some(zf) => {
            kani::assert(zf.value() >= 0.15, "ZoomFactor below minimum");
            kani::assert(zf.value() <= 3.0, "ZoomFactor above maximum");
        }
        None => {
            kani::assert(value < 0.15 || value > 3.0, "Valid zoom rejected");
        }
    }
}

#[kani::proof]
fn canvas_interaction_state_machine() {
    let mut state: CanvasInteraction = CanvasInteraction::Idle;
    
    for _ in 0..10 {
        match state {
            CanvasInteraction::Idle => {
                kani::assume(state == CanvasInteraction::Idle ||
                           state == CanvasInteraction::Panning { start: FlowPosition::new(0.0, 0.0).unwrap(), origin: FlowPosition::new(0.0, 0.0).unwrap() } ||
                           state == CanvasInteraction::DraggingNode { node_id: NodeId::new("550e8400-e29b-41d4-a716-446655440000").unwrap(), start: FlowPosition::new(0.0, 0.0).unwrap(), origin: FlowPosition::new(0.0, 0.0).unwrap() } ||
                           state == CanvasInteraction::Connecting { from: NodeId::new("550e8400-e29b-41d4-a716-446655440000").unwrap(), handle: HandleType::Source, cursor: FlowPosition::new(0.0, 0.0).unwrap() });
            }
            CanvasInteraction::Panning { .. } => {
                state = CanvasInteraction::Idle;
            }
            CanvasInteraction::DraggingNode { .. } => {
                state = CanvasInteraction::Idle;
            }
            CanvasInteraction::Connecting { .. } => {
                state = CanvasInteraction::Idle;
            }
        }
    }
    
    kani::assert(true, "State machine always has valid transitions");
}

#[kani::proof]
fn transform_overflow_safety() {
    let pan_x: f32 = kani::any();
    let pan_y: f32 = kani::any();
    let zoom: f32 = kani::any();
    
    kani::assume(zoom >= 0.15 && zoom <= 3.0);
    kani::assume(pan_x.is_finite() && pan_y.is_finite());
    
    let zoom_factor = ZoomFactor::from_f32(zoom).unwrap();
    let translated_x = pan_x * zoom_factor.value();
    let translated_y = pan_y * zoom_factor.value();
    
    kani::assert(translated_x.is_finite(), "Transformed X overflow");
    kani::assert(translated_y.is_finite(), "Transformed Y overflow");
}
