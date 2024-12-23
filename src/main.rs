use std::convert::Infallible;
use std::path::PathBuf;

use fugue_mptp::sources::DirectorySource;
use fugue_mptp::{TaskProcessor, TaskSink, Uuid};

use idalib::idb::IDB;

pub struct IDAProcessor;

impl TaskProcessor for IDAProcessor {
    type TaskError = (PathBuf, String);
    type TaskInput = PathBuf;
    type TaskOutput = PathBuf;

    fn process_task(&mut self, _id: Uuid, input: PathBuf) -> Result<PathBuf, (PathBuf, String)> {
        match IDB::open_with(&input, true, true) {
            Ok(_) => Ok(input),
            Err(e) => Err((input, e.to_string()))
        }
    }
}

pub struct IDAProcessorSink;

impl TaskSink for IDAProcessorSink {
    type Error = Infallible;

    type TaskError = (PathBuf, String);
    type TaskOutput = PathBuf;

    fn process_task_result(
        &mut self,
        id: Uuid,
        result: Result<Self::TaskOutput, Self::TaskError>,
    ) -> Result<(), Self::Error> {
        match result {
            Ok(path) => { println!("{id}: {}: ok", path.display()) },
            Err((path, err)) => {
                println!("{id}: {}: {err}", path.display())
            }
        }
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut source = DirectorySource::new_with("dataset", |path| {
        matches!(path.extension(), Some(ext) if ext == "i64") || !{
            path.with_extension("i64").exists()
        }
    });

    let mut sink = IDAProcessorSink;
    let mut processor = IDAProcessor;

    fugue_mptp::run(&mut source, &mut processor, &mut sink)?;

    Ok(())
}
