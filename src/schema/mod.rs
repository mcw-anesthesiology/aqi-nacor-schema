//! # Schema types
//! Enumerate valid types in AQI Schema

extern crate chrono;
extern crate regex;

use self::chrono::prelude::{NaiveDate, NaiveDateTime};
use self::regex::Regex;
use AQIError;

pub mod writer;

// FIXME: Normalize Options on 0-min Vecs

pub trait SchemaStringType {
    fn value(&self) -> &str;
}

pub trait SchemaIntType {
    fn value(&self) -> u64;
}

pub trait SchemaRegexInput: Sized {
    fn from_str(&str) -> Result<Self, AQIError>;
}

pub struct AnesthesiaRecordsType {
    pub record_header: RecordHeaderType,
    pub anesthesia_records: Vec<AnesthesiaRecordType>,
}

pub struct RecordHeaderType {
    pub practice_id: PracticeIdType,
    pub created_by: String,
    pub create_date: NaiveDateTime,
    pub email_set: EmailSetType,
    pub aqi_xml_version: AQIXMLVersionType,
    pub vendor_set: Option<VendorSetType>,
}

pub struct EmailSetType {
    pub email_notification_set: Vec<EmailNotificationSetType>,
}

pub struct EmailNotificationSetType {
    pub email_notification_first_name: String,
    pub email_notification_last_name: String,
    pub email_notification_address: EmailAddressType,
}

pub struct VendorSetType {
    pub vendor: Vec<Vendors>,
}

pub struct Vendors {
    pub vendor_id: Option<VendorIDType>,
    pub vendor_set_type: SetVendorSetType,
    pub vendor_name: String,
}

pub struct SetVendorSetType {
    pub vendor_type: Vec<TypeVendorType>,
}

pub struct AnesthesiaRecordType {
    pub demographic: DemographicType,
    pub procedure: ProcedureType,
    pub anesthesia_case: AnesthesiaCaseType,
    pub pre_op: PreOpType,
    pub intra_op: IntraOpType,
    pub post_op: PostOpType,
    pub timing_milestones: Option<TimingMilestonesSetType>,
    pub outcomes_events: Option<OutcomesEventsType>,
    pub anesthesia_details: Option<AnesthesiaDetailsType>,
}

pub struct DemographicType {
    pub patient_id: Option<String>,
    pub dob: Option<NaiveDate>,
    pub home_zip: Option<ZipCodeType>,
    pub home_state: Option<USStateCodeType>,
    pub home_city: Option<String>,
    pub race: Option<RaceCodeType>,
    pub patient_sex: PatientSexCodeType,
}

pub struct ProcedureType {
    pub procedure_id: Option<String>,
    pub facility_id: String,
    pub procedure_location: Option<ProcedureLocationType>,
    pub proc_start_time: Option<NaiveDateTime>,
    pub proc_end_time: Option<NaiveDateTime>,
    pub admission_status: Option<AdmissionStatusCodeType>,
    pub proc_status: ProcStatusCodeType,
    pub transfer_status: Option<TransferStatusCodeType>,
    pub admission_date: Option<NaiveDateTime>,
    pub procedure_notes: Option<String>,
    pub cpt_set: Option<CPTSetType>,
}

pub struct ProcedureLocationType {
    pub location_type: LocationTypeCodeType,
    pub location_details: String,
}

pub struct CPTSetType {
    pub cpt: Vec<CPTType>,
}

pub struct CPTType {
    pub cpt_rank: Option<String>,
    pub cpt_value: CPTValueType,
    pub cpt_modifier: Option<CPTModifierType>,
}

pub struct AnesthesiaCaseType {
    pub anesthesia_record_id: String,
    pub anesthesia_coverage: Option<CoverageCodeType>,
    pub anesthesia_staff_set: AnesthesiaStaffSetType,
    pub monitoring_set: Option<MonitoringSetType>,
    pub anesthesia_method_set: AnesthesiaMethodSetType,
    pub airway_management_set: Option<AirwayManagementSetType>,
    pub cpt_anes_set: Option<CPTAnesSetType>,
}

pub struct AnesthesiaStaffSetType {
    pub anesthesia_staff: Vec<AnesthesiaStaffType>,
}

pub struct AnesthesiaStaffType {
    pub tax_id: TaxIdType,
    pub npi: NPIType,
    pub staff_responsibility: Option<StaffResponsibilityCodeType>,
    pub provider_credentials: ProviderCredentialsCodeType,
    pub staff_sign_in: Option<NaiveDateTime>,
    pub staff_sign_out: Option<NaiveDateTime>,
    pub staff_notes: Option<String>,
}

pub struct MonitoringSetType {
    pub monitor: Vec<MonitorCodeType>,
}

pub struct AnesthesiaMethodSetType {
    pub anesthesia_method: Vec<AnesthesiaMethodType>,
}

pub struct AnesthesiaMethodType {
    pub anesthesia_category: AnesthesiaCategoryCodeType,
    pub anesthesia_subcategory: Option<AnesthesiaSubCategoryCodeType>,
    pub anesthesia_start_time: NaiveDateTime,
    pub anesthesia_end_time: NaiveDateTime,
    pub anesthesia_induction: Option<AnesthesiaInductionCodeType>,
    pub anesthesia_induction_start_time: Option<NaiveDateTime>,
    pub anesthesia_maintenance: Option<AnesthesiaMaintenanceCodeType>,
    pub anesthesia_notes: Option<String>,
}

