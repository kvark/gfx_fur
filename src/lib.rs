#[macro_use]
extern crate gfx;
extern crate gfx_phase;

use std::marker::PhantomData;


gfx_parameters!( Params {
    u_Transform@ transform: [[f32; 4]; 4],
    u_Color@ color: [f32; 4],
});

pub struct Material<R: gfx::Resources> {
    pub something: PhantomData<R>,
    //tex_color: gfx::handle::Texture<R>,
    //tex_blade: gfx::handle::Texture<R>,
}

impl<R: gfx::Resources> gfx_phase::Material for Material<R> {}

pub trait ViewInfo: gfx_phase::ToDepth {
    fn get_transform(&self) -> [[f32; 4]; 4];
}

pub struct Technique<R: gfx::Resources> {
    program: gfx::handle::Program<R>,
    state: gfx::DrawState,
}

impl<R: gfx::Resources> Technique<R> {
    pub fn new<F: gfx::Factory<R>>(factory: &mut F) -> Technique<R> {
        use gfx::traits::FactoryExt;
        Technique {
            program: factory.link_program(
                include_bytes!("../gpu/shader.glslv"),
                include_bytes!("../gpu/shader.glslf"),
            ).unwrap(),
            state: gfx::DrawState::new().depth(gfx::state::Comparison::LessEqual, true),
        }
    }
}

impl<
    R: gfx::Resources,
    V: ViewInfo,
> gfx_phase::Technique<R, Material<R>, V> for Technique<R> {
    type Kernel = ();
    type Params = Params<R>;

    fn test(&self, _: &gfx::Mesh<R>, _: &Material<R>) -> Option<()> {
        Some(())
    }

    fn compile<'a>(&'a self, kernel: (), space: &V)
                   -> gfx_phase::TechResult<'a, R, Params<R>> {
        (   &self.program,
            Params {
                transform: space.get_transform(),
                color: [0.0; 4],
                _r: PhantomData,
            },
            &self.state,
            None,
        )
    }

    fn fix_params(&self, _mat: &Material<R>, _space: &V, _params: &mut Params<R>) {
    }
}

pub type Phase<R, V> = gfx_phase::CachedPhase<R, Material<R>, V, Technique<R>>;

pub fn create<
    R: gfx::Resources,
    F: gfx::Factory<R>,
    V: ViewInfo,
>(factory: &mut F) -> Phase<R, V> {
    gfx_phase::Phase::new("Fur", Technique::new(factory))
                     .with_cache()
}
