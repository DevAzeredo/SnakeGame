use bevy::prelude::*;
fn main() {
    App::new()
        .add_startup_system(setup_camera)
        .add_startup_system(spawn_snake)
        .add_system(snake_movement)
        .add_plugins(DefaultPlugins)
        .run();
}
const ARENA_WIDTH: u32 = 10;
const ARENA_HEIGHT: u32 = 10;
/* Setando a camera 2d */
/*Commands é usado para enfilerar comandos que vão mudar o mundo ou os recursos, nesse caso spawnando uma nova entidade que possui o componente camera2d */
fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}
const SNAKE_HEAD_COLOR: Color = Color::rgb(0.7, 0.7, 0.7);
#[derive(Component)]
struct SnakeHead;
/*Esse sistema spawna uma nova entidade, que terá todos os componentes que a entidade SpriteBundle e a entidade SnakeHead tem.
A entidade SpriteBundle possui dois componentes Sprite e Transform, alteramos apenas a cor do componente Sprite e a escala do componente Transform */
fn spawn_snake(mut commands: Commands) {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: SNAKE_HEAD_COLOR,
                ..default()
            },
            transform: Transform {
                scale: Vec3::new(10.0, 10.0, 10.0),
                ..default()
            },
            ..default()
        })
        .insert(SnakeHead);
}

/*Criando o sistema que irá mover a snake */
fn snake_movement(
    /*Acessa o recurso de entrada do keycode, o keycode contém as informações do teclado*/
    /*Res significa que está acessando a algum recurso */
    /*Input guarda as informações de uma entrada, por exemplo, input<teclado> guarda as informações do teclado,
     se está pressionado ou não, também poderia ser input<mouse> se está clicando ou não */
     /*KeyCode guarda as teclas do teclado, por exemplo abcde ou setinha pra cima, baixo esquerda direita, numeros 16584156 etc */
    keyboard_input: Res<Input<KeyCode>>,
    /*Busca no mundo o componente Transform que contém o componente SnakeHead, o componente SnakeHead é uma struct vazia, está sendo utilizado como forma de ID */
    mut head_positions: Query<&mut Transform, With<SnakeHead>>,
) {
    for mut transform in head_positions.iter_mut() {
        /*Verificar se está pressionando alguma setinha, se estiver acrescentar 2 pontos para o lado que a setinha aponta */
        if keyboard_input.pressed(KeyCode::Left) {
            transform.translation.x -= 2.;
        }
        if keyboard_input.pressed(KeyCode::Right) {
            transform.translation.x += 2.;
        }
        if keyboard_input.pressed(KeyCode::Down) {
            transform.translation.y -= 2.;
        }
        if keyboard_input.pressed(KeyCode::Up) {
            transform.translation.y += 2.;
        }
    }
}

#[derive(Component, Clone, Copy, PartialEq, Eq)]
struct Position {
    x: i32,
    y: i32,
}
#[derive(Component)]
struct Size {
    width: f32,
    height: f32,
}/*Tenho que entnder isso ainda */
impl Size {
    pub fn square(x: f32) -> Self {
        Self {
            width: x,
            height: x,
        }
    }
}
