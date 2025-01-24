use lunar_engine::{
    asset_managment::AssetStore,
    assets,
    components::{
        camera::{MainCamera, ProjectionType},
        mesh::Mesh,
        transform::Transform,
    },
    ecs::{Component, ComponentReference, EntityBuilder},
    math::Vec3,
    rendering::{self, extensions::Base},
};
use lunar_engine_derive::{self, dependencies};

#[derive(Debug)]
struct Spin {
    pub speed: f32,
    transform: Option<ComponentReference<Transform>>,
}

impl Spin {
    const fn new(speed: f32) -> Self {
        Self {
            speed,
            transform: None,
        }
    }
}

impl Component for Spin {
    #[dependencies(Transform)]
    fn mew() -> Self
    where
        Self: Sized,
    {
        Self {
            speed: 0.0,
            transform: None,
        }
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self as &dyn std::any::Any
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self as &mut dyn std::any::Any
    }

    fn set_self_reference(&mut self, reference: lunar_engine::ecs::SelfReferenceGuard) {
        self.transform = Some(reference.get_component::<Transform>().unwrap());
    }

    fn update(&mut self) {
        let dt = lunar_engine::delta_time();
        let mut t = self.transform.as_ref().unwrap().borrow_mut();
        let r = self.speed * dt;
        t.rotation.y += r;
    }
}

#[derive(Default)]
struct State {
    asset_store: AssetStore,
    world: lunar_engine::ecs::World,
    extension: Base,
}

fn init(state: &mut State) {
    let mesh = state
        .asset_store
        .register(assets::Mesh::new_from_static_obj(include_str!(
            "../assets/blahaj.obj"
        )));
    let texture = state
        .asset_store
        .register(assets::Texture::static_png(include_bytes!(
            "../assets/blahaj.png"
        )));

    let mat = state
        .asset_store
        .register::<assets::Material>(assets::materials::TextureUnlit::new(texture));

    state.world.add_entity(
        EntityBuilder::new()
            .create_component(|| Transform {
                scale: Vec3::new(1.5, 1.5, 1.5),
                ..Default::default()
            })
            .create_component(|| Spin::new(256.0))
            .create_component(|| Mesh::new(mesh, mat))
            .create()
            .unwrap(),
    );

    state.world.add_entity(
        EntityBuilder::new()
            .create_component(|| {
                Transform::new(Vec3::new(0.0, 0.0, -7.5), 0.0.into(), (1.0).into())
            })
            .create_component(|| {
                let mut c = MainCamera::mew();
                c.projection_type = ProjectionType::Perspective {
                    fov: std::f32::consts::FRAC_PI_3,
                };
                c.near = 0.1;
                c.far = 100.0;
                c
            })
            .create()
            .unwrap(),
    );
}

fn run(state: &mut State) {
    state.world.update();
    rendering::render(
        &state.world,
        &state.asset_store,
        &mut [&mut state.extension],
    );
}

fn close(_: &mut State) {}

fn main() {
    let state = lunar_engine::State::<State>::default();

    state.run(init, run, close);
}