pub struct AirwayManagementSetType {
    pub airway_management: Vec<AirwayManagementType>,
}

pub struct AirwayManagementType {
    pub airway_management_method: AirwayManagementMethodCodeType,
    pub airway_sub_management_method: Option<AirwayManagementSubMethodCodeType>,
}

pub struct CPTAnesSetType {
    pub cpt_anes: Vec<CPTAnesType>,
}

pub struct CPTAnesType {
    pub cpt_anes_value: CPTValueType,
    pub cpt_anes_modifier: Option<CPTModifierType>,
    pub cpt_anes_description: Option<String>,
}

pub struct PreOpType {
    pub age: u64,
    pub weight: Option<u64>,
    pub weight_in_kg: Option<u64>,
    pub height: Option<u64>,
    pub height_in_cm: Option<u64>,
    pub asa_class: ASAClassCodeType,
    pub pre_anesth_status: Option<PreAnesthStatusCodeType>,
    pub icd_set: Option<ICDSetType>,
    pub pre_lab_set: Option<PreLabDataSetType>,
}

pub struct ICDSetType {
    pub icd: Vec<ICDType>,
}

pub struct ICDType {
    pub icd_rank: Option<u64>,
    pub icd_value: ICDValueType,
    pub icd_version: ICDVersionType,
}

pub struct PreLabDataSetType {
    pub pre_lab_data: Vec<LabDataType>,
}

pub struct LabDataType {
    pub lab_name: LabDataNameCodeType,
    pub lab_category_name: Option<LabDataCategoryCodeType>,
    pub lab_unit: CommonUnit,
    pub lab_value: String,
    pub lab_value_text: Option<String>,
    pub lab_range_high: Option<String>,
    pub lab_range_low: Option<String>,
    pub lab_date_time: NaiveDateTime,
    pub lab_comments: Option<String>,
}

pub struct IntraOpType {
    pub medications_set: Option<MedicationsSetType>,
    pub monitoring_physiologic_set: Option<MonitoringPhysiologicSetType>,
    pub outputs_set: Option<OutputsSetType>,
}

pub struct MedicationsSetType {
    pub medication: Vec<MedicationType>,
}

pub struct MedicationType {
    pub medication_name: String,
    pub medication_type: Option<Vec<MedicationTypeCodeType>>,
    pub med_dose: Option<u64>,
    pub dose_units: Option<CommonUnit>,
    pub dose_start: Option<NaiveDateTime>,
    pub dose_end: Option<NaiveDateTime>,
    pub med_concentration: Option<u64>,
    pub med_concentration_unit: Option<CommonUnit>,
    pub medication_route: Option<Vec<RouteCodeType>>,
    pub mixture_medications: Option<Vec<MixtureMedicationType>>,
}

pub struct MixtureMedicationType {
    pub mixture_medication_name: String,
    pub mixture_medication_type: Option<Vec<MedicationTypeCodeType>>,
    pub mixture_med_dose: Option<u64>,
    pub mixture_dose_units: Option<CommonUnit>,
    pub mixture_med_concentration: Option<u64>,
    pub mixture_med_concentration_unit: Option<CommonUnit>,
}

pub struct MonitoringPhysiologicSetType {
    pub monitoring: Vec<MonitoringPhysiologicType>,
}

pub struct MonitoringPhysiologicType {
    pub monitoring_name: MonitoringNameCodeType,
    pub monitoring_time: Option<NaiveDateTime>,
    pub monitoring_units: Option<CommonUnit>,
    pub monitoring_value_numeric: Option<u64>,
    pub monitoring_value_text: Option<String>,
    pub monitoring_source: Option<MonitoringSourceCodeType>,
}

pub struct OutputsSetType {
    pub output_event: Vec<OutputEventType>,
}

pub struct OutputEventType {
    pub output_event_name: OutputCodeType,
    pub output_units: Option<CommonUnit>,
    pub output_value_numeric: Option<u64>,
    pub output_value_text: Option<String>,
    pub output_start_date_time: Option<NaiveDateTime>,
    pub output_end_date_time: Option<NaiveDateTime>,
}

pub struct PostOpType {
    pub post_op_disposition: Option<PostOpDispositionCodeType>,
    pub post_op_disp_date_time: Option<NaiveDateTime>,
    pub post_op_discharge: Option<PostOpDischargeCodeType>,
    pub post_op_discharge_date_time: Option<NaiveDateTime>,
    pub length_of_hospital_stay: Option<u64>,
    pub payment_method: Vec<PaymentMethodType>,
    pub post_op_lab_set: Option<PostOpLabSetType>,
    pub icd_set: Option<ICDSetType>,
}

pub struct PaymentMethodType {
    pub payment_code: PaymentMethodCodeType,
    pub payment_description: Option<PaymentMethodCodeType>,
    pub payment_description_sec: Option<PaymentMethodCodeType>,
    pub payment_description_th: Option<PaymentMethodCodeType>,
}

