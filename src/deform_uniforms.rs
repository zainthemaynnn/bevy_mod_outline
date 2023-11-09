use bevy::ecs::system::lifetimeless::{Read, SRes};
use bevy::ecs::system::SystemParamItem;
use bevy::prelude::*;
use bevy::render::extract_component::{ComponentUniforms, DynamicUniformIndex};
use bevy::render::render_phase::{
    PhaseItem, RenderCommand, RenderCommandResult, TrackedRenderPass,
};
use bevy::render::render_resource::ShaderType;
use bevy::render::render_resource::{BindGroup, BindGroupEntry};
use bevy::render::renderer::RenderDevice;
use bevy::render::Extract;

use crate::pipeline::OutlinePipeline;
use crate::ComputedOutline;

#[derive(Clone, Component, ShaderType)]
pub(crate) struct OutlineDeformUniform {
    #[align(4)]
    pub seed: f32,
}

#[derive(Resource)]
pub(crate) struct OutlineDeformBindGroup {
    pub bind_group: BindGroup,
}

#[allow(clippy::type_complexity)]
pub(crate) fn extract_outline_deform_uniforms(
    mut commands: Commands,
    query: Extract<Query<(Entity, &ComputedOutline)>>,
) {
    for (entity, computed) in query.iter() {
        if let ComputedOutline(Some(computed)) = computed {
            commands.get_or_spawn(entity).insert(OutlineDeformUniform {
                seed: computed.deform.value.seed,
            });
        }
    }
}

pub(crate) fn prepare_outline_deform_bind_group(
    mut commands: Commands,
    render_device: Res<RenderDevice>,
    outline_pipeline: Res<OutlinePipeline>,
    outline_deform_uniforms: Res<ComponentUniforms<OutlineDeformUniform>>,
) {
    if let Some(deform_binding) = outline_deform_uniforms.binding() {
        let bind_group = render_device.create_bind_group(
            "outline_deform_bind_group",
            &outline_pipeline.outline_deform_bind_group_layout,
            &[BindGroupEntry {
                binding: 0,
                resource: deform_binding.clone(),
            }],
        );
        commands.insert_resource(OutlineDeformBindGroup { bind_group });
    }
}

pub(crate) struct SetOutlineDeformBindGroup<const I: usize>();

impl<P: PhaseItem, const I: usize> RenderCommand<P> for SetOutlineDeformBindGroup<I> {
    type ViewWorldQuery = ();
    type ItemWorldQuery = Read<DynamicUniformIndex<OutlineDeformUniform>>;
    type Param = SRes<OutlineDeformBindGroup>;
    fn render<'w>(
        _item: &P,
        _view_data: (),
        entity_data: &DynamicUniformIndex<OutlineDeformUniform>,
        bind_group: SystemParamItem<'w, '_, Self::Param>,
        pass: &mut TrackedRenderPass<'w>,
    ) -> RenderCommandResult {
        pass.set_bind_group(
            I,
            &bind_group.into_inner().bind_group,
            &[entity_data.index()],
        );
        RenderCommandResult::Success
    }
}
