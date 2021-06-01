// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

/*!
Unit tests for gpubox file metadata
*/
#[cfg(test)]
use super::*;
use crate::misc::test::*;
use fitsio::images::{ImageDescription, ImageType};
use std::time::SystemTime;

#[test]
fn test_determine_gpubox_batches_proper_format() {
    let files = vec![
        "1065880128_20131015134930_gpubox20_01.fits",
        "1065880128_20131015134930_gpubox01_00.fits",
        "1065880128_20131015134930_gpubox15_02.fits",
    ];
    let result = determine_gpubox_batches(&files);
    assert!(result.is_ok());
    let (temp_gpuboxes, corr_format) = result.unwrap();
    assert_eq!(corr_format, MWAVersion::CorrLegacy);

    let expected_gpuboxes = vec![
        TempGpuBoxFile {
            filename: "1065880128_20131015134930_gpubox01_00.fits",
            channel_identifier: 1,
            batch_number: 0,
        },
        TempGpuBoxFile {
            filename: "1065880128_20131015134930_gpubox20_01.fits",
            channel_identifier: 20,
            batch_number: 1,
        },
        TempGpuBoxFile {
            filename: "1065880128_20131015134930_gpubox15_02.fits",
            channel_identifier: 15,
            batch_number: 2,
        },
    ];

    assert_eq!(temp_gpuboxes, expected_gpuboxes);
}

#[test]
fn test_determine_gpubox_batches_proper_format2() {
    let files = vec![
        "/home/chj/1065880128_20131015134930_gpubox01_00.fits",
        "/home/gs/1065880128_20131015134930_gpubox20_01.fits",
        "/var/cache/1065880128_20131015134930_gpubox15_02.fits",
    ];
    let result = determine_gpubox_batches(&files);
    assert!(result.is_ok());
    let (gpubox_batches, corr_format) = result.unwrap();
    assert_eq!(corr_format, MWAVersion::CorrLegacy);
    let expected_batches = vec![
        TempGpuBoxFile {
            filename: "/home/chj/1065880128_20131015134930_gpubox01_00.fits",
            channel_identifier: 1,
            batch_number: 0,
        },
        TempGpuBoxFile {
            filename: "/home/gs/1065880128_20131015134930_gpubox20_01.fits",
            channel_identifier: 20,
            batch_number: 1,
        },
        TempGpuBoxFile {
            filename: "/var/cache/1065880128_20131015134930_gpubox15_02.fits",
            channel_identifier: 15,
            batch_number: 2,
        },
    ];

    assert_eq!(gpubox_batches, expected_batches);
}

#[test]
fn test_determine_gpubox_batches_proper_format3() {
    let files = vec![
        "/home/chj/1065880128_20131015134930_gpubox02_00.fits",
        "/home/chj/1065880128_20131015134930_gpubox01_00.fits",
        "/home/chj/1065880128_20131015134930_gpubox20_01.fits",
        "/home/chj/1065880128_20131015134930_gpubox19_01.fits",
        "/home/chj/1065880128_20131015134930_gpubox14_02.fits",
        "/home/chj/1065880128_20131015134930_gpubox15_02.fits",
    ];
    let result = determine_gpubox_batches(&files);
    assert!(result.is_ok());
    let (gpubox_batches, corr_format) = result.unwrap();
    assert_eq!(corr_format, MWAVersion::CorrLegacy);

    let expected_batches = vec![
        TempGpuBoxFile {
            filename: "/home/chj/1065880128_20131015134930_gpubox01_00.fits",
            channel_identifier: 1,
            batch_number: 0,
        },
        TempGpuBoxFile {
            filename: "/home/chj/1065880128_20131015134930_gpubox02_00.fits",
            channel_identifier: 2,
            batch_number: 0,
        },
        TempGpuBoxFile {
            filename: "/home/chj/1065880128_20131015134930_gpubox19_01.fits",
            channel_identifier: 19,
            batch_number: 1,
        },
        TempGpuBoxFile {
            filename: "/home/chj/1065880128_20131015134930_gpubox20_01.fits",
            channel_identifier: 20,
            batch_number: 1,
        },
        TempGpuBoxFile {
            filename: "/home/chj/1065880128_20131015134930_gpubox14_02.fits",
            channel_identifier: 14,
            batch_number: 2,
        },
        TempGpuBoxFile {
            filename: "/home/chj/1065880128_20131015134930_gpubox15_02.fits",
            channel_identifier: 15,
            batch_number: 2,
        },
    ];

    assert_eq!(gpubox_batches, expected_batches);
}