pub struct PostOpLabSetType {
    pub post_lab_data: Vec<LabDataType>, // minOccurs="1"
}

pub struct TimingMilestonesSetType {
    pub timing_milestone: Vec<TimingMilestoneType>,
}

pub struct TimingMilestoneType {
    pub tm_type: TimingMilestoneCodeType,
    pub tm_start_time: NaiveDateTime,
    pub tm_end_time: Option<NaiveDateTime>,
}

pub struct OutcomesEventsType {
    pub ic_event_set: Option<ICEventSetType>,
    pub outcome_set: Option<OutcomeSetSetType>,
    pub qcdr_set: Option<QCDRSetTypeSet>,
}

pub struct ICEventSetType {
    pub ic_event: Vec<ICEventType>,
}

pub struct ICEventType {
    pub ic_event_time_date: Option<NaiveDateTime>,
    pub ic_category: Option<ICCategoryCodeType>,
    pub ic_severity: Option<String>,
    pub ic_name: Option<String>,
    pub ic_description: Option<String>,
    pub ic_value: Option<String>,
    pub ic_notes: Option<String>,
}

pub struct OutcomeSetSetType {
    pub outcome: Vec<OutcomeCodeType>,
}

pub struct OutcomeCodeType {
    pub outcome_id: OutcomeIDType,
    pub outcome_occurred: bool,
    pub outcome_time_stamp: Option<NaiveDateTime>,
    pub outcome_severity: Option<OutcomeSeverityCodeType>,
    pub outcome_time_frame: Option<OutcomeTimeFrameCodeType>,
}

pub struct QCDRSetTypeSet {
    pub qcdr: Vec<QCDRSetType>,
}

pub struct QCDRSetType {
    pub qcdr_measure: QCDRMeasureType,
    pub qcdr_code_value: QCDRCodeValueType,
    pub qcdr_modifier: Option<QCDRModifierType>,
}

pub struct AnesthesiaDetailsType {
    pub intake_output_set: Option<IntakeOutputSetType>,
    pub intubation_details: Option<IntubationDetailsType>,
    pub anesthesia_details_set: Option<AnesthesiaDetailsSetType>,
    pub medications_total_set: Option<MedicationsTotalSetType>,
}

pub struct IntakeOutputSetType {
    pub intake_output_total: Vec<IntakeOutputTotalType>,
}

pub struct IntakeOutputTotalType {
    pub intake_output_direction: Option<IntakeOutputDirectionCodeType>,
    pub input_output_name: Option<OutputCodeType>,
    pub output_units: Option<CommonUnit>,
    pub input_output_total: Option<u64>,
    pub input_output_route: Option<RouteCodeType>,
}

pub struct IntubationDetailsType {
    pub intubation_approach: Option<IntubationApproachCodeType>,
    pub intubation_attempts: Option<u64>,
    pub tube_size: Option<u64>,
    pub tube_type: Option<String>,
    pub intubation_details_properties_set: Option<IntubationDetailsPropertiesType>,
}

pub struct IntubationDetailsPropertiesType {
    pub intubation_details_properties: Vec<IntubationDetailsPropertyType>,
}

pub struct IntubationDetailsPropertyType {
    pub intubation_property: String,
    pub intubation_details_property_value: Option<String>,
}

pub struct AnesthesiaDetailsSetType {
    pub anesthesia_details_data: Vec<AnesthesiaDetailsDataType>,
}

pub struct AnesthesiaDetailsDataType {
    pub anesthesia_details_category: AnesthesiaDetailsCategoryCodeType,
    pub anesthesia_details_value: Option<String>,
}

pub struct MedicationsTotalSetType {
    pub medications_totals: Vec<MedicationType>,
}

enum_map! {
    USStateCodeType: SchemaStringType; value {
        AL => "AL",
        AK => "AK",
        AS => "AS",
        AZ => "AZ",
        AR => "AR",
        CA => "CA",
        CO => "CO",
        CT => "CT",
        DE => "DE",
        DC => "DC",
        FM => "FM",
        FL => "FL",
        GA => "GA",
        GU => "GU",
        HI => "HI",
        ID => "ID",
        IL => "IL",
        IN => "IN",
        IA => "IA",
        KS => "KS",
        KY => "KY",
        LA => "LA",
        ME => "ME",
        MH => "MH",
        MD => "MD",
        MA => "MA",
        MI => "MI",
        MN => "MN",
        MS => "MS",
        MO => "MO",
        MT => "MT",
        NE => "NE",
        NV => "NV",
        NH => "NH",
        NJ => "NJ",
        NM => "NM",
        NY => "NY",
        NC => "NC",
        ND => "ND",
        MP => "MP",
        OH => "OH",
        OK => "OK",
        OR => "OR",
        PW => "PW",
        PA => "PA",
        PR => "PR",
        RI => "RI",
        SC => "SC",
        SD => "SD",
        TN => "TN",
        TX => "TX",
        UT => "UT",
        VT => "VT",
        VI => "VI",
        VA => "VA",
        WA => "WA",
        WV => "WV",
        WI => "WI",
        WY => "WY",
        AE => "AE",
        AA => "AA",
        AP => "AP",
        UK => "UK"
    }
}

