# This file is automatically generated by pyo3_stub_gen
# ruff: noqa: E501, F401

import numpy
import numpy.typing
import typing
import datetime
from enum import Enum, auto

__version__: str

class CableDelaysApplied(Enum):
    r"""
    The type of cable delays applied to the data
    """

    NoCableDelaysApplied = auto()
    CableAndRecClock = auto()
    CableAndRecClockAndBeamformerDipoleDelays = auto()

class GeometricDelaysApplied(Enum):
    r"""
    The type of geometric delays applied to the data
    """

    No = auto()
    Zenith = auto()
    TilePointing = auto()
    AzElTracking = auto()

class MWAMode(Enum):
    r"""
    The MODE the system was in for this observation
    """

    No_Capture = auto()
    Burst_Vsib = auto()
    Sw_Cor_Vsib = auto()
    Hw_Cor_Pkts = auto()
    Rts_32t = auto()
    Hw_Lfiles = auto()
    Hw_Lfiles_Nomentok = auto()
    Sw_Cor_Vsib_Nomentok = auto()
    Burst_Vsib_Synced = auto()
    Burst_Vsib_Raw = auto()
    Lfiles_Client = auto()
    No_Capture_Burst = auto()
    Enter_Burst = auto()
    Enter_Channel = auto()
    Voltage_Raw = auto()
    Corr_Mode_Change = auto()
    Voltage_Start = auto()
    Voltage_Stop = auto()
    Voltage_Buffer = auto()
    Mwax_Correlator = auto()
    Mwax_Vcs = auto()
    Mwax_Buffer = auto()

class MWAVersion(Enum):
    r"""
    Enum for all of the known variants of file format based on Correlator version
    """

    CorrOldLegacy = auto()
    CorrLegacy = auto()
    CorrMWAXv2 = auto()
    VCSLegacyRecombined = auto()
    VCSMWAXv2 = auto()

class Pol(Enum):
    r"""
    Instrument polarisation.
    """

    X = auto()
    Y = auto()

class ReceiverType(Enum):
    r"""
    ReceiverType enum.
    """

    Unknown = auto()
    RRI = auto()
    NI = auto()
    Pseudo = auto()
    SHAO = auto()
    EDA2 = auto()

class VisPol(Enum):
    r"""
    Visibility polarisations
    """

    XX = auto()
    XY = auto()
    YX = auto()
    YY = auto()

class Rfinput:
    r"""
    Structure for storing MWA rf_chains (tile with polarisation) information from the metafits file
    """

    input: int
    ant: int
    tile_id: int
    tile_name: str
    pol: Pol
    electrical_length_m: float
    north_m: float
    east_m: float
    height_m: float
    vcs_order: int
    subfile_order: int
    flagged: bool
    digital_gains: list[float]
    dipole_gains: list[float]
    dipole_delays: list[int]
    rec_number: int
    rec_slot_number: int
    rec_type: ReceiverType
    flavour: str
    has_whitening_filter: bool
    calib_delay: typing.Optional[float]
    calib_gains: typing.Optional[list[float]]
    signal_chain_corrections_index: typing.Optional[int]

class Antenna:
    r"""
    Structure for storing MWA antennas (tiles without polarisation) information from the metafits file
    """

    ant: int
    tile_id: int
    tile_name: str
    rfinput_x: Rfinput
    rfinput_y: Rfinput
    electrical_length_m: float
    north_m: float
    east_m: float
    height_m: float

class Baseline:
    r"""
    This is a struct for our baselines, so callers know the antenna ordering
    """

    ant1_index: int
    ant2_index: int

class CoarseChannel:
    r"""
    This is a struct for coarse channels
    """

    corr_chan_number: int
    rec_chan_number: int
    gpubox_number: int
    chan_width_hz: int
    chan_start_hz: int
    chan_centre_hz: int
    chan_end_hz: int

class GpuBoxFile:
    r"""
    This represents one gpubox file
    """

    filename: str
    channel_identifier: int

class GpuBoxBatch:
    r"""
    This represents one group of gpubox files with the same "batch" identitifer.
    e.g. obsid_datetime_chan_batch
    """

    batch_number: int
    gpubox_files: list[GpuBoxFile]

