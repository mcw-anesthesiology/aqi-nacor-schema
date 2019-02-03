extern crate xml;

use self::xml::writer::{Error as EmitterError, EventWriter, XmlEvent};
use super::*;

use std::fmt::Display;
use std::io::Write;

const DATE_TIME_FORMAT: &'static str = "%Y-%m-%dT%H:%M:%S%.f";

fn write_value<T: Display, W: Write>(
    name: &str,
    value: T,
    writer: &mut EventWriter<W>,
) -> Result<(), EmitterError> {
    writer.write(XmlEvent::start_element(name))?;
    writer.write(XmlEvent::characters(&value.to_string()))?;
    writer.write(XmlEvent::end_element())?;

    Ok(())
}

pub trait WritableSchemaType {
    fn write<W: Write>(&self, name: &str, &mut EventWriter<W>) -> Result<(), EmitterError>;
}

impl WritableSchemaType for AnesthesiaRecordsType {
    fn write<W: Write>(&self, name: &str, writer: &mut EventWriter<W>) -> Result<(), EmitterError> {
        writer.write(XmlEvent::start_element(name))?;

        self.record_header.write("RecordHeader", writer)?;

        for record in &self.anesthesia_records {
            record.write("AnesthesiaRecord", writer)?;
        }

        writer.write(XmlEvent::end_element())?;

        Ok(())
    }
}

impl WritableSchemaType for RecordHeaderType {
    fn write<W: Write>(&self, name: &str, writer: &mut EventWriter<W>) -> Result<(), EmitterError> {
        writer.write(XmlEvent::start_element(name))?;

        write_value("PracticeID", &self.practice_id.value(), writer)?;
        write_value("CreatedBy", &self.created_by, writer)?;
        write_value(
            "CreateDate",
            &self.create_date.format(DATE_TIME_FORMAT),
            writer,
        )?;

        self.email_set.write("EmailSet", writer)?;

        write_value("AQIXMLVersion", &self.aqi_xml_version.value(), writer)?;

        if let Some(ref vendor_set) = self.vendor_set {
            vendor_set.write("VendorSet", writer)?;
        }

        writer.write(XmlEvent::end_element())?;

        Ok(())
    }
}

impl WritableSchemaType for EmailSetType {
    fn write<W: Write>(&self, name: &str, writer: &mut EventWriter<W>) -> Result<(), EmitterError> {
        writer.write(XmlEvent::start_element(name))?;

        for set in &self.email_notification_set {
            set.write("EmailNotificationSet", writer)?;
        }

        writer.write(XmlEvent::end_element())?;

        Ok(())
    }
}

impl WritableSchemaType for EmailNotificationSetType {
    fn write<W: Write>(&self, name: &str, writer: &mut EventWriter<W>) -> Result<(), EmitterError> {
        writer.write(XmlEvent::start_element(name))?;

        write_value(
            "EmailNotificationFirstName",
            &self.email_notification_first_name,
            writer,
        )?;
        write_value(
            "EmailNotificationLastName",
            &self.email_notification_last_name,
            writer,
        )?;
        write_value(
            "EmailNotificationAddress",
            &self.email_notification_address.value(),
            writer,
        )?;

        writer.write(XmlEvent::end_element())?;

        Ok(())
    }
}

impl WritableSchemaType for VendorSetType {
    fn write<W: Write>(&self, name: &str, writer: &mut EventWriter<W>) -> Result<(), EmitterError> {
        writer.write(XmlEvent::start_element(name))?;

        for vendor in &self.vendor {
            vendor.write("Vendor", writer)?;
        }

        writer.write(XmlEvent::end_element())?;

        Ok(())
    }
}

impl WritableSchemaType for Vendors {
    fn write<W: Write>(&self, name: &str, writer: &mut EventWriter<W>) -> Result<(), EmitterError> {
        writer.write(XmlEvent::start_element(name))?;

        if let Some(ref vendor_id) = self.vendor_id {
            write_value("VendorID", &vendor_id.value(), writer)?;
        }
        self.vendor_set_type.write("VendorSetType", writer)?;
        write_value("VendorName", &self.vendor_name, writer)?;

        writer.write(XmlEvent::end_element())?;

        Ok(())
    }
}

impl WritableSchemaType for SetVendorSetType {
    fn write<W: Write>(&self, name: &str, writer: &mut EventWriter<W>) -> Result<(), EmitterError> {
        writer.write(XmlEvent::start_element(name))?;

        for vendor_type in &self.vendor_type {
            write_value("VendorType", &vendor_type.to_string(), writer)?;
        }

        writer.write(XmlEvent::end_element())?;

        Ok(())
    }
}

impl WritableSchemaType for AnesthesiaRecordType {
    fn write<W: Write>(&self, name: &str, writer: &mut EventWriter<W>) -> Result<(), EmitterError> {
        writer.write(XmlEvent::start_element(name))?;

        self.demographic.write("Demographic", writer)?;
        self.procedure.write("Procedure", writer)?;
        self.anesthesia_case.write("AnesthesiaCase", writer)?;
        self.pre_op.write("PreOp", writer)?;
        self.intra_op.write("IntraOp", writer)?;
        self.post_op.write("PostOp", writer)?;

        if let Some(ref timing_milestones) = self.timing_milestones {
            timing_milestones.write("TimingMilestones", writer)?;
        }

        if let Some(ref outcomes_events) = self.outcomes_events {
            outcomes_events.write("OutcomesEvents", writer)?;
        }

        if let Some(ref anesthesia_details) = self.anesthesia_details {
            anesthesia_details.write("AnesthesiaDetails", writer)?;
        }

        writer.write(XmlEvent::end_element())?;

        Ok(())
    }
}