enum_map! {
    RaceCodeType: SchemaStringType; value {
        Native => "American Indian or Alaska Native",
        Asian => "Asian or Pacific Islander",
        Multiracial => "Bi or Multi Racial",
        Black => "Black, not of Hispanic Origin",
        HispanicBlack => "Hispanic, Black",
        Hispanic => "Hispanic, Color Unknown",
        HispanicWhite => "Hispanic, White",
        MiddleEastern => "Middle Eastern",
        White => "White, not of Hispanic Origin",
        Other => "OTHER",
        Unknown => "Unknown"
    }
}

enum_map! {
    PatientSexCodeType: SchemaStringType; value {
        Male => "Male",
        Female => "Female",
        Missing => "Missing",
        Unknown => "Unknown"
    }
}

enum_map! {
    LocationTypeCodeType: SchemaIntType; value -> u64 {
        Pharmacy => 1,
        Telehealth => 2,
        School => 3,
        HomelessShelter => 4,
        IndianHealthServiceFreeStandingFacility => 5,
        IndianHealthServiceProviderBasedFacility => 6,
        Tribal638FreeStandingFacility => 7,
        Tribal638ProviderBasedFacility => 8,
        Prison => 9,
        Office => 11,
        Home => 12,
        AssistedLivingFacility => 13,
        GroupHome => 14,
        MobileUnit => 15,
        TemporaryLodging => 16,
        WalkInRetailHealthClinic => 17,
        PlaceOfEmploymentWorksite => 18,
        OffCampusOutpatientHospital => 19,
        UrgentCareFacility => 20,
        InpatientHospital => 21,
        OnCampusOutpatientHospital => 22,
        EmergencyRoom => 23,
        AmbulatorySurgicalCenter => 24,
        BirthingCenter => 25,
        MilitaryTreatmentFacility => 26,
        SkilledNursingFacility => 31,
        NursingFacility => 32,
        CustodialCareFacility => 33,
        Hospice => 34,
        AmbulanceLand => 41,
        AmbulanceAirOrWater => 42,
        IndependentClinic => 49,
        FederallyQualifiedHealthCenter => 50,
        InpatientPsychiatricFacility => 51,
        PsychiatricFacilityPartialHospitalization => 52,
        CommunityMentalHealthCenter => 53,
        IntermediateCareFacility => 54,
        ResidentialSubstanceAbuseTreatmentFacility => 55,
        PsychiatricResidentialTreatmentCenter => 56,
        NonResidentialSubstanceAbuseTreatmentFacility => 57,
        MassImmunizationCenter => 60,
        ComprehensiveInpatientRehabFacility => 61,
        ComprehensiveOutpatientRehabFacility => 62,
        EndStageRenalDiseaseTreatmentFacility => 65,
        PublicHealthClinic => 71,
        RuralHealthClinic => 72,
        IndependentLaboratory => 81,
        Other => 99
    }
}

enum_map! {
    AdmissionStatusCodeType: SchemaStringType; value {
        Ambulatory => "Ambulatory",
        Inpatient => "Inpatient",
        Observation => "Observation",
        Unknown => "Unknown"
    }
}

enum_map! {
    ProcStatusCodeType: SchemaStringType; value {
        Elective => "Elective",
        Emergency => "Emergency",
        Urgent => "Urgent",
        Unknown => "Unknown"
    }
}

enum_map! {
    TransferStatusCodeType: SchemaStringType; value {
        AmbulatorySurgicalCenter => "Ambulatory Surgical Center",
        ClinicPhysicianOffice => "Clinic Physician Office",
        Died => "Died",
        ExtendedCareFacility => "Extended Care Facility",
        FloorBed => "Floor bed",
        HealthCareFacility => "Health Care Facility",
        HomeWithoutServices => "Home without services",
        HomeWithServices => "Home with services",
        Hospice => "Hospice",
        Hospital => "Hospital",
        IntensiveCareUnit => "Intensive Care Unit",
        LeftAgainstMedicalAdvice => "Left against medical advice",
        ObservationUnit => "Observation unit",
        OperatingRoom => "Operating Room",
        PhysicianReferral => "Physician Referral",
        SameFacility => "Same Facility",
        TelemetryStepDownUnit => "Telemetry/step-down unit",
        Transferred => "Transferred to another hospital",
        Other => "Other",
        Unknown => "Unknown"
    }
}

enum_map! {
    CoverageCodeType: SchemaStringType; value {
        MdAlone => "MD-ALONE",
        MdDirecting => "MD-DIRECTING",
        MdPresent => "MD PRESENT BUT NOT DIRECTING",
        CrnaAlone => "CRNA-ALONE",
        CrnaDirected => "CRNA-DIRECTED",
        CrnaSupervising => "CRNA-SUPERVISING",
        CrnaSupervised => "CRNA-SUPERVISED",
        MdSupervising => "MD-SUPERVISING",
        Md => "MD-MD",
        PaAlone => "PA-ALONE",
        PaDirected => "PA-DIRECTED",
        CaaDirected => "CAA-DIRECTED"
    }
}

enum_map! {
    StaffResponsibilityCodeType: SchemaStringType; value {
        Supervisory => "Supervisory",
        Monitoring => "Monitoring",
        Administrative => "Administrative",
        InCharge => "In charge",
        Performing => "Performing the case",
        Responsible => "Medically responsible"
    }
}

