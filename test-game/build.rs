use sp1_build::BuildArgs;

fn main() {
    // build login program
    sp1_build::build_program_with_args(
        "../zk-games-programs/login",
        BuildArgs {
            docker: true,
            output_directory: Some("../zk-games-programs/login/elf".to_string()),
            ..Default::default()
        },
    );

    sp1_build::build_program_with_args(
        "../zk-games-programs/rps-basic",
        BuildArgs {
            docker: true,
            output_directory: Some("../zk-games-programs/rps-basic/elf".to_string()),
            ..Default::default()
        },
    );
}