impl WritableSchemaType for DemographicType {
    fn write<W: Write>(&self, name: &str, writer: &mut EventWriter<W>) -> Result<(), EmitterError> {
        writer.write(XmlEvent::start_element(name))?;

        if let Some(ref patient_id) = self.patient_id {
            write_value("PatientID", patient_id, writer)?;
        }
        if let Some(ref dob) = self.dob {
            write_value("DOB", &dob.format("%F"), writer)?;
        }
        if let Some(ref home_zip) = self.home_zip {
            write_value("HomeZip", home_zip.value(), writer)?;
        }
        if let Some(ref home_city) = self.home_city {
            write_value("HomeCity", home_city, writer)?;
        }
        if let Some(ref race) = self.race {
            write_value("Race", race.value(), writer)?;
        }

        write_value("PatientSex", self.patient_sex.value(), writer)?;

        writer.write(XmlEvent::end_element())?;

        Ok(())
    }
}

impl WritableSchemaType for ProcedureType {
    fn write<W: Write>(&self, name: &str, writer: &mut EventWriter<W>) -> Result<(), EmitterError> {
        writer.write(XmlEvent::start_element(name))?;

        if let Some(ref procedure_id) = self.procedure_id {
            write_value("ProcedureID", &procedure_id, writer)?;
        }

        write_value("FacilityID", &self.facility_id, writer)?;

        if let Some(ref procedure_location) = self.procedure_location {
            procedure_location.write("ProcedureLocation", writer)?;
        }

        if let Some(ref proc_start_time) = self.proc_start_time {
            write_value(
                "ProcStartTime",
                &proc_start_time.format(DATE_TIME_FORMAT),
                writer,
            )?;
        }

        if let Some(ref proc_end_time) = self.proc_end_time {
            write_value(
                "ProcEndTime",
                &proc_end_time.format(DATE_TIME_FORMAT),
                writer,
            )?;
        }

        if let Some(ref admission_status) = self.admission_status {
            write_value("AdmissionStatus", admission_status.value(), writer)?;
        }

        write_value("ProcStatus", self.proc_status.value(), writer)?;

        if let Some(ref transfer_status) = self.transfer_status {
            write_value("TransferStatus", transfer_status.value(), writer)?;
        }

        if let Some(ref admission_date) = self.admission_date {
            write_value(
                "AdmissionDate",
                &admission_date.format(DATE_TIME_FORMAT),
                writer,
            )?;
        }

        if let Some(ref procedure_notes) = self.procedure_notes {
            write_value("ProcedureNotes", procedure_notes, writer)?;
        }

        if let Some(ref cpt_set) = self.cpt_set {
            cpt_set.write("CPTSet", writer)?;
        }

        writer.write(XmlEvent::end_element())?;

        Ok(())
    }
}

impl WritableSchemaType for ProcedureLocationType {
    fn write<W: Write>(&self, name: &str, writer: &mut EventWriter<W>) -> Result<(), EmitterError> {
        writer.write(XmlEvent::start_element(name))?;

        write_value(
            "LocationType",
            &self.location_type.value().to_string(),
            writer,
        )?;
        write_value("LocationDetails", &self.location_details, writer)?;

        writer.write(XmlEvent::end_element())?;

        Ok(())
    }
}

impl WritableSchemaType for CPTSetType {
    fn write<W: Write>(&self, name: &str, writer: &mut EventWriter<W>) -> Result<(), EmitterError> {
        writer.write(XmlEvent::start_element(name))?;

        for cpt in &self.cpt {
            cpt.write("CPT", writer)?;
        }

        writer.write(XmlEvent::end_element())?;

        Ok(())
    }
}

impl WritableSchemaType for CPTType {
    fn write<W: Write>(&self, name: &str, writer: &mut EventWriter<W>) -> Result<(), EmitterError> {
        writer.write(XmlEvent::start_element(name))?;

        if let Some(ref cpt_rank) = self.cpt_rank {
            write_value("CPTRank", cpt_rank, writer)?;
        }

        write_value("CPTValue", self.cpt_value.value(), writer)?;

        if let Some(ref cpt_modifier) = self.cpt_modifier {
            write_value("CPTModifier", cpt_modifier.value(), writer)?;
        }

        writer.write(XmlEvent::end_element())?;

        Ok(())
    }
}

impl WritableSchemaType for AnesthesiaCaseType {
    fn write<W: Write>(&self, name: &str, writer: &mut EventWriter<W>) -> Result<(), EmitterError> {
        writer.write(XmlEvent::start_element(name))?;

        write_value("AnesthesiaRecordID", &self.anesthesia_record_id, writer)?;

        if let Some(ref anesthesia_coverage) = self.anesthesia_coverage {
            write_value("AnesthesiaCoverage", anesthesia_coverage.value(), writer)?;
        }

        self.anesthesia_staff_set
            .write("AnesthesiaStaffSet", writer)?;

        if let Some(ref monitoring_set) = self.monitoring_set {
            monitoring_set.write("MonitoringSet", writer)?;
        }

        self.anesthesia_method_set
            .write("AnesthesiaMethodSet", writer)?;

        if let Some(ref airway_management_set) = self.airway_management_set {
            airway_management_set.write("AirwayManagementSet", writer)?;
        }

        if let Some(ref cpt_anes_set) = self.cpt_anes_set {
            cpt_anes_set.write("CPTAnesSet", writer)?;
        }

        writer.write(XmlEvent::end_element())?;

        Ok(())
    }
}