enum_map! {
    ProviderCredentialsCodeType: SchemaStringType; value {
        AP => "Advanced Practice Nurse",
        Anesthesiologist => "Anesthesiologist",
        CAA => "Certified Anesthesiologist Assistant",
        CRNA => "Certified Registered Nurse Anesthetist",
        Dentist => "Dentist or Oral Surgeon",
        Fellow => "Fellow (Anesthesiology)",
        Surgeon => "Surgeon",
        PA => "Physician Assistant",
        Podiatrist => "Podiatrist",
        RN => "Registered Nurse",
        Resident => "Resident (Anesthesia)",
        SRNA => "Student Registered Nurse Anesthetist"
    }
}

enum_map! {
    MedicalSpecialtyCodeType: SchemaStringType; value {
        CardiacSurgery => "Cardiac Surgery",
        CardiologyEp => "Cardiology-EP",
        CardiologyGeneral => "Cardiology-General",
        CardiologyInterventional => "Cardiology-Interventional",
        Dental => "Dental/oral",
        Dermatology => "Dermatology",
        Endocrinology => "Endocrinology",
        Gastroenterology => "Gastroenterology",
        GeneralMedicine => "General Medicine",
        GeneralSurgery => "General Surgery",
        Gynecology => "Gynecology",
        GyneOnc => "Gyne-Onc",
        InfectiousDisease => "Infectious Disease",
        Neonatology => "Neonatology",
        Nephrology => "Nephrology",
        Neurology => "Neurology",
        Neurosurgery => "Neurosurgery",
        NormalNewborn => "Normal Newborn",
        Obstetrics => "Obstetrics",
        Oncology => "Oncology",
        Opthalmology => "Opthalmology",
        OrthoFoot => "Orthopedics-Foot",
        OrthoHandWrist => "Orthopedics-Hand/Wrist",
        OrthoMajorJoint => "Orthopedics-Major Joint",
        OrthoOther  => "Orthopedics-Other",
        OrthoSportsMed => "Orthopedics-Sports Med",
        OrthoTrauma => "Orthopedics-Trauma",
        Otolaryngology => "Otolaryngology",
        Psychiatry => "Psychiatry",
        Rehab => "Rehabilitation",
        Respiratory => "Respiratory",
        Rheumatology => "Rheumatology",
        Spine => "Spine",
        SubstanceAbuse => "Substance Abuse",
        SurgicalOncology => "Surgical Oncology",
        ThoracicSurgery => "Thoracic Surgery",
        Transplant => "Transplant",
        Trauma => "Trauma",
        Ungroupable => "Ungroupable",
        Urology => "Urology",
        Vascular => "Vascular"
    }
}

/// Examples provided in schema
schema_string_tuple_struct!(MonitorCodeType);

enum_map! {
    AnesthesiaCategoryCodeType: SchemaStringType; value {
        GeneralAnesthesia => "General Anesthesia",
        MonitoredAnesthesiaCare => "Monitored Anesthesia Care",
        Neuraxial => "Neuraxial",
        PeripheralNerveBlock => "Peripheral Nerve Block",
        NoAnesthesia => "No Anesthesia Provided",
        Unknown => "Unknown"
    }
}

enum_map! {
    AnesthesiaSubCategoryCodeType: SchemaStringType; value {
        Combined => "Combined Spinal and Epidural",
        Epidural => "Epidural",
        Spinal => "Spinal",
        InhalationalGeneral => "Inhalational General Anesthesia",
        TotalIntravenous => "Total Intravenous Anesthesia"
    }
}

/// Examples provided in schema
schema_string_tuple_struct!(AnesthesiaInductionCodeType);

// pub enum AnesthesiaInductionCodeType {
// 	Inhalation,
// 	Intravenous,
// 	Rectal,
// 	Intramuscular,
// 	Other,
// 	Unknown,
// 	AnyString(String)
// }

/// Examples provided in schema
schema_string_tuple_struct!(AnesthesiaMaintenanceCodeType);
// pub enum AnesthesiaMaintenanceCodeType {
// 	CircleSystem,
// 	NonRebreathing,
// 	Other,
// 	Unknown,
// 	AnyString(String)
// }

/// Examples provided in schema
schema_string_tuple_struct!(AirwayManagementMethodCodeType);
// pub enum AirwayManagementMethodCodeType {
// 	AmbuBagRescue,
// 	ArtificialRespiration,
// 	Awake,
// 	ControlledVentilation,
// 	DoubleLumenEndotrachealtubes,
// 	EndobronchialBlocker,
// 	EndotrachealTube,
// 	EndotrachealAwake,
// 	EndotrachealFiberopticBronchoscope,
// 	EndotrachealFiberopticEndoscope,
// 	EndotrachealFiberopticLaryngoscope,
// 	EndotrachealVideoLaryngoscope,
// 	FacialMask,
// 	Fiberoptic,
// 	FiberopticEndoscope,
// 	FiberopticLaryngoscope,
// 	FOBElective,
// 	FOBRequired,
// 	GlidescopeElective,
// 	GlidescopeRequired,
// 	LaryngealMaskAirway,
// 	LaryngealMaskAirwayClassic,
// 	LaryngealMaskAirwayNonClassic,
// 	LightedStylet,
// 	Nasalcannula,
// 	Natural,
// 	OxygenTherapy,
// 	RoomAir,
// 	SupraglotticDevice,
// 	VideoLaryngoscope,
// 	Other,
// 	Unknown,
// 	AnyString(String)
// }