class SignalChainCorrection:
    r"""
    Signal chain correction table
    """

    receiver_type: ReceiverType
    whitening_filter: bool
    corrections: list[float]

class TimeStep:
    r"""
    This is a struct for our timesteps
    NOTE: correlator timesteps use unix time, voltage timesteps use gpstime, but we convert the two depending on what we are given
    """

    unix_time_ms: int
    gps_time_ms: int

class MetafitsContext:
    r"""
    Metafits context. This represents the basic metadata for an MWA observation.
    """

    mwa_version: typing.Optional[MWAVersion]
    obs_id: int
    sched_start_gps_time_ms: int
    sched_end_gps_time_ms: int
    sched_start_unix_time_ms: int
    sched_end_unix_time_ms: int
    sched_start_utc: datetime.datetime
    sched_end_utc: datetime.datetime
    sched_start_mjd: float
    sched_end_mjd: float
    sched_duration_ms: int
    dut1: typing.Optional[float]
    ra_tile_pointing_degrees: float
    dec_tile_pointing_degrees: float
    ra_phase_center_degrees: typing.Optional[float]
    dec_phase_center_degrees: typing.Optional[float]
    az_deg: float
    alt_deg: float
    za_deg: float
    az_rad: float
    alt_rad: float
    za_rad: float
    sun_alt_deg: typing.Optional[float]
    sun_distance_deg: typing.Optional[float]
    moon_distance_deg: typing.Optional[float]
    jupiter_distance_deg: typing.Optional[float]
    lst_deg: float
    lst_rad: float
    hour_angle_string: str
    grid_name: str
    grid_number: int
    creator: str
    project_id: str
    obs_name: str
    mode: MWAMode
    geometric_delays_applied: GeometricDelaysApplied
    cable_delays_applied: CableDelaysApplied
    calibration_delays_and_gains_applied: bool
    corr_fine_chan_width_hz: int
    corr_int_time_ms: int
    corr_raw_scale_factor: float
    num_corr_fine_chans_per_coarse: int
    volt_fine_chan_width_hz: int
    num_volt_fine_chans_per_coarse: int
    receivers: list[int]
    num_receivers: int
    delays: list[int]
    num_delays: int
    calibrator: bool
    calibrator_source: str
    global_analogue_attenuation_db: float
    quack_time_duration_ms: int
    good_time_unix_ms: int
    good_time_gps_ms: int
    num_ants: int
    antennas: list[Antenna]
    num_rf_inputs: int
    rf_inputs: list[Rfinput]
    num_ant_pols: int
    num_metafits_timesteps: int
    metafits_timesteps: list[TimeStep]
    num_metafits_coarse_chans: int
    metafits_coarse_chans: list[CoarseChannel]
    num_metafits_fine_chan_freqs: int
    metafits_fine_chan_freqs_hz: list[float]
    obs_bandwidth_hz: int
    coarse_chan_width_hz: int
    centre_freq_hz: int
    num_baselines: int
    baselines: list[Baseline]
    num_visibility_pols: int
    metafits_filename: str
    oversampled: bool
    deripple_applied: bool
    deripple_param: str
    best_cal_fit_id: typing.Optional[int]
    best_cal_obs_id: typing.Optional[int]
    best_cal_code_ver: typing.Optional[str]
    best_cal_fit_timestamp: typing.Optional[str]
    best_cal_creator: typing.Optional[str]
    best_cal_fit_iters: typing.Optional[int]
    best_cal_fit_iter_limit: typing.Optional[int]
    signal_chain_corrections: typing.Optional[list[SignalChainCorrection]]
    num_signal_chain_corrections: int
    def __new__(cls, metafits_filename: str, mwa_version: typing.Optional[MWAVersion] = None) -> MetafitsContext: ...
    def __repr__(self) -> str: ...
    def __enter__(self) -> MetafitsContext: ...
    def __exit__(self, _exc_type: typing.Any, _exc_value: typing.Any, _traceback: typing.Any) -> None: ...

