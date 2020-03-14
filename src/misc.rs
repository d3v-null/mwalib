// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

/*!
General helper/utility methods
*/
extern crate tempdir;
use crate::antenna;
use fitsio::FitsFile;

/// Function to allow access to a temporary file. Temp directory and File is dropped once out of scope.
/// This is derived from fitsio crate.
///
/// # Arguments
///
/// * `filename` - string filename to use when creating a temp file
///
///
/// # Returns
///
/// * A temporary file which will be deleted (along with the temp directory created) once out of scope
///
pub fn with_temp_file<F>(filename: &str, callback: F)
where
    F: for<'a> Fn(&'a str),
{
    let tdir = tempdir::TempDir::new("fitsio-").unwrap();
    let tdir_path = tdir.path();
    let filename = tdir_path.join(filename);

    let filename_str = filename.to_str().expect("cannot create string filename");
    callback(filename_str);
}

/// Function to allow access to a temporary FITS file. Temp directory and File is dropped once out of scope.
/// This is derived from fitsio crate.
///
/// # Arguments
///
/// * `filename` - string filename to use when creating a temp FITS file
///
///
/// # Returns
///
/// * A temporary FITS file which will be deleted (along with the temp directory created) once out of scope
///
pub fn with_new_temp_fits_file<F>(filename: &str, callback: F)
where
    F: for<'a> Fn(&'a mut FitsFile),
{
    let tdir = tempdir::TempDir::new("fitsio-").unwrap();
    let tdir_path = tdir.path();
    let filename = tdir_path.join(filename);

    let filename_str = filename.to_str().expect("cannot create string filename");

    let mut fptr = FitsFile::create(filename_str)
        .open()
        .expect("Couldn't open tempfile");

    callback(&mut fptr);
}

/// Given the number of antennas, calculate the number of baselines (cross+autos)
///
/// # Arguments
///
/// * `antennas` - number of antennas in the array
///
///
/// # Returns
///
/// * total number of baselines (including autos)
///
pub fn get_baseline_count(antennas: usize) -> usize {
    antennas * (antennas + 1) / 2
}

/// Given a baseline index, return a tuple of (ant1,ant2) for a std right upper triangle e.g. (where N is num antennas)
/// Returns None if baseline was not found (your baseline is out of range)
/// 0,0
/// 0,1
/// ...
/// 0,N-1
/// 1,1
/// 1,2
/// 1,N-1
/// ...
/// 2,2
/// ...
/// N-1,N-1
///
/// # Arguments
///
/// * `baseline` - index of baseline.
///
/// * `num_antennas` - total number of antennas in the array.
///
///
/// # Returns
///
/// * An Option containing antenna1 index and antenna2 index if baseline exists, or None if doesn't exist.
///
pub fn get_antennas_from_baseline(baseline: usize, num_antennas: usize) -> Option<(usize, usize)> {
    let ant1 = (-0.5
        * ((4 * num_antennas * num_antennas + 4 * num_antennas - 8 * baseline + 1) as f32).sqrt()
        + num_antennas as f32
        + 1. / 2.) as usize;

    let ant2 = baseline - (ant1 * num_antennas - (ant1 * ant1 + ant1) / 2);

    if ant1 > num_antennas - 1 || ant2 > num_antennas - 1 {
        None
    } else {
        Some((ant1, ant2))
    }
}

/// Given two antenna indicies, return the baseline index.
///
/// # Arguments
///
/// * `antenna1` - index of antenna1
///
/// * `antenna2` - index of antenna2
///
/// * `num_antennas` - total number of antennas in the array.
///
///
/// # Returns
///
/// * An Option containing a baseline index if baseline exists, or None if doesn't exist.
///
pub fn get_baseline_from_antennas(
    antenna1: usize,
    antenna2: usize,
    num_antennas: usize,
) -> Option<usize> {
    let mut baseline_index = 0;
    for ant1 in 0..num_antennas {
        for ant2 in ant1..num_antennas {
            if ant1 == antenna1 && ant2 == antenna2 {
                return Some(baseline_index);
            }
            baseline_index += 1;
        }
    }

    // Baseline was not found at all
    None
}