impl WritableSchemaType for AnesthesiaStaffSetType {
    fn write<W: Write>(&self, name: &str, writer: &mut EventWriter<W>) -> Result<(), EmitterError> {
        writer.write(XmlEvent::start_element(name))?;

        for anesthesia_staff in &self.anesthesia_staff {
            anesthesia_staff.write("AnesthesiaStaff", writer)?;
        }

        writer.write(XmlEvent::end_element())?;

        Ok(())
    }
}

impl WritableSchemaType for AnesthesiaStaffType {
    fn write<W: Write>(&self, name: &str, writer: &mut EventWriter<W>) -> Result<(), EmitterError> {
        writer.write(XmlEvent::start_element(name))?;

        write_value("TaxID", self.tax_id.value(), writer)?;
        write_value("NPI", self.npi.value(), writer)?;

        if let Some(ref staff_responsibility) = self.staff_responsibility {
            write_value("StaffResponsibility", staff_responsibility.value(), writer)?;
        }

        write_value(
            "ProviderCredentials",
            self.provider_credentials.value(),
            writer,
        )?;

        if let Some(ref staff_sign_in) = self.staff_sign_in {
            write_value(
                "StaffSignIn",
                &staff_sign_in.format(DATE_TIME_FORMAT),
                writer,
            )?;
        }

        if let Some(ref staff_sign_out) = self.staff_sign_out {
            write_value(
                "StaffSignOut",
                &staff_sign_out.format(DATE_TIME_FORMAT),
                writer,
            )?;
        }

        if let Some(ref staff_notes) = self.staff_notes {
            write_value("StaffNotes", staff_notes, writer)?;
        }

        writer.write(XmlEvent::end_element())?;

        Ok(())
    }
}

impl WritableSchemaType for MonitoringSetType {
    fn write<W: Write>(&self, name: &str, writer: &mut EventWriter<W>) -> Result<(), EmitterError> {
        writer.write(XmlEvent::start_element(name))?;

        for monitor in &self.monitor {
            write_value("Monitor", monitor.value(), writer)?;
        }

        writer.write(XmlEvent::end_element())?;

        Ok(())
    }
}

impl WritableSchemaType for AnesthesiaMethodSetType {
    fn write<W: Write>(&self, name: &str, writer: &mut EventWriter<W>) -> Result<(), EmitterError> {
        writer.write(XmlEvent::start_element(name))?;

        for anesthesia_method in &self.anesthesia_method {
            anesthesia_method.write("AnesthesiaMethod", writer)?;
        }

        writer.write(XmlEvent::end_element())?;

        Ok(())
    }
}

impl WritableSchemaType for AnesthesiaMethodType {
    fn write<W: Write>(&self, name: &str, writer: &mut EventWriter<W>) -> Result<(), EmitterError> {
        writer.write(XmlEvent::start_element(name))?;

        write_value(
            "AnesthesiaCategory",
            self.anesthesia_category.value(),
            writer,
        )?;

        if let Some(ref anesthesia_subcategory) = self.anesthesia_subcategory {
            write_value(
                "AnesthesiaSubCategory",
                anesthesia_subcategory.value(),
                writer,
            )?;
        }

        write_value(
            "AnesthesiaStartTime",
            &self.anesthesia_start_time.format(DATE_TIME_FORMAT),
            writer,
        )?;
        write_value(
            "AnesthesiaEndTime",
            &self.anesthesia_end_time.format(DATE_TIME_FORMAT),
            writer,
        )?;

        if let Some(ref anesthesia_induction) = self.anesthesia_induction {
            write_value("AnesthesiaInduction", anesthesia_induction.value(), writer)?;
        }

        if let Some(ref anesthesia_induction_start_time) = self.anesthesia_induction_start_time {
            write_value(
                "AnesthesiaInductionStartTime",
                &anesthesia_induction_start_time.format(DATE_TIME_FORMAT),
                writer,
            )?;
        }

        if let Some(ref anesthesia_maintenance) = self.anesthesia_maintenance {
            write_value(
                "AnesthesiaMaintenance",
                anesthesia_maintenance.value(),
                writer,
            )?;
        }

        if let Some(ref anesthesia_notes) = self.anesthesia_notes {
            write_value("AnesthesiaNotes", anesthesia_notes, writer)?;
        }

        writer.write(XmlEvent::end_element())?;

        Ok(())
    }
}

impl WritableSchemaType for AirwayManagementSetType {
    fn write<W: Write>(&self, name: &str, writer: &mut EventWriter<W>) -> Result<(), EmitterError> {
        writer.write(XmlEvent::start_element(name))?;

        for airway_management in &self.airway_management {
            airway_management.write("AirwayManagement", writer)?;
        }

        writer.write(XmlEvent::end_element())?;

        Ok(())
    }
}

impl WritableSchemaType for AirwayManagementType {
    fn write<W: Write>(&self, name: &str, writer: &mut EventWriter<W>) -> Result<(), EmitterError> {
        writer.write(XmlEvent::start_element(name))?;

        write_value(
            "AirwayManagementMethod",
            self.airway_management_method.value(),
            writer,
        )?;

        if let Some(ref airway_sub_management_method) = self.airway_sub_management_method {
            write_value(
                "AirwaySubManagementMethod",
                airway_sub_management_method.value(),
                writer,
            )?;
        }

        writer.write(XmlEvent::end_element())?;

        Ok(())
    }
}