/// Examples provided in schema
schema_string_tuple_struct!(AirwayManagementSubMethodCodeType);
// pub enum AirwayManagementSubMethodCodeType {
// 	InverseRatioVentilation,
// 	HighFrequencyVentilation,
// 	TranstrachealJetVentilation,
// 	ContinuousFlowVentilation,
// 	Other,
// 	Unknown,
// 	AnyString(String)
// }

enum_map! {
    ASAClassCodeType: SchemaStringType; value {
        I => "I",
        IE => "IE",
        II => "II",
        IIE => "IIE",
        III => "III",
        IIIE => "IIIE",
        IV => "IV",
        IVE => "IVE",
        V => "V",
        VE => "VE",
        VI => "VI",
        VIE => "VIE",
        Unknown => "Unknown"
    }
}

enum_map! {
    PreAnesthStatusCodeType: SchemaStringType; value {
        Awake => "Awake",
        Calm => "Calm",
        Asleep => "Asleep",
        Confused => "Confused",
        Unresponsive => "Unresponsive",
        Apprehensive => "Apprehensive",
        Uncooperative => "Uncooperative",
        Other => "Other",
        Unknown => "Unknown"
    }
}

pub enum ICDValueType {
    ICDValueType9CM(String),
    ICDValueType10CM(String),
    ICDValueType9SG(String),
    ICDValueType10SG(String),
    NacorRegistryCodeType(String),
}

impl SchemaStringType for ICDValueType {
    fn value(&self) -> &str {
        use self::ICDValueType::*;

        match self {
            &ICDValueType9CM(ref x) => &x,
            &ICDValueType10CM(ref x) => &x,
            &ICDValueType9SG(ref x) => &x,
            &ICDValueType10SG(ref x) => &x,
            &NacorRegistryCodeType(ref x) => &x,
        }
    }
}

impl SchemaRegexInput for ICDValueType {
    fn from_str(val: &str) -> Result<ICDValueType, AQIError> {
        use self::ICDValueType::*;

        lazy_static! {
            static ref RE9CM: Regex =
                Regex::new(r"^(V\d{2}(\.\d{1,2})?|\d{3}(\.\d{1,2})?|E\d{3}(\.\d)?)$").unwrap();
            static ref RE10CM: Regex =
                Regex::new(r"^[A-TV-Z][0-9][A-Z0-9](\.[A-Z0-9]{1,4})?$").unwrap();
            static ref RE9SG: Regex = Regex::new(r"^\d{3,4}$").unwrap();
            static ref RE10SG: Regex = Regex::new(r"^[a-zA-Z0-9]{7}$").unwrap();
            static ref RENACOR: Regex = Regex::new(r"[0-9][0-9][a-zA-Z][0-9][0-9]").unwrap();
        }

        if RE9CM.is_match(val) {
            return Ok(ICDValueType9CM(val.to_string()));
        }

        if RE10CM.is_match(val) {
            return Ok(ICDValueType10CM(val.to_string()));
        }

        if RE9SG.is_match(val) {
            return Ok(ICDValueType9SG(val.to_string()));
        }

        if RE10SG.is_match(val) {
            return Ok(ICDValueType10SG(val.to_string()));
        }

        if RENACOR.is_match(val) {
            return Ok(NacorRegistryCodeType(val.to_string()));
        }

        Err(AQIError::RegexError(format!(
            "Failed matching ICDValueType: {}",
            val
        )))
    }
}

enum_map! {
    ICDVersionType: SchemaStringType; value {
        Nine => "9",
        Ten => "10"
    }
}

/// Examples provided in schema
schema_string_tuple_struct!(LabDataNameCodeType);

/// Examples provided in schema
schema_string_tuple_struct!(CommonUnit);

/// Examples provided in schema
schema_string_tuple_struct!(MedicationTypeCodeType);

// TODO: Consider making this an enum
/// Examles provided in schema
schema_string_tuple_struct!(RouteCodeType);

/// Examples provided in schema
schema_string_tuple_struct!(MonitoringNameCodeType);

/// Examples provided in schema
schema_string_tuple_struct!(OutputCodeType);

/// Examples provided in schema
schema_string_tuple_struct!(PostOpDispositionCodeType);

enum_map! {
    PostOpDischargeCodeType: SchemaStringType; value {
        Home => "Home or self care (routine discharge)",
        ShortTermGeneralHospital => "Short term general hospital for inpatient care",
        SkilledNursingFacility => "Skilled nursing facility (SNF)",
        IntermediateCareFacility => "Intermediate care facility (ICF)",
        OtherInstitution => "Another type of institution",
        HomeUnderOrganizedCare => "Home under care of organized home health service organization",
        LeftAgainstMedicalAdvice => "Left against medical advice or discontinued care",
        HomeIVProvider => "Home under care of Home IV provider",
        AdmittedAsInpatient => "Admitted as an inpatient to this hospital",
        Expired => "Expired (i.e. dead)",
        StillPatient => "Still patient or expected to return for outpatient services (i.e. still a patient)",
        ExpiredAtHome => "Expired (i.e. died) at home",
        ExpiredAtMedicalFacility => "Expired (i.e. died) in a medical facility; e.g., hospital, SNF, ICF, or free standing hospice",
        ExpiredAtUnknown => "Expired (i.e. died) - place unknown",
        Other => "Other",
        Unknown => "Unknown"
    }
}

