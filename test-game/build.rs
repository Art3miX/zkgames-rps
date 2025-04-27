use sp1_build::BuildArgs;

fn main() {
    // build login program
    sp1_build::build_program_with_args(
        "../zk-games-programs/login",
        BuildArgs {
            docker: true,
            ..Default::default()
        },
    );
}