impl WritableSchemaType for CPTAnesSetType {
    fn write<W: Write>(&self, name: &str, writer: &mut EventWriter<W>) -> Result<(), EmitterError> {
        writer.write(XmlEvent::start_element(name))?;

        for cpt_anes in &self.cpt_anes {
            cpt_anes.write("CPTAnes", writer)?;
        }

        writer.write(XmlEvent::end_element())?;

        Ok(())
    }
}

impl WritableSchemaType for CPTAnesType {
    fn write<W: Write>(&self, name: &str, writer: &mut EventWriter<W>) -> Result<(), EmitterError> {
        writer.write(XmlEvent::start_element(name))?;

        write_value("CPTAnesValue", self.cpt_anes_value.value(), writer)?;

        if let Some(ref cpt_anes_modifier) = self.cpt_anes_modifier {
            write_value("CPTAnesModifier", cpt_anes_modifier.value(), writer)?;
        }

        if let Some(ref cpt_anes_description) = self.cpt_anes_description {
            write_value("CPTAnesDescription", cpt_anes_description, writer)?;
        }

        writer.write(XmlEvent::end_element())?;

        Ok(())
    }
}

impl WritableSchemaType for PreOpType {
    fn write<W: Write>(&self, name: &str, writer: &mut EventWriter<W>) -> Result<(), EmitterError> {
        writer.write(XmlEvent::start_element(name))?;

        write_value("Age", &self.age.to_string(), writer)?;

        if let Some(ref weight) = self.weight {
            write_value("Weight", &weight.to_string(), writer)?;
        }

        if let Some(ref weight_in_kg) = self.weight_in_kg {
            write_value("WeightInKg", &weight_in_kg.to_string(), writer)?;
        }

        if let Some(ref height) = self.height {
            write_value("Height", &height.to_string(), writer)?;
        }

        if let Some(ref height_in_cm) = self.height_in_cm {
            write_value("HeightInCm", &height_in_cm.to_string(), writer)?;
        }

        write_value("ASAClass", self.asa_class.value(), writer)?;

        if let Some(ref pre_anesth_status) = self.pre_anesth_status {
            write_value("PreAnesthStatus", pre_anesth_status.value(), writer)?;
        }

        if let Some(ref icd_set) = self.icd_set {
            icd_set.write("ICDSet", writer)?;
        }

        if let Some(ref pre_lab_set) = self.pre_lab_set {
            pre_lab_set.write("PreLabSet", writer)?;
        }

        writer.write(XmlEvent::end_element())?;

        Ok(())
    }
}

impl WritableSchemaType for ICDSetType {
    fn write<W: Write>(&self, name: &str, writer: &mut EventWriter<W>) -> Result<(), EmitterError> {
        writer.write(XmlEvent::start_element(name))?;

        for icd in &self.icd {
            icd.write("ICD", writer)?;
        }

        writer.write(XmlEvent::end_element())?;

        Ok(())
    }
}

impl WritableSchemaType for ICDType {
    fn write<W: Write>(&self, name: &str, writer: &mut EventWriter<W>) -> Result<(), EmitterError> {
        writer.write(XmlEvent::start_element(name))?;

        if let Some(ref icd_rank) = self.icd_rank {
            write_value("ICDRank", &icd_rank.to_string(), writer)?;
        }

        write_value("ICDValue", self.icd_value.value(), writer)?;
        write_value("ICDVersion", self.icd_version.value(), writer)?;

        writer.write(XmlEvent::end_element())?;

        Ok(())
    }
}

impl WritableSchemaType for PreLabDataSetType {
    fn write<W: Write>(&self, name: &str, writer: &mut EventWriter<W>) -> Result<(), EmitterError> {
        writer.write(XmlEvent::start_element(name))?;

        for pre_lab_data in &self.pre_lab_data {
            pre_lab_data.write("PreLabData", writer)?;
        }

        writer.write(XmlEvent::end_element())?;

        Ok(())
    }
}

impl WritableSchemaType for LabDataType {
    fn write<W: Write>(&self, name: &str, writer: &mut EventWriter<W>) -> Result<(), EmitterError> {
        writer.write(XmlEvent::start_element(name))?;

        write_value("LabName", self.lab_name.value(), writer)?;

        if let Some(ref lab_category_name) = self.lab_category_name {
            write_value("LabCategoryName", lab_category_name.value(), writer)?;
        }

        write_value("LabUnit", self.lab_unit.value(), writer)?;
        write_value("LabValue", &self.lab_value, writer)?;

        if let Some(ref lab_value_text) = self.lab_value_text {
            write_value("LabValueText", lab_value_text, writer)?;
        }

        if let Some(ref lab_range_high) = self.lab_range_high {
            write_value("LabRangeHigh", lab_range_high, writer)?;
        }

        if let Some(ref lab_range_low) = self.lab_range_low {
            write_value("LabRangeLow", lab_range_low, writer)?;
        }

        write_value(
            "LabDateTime",
            &self.lab_date_time.format(DATE_TIME_FORMAT),
            writer,
        )?;

        if let Some(ref lab_comments) = self.lab_comments {
            write_value("LabComments", lab_comments, writer)?;
        }

        writer.write(XmlEvent::end_element())?;

        Ok(())
    }
}

impl WritableSchemaType for IntraOpType {
    fn write<W: Write>(&self, name: &str, writer: &mut EventWriter<W>) -> Result<(), EmitterError> {
        writer.write(XmlEvent::start_element(name))?;

        if let Some(ref medications_set) = self.medications_set {
            medications_set.write("MedicationsSet", writer)?;
        }

        if let Some(ref monitoring_physiologic_set) = self.monitoring_physiologic_set {
            monitoring_physiologic_set.write("MonitoringPhysiologicSet", writer)?;
        }

        if let Some(ref outputs_set) = self.outputs_set {
            outputs_set.write("OutputsSet", writer)?;
        }

        writer.write(XmlEvent::end_element())?;

        Ok(())
    }
}