/// Given two antenna names and the vector of Antenna structs from metafits, return the baseline index.
///
/// # Arguments
///
/// * `antenna1` - Tile name of antenna1
///
/// * `antenna2` - Tile name of antenna2
///
/// * `antennas` - A vector of mwalibAntenna structs.
///
///
/// # Returns
///
/// * An Option containing a baseline index if baseline exists, or None if doesn't exist.
///
pub fn get_baseline_from_antenna_names(
    antenna1_tile_name: String,
    antenna2_tile_name: String,
    antennas: &[antenna::mwalibAntenna],
) -> Option<usize> {
    let mut baseline_index = 0;

    let antenna1_index = antennas
        .iter()
        .position(|a| a.tile_name == antenna1_tile_name)
        .unwrap();
    let antenna2_index = antennas
        .iter()
        .position(|a| a.tile_name == antenna2_tile_name)
        .unwrap();

    for ant1 in 0..antennas.len() {
        for ant2 in ant1..antennas.len() {
            if ant1 == antenna1_index && ant2 == antenna2_index {
                return Some(baseline_index);
            }
            baseline_index += 1;
        }
    }

    // Baseline was not found at all
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::antenna::*;
    use crate::rfinput::*;

    #[test]
    fn test_get_baseline_count() {
        assert_eq!(3, get_baseline_count(2));
        assert_eq!(8256, get_baseline_count(128));
    }

    #[test]
    fn test_get_antennas_from_baseline() {
        assert_eq!(Some((0, 0)), get_antennas_from_baseline(0, 128));
        assert_eq!(Some((1, 1)), get_antennas_from_baseline(128, 128));
        assert_eq!(Some((127, 127)), get_antennas_from_baseline(8255, 128));
    }

    #[test]
    fn test_get_baseline_from_antennas() {
        assert_eq!(Some(0), get_baseline_from_antennas(0, 0, 128));
        assert_eq!(Some(128), get_baseline_from_antennas(1, 1, 128));
        assert_eq!(Some(8255), get_baseline_from_antennas(127, 127, 128));
    }

    #[test]
    fn test_get_baseline_from_antenna_names1() {
        // Create a small antenna vector
        let mut ants: Vec<mwalibAntenna> = Vec::new();

        // We need a dummy rf inputs
        let dummy_rf_input_x = mwalibRFInput {input: 0, 
            antenna: 0, 
            tile_id: 0, 
            tile_name: String::from("dummyX"), 
            pol: String::from("X"), 
            electrical_length_m: 0., 
            north_m: 0., east_m: 0., 
            height_m: 0., 
            vcs_order: 0, 
            subfile_order: 0, 
            flagged: false };
        
        let dummy_rf_input_y = mwalibRFInput {input: 1, 
            antenna: 0, 
            tile_id: 1, 
            tile_name: String::from("dummyY"), 
            pol: String::from("Y"), 
            electrical_length_m: 0., 
            north_m: 0., east_m: 0., 
            height_m: 0., 
            vcs_order: 0, 
            subfile_order: 1, 
            flagged: false };

        ants.push(mwalibAntenna {antenna: 101,
            tile_id: 101, tile_name: String::from("tile101"), x_pol: dummy_rf_input_x.clone(), y_pol: dummy_rf_input_y.clone() });
        
        ants.push(mwalibAntenna {antenna: 102,
            tile_id: 102, tile_name: String::from("tile102"), x_pol: dummy_rf_input_x.clone(), y_pol: dummy_rf_input_y.clone() });

        ants.push(mwalibAntenna {antenna: 103,
            tile_id: 103, tile_name: String::from("tile103"), x_pol: dummy_rf_input_x.clone(), y_pol: dummy_rf_input_y.clone() });
        
        ants.push(mwalibAntenna {antenna: 104,
            tile_id: 104, tile_name: String::from("tile104"), x_pol: dummy_rf_input_x.clone(), y_pol: dummy_rf_input_y.clone() });

        ants.push(mwalibAntenna {antenna: 105,
            tile_id: 105, tile_name: String::from("tile105"), x_pol: dummy_rf_input_x.clone(), y_pol: dummy_rf_input_y.clone() });
        
        ants.push(mwalibAntenna {antenna: 106,
            tile_id: 106, tile_name: String::from("tile106"), x_pol: dummy_rf_input_x.clone(), y_pol: dummy_rf_input_y.clone() });
        
        ants.push(mwalibAntenna {antenna: 107,
            tile_id: 107, tile_name: String::from("tile107"), x_pol: dummy_rf_input_x.clone(), y_pol: dummy_rf_input_y.clone() });
        
        ants.push(mwalibAntenna {antenna: 108,
            tile_id: 108, tile_name: String::from("tile108"), x_pol: dummy_rf_input_x, y_pol: dummy_rf_input_y });

        // Now do some tests!
        assert_eq!(Some(0), get_baseline_from_antenna_names(String::from("tile101"), String::from("tile101"), &ants), "Baseline from antenna names test 1 is wrong");
        assert_eq!(Some(1), get_baseline_from_antenna_names(String::from("tile101"), String::from("tile102"), &ants), "Baseline from antenna names test 2 is wrong");
        assert_eq!(Some(7), get_baseline_from_antenna_names(String::from("tile101"), String::from("tile108"), &ants), "Baseline from antenna names test 3 is wrong");
        assert_eq!(Some(8), get_baseline_from_antenna_names(String::from("tile102"), String::from("tile102"), &ants), "Baseline from antenna names test 4 is wrong");
        assert_eq!(Some(14), get_baseline_from_antenna_names(String::from("tile102"), String::from("tile108"), &ants), "Baseline from antenna names test 5 is wrong");
    }

    #[test]
    #[should_panic]
    fn test_get_baseline_from_antenna_names_ant1_not_valid() {
        // Create a small antenna vector
        let mut ants: Vec<mwalibAntenna> = Vec::new();

        // We need a dummy rf inputs
        let dummy_rf_input_x = mwalibRFInput {input: 0, 
            antenna: 0, 
            tile_id: 0, 
            tile_name: String::from("dummyX"), 
            pol: String::from("X"), 
            electrical_length_m: 0., 
            north_m: 0., east_m: 0., 
            height_m: 0., 
            vcs_order: 0, 
            subfile_order: 0, 
            flagged: false };
        
        let dummy_rf_input_y = mwalibRFInput {input: 1, 
            antenna: 0, 
            tile_id: 1, 
            tile_name: String::from("dummyY"), 
            pol: String::from("Y"), 
            electrical_length_m: 0., 
            north_m: 0., east_m: 0., 
            height_m: 0., 
            vcs_order: 0, 
            subfile_order: 1, 
            flagged: false };

        ants.push(mwalibAntenna {antenna: 101,
            tile_id: 101, tile_name: String::from("tile101"), x_pol: dummy_rf_input_x.clone(), y_pol: dummy_rf_input_y.clone() });
        
        ants.push(mwalibAntenna {antenna: 102,
            tile_id: 102, tile_name: String::from("tile102"), x_pol: dummy_rf_input_x, y_pol: dummy_rf_input_y });

        // Now do some tests!
        let _panic_result = get_baseline_from_antenna_names(String::from("tile110"), String::from("tile102"), &ants).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_get_baseline_from_antenna_names_ant2_not_valid() {
        // Create a small antenna vector
        let mut ants: Vec<mwalibAntenna> = Vec::new();

        // We need a dummy rf inputs
        let dummy_rf_input_x = mwalibRFInput {input: 0, 
            antenna: 0, 
            tile_id: 0, 
            tile_name: String::from("dummyX"), 
            pol: String::from("X"), 
            electrical_length_m: 0., 
            north_m: 0., east_m: 0., 
            height_m: 0., 
            vcs_order: 0, 
            subfile_order: 0, 
            flagged: false };
        
        let dummy_rf_input_y = mwalibRFInput {input: 1, 
            antenna: 0, 
            tile_id: 1, 
            tile_name: String::from("dummyY"), 
            pol: String::from("Y"), 
            electrical_length_m: 0., 
            north_m: 0., east_m: 0., 
            height_m: 0., 
            vcs_order: 0, 
            subfile_order: 1, 
            flagged: false };

        ants.push(mwalibAntenna {antenna: 101,
            tile_id: 101, tile_name: String::from("tile101"), x_pol: dummy_rf_input_x.clone(), y_pol: dummy_rf_input_y.clone() });
        
        ants.push(mwalibAntenna {antenna: 102,
            tile_id: 102, tile_name: String::from("tile102"), x_pol: dummy_rf_input_x, y_pol: dummy_rf_input_y });

        // Now do some tests!
        let _panic_result = get_baseline_from_antenna_names(String::from("tile101"), String::from("tile112"), &ants).unwrap();
    }
}