#[test]
fn test_determine_gpubox_batches_proper_format4() {
    let files = vec![
        "/home/chj/1065880128_20131015134929_gpubox02_00.fits",
        "/home/chj/1065880128_20131015134930_gpubox01_00.fits",
        "/home/chj/1065880128_20131015134929_gpubox20_01.fits",
        "/home/chj/1065880128_20131015134930_gpubox19_01.fits",
        "/home/chj/1065880128_20131015134931_gpubox14_02.fits",
        "/home/chj/1065880128_20131015134930_gpubox15_02.fits",
    ];
    let result = determine_gpubox_batches(&files);
    assert!(result.is_ok());
    let (gpubox_batches, corr_format) = result.unwrap();
    assert_eq!(corr_format, MWAVersion::CorrLegacy);

    let expected_batches = vec![
        TempGpuBoxFile {
            filename: "/home/chj/1065880128_20131015134930_gpubox01_00.fits",
            channel_identifier: 1,
            batch_number: 0,
        },
        TempGpuBoxFile {
            filename: "/home/chj/1065880128_20131015134929_gpubox02_00.fits",
            channel_identifier: 2,
            batch_number: 0,
        },
        TempGpuBoxFile {
            filename: "/home/chj/1065880128_20131015134930_gpubox19_01.fits",
            channel_identifier: 19,
            batch_number: 1,
        },
        TempGpuBoxFile {
            filename: "/home/chj/1065880128_20131015134929_gpubox20_01.fits",
            channel_identifier: 20,
            batch_number: 1,
        },
        TempGpuBoxFile {
            filename: "/home/chj/1065880128_20131015134931_gpubox14_02.fits",
            channel_identifier: 14,
            batch_number: 2,
        },
        TempGpuBoxFile {
            filename: "/home/chj/1065880128_20131015134930_gpubox15_02.fits",
            channel_identifier: 15,
            batch_number: 2,
        },
    ];

    assert_eq!(gpubox_batches, expected_batches);
}

#[test]
fn test_determine_gpubox_batches_invalid_filename() {
    let files = vec!["1065880128_20131015134930_gpubox0100.fits"];
    let result = determine_gpubox_batches(&files);
    assert!(result.is_err());
}

#[test]
fn test_determine_gpubox_batches_invalid_filename2() {
    let files = vec!["1065880128x_20131015134930_gpubox01_00.fits"];
    let result = determine_gpubox_batches(&files);
    assert!(result.is_err());
}

#[test]
fn test_determine_gpubox_batches_invalid_filename3() {
    let files = vec!["1065880128_920131015134930_gpubox01_00.fits"];
    let result = determine_gpubox_batches(&files);
    assert!(result.is_err());
}

#[test]
fn test_determine_gpubox_batches_valid() {
    // There are no gpubox files for batch "01".
    let files = vec![
        "1065880128_20131015134930_gpubox01_00.fits",
        "1065880128_20131015134930_gpubox15_02.fits",
    ];
    let result = determine_gpubox_batches(&files);
    assert!(result.is_ok());
}

#[test]
fn test_determine_gpubox_batches_valid2() {
    // There are not different numbers of gpubox files for batches "00" and "01" vs "02".
    let files = vec![
        "1065880128_20131015134930_gpubox01_00.fits",
        "1065880128_20131015134930_gpubox02_00.fits",
        "1065880128_20131015134930_gpubox01_01.fits",
        "1065880128_20131015134930_gpubox02_01.fits",
        "1065880128_20131015134930_gpubox15_02.fits",
    ];
    let result = determine_gpubox_batches(&files);
    assert!(result.is_ok());
}