impl WritableSchemaType for MedicationsSetType {
    fn write<W: Write>(&self, name: &str, writer: &mut EventWriter<W>) -> Result<(), EmitterError> {
        writer.write(XmlEvent::start_element(name))?;

        for medication in &self.medication {
            medication.write("Medication", writer)?;
        }

        writer.write(XmlEvent::end_element())?;

        Ok(())
    }
}

impl WritableSchemaType for MedicationType {
    fn write<W: Write>(&self, name: &str, writer: &mut EventWriter<W>) -> Result<(), EmitterError> {
        writer.write(XmlEvent::start_element(name))?;

        write_value("MedicationName", &self.medication_name, writer)?;

        if let Some(ref medication_types) = self.medication_type {
            for medication_type in medication_types {
                write_value("MedicationType", medication_type.value(), writer)?;
            }
        }

        if let Some(ref med_dose) = self.med_dose {
            write_value("MedDose", &med_dose.to_string(), writer)?;
        }

        if let Some(ref dose_units) = self.dose_units {
            write_value("DoseUnits", dose_units.value(), writer)?;
        }

        if let Some(ref dose_start) = self.dose_start {
            write_value("DoseStart", &dose_start.format(DATE_TIME_FORMAT), writer)?;
        }

        if let Some(ref dose_end) = self.dose_end {
            write_value("DoseEnd", &dose_end.format(DATE_TIME_FORMAT), writer)?;
        }

        if let Some(ref med_concentration) = self.med_concentration {
            write_value("MedConcentration", &med_concentration.to_string(), writer)?;
        }

        if let Some(ref med_concentration_unit) = self.med_concentration_unit {
            write_value(
                "MedConcentrationUnit",
                med_concentration_unit.value(),
                writer,
            )?;
        }

        if let Some(ref medication_routes) = self.medication_route {
            for medication_route in medication_routes {
                write_value("MedicationRoute", medication_route.value(), writer)?;
            }
        }

        if let Some(ref mixture_medications) = self.mixture_medications {
            for mixture_medication in mixture_medications {
                mixture_medication.write("MixtureMedication", writer)?;
            }
        }

        writer.write(XmlEvent::end_element())?;

        Ok(())
    }
}

impl WritableSchemaType for MixtureMedicationType {
    fn write<W: Write>(&self, name: &str, writer: &mut EventWriter<W>) -> Result<(), EmitterError> {
        writer.write(XmlEvent::start_element(name))?;

        write_value(
            "MixtureMedicationName",
            &self.mixture_medication_name,
            writer,
        )?;

        if let Some(ref mixture_medication_types) = self.mixture_medication_type {
            for mixture_medication_type in mixture_medication_types {
                write_value(
                    "MixtureMedicationType",
                    mixture_medication_type.value(),
                    writer,
                )?;
            }
        }

        if let Some(ref mixture_med_dose) = self.mixture_med_dose {
            write_value("MixtureMedDose", &mixture_med_dose.to_string(), writer)?;
        }

        if let Some(ref mixture_dose_units) = self.mixture_dose_units {
            write_value("MixtureDoseUnits", mixture_dose_units.value(), writer)?;
        }

        if let Some(ref mixture_med_concentration) = self.mixture_med_concentration {
            write_value(
                "MixtureMedConcentration",
                &mixture_med_concentration.to_string(),
                writer,
            )?;
        }

        if let Some(ref mixture_med_concentration_unit) = self.mixture_med_concentration_unit {
            write_value(
                "MixtureMedConcentrationUnit",
                mixture_med_concentration_unit.value(),
                writer,
            )?;
        }

        writer.write(XmlEvent::end_element())?;

        Ok(())
    }
}

impl WritableSchemaType for MonitoringPhysiologicSetType {
    fn write<W: Write>(&self, name: &str, writer: &mut EventWriter<W>) -> Result<(), EmitterError> {
        writer.write(XmlEvent::start_element(name))?;

        for monitoring in &self.monitoring {
            monitoring.write("Monitoring", writer)?;
        }

        writer.write(XmlEvent::end_element())?;

        Ok(())
    }
}

impl WritableSchemaType for MonitoringPhysiologicType {
    fn write<W: Write>(&self, name: &str, writer: &mut EventWriter<W>) -> Result<(), EmitterError> {
        writer.write(XmlEvent::start_element(name))?;

        write_value("MonitoringName", self.monitoring_name.value(), writer)?;

        if let Some(ref monitoring_time) = self.monitoring_time {
            write_value(
                "MonitoringTime",
                &monitoring_time.format(DATE_TIME_FORMAT),
                writer,
            )?;
        }

        if let Some(ref monitoring_units) = self.monitoring_units {
            write_value("MonitoringUnits", monitoring_units.value(), writer)?;
        }

        if let Some(ref monitoring_value_numeric) = self.monitoring_value_numeric {
            write_value(
                "MonitoringValueNumeric",
                &monitoring_value_numeric.to_string(),
                writer,
            )?;
        }

        if let Some(ref monitoring_value_text) = self.monitoring_value_text {
            write_value("MonitoringValueText", monitoring_value_text, writer)?;
        }

        if let Some(ref monitoring_source) = self.monitoring_source {
            write_value("MonitoringSource", monitoring_source.value(), writer)?;
        }

        writer.write(XmlEvent::end_element())?;

        Ok(())
    }
}

