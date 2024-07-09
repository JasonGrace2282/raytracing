use raytrace::eventloop::run;

fn main() {
    pollster::block_on(run());
}