class CorrelatorContext:
    r"""
    This represents the basic metadata and methods for an MWA correlator observation.
    """

    metafits_context: MetafitsContext
    mwa_version: MWAVersion
    timesteps: list[TimeStep]
    num_timesteps: int
    coarse_chans: list[CoarseChannel]
    num_coarse_chans: int
    common_timestep_indices: list[int]
    num_common_timesteps: int
    common_coarse_chan_indices: list[int]
    num_common_coarse_chans: int
    common_start_unix_time_ms: int
    common_end_unix_time_ms: int
    common_start_gps_time_ms: int
    common_end_gps_time_ms: int
    common_duration_ms: int
    common_bandwidth_hz: int
    common_good_timestep_indices: list[int]
    num_common_good_timesteps: int
    common_good_coarse_chan_indices: list[int]
    num_common_good_coarse_chans: int
    common_good_start_unix_time_ms: int
    common_good_end_unix_time_ms: int
    common_good_start_gps_time_ms: int
    common_good_end_gps_time_ms: int
    common_good_duration_ms: int
    common_good_bandwidth_hz: int
    provided_timestep_indices: list[int]
    num_provided_timesteps: int
    provided_coarse_chan_indices: list[int]
    num_provided_coarse_chans: int
    num_timestep_coarse_chan_bytes: int
    num_timestep_coarse_chan_floats: int
    num_timestep_coarse_chan_weight_floats: int
    num_gpubox_files: int
    gpubox_batches: list[GpuBoxBatch]
    gpubox_time_map: dict[int, dict[int, tuple[int, int]]]
    def __new__(cls, metafits_filename: str, gpubox_filenames: list[str]) -> CorrelatorContext: ...
    def get_fine_chan_freqs_hz_array(self, corr_coarse_chan_indices: typing.Sequence[int]) -> list[float]:
        r"""
        For a given list of correlator coarse channel indices, return a list of the center frequencies for all the fine channels in the given coarse channels

        Args:
            corr_coarse_chan_indices (list[int]): a list containing correlator coarse channel indices for which you want fine channels for. Does not need to be contiguous.

        Returns:
            fine_chan_freqs_hz_array (list[float]): a vector of floats containing the centre sky frequencies of all the fine channels for the given coarse channels.
        """
        ...

    def read_by_baseline(
        self, corr_timestep_index: int, corr_coarse_chan_index: int
    ) -> numpy.typing.NDArray[numpy.float32]:
        r"""
        Read a single timestep for a single coarse channel. The output visibilities are in order: baseline,frequency,pol,r,i

        Args:
            corr_timestep_index (int): index within the CorrelatorContext timestep array for the desired timestep. This corresponds to the element within CorrelatorContext.timesteps.
            corr_coarse_chan_index (int): index within the CorrelatorContext coarse_chan array for the desired coarse channel. This corresponds to the element within CorrelatorContext.coarse_chans.

        Returns:
            data (numpy.typing.NDArray[numpy.float32]): 3 dimensional ndarray of 32 bit floats containing the data in [baseline],[frequency],[pol,r,i] order, if Ok.
        """
        ...

    def read_by_frequency(
        self, corr_timestep_index: int, corr_coarse_chan_index: int
    ) -> numpy.typing.NDArray[numpy.float32]:
        r"""
        Read a single timestep for a single coarse channel. The output visibilities are in order: frequency,baseline,pol,r,i

        Args:
            corr_timestep_index (int): index within the CorrelatorContext timestep array for the desired timestep. This corresponds to the element within CorrelatorContext.timesteps.
            corr_coarse_chan_index (int): index within the CorrelatorContext coarse_chan array for the desired coarse channel. This corresponds to the element within CorrelatorContext.coarse_chans.

        Returns:
            data (numpy.typing.NDArray[numpy.float32]): 3 dimensional ndarray of 32 bit floats containing the data in [frequency],[baseline],[pol,r,i] order, if Ok.
        """
        ...

    def read_weights_by_baseline(
        self, corr_timestep_index: int, corr_coarse_chan_index: int
    ) -> numpy.typing.NDArray[numpy.float32]:
        r"""
        Read weights for a single timestep for a single coarse channel. The output weights are in order: baseline,pol

        Args:
            corr_timestep_index (int): index within the CorrelatorContext timestep array for the desired timestep. This corresponds to the element within CorrelatorContext.timesteps.
            corr_coarse_chan_index (int): index within the CorrelatorContext coarse_chan array for the desired coarse channel. This corresponds to the element within CorrelatorContext.coarse_chans.

        Returns:
            data (numpy.typing.NDArray[numpy.float32]): A 2 dimensional ndarray of 32 bit floats containing the data in [baseline],[pol] order, if Ok.
        """
        ...

    def __repr__(self) -> str: ...
    def __enter__(self) -> CorrelatorContext: ...
    def __exit__(self, _exc_type: typing.Any, _exc_value: typing.Any, _traceback: typing.Any) -> None: ...