enum_map! {
    PaymentMethodCodeType: SchemaStringType; value {
        Charity  => "Charity",
        Commercial => "Commercial",
        GovernmentMedicaid => "Government: Medicaid",
        GovernmentMedicareA => "Government: Medicare Fee for Service - Part A",
        GovernmentMedicareB => "Government: Medicare Fee for Service - Part B",
        GovernmentMedicareC => "Government: Medicare Fee for Service - Part C",
        GovernmentMilitaryVeteran => "Government: Military/Veteran",
        GovernmentOther => "Government: Other",
        SelfPay => "Self-Pay",
        WorkersCompensation => "Worker's Compensation",
        Other => "Other",
        Unknown => "Unknown"
    }
}

/// Examples provided in schema
schema_string_tuple_struct!(TimingMilestoneCodeType);

enum_map! {
    ICCategoryCodeType: SchemaStringType; value {
        MedicalDeviceEquipment => "MEDICAL DEVICE/EQUIPMENT",
        Medication => "MEDICATION",
        InfrastructureSystem => "INFRASTRUCTURE/SYSTEM",
        AssessmentDocumentation => "ASSESSMENT/DOCUMENTATION",
        RespiratoryAirway => "RESPIRATORY/AIRWAY",
        Cardiovascular => "CARDIOVASCULAR",
        ProcedureRelated => "PROCEDURE RELATED",
        Other => "OTHER",
        Unknown => "UNKNOWN"
    }
}

enum_map! {
    OutcomeIDType: SchemaIntType; value -> u64 {
        Acidemia => 1,
        AcuteKidneyInjury => 2,
        AdverseDrugReaction => 3,
        AirwayObstruction => 4,
        AirwayTrauma => 5,
        AmnioticFluidEmbolism => 6,
        Anaphylaxis => 7,
        Arrhythmia => 8,
        Aspiration => 9,
        Awareness => 10,
        Bradycardia => 11,
        BurnInjury => 12,
        CannotVentilate => 13,
        CardiacArrest => 14,
        CaseCancelledAfterAnesthesiaInductionTime => 15,
        CaseCancelledBeforeAnesthesiaInductionTime => 16,
        CaseCancelledBeforeAnesthesiaStartTime => 17,
        CaseDelay => 18,
        CentralLinePlacementInjury => 19,
        CentralLineAssociatedBloodstreamInfection => 20,
        CerebrovascularAccident => 21,
        Coagulopathy => 22,
        Coma => 23,
        CornealInjury => 24,
        Death => 25,
        DeepVeinThrombosis => 26,
        DelayedEmergence => 27,
        Delirium => 28,
        DifficultIntubation => 29,
        DifficultMaskVentilation => 30,
        ElectrolyteAbnormality => 31,
        EmergenceDelirium => 32,
        EquipmentDeviceFailureOrMalfunction => 33,
        FailedIntubation => 34,
        Fall => 35,
        FatEmbolism => 36,
        HemodynamicInstability => 37,
        HighSpinalOrEpidural => 38,
        Hypercapnia => 39,
        Hyperglycemia => 40,
        HypertensiveEpisode => 41,
        Hyperthermia => 42,
        Hypoglycemia => 43,
        HypotensiveEpisode => 44,
        Hypoxemia => 45,
        InadequatePainControl => 46,
        InadequateReversalOfNeuromuscularBlock => 47,
        InfectionFollowingEpiduralOrSpinalAnesthesia => 48,
        InfectionFollowingPeripheralNerveBlock => 49,
        Itching => 50,
        IVInfiltration => 51,
        KidneyFailure => 52,
        LocalAnestheticSystemicToxicity => 53,
        MalignantHyperthermia => 54,
        MedicationError => 55,
        MultipleOrganFailure => 56,
        MyocardiaIschemia => 57,
        MyocardialInfarction => 58,
        ORFire => 59,
        PerioperativeVisualLoss => 60,
        PeripheralNeurologicDeficitAfterRegionalAnesthesia => 61,
        Pneumonia => 62,
        Pneumothorax => 63,
        PositioningInjury => 64,
        PostDischargeNauseaAndVomiting => 65,
        PostDuralPunctureHeadache => 66,
        PostOperativeCognitiveDysfunction => 67,
        PostOperativeNauseaAndVomiting => 68,
        ProlongedNeuromuscularBlock => 69,
        PulmonaryEdema => 70,
        PulmonaryEmbolus => 71,
        RespiratoryArrest => 72,
        RespiratoryFailure => 73,
        Seizure => 74,
        Sepsis => 75,
        Shivering => 76,
        SkinOrMucousMembraneInjury => 77,
        SpinalCordInjury => 78,
        SpinalHematomaFollowingSpinalOrEpiduralAnesthesia => 79,
        SurgicalSiteInfection => 80,
        TransfusionReaction => 81,
        Ulcer => 82,
        UnanticipatedTransfusion => 83,
        UnplannedConversionToGeneralAnesthesia => 84,
        UnplannedDuralPuncture => 85,
        UnplannedEndobronchialIntubation => 86,
        UnplannedEsophagealIntubation => 87,
        UnplannedHospitalAdmission => 88,
        UnplannedHypothermia => 89,
        UnplannedICUAdmission => 90,
        UnplannedPostoperativeVentilation => 91,
        UnplannedReintubation => 92,
        UnplannedReoperation => 93,
        UnplannedTrachealExtubation => 94,
        VascularInjury => 95,
        VenousAirEmbolism => 96,
        VentilationForMoreThan24HoursPostProcedure => 97,
        WrongPatient => 98,
        WrongProcedure => 99,
        WrongSiteRegionalAnesthesia => 100,
        WrongSiteSurgery => 101
    }
}

