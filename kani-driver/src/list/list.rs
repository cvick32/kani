use anyhow::Error;
use kani_metadata::HarnessMetadata;

use crate::{args::list_args::CargoListArgs, project, session::KaniSession};

pub fn list_cargo(session: KaniSession, _args: CargoListArgs) -> Result<(), Error> {
    let project = project::cargo_project(&session, true)?;

    // if (--full was supplied), print out all of the Harness Metadata
    // if (--file was supplied), only output the results for a single file
    // if (--tree was supplied), print out the output as a file tree with leafs annotated by the number of proofs
    println!("Printing Kani Metadata:");
    for crate_metadata in project.metadata {

        let proof_harness_string = get_string_for_harnesses("Proof", crate_metadata.proof_harnesses);
        let test_harness_string = get_string_for_harnesses("Test", crate_metadata.test_harnesses);
        println!("Crate {}", crate_metadata.crate_name);
        println!("{}", proof_harness_string);
        println!("{}", test_harness_string);
    }
    Ok(())
}


fn get_string_for_harnesses(harness_type: &str, harnesses: Vec<HarnessMetadata>) -> String {
    let mut harnesses_string = format!(
        "\tTotal Number of {} Harnesses: {}",
        harness_type,
        harnesses.len()
    );
    for harness in harnesses {
        harnesses_string += format!("\n{}", get_harness_string(harness)).as_ref();
    }
    harnesses_string
}

fn get_harness_string(harness: HarnessMetadata) -> String {
    format!(
        "\t\t- {}: lines {} - {}",
        harness.original_file, harness.original_start_line, harness.original_end_line
    )
}
