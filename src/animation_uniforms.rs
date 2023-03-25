use bevy::ecs::system::lifetimeless::{Read, SRes};
use bevy::ecs::system::SystemParamItem;
use bevy::prelude::*;
use bevy::render::extract_component::{ComponentUniforms, DynamicUniformIndex};
use bevy::render::render_phase::{
    PhaseItem, RenderCommand, RenderCommandResult, TrackedRenderPass,
};
use bevy::render::render_resource::ShaderType;
use bevy::render::render_resource::{BindGroup, BindGroupDescriptor, BindGroupEntry};
use bevy::render::renderer::RenderDevice;
use bevy::render::Extract;

use crate::OutlineAnimation;
use crate::pipeline::OutlinePipeline;

#[derive(Clone, Component, ShaderType)]
pub(crate) struct OutlineAnimationUniform {
    #[align(8)]
    pub time: f32,
    pub rate_millis: f32,
}

#[derive(Resource)]
pub(crate) struct OutlineAnimationBindGroup {
    pub bind_group: BindGroup,
}

#[allow(clippy::type_complexity)]
pub(crate) fn extract_outline_animation_uniforms(
    mut commands: Commands,
    query: Extract<Query<(Entity, &OutlineAnimation)>>,
    time: Res<Time>,
) {
    for (entity, animation) in query.iter() {
        commands
            .get_or_spawn(entity)
            .insert(OutlineAnimationUniform {
                time: time.elapsed().as_millis() as f32,
                rate_millis: animation.rate_millis,
            });
    }
}

pub(crate) fn queue_outline_animation_bind_group(
    mut commands: Commands,
    render_device: Res<RenderDevice>,
    outline_pipeline: Res<OutlinePipeline>,
    outline_animation_uniforms: Res<ComponentUniforms<OutlineAnimationUniform>>,
) {
    if let Some(animation_binding) = outline_animation_uniforms.binding() {
        let bind_group = render_device.create_bind_group(&BindGroupDescriptor {
            entries: &[BindGroupEntry {
                binding: 0,
                resource: animation_binding.clone(),
            }],
            label: Some("outline_animation_bind_group"),
            layout: &outline_pipeline.outline_animation_bind_group_layout,
        });
        commands.insert_resource(OutlineAnimationBindGroup { bind_group });
    }
}

pub(crate) struct SetOutlineAnimationBindGroup<const I: usize>();

impl<P: PhaseItem, const I: usize> RenderCommand<P> for SetOutlineAnimationBindGroup<I> {
    type ViewWorldQuery = ();
    type ItemWorldQuery = Read<DynamicUniformIndex<OutlineAnimationUniform>>;
    type Param = SRes<OutlineAnimationBindGroup>;
    fn render<'w>(
        _item: &P,
        _view_data: (),
        entity_data: &DynamicUniformIndex<OutlineAnimationUniform>,
        bind_group: SystemParamItem<'w, '_, Self::Param>,
        pass: &mut TrackedRenderPass<'w>,
    ) -> RenderCommandResult {
        pass.set_bind_group(I, &bind_group.into_inner().bind_group, &[entity_data.index()]);
        RenderCommandResult::Success
    }
}
