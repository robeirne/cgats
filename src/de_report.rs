use super::*;
use std::convert::TryFrom;
use deltae::DEMethod;
use statistical::{mean, standard_deviation};
use std::fmt;

#[derive(Debug)]
pub struct DeReport {
    de_cgats:  Cgats,
    de_method: DEMethod,
    overall:   DeSummary,
    best_90:   DeSummary,
    worst_10:  DeSummary,
}

impl DeReport {
    pub fn new(de_cgats: &Cgats) -> CgatsResult<DeReport> {
        let mut de_list =  DeList::try_from(de_cgats)?;
        let overall =  DeSummary::from(&de_list);

        let split =    DeList::split_90_10(&mut de_list)?;
        let best_90 =  DeSummary::from(&split[0]);
        let worst_10 = DeSummary::from(&split[1]);

        let de_method = de_list.de_method;

        Ok(
            DeReport {
                de_cgats: de_cgats.clone(),
                de_method,
                overall,
                best_90,
                worst_10,
            }
        )
    }
}

impl fmt::Display for DeReport {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Number of Samples: {}", self.overall.sample_count)?;
        writeln!(f, "DE Formula: {}\n", self.de_method)?;

        writeln!(f, "OVERALL - ({} colors)", self.overall.sample_count)?;
        writeln!(f, "{}\n", self.overall)?;

        writeln!(f, "BEST 90% - ({} colors)", self.best_90.sample_count)?;
        writeln!(f, "{}\n", self.best_90)?;

        writeln!(f, "WORST 10% - ({} colors)", self.worst_10.sample_count)?;
        writeln!(f, "{}", self.worst_10)?;

        Ok(())
    }
}

#[derive(Debug)]
struct DeSummary {
    sample_count: usize,
    mean:  Float,
    min:   Float,
    max:   Float,
    stdev: Float,
}

impl fmt::Display for DeSummary {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "\t{:>10}: {:0.4}",   "Average DE", self.mean)?;
        writeln!(f, "\t{:>10}: {:0.4}",   "Max DE",     self.max)?;
        writeln!(f, "\t{:>10}: {:0.4}",   "Min DE",     self.min)?;
        writeln!(f, "\t{:>10}: {:0.4}",   "StdDev DE", self.stdev)
    }
}

impl From<&DeList> for DeSummary {
    fn from(de_list: &DeList) -> DeSummary {

        DeSummary {
            sample_count: de_list.list.len(),
            mean:  de_list.mean(),
            min:   de_list.min(),
            max:   de_list.max(),
            stdev: de_list.stdev(),
        }
    }
}

#[derive(Debug)]
pub struct DeList {
    de_method: DEMethod,
    list: Vec<Float>,
}

impl DeList {
    fn mean(&self) -> Float {
        mean(&self.list) as Float
    }

    fn min(&self) -> Float {
        *self.list.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap()
    }

    fn max(&self) -> Float {
        *self.list.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap()
    }

    fn stdev(&self) -> Float {
        standard_deviation(&self.list.as_slice(), None)
    }

    fn split_90_10(&mut self) -> CgatsResult<[DeList; 2]> {
        let de_method = self.de_method;

        self.list.sort_by(|a, b| a.partial_cmp(b).unwrap());
        
        let split_index = ((self.list.len() as Float) * 0.9) as usize; 
        let (best_90, worst_10) = self.list.split_at(split_index);

        Ok([
            DeList {
                de_method,
                list: best_90.into_iter().map(|f| *f).collect(),
            },
            DeList {
                de_method,
                list: worst_10.into_iter().map(|f| *f).collect(),
            }
        ])
    }
}

impl std::convert::TryFrom<&Cgats> for DeList {
    type Error = CgatsError;
    fn try_from(cgats: &Cgats) -> Result<DeList, Self::Error> {

        let (method_index, de_method) = cgats.de_method()?;
        let list = cgats.data_map.values()
            .filter_map(|sample| sample.values.iter().nth(method_index))
            .filter_map(|cgv| cgv.float)
            .collect();
        
        Ok(
            DeList {
                de_method,
                list,
            }
        )
    }
}

#[test]
fn de_report() -> CgatsResult<()> {
    let cg0 = Cgats::from_file("test_files/colorburst2.lin")?;
    let cg1 = Cgats::from_file("test_files/colorburst3.lin")?;
    let cgd = cg0.deltae(cg1, DEMethod::DE2000)?;

    let report = DeReport::new(&cgd)?;

    println!("{}", report);

    Ok(())
}