impl WritableSchemaType for OutputsSetType {
    fn write<W: Write>(&self, name: &str, writer: &mut EventWriter<W>) -> Result<(), EmitterError> {
        writer.write(XmlEvent::start_element(name))?;

        for output_event in &self.output_event {
            output_event.write("OutputEvent", writer)?;
        }

        writer.write(XmlEvent::end_element())?;

        Ok(())
    }
}

impl WritableSchemaType for OutputEventType {
    fn write<W: Write>(&self, name: &str, writer: &mut EventWriter<W>) -> Result<(), EmitterError> {
        writer.write(XmlEvent::start_element(name))?;

        write_value("OutputEventName", self.output_event_name.value(), writer)?;

        if let Some(ref output_units) = self.output_units {
            write_value("OutputUnits", output_units.value(), writer)?;
        }

        if let Some(ref output_value_numeric) = self.output_value_numeric {
            write_value(
                "OutputValueNumeric",
                &output_value_numeric.to_string(),
                writer,
            )?;
        }

        if let Some(ref output_value_text) = self.output_value_text {
            write_value("OutputValueText", output_value_text, writer)?;
        }

        if let Some(ref output_start_date_time) = self.output_start_date_time {
            write_value(
                "OutputStartDateTime",
                &output_start_date_time.format(DATE_TIME_FORMAT),
                writer,
            )?;
        }

        if let Some(ref output_end_date_time) = self.output_end_date_time {
            write_value(
                "OutputEndDateTime",
                &output_end_date_time.format(DATE_TIME_FORMAT),
                writer,
            )?;
        }

        writer.write(XmlEvent::end_element())?;

        Ok(())
    }
}

impl WritableSchemaType for PostOpType {
    fn write<W: Write>(&self, name: &str, writer: &mut EventWriter<W>) -> Result<(), EmitterError> {
        writer.write(XmlEvent::start_element(name))?;

        if let Some(ref post_op_disposition) = self.post_op_disposition {
            write_value("PostOpDisposition", post_op_disposition.value(), writer)?;
        }

        if let Some(ref post_op_disp_date_time) = self.post_op_disp_date_time {
            write_value(
                "PostOpDispDateTime",
                &post_op_disp_date_time.format(DATE_TIME_FORMAT),
                writer,
            )?;
        }

        if let Some(ref post_op_discharge) = self.post_op_discharge {
            write_value("PostOpDischarge", post_op_discharge.value(), writer)?;
        }

        if let Some(ref post_op_discharge_date_time) = self.post_op_discharge_date_time {
            write_value(
                "PostOpDischargeDateTime",
                &post_op_discharge_date_time.format(DATE_TIME_FORMAT),
                writer,
            )?;
        }

        if let Some(ref length_of_hospital_stay) = self.length_of_hospital_stay {
            write_value(
                "LengthOfHospitalStay",
                &length_of_hospital_stay.to_string(),
                writer,
            )?;
        }

        for payment_method in &self.payment_method {
            payment_method.write("PaymentMethod", writer)?;
        }

        if let Some(ref post_op_lab_set) = self.post_op_lab_set {
            post_op_lab_set.write("PostOpLabSet", writer)?;
        }

        if let Some(ref icd_set) = self.icd_set {
            icd_set.write("ICDSet", writer)?;
        }

        writer.write(XmlEvent::end_element())?;

        Ok(())
    }
}

impl WritableSchemaType for PaymentMethodType {
    fn write<W: Write>(&self, name: &str, writer: &mut EventWriter<W>) -> Result<(), EmitterError> {
        writer.write(XmlEvent::start_element(name))?;

        write_value("PaymentCode", self.payment_code.value(), writer)?;

        if let Some(ref payment_description) = self.payment_description {
            write_value("PaymentDescription", payment_description.value(), writer)?;
        }

        if let Some(ref payment_description_sec) = self.payment_description_sec {
            write_value(
                "PaymentDescriptionSec",
                payment_description_sec.value(),
                writer,
            )?;
        }

        if let Some(ref payment_description_th) = self.payment_description_th {
            write_value(
                "PaymentDescriptionTh",
                payment_description_th.value(),
                writer,
            )?;
        }

        writer.write(XmlEvent::end_element())?;

        Ok(())
    }
}

impl WritableSchemaType for PostOpLabSetType {
    fn write<W: Write>(&self, name: &str, writer: &mut EventWriter<W>) -> Result<(), EmitterError> {
        writer.write(XmlEvent::start_element(name))?;

        for post_lab_data in &self.post_lab_data {
            post_lab_data.write("PostLabData", writer)?; // minOccurs="1"
        }

        writer.write(XmlEvent::end_element())?;

        Ok(())
    }
}

impl WritableSchemaType for TimingMilestonesSetType {
    fn write<W: Write>(&self, name: &str, writer: &mut EventWriter<W>) -> Result<(), EmitterError> {
        writer.write(XmlEvent::start_element(name))?;

        for timing_milestone in &self.timing_milestone {
            timing_milestone.write("TimingMilestone", writer)?;
        }

        writer.write(XmlEvent::end_element())?;

        Ok(())
    }
}

impl WritableSchemaType for TimingMilestoneType {
    fn write<W: Write>(&self, name: &str, writer: &mut EventWriter<W>) -> Result<(), EmitterError> {
        writer.write(XmlEvent::start_element(name))?;

        write_value("TMType", self.tm_type.value(), writer)?;
        write_value(
            "TMStartTime",
            &self.tm_start_time.format(DATE_TIME_FORMAT),
            writer,
        )?;

        if let Some(ref tm_end_time) = self.tm_end_time {
            write_value("TMEndTime", &tm_end_time.format(DATE_TIME_FORMAT), writer)?;
        }

        writer.write(XmlEvent::end_element())?;

        Ok(())
    }
}

