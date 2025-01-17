use crate::utils::{file_reader, file_writer};
use log::info;
use noodles::gff::{self, record::attributes::field::Value};
use std::{error::Error, time::Instant};

pub fn feature_select(
    gff: Option<String>,
    types: Option<String>,
    key: &str,
    name: &str,
    out: Option<String>,
) -> Result<(), Box<dyn Error>> {
    let start = Instant::now();
    let mut reader = file_reader(gff.as_ref()).map(gff::Reader::new)?;
    if let Some(file) = gff {
        info!("read file: {}", file);
    } else {
        info!("read from stdin");
    }
    let mut fo = file_writer(out.as_ref(), 6u32)?;
    if let Some(file) = out {
        info!("write to file: {}", file);
    } else {
        info!("write to stdin");
    }

    let key_wrap = Value::from(name);
    for record in reader.records().flatten() {
        let rec = record.attributes();
        if let Some(ref types) = types {
            if types == record.ty() {
                if let Some(key) = rec.get(key) {
                    if key.eq(&key_wrap) {
                        let row = format!(
                            "{}\t{}\t{}\t{}\t{}\t{}\n",
                            record.reference_sequence_name(),
                            record.ty(),
                            record.start(),
                            record.end(),
                            record.strand(),
                            key
                        );
                        fo.write_all(row.as_bytes())?;
                    }
                }
            }
        } else if let Some(key) = rec.get(key) {
            if key.eq(&key_wrap) {
                let row = format!(
                    "{}\t{}\t{}\t{}\t{}\t{}\n",
                    record.reference_sequence_name(),
                    record.ty(),
                    record.start(),
                    record.end(),
                    record.strand(),
                    key
                );
                fo.write_all(row.as_bytes())?;
            }
        }
    }
    fo.flush()?;

    info!("all done");
    info!("time elapsed is: {:?}", start.elapsed());
    Ok(())
}
