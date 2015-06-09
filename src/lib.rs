#[macro_use]
extern crate gfx;
extern crate gfx_phase;

use std::marker::PhantomData;


gfx_parameters!( Params {
    //t_Diffuse@ texture: gfx::TextureParam<R>,
});

pub struct Material<R: gfx::Resources> {
    tex_color: gfx::handle::Texture<R>,
    tex_blade: gfx::handle::Texture<R>,
}

impl<R: gfx::Resources> gfx_phase::Material for Material<R> {}

pub trait ViewInfo: gfx_phase::ToDepth {
    //TODO
}

pub struct Technique<R: gfx::Resources> {
    program: gfx::handle::Program<R>,
    state: gfx::DrawState,
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
                _r: PhantomData,
            },
            &self.state,
            None,
        )
    }

    fn fix_params(&self, _mat: &Material<R>, _space: &V, _params: &mut Params<R>) {
    }
}
