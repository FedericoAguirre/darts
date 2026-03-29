use serde::Deserialize;
use std::path::Path;

const BULLSEYE_RADIUS_SQ: f32 = 1.0; // 1² = 1
const INNER_RING_RADIUS_SQ: f32 = 25.0; // 5² = 25
const OUTER_RING_RADIUS_SQ: f32 = 100.0; // 10² = 100

#[inline]
pub fn calculate_score(x: f32, y: f32) -> Option<i8> {
    if x.is_nan() || y.is_nan() || x.is_infinite() || y.is_infinite() {
        return None;
    }
    let r_sq = x * x + y * y;
    if r_sq < BULLSEYE_RADIUS_SQ {
        Some(10)
    } else if r_sq < INNER_RING_RADIUS_SQ {
        Some(5)
    } else if r_sq < OUTER_RING_RADIUS_SQ {
        Some(1)
    } else {
        Some(0)
    }
}

#[derive(Debug, Deserialize)]
struct Coordinate {
    x: String,
    y: String,
}

pub fn process_csv(
    input_path: &str,
    output_path: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    // Determine output file name
    let output_path = match output_path {
        Some(path) => path.to_string(),
        None => {
            let path = Path::new(input_path);
            let stem = path.file_stem().unwrap_or_default();
            let parent = path.parent().unwrap_or_else(|| Path::new(""));
            parent
                .join(stem)
                .with_extension("score.csv")
                .to_string_lossy()
                .into_owned()
        }
    };

    // Create reader
    let mut rdr = csv::Reader::from_path(input_path)?;
    // Create writer
    let mut wtr = csv::Writer::from_path(&output_path)?;

    // Write header
    wtr.write_record(&["x", "y", "score"])?;

    // Process each record
    for result in rdr.records() {
        let record: csv::StringRecord = result?;
        let coord: Coordinate = record.deserialize(None)?;
        let x_parsed: Result<f32, _> = coord.x.parse();
        let y_parsed: Result<f32, _> = coord.y.parse();
        let score = match (x_parsed, y_parsed) {
            (Ok(x), Ok(y)) => match calculate_score(x, y) {
                Some(score) => score.to_string(),
                None => "ERROR".to_string(),
            },
            _ => "ERROR".to_string(),
        };
        wtr.write_record(&[coord.x, coord.y, score])?;
    }

    wtr.flush()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_score_10() {
        assert_eq!(calculate_score(0.0, 0.0), Some(10));
        assert_eq!(calculate_score(0.7, 0.7), Some(10));
        assert_eq!(calculate_score(-0.7, 0.7), Some(10));
        assert_eq!(calculate_score(0.7, -0.7), Some(10));
        assert_eq!(calculate_score(-0.7, -0.7), Some(10));
    }

    #[test]
    fn test_score_5() {
        assert_eq!(calculate_score(1.4, 1.4), Some(5));
        assert_eq!(calculate_score(-1.4, 1.4), Some(5));
        assert_eq!(calculate_score(1.4, -1.4), Some(5));
        assert_eq!(calculate_score(-1.4, -1.4), Some(5));
    }

    #[test]
    fn test_score_1() {
        assert_eq!(calculate_score(4.0, 4.0), Some(1));
        assert_eq!(calculate_score(-4.0, 4.0), Some(1));
        assert_eq!(calculate_score(4.0, -4.0), Some(1));
        assert_eq!(calculate_score(-4.0, -4.0), Some(1));
    }

    #[test]
    fn test_score_0() {
        assert_eq!(calculate_score(11.0, 11.0), Some(0));
        assert_eq!(calculate_score(-11.0, 11.0), Some(0));
        assert_eq!(calculate_score(11.0, -11.0), Some(0));
        assert_eq!(calculate_score(-11.0, -11.0), Some(0));
    }

    #[test]
    fn test_invalid_nan() {
        assert_eq!(calculate_score(f32::NAN, 0.0), None);
        assert_eq!(calculate_score(0.0, f32::NAN), None);
        assert_eq!(calculate_score(f32::NAN, f32::NAN), None);
    }

    #[test]
    fn test_invalid_infinity() {
        assert_eq!(calculate_score(f32::INFINITY, 0.0), None);
        assert_eq!(calculate_score(0.0, f32::NEG_INFINITY), None);
        assert_eq!(calculate_score(f32::INFINITY, f32::NEG_INFINITY), None);
    }

    #[test]
    fn test_csv_processing() -> Result<(), Box<dyn std::error::Error>> {
        // Create temporary input CSV
        let input_file = NamedTempFile::new()?;
        let input_path = input_file.path().to_str().unwrap();
        let mut wtr = csv::Writer::from_path(input_path)?;
        wtr.write_record(&["x", "y"])?;
        wtr.write_record(&["0.0", "0.0"])?; // score 10
        wtr.write_record(&["1.4", "1.4"])?; // score 5
        wtr.write_record(&["4.0", "4.0"])?; // score 1
        wtr.write_record(&["11.0", "11.0"])?; // score 0
        wtr.write_record(&["invalid", "0.0"])?; // error
        wtr.write_record(&["0.0", "NaN"])?; // error
        wtr.flush()?;

        // Create temporary output file path
        let output_file = NamedTempFile::new()?;
        let output_path = output_file.path().to_str().unwrap();

        // Process CSV
        process_csv(input_path, Some(output_path))?;

        // Read output CSV
        let mut rdr = csv::Reader::from_path(output_path)?;
        // Check header
        assert_eq!(rdr.headers()?, &["x", "y", "score"] as &[&str]);
        let records: Vec<csv::StringRecord> = rdr.records().collect::<Result<_, _>>()?;
        assert_eq!(records.len(), 6); // 6 data rows
                                      // Check rows
        assert_eq!(records[0], vec!["0.0", "0.0", "10"]);
        assert_eq!(records[1], vec!["1.4", "1.4", "5"]);
        assert_eq!(records[2], vec!["4.0", "4.0", "1"]);
        assert_eq!(records[3], vec!["11.0", "11.0", "0"]);
        assert_eq!(records[4], vec!["invalid", "0.0", "ERROR"]);
        assert_eq!(records[5], vec!["0.0", "NaN", "ERROR"]);
        Ok(())
    }

    #[test]
    fn test_csv_default_output_name() -> Result<(), Box<dyn std::error::Error>> {
        let input_file = NamedTempFile::new()?;
        let input_path = input_file.path().to_str().unwrap();
        let mut wtr = csv::Writer::from_path(input_path)?;
        wtr.write_record(&["x", "y"])?;
        wtr.write_record(&["1.0", "1.0"])?;
        wtr.flush()?;

        // No output path provided, should create stem.score.csv
        process_csv(input_path, None)?;
        // Compute expected output path
        let path = Path::new(input_path);
        let stem = path.file_stem().unwrap_or_default();
        let parent = path.parent().unwrap_or_else(|| Path::new(""));
        let expected_output = parent.join(stem).with_extension("score.csv");
        assert!(expected_output.exists());
        // Clean up
        std::fs::remove_file(expected_output)?;
        Ok(())
    }
}
