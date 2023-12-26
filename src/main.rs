use learn_wgpu::run;

fn main() {
    println!("Hello, world!");
    pollster::block_on(run());
}