impl WritableSchemaType for OutcomesEventsType {
    fn write<W: Write>(&self, name: &str, writer: &mut EventWriter<W>) -> Result<(), EmitterError> {
        writer.write(XmlEvent::start_element(name))?;

        if let Some(ref ic_event_set) = self.ic_event_set {
            ic_event_set.write("ICEventSet", writer)?;
        }

        if let Some(ref outcome_set) = self.outcome_set {
            outcome_set.write("OutcomeSet", writer)?;
        }

        if let Some(ref qcdr_set) = self.qcdr_set {
            qcdr_set.write("QCDRSet", writer)?;
        }

        writer.write(XmlEvent::end_element())?;

        Ok(())
    }
}

impl WritableSchemaType for ICEventSetType {
    fn write<W: Write>(&self, name: &str, writer: &mut EventWriter<W>) -> Result<(), EmitterError> {
        writer.write(XmlEvent::start_element(name))?;

        for ic_event in &self.ic_event {
            ic_event.write("ICEvent", writer)?;
        }

        writer.write(XmlEvent::end_element())?;

        Ok(())
    }
}

impl WritableSchemaType for ICEventType {
    fn write<W: Write>(&self, name: &str, writer: &mut EventWriter<W>) -> Result<(), EmitterError> {
        writer.write(XmlEvent::start_element(name))?;

        if let Some(ref ic_event_time_date) = self.ic_event_time_date {
            write_value(
                "ICEventTimeDate",
                &ic_event_time_date.format(DATE_TIME_FORMAT),
                writer,
            )?;
        }

        if let Some(ref ic_category) = self.ic_category {
            write_value("ICCategory", ic_category.value(), writer)?;
        }

        if let Some(ref ic_severity) = self.ic_severity {
            write_value("ICSeverity", ic_severity, writer)?;
        }

        if let Some(ref ic_name) = self.ic_name {
            write_value("ICName", ic_name, writer)?;
        }

        if let Some(ref ic_description) = self.ic_description {
            write_value("ICDescription", ic_description, writer)?;
        }

        if let Some(ref ic_value) = self.ic_value {
            write_value("ICValue", ic_value, writer)?;
        }

        if let Some(ref ic_notes) = self.ic_notes {
            write_value("ICNotes", ic_notes, writer)?;
        }

        writer.write(XmlEvent::end_element())?;

        Ok(())
    }
}

impl WritableSchemaType for OutcomeSetSetType {
    fn write<W: Write>(&self, name: &str, writer: &mut EventWriter<W>) -> Result<(), EmitterError> {
        writer.write(XmlEvent::start_element(name))?;

        for outcome in &self.outcome {
            outcome.write("Outcome", writer)?;
        }

        writer.write(XmlEvent::end_element())?;

        Ok(())
    }
}

impl WritableSchemaType for OutcomeCodeType {
    fn write<W: Write>(&self, name: &str, writer: &mut EventWriter<W>) -> Result<(), EmitterError> {
        writer.write(XmlEvent::start_element(name))?;

        write_value("OutcomeID", &self.outcome_id.value().to_string(), writer)?;
        write_value(
            "OutcomeOccurred",
            &self.outcome_occurred.to_string(),
            writer,
        )?;

        if let Some(ref outcome_time_stamp) = self.outcome_time_stamp {
            write_value(
                "OutcomeTimeStamp",
                &outcome_time_stamp.format(DATE_TIME_FORMAT),
                writer,
            )?;
        }

        if let Some(ref outcome_severity) = self.outcome_severity {
            write_value("OutcomeSeverity", outcome_severity.value(), writer)?;
        }

        if let Some(ref outcome_time_frame) = self.outcome_time_frame {
            write_value("OutcomeTimeFrame", outcome_time_frame.value(), writer)?;
        }

        writer.write(XmlEvent::end_element())?;

        Ok(())
    }
}

impl WritableSchemaType for QCDRSetTypeSet {
    fn write<W: Write>(&self, name: &str, writer: &mut EventWriter<W>) -> Result<(), EmitterError> {
        writer.write(XmlEvent::start_element(name))?;

        for qcdr in &self.qcdr {
            qcdr.write("QCDR", writer)?;
        }

        writer.write(XmlEvent::end_element())?;

        Ok(())
    }
}

impl WritableSchemaType for QCDRSetType {
    fn write<W: Write>(&self, name: &str, writer: &mut EventWriter<W>) -> Result<(), EmitterError> {
        writer.write(XmlEvent::start_element(name))?;

        write_value("QCDRMeasure", self.qcdr_measure.value(), writer)?;
        write_value("QCDRCodeValue", self.qcdr_code_value.value(), writer)?;

        if let Some(ref qcdr_modifier) = self.qcdr_modifier {
            write_value("QCDRModifier", qcdr_modifier.value(), writer)?;
        }

        writer.write(XmlEvent::end_element())?;

        Ok(())
    }
}

impl WritableSchemaType for AnesthesiaDetailsType {
    fn write<W: Write>(&self, name: &str, writer: &mut EventWriter<W>) -> Result<(), EmitterError> {
        writer.write(XmlEvent::start_element(name))?;

        if let Some(ref intake_output_set) = self.intake_output_set {
            intake_output_set.write("IntakeOutputSet", writer)?;
        }

        if let Some(ref intubation_details) = self.intubation_details {
            intubation_details.write("IntubationDetails", writer)?;
        }

        if let Some(ref anesthesia_details_set) = self.anesthesia_details_set {
            anesthesia_details_set.write("AnesthesiaDetailsSet", writer)?;
        }

        if let Some(ref medications_total_set) = self.medications_total_set {
            medications_total_set.write("MedicationsTotalSet", writer)?;
        }

        writer.write(XmlEvent::end_element())?;

        Ok(())
    }
}