#[test]
fn test_determine_gpubox_batches_old_format() {
    let files = vec![
        "1065880128_20131015134930_gpubox01.fits",
        "1065880128_20131015134930_gpubox20.fits",
        "1065880128_20131015134930_gpubox15.fits",
    ];
    let result = determine_gpubox_batches(&files);
    assert!(result.is_ok());
    let (gpubox_batches, corr_format) = result.unwrap();
    assert_eq!(corr_format, MWAVersion::CorrOldLegacy);

    let expected_batches = vec![
        TempGpuBoxFile {
            filename: "1065880128_20131015134930_gpubox01.fits",
            channel_identifier: 1,
            batch_number: 0,
        },
        TempGpuBoxFile {
            filename: "1065880128_20131015134930_gpubox15.fits",
            channel_identifier: 15,
            batch_number: 0,
        },
        TempGpuBoxFile {
            filename: "1065880128_20131015134930_gpubox20.fits",
            channel_identifier: 20,
            batch_number: 0,
        },
    ];

    assert_eq!(gpubox_batches, expected_batches);
}

#[test]
fn test_determine_gpubox_batches_new_format() {
    let files = vec![
        "1065880128_20131015134930_ch101_000.fits",
        "1065880128_20131015134930_ch102_000.fits",
        "1065880128_20131015135030_ch101_001.fits",
        "1065880128_20131015135030_ch102_001.fits",
    ];
    let result = determine_gpubox_batches(&files);
    assert!(result.is_ok());
    let (gpubox_batches, corr_format) = result.unwrap();
    assert_eq!(corr_format, MWAVersion::CorrMWAXv2);

    let expected_batches = vec![
        TempGpuBoxFile {
            filename: "1065880128_20131015134930_ch101_000.fits",
            channel_identifier: 101,
            batch_number: 0,
        },
        TempGpuBoxFile {
            filename: "1065880128_20131015134930_ch102_000.fits",
            channel_identifier: 102,
            batch_number: 0,
        },
        TempGpuBoxFile {
            filename: "1065880128_20131015135030_ch101_001.fits",
            channel_identifier: 101,
            batch_number: 1,
        },
        TempGpuBoxFile {
            filename: "1065880128_20131015135030_ch102_001.fits",
            channel_identifier: 102,
            batch_number: 1,
        },
    ];

    assert_eq!(gpubox_batches, expected_batches);
}

#[test]
fn test_determine_gpubox_batches_mix() {
    let files = vec![
        "1065880128_20131015134930_gpubox01.fits",
        "1065880128_20131015134930_gpubox15_01.fits",
    ];
    let result = determine_gpubox_batches(&files);
    assert!(result.is_err());
}

#[test]
fn test_determine_hdu_time_test1() {
    // with_temp_file creates a temp dir and temp file, then removes them once out of scope
    with_new_temp_fits_file("determine_hdu_time_test1.fits", |fptr| {
        let hdu = fptr.hdu(0).expect("Couldn't open HDU 0");

        // Write the TIME and MILLITIM keys. Key types must be i64 to get any
        // sort of sanity.
        hdu.write_key(fptr, "TIME", 1_434_494_061)
            .expect("Couldn't write key 'TIME'");
        hdu.write_key(fptr, "MILLITIM", 0)
            .expect("Couldn't write key 'MILLITIM'");

        let result = determine_hdu_time(fptr, &hdu);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 1_434_494_061_000);
    });
}

#[test]
fn test_determine_hdu_time_test2() {
    // with_temp_file creates a temp dir and temp file, then removes them once out of scope
    with_new_temp_fits_file("determine_hdu_time_test2.fits", |fptr| {
        let hdu = fptr.hdu(0).expect("Couldn't open HDU 0");

        hdu.write_key(fptr, "TIME", 1_381_844_923)
            .expect("Couldn't write key 'TIME'");
        hdu.write_key(fptr, "MILLITIM", 500)
            .expect("Couldn't write key 'MILLITIM'");

        let result = determine_hdu_time(fptr, &hdu);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 1_381_844_923_500);
    });
}

#[test]
fn test_determine_hdu_time_test3() {
    // Use the current UNIX time.
    let current = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Err(e) => panic!("Something is wrong with time on your system: {}", e),
        Ok(n) => n.as_secs(),
    };

    // with_temp_file creates a temp dir and temp file, then removes them once out of scope
    with_new_temp_fits_file("determine_hdu_time_test3.fits", |fptr| {
        let hdu = fptr.hdu(0).expect("Couldn't open HDU 0");

        hdu.write_key(fptr, "TIME", current)
            .expect("Couldn't write key 'TIME'");
        hdu.write_key(fptr, "MILLITIM", 500)
            .expect("Couldn't write key 'MILLITIM'");

        let result = determine_hdu_time(fptr, &hdu);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), current * 1000 + 500);
    });
}