class VoltageFile:
    r"""
    This represents one voltage file
    """

    filename: str
    channel_identifier: int

class VoltageFileBatch:
    r"""
    This represents one group of voltage files with the same "batch" identitifer (gps time).
    e.g.
    MWA Legacy: obsid_gpstime_datetime_chan
    MWAX      : obsid_gpstime_datetime_chan
    """

    gps_time_seconds: int
    voltage_files: list[VoltageFile]

class VoltageContext:
    r"""
    This represents the basic metadata and methods for an MWA voltage capture system (VCS) observation.
    """

    metafits_context: MetafitsContext
    mwa_version: MWAVersion
    timesteps: list[TimeStep]
    num_timesteps: int
    timestep_duration_ms: int
    coarse_chans: list[CoarseChannel]
    num_coarse_chans: int
    common_timestep_indices: list[int]
    num_common_timesteps: int
    common_coarse_chan_indices: list[int]
    num_common_coarse_chans: int
    common_start_unix_time_ms: int
    common_end_unix_time_ms: int
    common_start_gps_time_ms: int
    common_end_gps_time_ms: int
    common_duration_ms: int
    common_bandwidth_hz: int
    common_good_timestep_indices: list[int]
    num_common_good_timesteps: int
    common_good_coarse_chan_indices: list[int]
    num_common_good_coarse_chans: int
    common_good_start_unix_time_ms: int
    common_good_end_unix_time_ms: int
    common_good_start_gps_time_ms: int
    common_good_end_gps_time_ms: int
    common_good_duration_ms: int
    common_good_bandwidth_hz: int
    provided_timestep_indices: list[int]
    num_provided_timesteps: int
    provided_coarse_chan_indices: list[int]
    num_provided_coarse_chans: int
    coarse_chan_width_hz: int
    fine_chan_width_hz: int
    num_fine_chans_per_coarse: int
    sample_size_bytes: int
    num_voltage_blocks_per_timestep: int
    num_voltage_blocks_per_second: int
    num_samples_per_voltage_block: int
    voltage_block_size_bytes: int
    delay_block_size_bytes: int
    data_file_header_size_bytes: int
    expected_voltage_data_file_size_bytes: int
    voltage_batches: list[VoltageFileBatch]
    def __new__(cls, metafits_filename: str, voltage_filenames: list[str]) -> VoltageContext: ...
    def get_fine_chan_freqs_hz_array(self, volt_coarse_chan_indices: typing.Sequence[int]) -> list[float]:
        r"""
        For a given list of voltage coarse channel indices, return a list of the center frequencies for all the fine channels in the given coarse channels.

        Args:
            volt_coarse_chan_indices (list[int]): a list containing correlator coarse channel indices for which you want fine channels for. Does not need to be contiguous.

        Returns:
            fine_chan_freqs_hz_array (list[float]): a vector of floats containing the centre sky frequencies of all the fine channels for the given coarse channels.
        """
        ...

    def read_file(self, volt_timestep_index: int, volt_coarse_chan_index: int) -> numpy.typing.NDArray[numpy.int8]:
        r"""
        Read a single timestep / coarse channel worth of data

        Args:
            volt_timestep_index (int): index within the timestep array for the desired timestep. This corresponds to the element within VoltageContext.timesteps. For mwa legacy each index represents 1 second increments, for mwax it is 8 second increments.
            volt_coarse_chan_index (int): index within the coarse_chan array for the desired coarse channel. This corresponds to the element within VoltageContext.coarse_chans.

        Returns:
            data (numpy.typing.NDArray[numpy.int8]): A 6 dimensional ndarray of signed bytes containing the data, if Ok.

        NOTE: The shape of the ndarray is different between LegacyVCS and MWAX VCS
        Legacy: [second],[time sample],[chan],[ant],[pol],[complexity]
                where complexity is a byte (first 4 bits for real, second 4 bits for imaginary) in 2's compliment
        MWAX  : [second],[voltage_block],[antenna],[pol],[sample],[r,i]
        """
        ...

    def read_second(
        self, gps_second_start: int, gps_second_count: int, volt_coarse_chan_index: int
    ) -> numpy.typing.NDArray[numpy.int8]:
        r"""
        Read a single or multiple seconds of data for a coarse channel

        Args:
            gps_second_start (int): GPS second within the observation to start returning data.
            gps_second_count (int): number of seconds of data to return.
            volt_coarse_chan_index (int): index within the coarse_chan array for the desired coarse channel. This corresponds to the element within VoltageContext.coarse_chans.

        Returns:
            data (numpy.typing.NDArray[numpy.int8]): A 6 dimensional ndarray of signed bytes containing the data, if Ok.

        NOTE: The shape is different between LegacyVCS and MWAX VCS
        Legacy: [second],[time sample],[chan],[ant],[pol],[complexity]
                where complexity is a byte (first 4 bits for real, second 4 bits for imaginary) in 2's compliment
        MWAX  : [second],[voltage_block],[antenna],[pol],[sample],[r,i]
        """
        ...

    def __repr__(self) -> str: ...
    def __enter__(self) -> VoltageContext: ...
    def __exit__(self, _exc_type: typing.Any, _exc_value: typing.Any, _traceback: typing.Any) -> None: ...