impl WritableSchemaType for IntakeOutputSetType {
    fn write<W: Write>(&self, name: &str, writer: &mut EventWriter<W>) -> Result<(), EmitterError> {
        writer.write(XmlEvent::start_element(name))?;

        for intake_output_total in &self.intake_output_total {
            intake_output_total.write("IntakeOutputTotal", writer)?;
        }

        writer.write(XmlEvent::end_element())?;

        Ok(())
    }
}

impl WritableSchemaType for IntakeOutputTotalType {
    fn write<W: Write>(&self, name: &str, writer: &mut EventWriter<W>) -> Result<(), EmitterError> {
        writer.write(XmlEvent::start_element(name))?;

        if let Some(ref intake_output_direction) = self.intake_output_direction {
            write_value(
                "IntakeOutputDirection",
                intake_output_direction.value(),
                writer,
            )?;
        }

        if let Some(ref input_output_name) = self.input_output_name {
            write_value("InputOutputName", input_output_name.value(), writer)?;
        }

        if let Some(ref output_units) = self.output_units {
            write_value("OutputUnits", output_units.value(), writer)?;
        }

        if let Some(ref input_output_total) = self.input_output_total {
            write_value("InputOutputTotal", &input_output_total.to_string(), writer)?;
        }

        if let Some(ref input_output_route) = self.input_output_route {
            write_value("InputOutputRoute", input_output_route.value(), writer)?;
        }

        writer.write(XmlEvent::end_element())?;

        Ok(())
    }
}

impl WritableSchemaType for IntubationDetailsType {
    fn write<W: Write>(&self, name: &str, writer: &mut EventWriter<W>) -> Result<(), EmitterError> {
        writer.write(XmlEvent::start_element(name))?;

        if let Some(ref intubation_approach) = self.intubation_approach {
            write_value("IntubationApproach", intubation_approach.value(), writer)?;
        }

        if let Some(ref intubation_attempts) = self.intubation_attempts {
            write_value(
                "IntubationAttempts",
                &intubation_attempts.to_string(),
                writer,
            )?;
        }

        if let Some(ref tube_size) = self.tube_size {
            write_value("TubeSize", &tube_size.to_string(), writer)?;
        }

        if let Some(ref tube_type) = self.tube_type {
            write_value("TubeType", tube_type, writer)?;
        }

        if let Some(ref intubation_details_properties_set) = self.intubation_details_properties_set
        {
            intubation_details_properties_set.write("IntubationDetailsPropertiesSet", writer)?;
        }

        writer.write(XmlEvent::end_element())?;

        Ok(())
    }
}

impl WritableSchemaType for IntubationDetailsPropertiesType {
    fn write<W: Write>(&self, name: &str, writer: &mut EventWriter<W>) -> Result<(), EmitterError> {
        writer.write(XmlEvent::start_element(name))?;

        for intubation_details_properties in &self.intubation_details_properties {
            intubation_details_properties.write("IntubationDetailsProperties", writer)?;
        }

        writer.write(XmlEvent::end_element())?;

        Ok(())
    }
}

impl WritableSchemaType for IntubationDetailsPropertyType {
    fn write<W: Write>(&self, name: &str, writer: &mut EventWriter<W>) -> Result<(), EmitterError> {
        writer.write(XmlEvent::start_element(name))?;

        write_value("IntubationProperty", &self.intubation_property, writer)?;

        if let Some(ref intubation_details_property_value) = self.intubation_details_property_value
        {
            write_value(
                "IntubationDetailsPropertyValue",
                intubation_details_property_value,
                writer,
            )?;
        }

        writer.write(XmlEvent::end_element())?;

        Ok(())
    }
}

impl WritableSchemaType for AnesthesiaDetailsSetType {
    fn write<W: Write>(&self, name: &str, writer: &mut EventWriter<W>) -> Result<(), EmitterError> {
        writer.write(XmlEvent::start_element(name))?;

        for anesthesia_details_data in &self.anesthesia_details_data {
            anesthesia_details_data.write("AnesthesiaDetailsData", writer)?;
        }

        writer.write(XmlEvent::end_element())?;

        Ok(())
    }
}

impl WritableSchemaType for AnesthesiaDetailsDataType {
    fn write<W: Write>(&self, name: &str, writer: &mut EventWriter<W>) -> Result<(), EmitterError> {
        writer.write(XmlEvent::start_element(name))?;

        write_value(
            "AnesthesiaDetailsCategory",
            self.anesthesia_details_category.value(),
            writer,
        )?;

        if let Some(ref anesthesia_details_value) = self.anesthesia_details_value {
            write_value("AnesthesiaDetailsValue", anesthesia_details_value, writer)?;
        }

        writer.write(XmlEvent::end_element())?;

        Ok(())
    }
}

impl WritableSchemaType for MedicationsTotalSetType {
    fn write<W: Write>(&self, name: &str, writer: &mut EventWriter<W>) -> Result<(), EmitterError> {
        writer.write(XmlEvent::start_element(name))?;

        for medications_total in &self.medications_totals {
            medications_total.write("MedicationsTotal", writer)?;
        }

        writer.write(XmlEvent::end_element())?;

        Ok(())
    }
}
