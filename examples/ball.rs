extern crate cgmath;
extern crate glutin;
#[macro_use]
extern crate gfx;
extern crate gfx_window_glutin;
extern crate gfx_phase;
extern crate gfx_fur;


gfx_vertex!( Vertex {
    a_Pos@ pos: [f32; 3],
});

#[derive(Clone, Copy)]
struct ViewInfo(cgmath::Matrix4<f32>);

impl gfx_phase::ToDepth for ViewInfo {
    type Depth = f32;
    fn to_depth(&self) -> f32 {
        self.0[3][2] / self.0[3][3]
    }
}

impl gfx_fur::ViewInfo for ViewInfo {
    fn get_transform(&self) -> [[f32; 4]; 4] {
        use cgmath::FixedArray;
        self.0.into_fixed()
    }
}

struct Entity<R: gfx::Resources> {
    mesh: gfx::Mesh<R>,
    slice: gfx::Slice<R>,
    material: gfx_fur::Material<R>,
}

//----------------------------------------

pub struct App<R: gfx::Resources> {
    phase: gfx_fur::Phase<R, ViewInfo>,
    entities: Vec<Entity<R>>,
    proj_view: cgmath::Matrix4<f32>,
}

impl<R: gfx::Resources> App<R> {
    pub fn new<F: gfx::Factory<R>>(factory: &mut F, aspect: f32) -> App<R> {
        use std::marker::PhantomData;
        use cgmath::Matrix;
        use gfx::traits::{FactoryExt, ToIndexSlice};

        let vertex_data = [
            Vertex { pos: [-1f32, -1.0, -1.0] },
            Vertex { pos: [0f32, 2.0, -1.0] },
            Vertex { pos: [2f32, 0.0, -1.0] },
            Vertex { pos: [0f32, 0.0, 2.0] },
        ];
        let mesh = factory.create_mesh(&vertex_data);

        let index_data: &[u8] = &[
            0, 1, 2,
            0, 3, 1,
            1, 3, 2,
            2, 3, 0,
        ];
        let slice = index_data.to_slice(factory, gfx::PrimitiveType::TriangleList);

        let entities = vec![
            Entity {
                mesh: mesh.clone(),
                slice: slice.clone(),
                material: gfx_fur::Material {
                    something: PhantomData,
                },
            }
        ];

        let phase = gfx_fur::create(factory);

        let proj = cgmath::perspective(cgmath::deg(90.0f32), aspect, 1.0, 10.0);
        let view: cgmath::AffineMatrix3<f32> = cgmath::Transform::look_at(
            &cgmath::Point3::new(1.5f32, -5.0, 3.0),
            &cgmath::Point3::new(0f32, 0.0, 0.0),
            &cgmath::Vector3::unit_z(),
        );

        App {
            phase: phase,
            entities: entities,
            proj_view: proj.mul_m(&view.mat),
        }
    }

    pub fn render<S: gfx::Stream<R>>(&mut self, stream: &mut S) {
        use cgmath::Matrix;
        use gfx_phase::AbstractPhase;

        let clear_data = gfx::ClearData {
            color: [0.3, 0.3, 0.3, 1.0],
            depth: 1.0,
            stencil: 0,
        };
        stream.clear(clear_data);

        for ent in self.entities.iter() {
            use std::f32::consts::PI;
            let angle = 0.1 * PI * 2.0;
            let model = cgmath::Matrix4::from_translation(&cgmath::vec3(
                3.0 * angle.cos(), 0.0, 3.0 * angle.sin()
            ));
            let view_info = ViewInfo(self.proj_view.mul_m(&model));
            self.phase.enqueue(&ent.mesh, &ent.slice, &ent.material, &view_info).unwrap();
        }

        self.phase.flush(stream).unwrap();
    }
}


fn main() {
    use gfx::traits::Stream;

    let window = glutin::WindowBuilder::new()
        .with_title("Alpha: gfx_phase example".to_string())
        .with_vsync()
        .with_gl(glutin::GL_CORE)
        .build().unwrap();
    let (mut stream, mut device, mut factory) = gfx_window_glutin::init(window);

    let aspect = stream.get_aspect_ratio();
    let mut app = App::new(&mut factory, aspect);

    'main: loop {
        // quit when Esc is pressed.
        for event in stream.out.window.poll_events() {
            match event {
                glutin::Event::KeyboardInput(_, _, Some(glutin::VirtualKeyCode::Escape)) => break 'main,
                glutin::Event::Closed => break 'main,
                _ => {},
            }
        }

        app.render(&mut stream);

        stream.present(&mut device);
    }
}