#[test]
fn test_map_unix_times_to_hdus_test() {
    // with_temp_file creates a temp dir and temp file, then removes them once out of scope
    with_new_temp_fits_file("map_unix_times_to_hdus_test.fits", |fptr| {
        let times: Vec<(u64, u64)> =
            vec![(1_381_844_923, 500), (1_381_844_924, 0), (1_381_844_950, 0)];
        let mut expected = BTreeMap::new();
        let image_description = ImageDescription {
            data_type: ImageType::Float,
            dimensions: &[100, 100],
        };
        for (i, (time, millitime)) in times.iter().enumerate() {
            let hdu = fptr
                .create_image("EXTNAME".to_string(), &image_description)
                .expect("Couldn't create image");
            hdu.write_key(fptr, "TIME", *time)
                .expect("Couldn't write key 'TIME'");
            hdu.write_key(fptr, "MILLITIM", *millitime)
                .expect("Couldn't write key 'MILLITIM'");

            expected.insert(time * 1000 + millitime, i + 1);
        }

        let result = map_unix_times_to_hdus(fptr, MWAVersion::CorrLegacy);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected);
    });
}

#[test]
fn test_determine_common_times_test_many_timesteps() {
    // Create two files, with mostly overlapping times, but also a little
    // dangling at the start and end.
    let common_times: Vec<u64> = vec![
        1_381_844_923_500,
        1_381_844_924_000,
        1_381_844_924_500,
        1_381_844_925_000,
        1_381_844_925_500,
    ];
    let integration_time_ms = 500;

    let mut input = BTreeMap::new();
    let mut new_time_tree = BTreeMap::new();
    new_time_tree.insert(0, (0, 1));
    input.insert(1_381_844_923_000, new_time_tree);

    for (i, time) in common_times.iter().enumerate() {
        let mut new_time_tree = BTreeMap::new();
        // gpubox 0.
        new_time_tree.insert(0, (0, i + 2));
        // gpubox 1.
        new_time_tree.insert(1, (0, i + 1));
        input.insert(*time, new_time_tree);
    }

    let mut new_time_tree = BTreeMap::new();
    new_time_tree.insert(1, (0, common_times.len() + 1));
    input.insert(1_381_844_926_000, new_time_tree);

    let expected_start = *common_times.first().unwrap();
    let expected_end = *common_times.last().unwrap() + integration_time_ms;
    // Duration = common end - common start + integration time
    // == 1_381_844_925_500 - 1_381_844_923_500 + 500
    let expected_duration = 2500;

    let result = determine_common_obs_times_and_chans(&input, integration_time_ms, None);
    assert!(result.is_ok());
    let result = result.unwrap();
    assert_eq!(result.start_time_unix_ms, expected_start);
    assert_eq!(result.end_time_unix_ms, expected_end);
    assert_eq!(result.duration_ms, expected_duration);
    assert_eq!(result.coarse_chan_identifiers.len(), 2);
    assert_eq!(result.coarse_chan_identifiers[0], 0);
    assert_eq!(result.coarse_chan_identifiers[1], 1);
}