enum_map! {
    LabDataCategoryCodeType: SchemaStringType; value {
        BloodBank => "Blood bank",
        Chemistry => "Chemistry",
        Cytology => "Cytology",
        Genetics => "Genetics",
        Hematology => "Hematology",
        Histology => "Histology",
        Immunology => "Immunology",
        Microbiology => "Microbiology",
        Other => "Other",
        Unknown => "Unknown"
    }
}

enum_map! {
    MonitoringSourceCodeType: SchemaStringType; value {
        Electronic => "Electronic",
        UserEntered => "User entered",
        Other => "Other",
        Unknown => "Unknown"
    }
}

enum_map! {
    OutcomeTimeFrameCodeType: SchemaStringType; value {
        PreOp => "PreOp",
        IntraOp => "IntraOp",
        Pacu => "PACU",
        TwentyFourHour => "24 Hour",
        FourtyEightHour => "48 Hour",
        SevenDays => "7 Days",
        ThirtyDays => "30 Days",
        SixtyDays => "60 Days",
        NinetyDays => "90 Days",
        OneYear => "1 Year"
    }
}

enum_map! {
    OutcomeSeverityCodeType: SchemaStringType; value {
        NoHarm => "No Harm",
        MildHarm => "Mild Harm",
        ModerateHarm => "Moderate Harm",
        SevereHarm => "Severe Harm",
        Death => "Death"
    }
}

enum_map! {
    IntakeOutputDirectionCodeType: SchemaStringType; value {
        Input => "Input",
        Output => "Output",
        Other => "Other",
        Unknown => "Unknown"
    }
}

enum_map! {
    IntubationApproachCodeType: SchemaStringType; value {
        Endoctracheal => "Endoctracheal",
        Nasogastric => "Nasogastric",
        Nasotracheal => "Nasotracheal",
        Fiberoptic => "Fiberoptic",
        Tracheostomy => "Tracheostomy",
        SpeakingTracheostomy => "Speaking tracheostomy",
        Other => "Other",
        Unknown => "Unknown"
    }
}

enum_map! {
    AnesthesiaDetailsCategoryCodeType: SchemaStringType; value {
        Attempts => "attempts",
        Position => "position",
        NeedleType => "needle type",
        NeedleLength => "needle length",
        Other => "Other",
        Unknown => "Unknown"
    }
}

schema_pattern_type!(
    QCDRMeasureType,
    r"^(AQI[0-9][0-9])|(PQRS[0-9][0-9][0-9])|(IIM[0-9][0-9][0-9])|(Quantum[0-9][0-9])$"
);
schema_pattern_type!(
    QCDRCodeValueType,
    r"^([0-9][0-9][0-9][0-9]F)|(G[0-9][0-9][0-9][0-9])$"
);

enum_map! {
    QCDRModifierType: SchemaStringType; value {
        None => "",
        OneP => "1P",
        TwoP => "2P",
        ThreeP => "3P",
        EightP => "8P"
    }
}

enum_map! {
    AQIXMLVersionType: SchemaStringType; value {
        Version2018V10 => "2018V1.0",
        Version2018V10R => "2018V1.0R"
    }
}

/// 1 = Billing
/// 2 = Quality / Outcomes
/// 3 = AIMS only
/// 4 = EMR/EHR (with or without AIMS)
type TypeVendorType = u8;

schema_pattern_type!(PracticeIdType, r"^[0-9]{3,5}$");
schema_pattern_type!(VendorIDType, r"^[0-9]{3}[A-Z]{2}[0-9]{2}$");
schema_pattern_type!(
    EmailAddressType,
    r"^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+$"
);
schema_pattern_type!(ZipCodeType, r"^[0-9]{5}(-[0-9]{4})?$");
schema_pattern_type!(TaxIdType, r"^[0-9]{9}$");
schema_pattern_type!(NPIType, r"^[0-9]{10}$");
schema_pattern_type!(CPTValueType, r"^[a-zA-Z0-9][0-9][0-9][0-9][a-zA-Z0-9]$");
schema_pattern_type!(CPTModifierType, r"^[a-zA-Z0-9]{2}$");