class GpuboxErrorBatchMissing(Exception): ...
class GpuboxErrorCorrVerMismatch(Exception): ...
class GpuboxErrorEmptyBTreeMap(Exception): ...
class GpuboxErrorFits(Exception): ...
class GpuboxErrorInvalidCoarseChanIndex(Exception): ...
class GpuboxErrorInvalidMwaVersion(Exception): ...
class GpuboxErrorInvalidTimeStepIndex(Exception): ...
class GpuboxErrorLegacyNaxis1Mismatch(Exception): ...
class GpuboxErrorLegacyNaxis2Mismatch(Exception): ...
class GpuboxErrorMissingObsid(Exception): ...
class GpuboxErrorMixture(Exception): ...
class GpuboxErrorMwaxCorrVerMismatch(Exception): ...
class GpuboxErrorMwaxCorrVerMissing(Exception): ...
class GpuboxErrorMwaxNaxis1Mismatch(Exception): ...
class GpuboxErrorMwaxNaxis2Mismatch(Exception): ...
class GpuboxErrorNoDataForTimeStepCoarseChannel(Exception): ...
class GpuboxErrorNoDataHDUsInGpuboxFile(Exception): ...
class GpuboxErrorNoGpuboxes(Exception): ...
class GpuboxErrorObsidMismatch(Exception): ...
class GpuboxErrorUnequalHduSizes(Exception): ...
class GpuboxErrorUnevenCountInBatches(Exception): ...
class GpuboxErrorUnrecognised(Exception): ...
class MwalibError(Exception): ...
class VoltageErro(Exception): ...
class VoltageErrorEmptyBTreeMap(Exception): ...
class VoltageErrorGpsTimeMissing(Exception): ...
class VoltageErrorInvalidBufferSize(Exception): ...
class VoltageErrorInvalidCoarseChanIndex(Exception): ...
class VoltageErrorInvalidGpsSecondCount(Exception): ...
class VoltageErrorInvalidGpsSecondStart(Exception): ...
class VoltageErrorInvalidMwaVersion(Exception): ...
class VoltageErrorInvalidTimeStepIndex(Exception): ...
class VoltageErrorInvalidVoltageFileSize(Exception): ...
class VoltageErrorMetafitsObsidMismatch(Exception): ...
class VoltageErrorMissingObsid(Exception): ...
class VoltageErrorMixture(Exception): ...
class VoltageErrorNoDataForTimeStepCoarseChannel(Exception): ...
class VoltageErrorNoVoltageFiles(Exception): ...
class VoltageErrorObsidMismatch(Exception): ...
class VoltageErrorUnequalFileSizes(Exception): ...
class VoltageErrorUnevenChannelsForGpsTime(Exception): ...
class VoltageErrorUnrecognised(Exception): ...