#[test]
fn test_determine_common_times_test_two_common_sets() {
    // This tests for the case where there are 2 common sets of timesteps but for different coarse channel sets
    // also with dangling ones at the start and end.
    // The point of this test is to show we return the FIRST common set and ignore the latter common set.
    // E.g.
    //      t 0 1 2 3 4 5 6
    // Chan 0 X X X X X X X
    // Chan 1   X X
    // Chan 2       X X
    let common_times: Vec<u64> = vec![
        1_381_844_923_500,
        1_381_844_924_000,
        1_381_844_924_500,
        1_381_844_925_000,
        1_381_844_925_500,
    ];
    let integration_time_ms = 500;

    let mut input = BTreeMap::new();
    let mut new_time_tree = BTreeMap::new();
    new_time_tree.insert(0, (0, 1));
    input.insert(1_381_844_923_000, new_time_tree);

    for (i, time) in common_times.iter().enumerate() {
        let mut new_time_tree = BTreeMap::new();

        match i {
            0 | 1 => {
                // gpubox 0.
                new_time_tree.insert(0, (0, i + 2));
                // gpubox 1.
                new_time_tree.insert(1, (0, i + 1));
                input.insert(*time, new_time_tree);
            }
            2 | 3 => {
                // gpubox 0.
                new_time_tree.insert(0, (0, i + 2));
                // gpubox 1.
                new_time_tree.insert(2, (0, i + 1));
                input.insert(*time, new_time_tree);
            }
            5 => {
                // gpubox 0.
                new_time_tree.insert(0, (0, i + 2));
                input.insert(*time, new_time_tree);
            }
            _ => {}
        }
    }

    let mut new_time_tree = BTreeMap::new();
    new_time_tree.insert(1, (0, common_times.len() + 1));
    input.insert(1_381_844_926_000, new_time_tree);

    let expected_start = 1_381_844_923_500;
    let expected_end = 1_381_844_924_000 + integration_time_ms;
    let expected_duration = 1000;

    let result = determine_common_obs_times_and_chans(&input, integration_time_ms, None);
    assert!(result.is_ok());
    let result = result.unwrap();
    assert_eq!(result.start_time_unix_ms, expected_start);
    assert_eq!(result.end_time_unix_ms, expected_end);
    assert_eq!(result.duration_ms, expected_duration);
    assert_eq!(result.coarse_chan_identifiers.len(), 2);
    assert_eq!(result.coarse_chan_identifiers[0], 0);
    assert_eq!(result.coarse_chan_identifiers[1], 1);
}

#[test]
fn test_determine_common_times_test_nothing_common() {
    // This tests for the case where there are no common sets of timesteps but for different coarse channel sets
    // also with dangling ones at the start and end.
    // E.g.
    //      t 0 1 2
    // Chan 0 X
    // Chan 1   X
    // Chan 2     X
    let integration_time_ms = 500;

    let mut input = BTreeMap::new();

    // First timestep, channel 0
    let mut new_time_tree = BTreeMap::new();
    new_time_tree.insert(0, (0, 1));
    input.insert(1_381_844_923_000, new_time_tree);

    // Second timestep, channel 1
    new_time_tree = BTreeMap::new();
    new_time_tree.insert(1, (0, 2));
    input.insert(1_381_844_923_500, new_time_tree);

    // Third timestep, channel 2
    new_time_tree = BTreeMap::new();
    new_time_tree.insert(2, (0, 3));
    input.insert(1_381_844_924_000, new_time_tree);

    let expected_start = 1_381_844_923_000;
    let expected_end = 1_381_844_923_000 + integration_time_ms;
    let expected_duration = 500;

    let result = determine_common_obs_times_and_chans(&input, integration_time_ms, None);
    assert!(result.is_ok());
    let result = result.unwrap();
    assert_eq!(result.start_time_unix_ms, expected_start);
    assert_eq!(result.end_time_unix_ms, expected_end);
    assert_eq!(result.duration_ms, expected_duration);
    assert_eq!(result.coarse_chan_identifiers.len(), 1);
    assert_eq!(result.coarse_chan_identifiers[0], 0);
}

#[test]
fn test_determine_common_times_test_one_timestep() {
    // Create two files, with 1 overlapping times, but also a little
    // dangling at the start and end.
    let common_times: Vec<u64> = vec![1_381_844_923_500];
    let integration_time_ms = 500;

    let mut input = BTreeMap::new();
    let mut new_time_tree = BTreeMap::new();
    new_time_tree.insert(0, (0, 1));
    // Add a dangling time before the common time
    input.insert(1_381_844_923_000, new_time_tree);

    for (i, time) in common_times.iter().enumerate() {
        let mut new_time_tree = BTreeMap::new();
        // gpubox 0.
        new_time_tree.insert(0, (0, i + 2));
        // gpubox 1.
        new_time_tree.insert(1, (0, i + 1));
        input.insert(*time, new_time_tree);
    }

    let mut new_time_tree = BTreeMap::new();
    new_time_tree.insert(1, (0, common_times.len() + 1));
    // Add a dangling time after the common time
    input.insert(1_381_844_924_000, new_time_tree);

    let expected_start = *common_times.first().unwrap();
    let expected_end = *common_times.last().unwrap() + integration_time_ms;
    // Duration = common end - common start + integration time
    // == 1_381_844_923_500 - 1_381_844_923_500 + 500
    let expected_duration = 500;

    let result = determine_common_obs_times_and_chans(&input, integration_time_ms, None);
    assert!(result.is_ok());
    let result = result.unwrap();
    assert_eq!(result.start_time_unix_ms, expected_start);
    assert_eq!(result.end_time_unix_ms, expected_end);
    assert_eq!(result.duration_ms, expected_duration);
}

