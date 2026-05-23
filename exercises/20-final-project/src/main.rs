fn main() -> Result<(), final_project::TaskError> {
    let input = "\
Ship the Rust review,true
Record lesson notes,false
Refactor final project,false";

    let report = final_project::build_report(input)?;
    println!("{report}");
    Ok(())
}