#[test]
fn test_validate_gpubox_metadata_mwa_version() {
    // with_temp_file creates a temp dir and temp file, then removes them once out of scope
    with_new_temp_fits_file("test_validate_gpubox_metadata_mwa_version.fits", |fptr| {
        let hdu = fptr.hdu(0).expect("Couldn't open HDU 0");

        // This should succeed- LegacyOld should NOT have CORR_VER key
        assert!(validate_gpubox_metadata_mwa_version(
            fptr,
            &hdu,
            &String::from("test_file.fits"),
            MWAVersion::CorrOldLegacy,
        )
        .is_ok());

        // This should succeed- Legacy should NOT have CORR_VER key
        assert!(validate_gpubox_metadata_mwa_version(
            fptr,
            &hdu,
            &String::from("test_file.fits"),
            MWAVersion::CorrLegacy,
        )
        .is_ok());

        // This should fail- V2 needs CORR_VER key
        assert!(validate_gpubox_metadata_mwa_version(
            fptr,
            &hdu,
            &String::from("test_file.fits"),
            MWAVersion::CorrMWAXv2,
        )
        .is_err());

        // Now put in a corr version
        hdu.write_key(fptr, "CORR_VER", 2)
            .expect("Couldn't write key 'CORR_VER'");

        // This should succeed- V2 should have CORR_VER key
        assert!(validate_gpubox_metadata_mwa_version(
            fptr,
            &hdu,
            &String::from("test_file.fits"),
            MWAVersion::CorrMWAXv2,
        )
        .is_ok());

        // This should fail- OldLegacy should NOT have CORR_VER key
        assert!(validate_gpubox_metadata_mwa_version(
            fptr,
            &hdu,
            &String::from("test_file.fits"),
            MWAVersion::CorrOldLegacy,
        )
        .is_err());

        // This should fail- Legacy should NOT have CORR_VER key
        assert!(validate_gpubox_metadata_mwa_version(
            fptr,
            &hdu,
            &String::from("test_file.fits"),
            MWAVersion::CorrLegacy,
        )
        .is_err());
    });

    // with_temp_file creates a temp dir and temp file, then removes them once out of scope
    // This section tests CORR_VER where it is != 2
    with_new_temp_fits_file("test_validate_gpubox_metadata_mwa_version.fits", |fptr| {
        let hdu = fptr.hdu(0).expect("Couldn't open HDU 0");

        // This should not succeed- CORR_VER key if it exists should be 2
        // CORR_VER did not exist in OldLegacy or Legacy correlator
        // Now put in a corr version
        hdu.write_key(fptr, "CORR_VER", 1)
            .expect("Couldn't write key 'CORR_VER'");

        assert!(validate_gpubox_metadata_mwa_version(
            fptr,
            &hdu,
            &String::from("test_file.fits"),
            MWAVersion::CorrMWAXv2,
        )
        .is_err());
    });
}

#[test]
fn test_validate_gpubox_metadata_obsid() {
    // with_temp_file creates a temp dir and temp file, then removes them once out of scope
    with_new_temp_fits_file("test_validate_gpubox_metadata_mwa_version.fits", |fptr| {
        let hdu = fptr.hdu(0).expect("Couldn't open HDU 0");

        // OBSID is not there, this should be an error
        assert!(validate_gpubox_metadata_obs_id(
            fptr,
            &hdu,
            &String::from("test_file.fits"),
            1_234_567_890,
        )
        .is_err());

        // Now add the key
        hdu.write_key(fptr, "OBSID", 1_234_567_890)
            .expect("Couldn't write key 'OBSID'");

        // OBSID is there, but does not match metafits- this should be an error
        assert!(validate_gpubox_metadata_obs_id(
            fptr,
            &hdu,
            &String::from("test_file.fits"),
            2_345_678_901,
        )
        .is_err());

        // OBSID is there, and it matches
        assert!(validate_gpubox_metadata_obs_id(
            fptr,
            &hdu,
            &String::from("test_file.fits"),
            1_234_567_890,
        )
        .is_ok());
    });
}
