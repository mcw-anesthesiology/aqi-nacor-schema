extern crate xml;

use super::*;
use self::xml::writer::{EventWriter, Error as EmitterError, XmlEvent};

use std::io::Write;

fn write_value<W: Write>(name: &str, value: &str, writer: &mut EventWriter<W>)
		-> Result<(), EmitterError> {

	try!(writer.write(XmlEvent::start_element(name)));
	try!(writer.write(XmlEvent::characters(value)));
	try!(writer.write(XmlEvent::end_element()));

	Ok(())
}

pub trait WritableSchemaType {
	fn write<W: Write>(&self, name: &str, &mut EventWriter<W>) -> Result<(), EmitterError>;
}

impl WritableSchemaType for AnesthesiaRecordsType {
	fn write<W: Write>(&self, name: &str, writer: &mut EventWriter<W>) -> Result<(), EmitterError> {
		try!(writer.write(XmlEvent::start_element(name)));

		try!(self.record_header.write("RecordHeader", writer));

		for record in &self.anesthesia_records {
			try!(record.write("AnesthesiaRecord", writer));
		}

		try!(writer.write(XmlEvent::end_element()));

		Ok(())
	}
}

impl WritableSchemaType for RecordHeaderType {
	fn write<W: Write>(&self, name: &str, writer: &mut EventWriter<W>) -> Result<(), EmitterError> {
		try!(writer.write(XmlEvent::start_element(name)));

		try!(write_value("PracticeID", &self.practice_id.to_string(), writer));
		try!(write_value("CreatedBy", &self.created_by, writer));
		try!(write_value("CreateDate", &self.create_date.to_rfc3339(), writer));

		if let Some(ref email_set) = self.email_set {
			try!(email_set.write("EmailSet", writer));
		}

		try!(write_value("AQIXMLVersion", &self.aqi_xml_version.value(), writer));

		try!(writer.write(XmlEvent::end_element()));

		Ok(())
	}
}

impl WritableSchemaType for EmailSetType {
	fn write<W: Write>(&self, name: &str, writer: &mut EventWriter<W>) -> Result<(), EmitterError> {
		try!(writer.write(XmlEvent::start_element(name)));

		for set in &self.email_notification_set {
			try!(set.write("EmailNotificationSet", writer));
		}

		try!(writer.write(XmlEvent::end_element()));

		Ok(())
	}
}

impl WritableSchemaType for EmailNotificationSetType {
	fn write<W: Write>(&self, name: &str, writer: &mut EventWriter<W>) -> Result<(), EmitterError> {
		try!(writer.write(XmlEvent::start_element(name)));

		try!(write_value("EmailNotificationFirstName", &self.email_notification_first_name, writer));
		try!(write_value("EmailNotificationLastName", &self.email_notification_last_name, writer));
		try!(write_value("EmailNotificationAddress", &self.email_notification_address.value(), writer));

		try!(writer.write(XmlEvent::end_element()));

		Ok(())
	}
}

impl WritableSchemaType for AnesthesiaRecordType {
	fn write<W: Write>(&self, name: &str, writer: &mut EventWriter<W>) -> Result<(), EmitterError> {
		try!(writer.write(XmlEvent::start_element(name)));

		try!(self.demographic.write("Demographic", writer));
		try!(self.procedure.write("Procedure", writer));
		try!(self.anesthesia_case.write("AnesthesiaCase", writer));
		try!(self.pre_op.write("PreOp", writer));
		try!(self.intra_op.write("IntraOp", writer));
		try!(self.post_op.write("PostOp", writer));

		if let Some(ref timing_milestones) = self.timing_milestones {
			try!(timing_milestones.write("TimingMilestones", writer));
		}

		if let Some(ref outcomes_events) = self.outcomes_events {
			try!(outcomes_events.write("OutcomesEvents", writer));
		}

		if let Some(ref anesthesia_details) = self.anesthesia_details {
			try!(anesthesia_details.write("AnesthesiaDetails", writer));
		}

		try!(writer.write(XmlEvent::end_element()));

		Ok(())
	}
}

impl WritableSchemaType for DemographicType {
	fn write<W: Write>(&self, name: &str, writer: &mut EventWriter<W>) -> Result<(), EmitterError> {
		try!(writer.write(XmlEvent::start_element(name)));

		if let Some(ref patient_id) = self.patient_id {
			try!(write_value("PatientID", patient_id, writer));
		}
		if let Some(ref dob) = self.dob {
			try!(write_value("DOB", &dob.format("%F").to_string(), writer));
		}
		if let Some(ref home_zip) = self.home_zip {
			try!(write_value("HomeZip", home_zip.value(), writer));
		}
		if let Some(ref home_city) = self.home_city {
			try!(write_value("HomeCity", home_city, writer));
		}
		if let Some(ref race) = self.race {
			try!(write_value("Race", race.value(), writer));
		}

		try!(write_value("PatientSex", self.patient_sex.value(), writer));

		try!(writer.write(XmlEvent::end_element()));

		Ok(())
	}
}

impl WritableSchemaType for ProcedureType {
	fn write<W: Write>(&self, name: &str, writer: &mut EventWriter<W>) -> Result<(), EmitterError> {
		try!(writer.write(XmlEvent::start_element(name)));

		if let Some(ref procedure_id) = self.procedure_id {
			try!(write_value("ProcedureID", &procedure_id, writer));
		}

		try!(write_value("FacilityID", &self.facility_id, writer));

		if let Some(ref procedure_location) = self.procedure_location {
			try!(procedure_location.write("ProcedureLocation", writer));
		}

		if let Some(ref proc_start_time) = self.proc_start_time {
			try!(write_value("ProcStartTime", &proc_start_time.to_rfc3339(), writer));
		}

		if let Some(ref proc_end_time) = self.proc_end_time {
			try!(write_value("ProcEndTime", &proc_end_time.to_rfc3339(), writer));
		}

		if let Some(ref admission_status) = self.admission_status {
			try!(write_value("AdmissionStatus", admission_status.value(), writer));
		}

		try!(write_value("ProcStatus", self.proc_status.value(), writer));

		if let Some(ref transfer_status) = self.transfer_status {
			try!(write_value("TransferStatus", transfer_status.value(), writer));
		}

		if let Some(ref admission_date) = self.admission_date {
			try!(write_value("AdmissionDate", &admission_date.to_rfc3339(), writer));
		}

		if let Some(ref procedure_notes) = self.procedure_notes {
			try!(write_value("ProcedureNotes", procedure_notes, writer));
		}

		if let Some(ref cpt_set) = self.cpt_set {
			try!(cpt_set.write("CPTSet", writer));
		}

		try!(writer.write(XmlEvent::end_element()));

		Ok(())
	}
}

impl WritableSchemaType for ProcedureLocationType {
	fn write<W: Write>(&self, name: &str, writer: &mut EventWriter<W>) -> Result<(), EmitterError> {
		try!(writer.write(XmlEvent::start_element(name)));

		try!(write_value("LocationType", &self.location_type.value().to_string(), writer));
		try!(write_value("LocationDetails", &self.location_details, writer));

		try!(writer.write(XmlEvent::end_element()));

		Ok(())
	}
}

impl WritableSchemaType for CPTSetType {
	fn write<W: Write>(&self, name: &str, writer: &mut EventWriter<W>) -> Result<(), EmitterError> {
		try!(writer.write(XmlEvent::start_element(name)));

		for cpt in &self.cpt {
			try!(cpt.write("CPT", writer));
		}

		try!(writer.write(XmlEvent::end_element()));

		Ok(())
	}
}

impl WritableSchemaType for CPTType {
	fn write<W: Write>(&self, name: &str, writer: &mut EventWriter<W>) -> Result<(), EmitterError> {
		try!(writer.write(XmlEvent::start_element(name)));

		if let Some(ref cpt_rank) = self.cpt_rank {
			try!(write_value("CPTRank", cpt_rank, writer));
		}

		try!(write_value("CPTValue", self.cpt_value.value(), writer));

		if let Some(ref cpt_modifier) = self.cpt_modifier {
			try!(write_value("CPTModifier", cpt_modifier.value(), writer));
		}


		try!(writer.write(XmlEvent::end_element()));

		Ok(())
	}
}

impl WritableSchemaType for AnesthesiaCaseType {
	fn write<W: Write>(&self, name: &str, writer: &mut EventWriter<W>) -> Result<(), EmitterError> {
		try!(writer.write(XmlEvent::start_element(name)));

		try!(write_value("AnesthesiaRecordID", &self.anesthesia_record_id, writer));

		if let Some(ref anesthesia_coverage) = self.anesthesia_coverage {
			try!(write_value("AnesthesiaCoverage", anesthesia_coverage.value(), writer));
		}

		try!(self.anesthesia_staff_set.write("AnesthesiaStaffSet", writer));

		if let Some(ref monitoring_set) = self.monitoring_set {
			try!(monitoring_set.write("MonitoringSet", writer));
		}

		try!(self.anesthesia_method_set.write("AnesthesiaMethodSet", writer));

		if let Some(ref airway_management_set) = self.airway_management_set {
			try!(airway_management_set.write("AirwayManagementSet", writer));
		}

		if let Some(ref cpt_anes_set) = self.cpt_anes_set {
			try!(cpt_anes_set.write("CPTAnessSet", writer)); // Sic
		}

		try!(writer.write(XmlEvent::end_element()));

		Ok(())
	}
}

impl WritableSchemaType for AnesthesiaStaffSetType {
	fn write<W: Write>(&self, name: &str, writer: &mut EventWriter<W>) -> Result<(), EmitterError> {
		try!(writer.write(XmlEvent::start_element(name)));

		for anesthesia_staff in &self.anesthesia_staff {
			try!(anesthesia_staff.write("AnesthesiaStaff", writer));
		}

		try!(writer.write(XmlEvent::end_element()));

		Ok(())
	}
}

impl WritableSchemaType for AnesthesiaStaffType {
	fn write<W: Write>(&self, name: &str, writer: &mut EventWriter<W>) -> Result<(), EmitterError> {
		try!(writer.write(XmlEvent::start_element(name)));

		try!(write_value("TaxID", self.tax_id.value(), writer));
		try!(write_value("NPI", self.npi.value(), writer));

		if let Some(ref staff_responsibility) = self.staff_responsibility {
			try!(write_value("StaffResponsibility", staff_responsibility.value(), writer));
		}

		try!(write_value("ProviderCredentials", self.provider_credentials.value(), writer));

		if let Some(ref staff_sign_in) = self.staff_sign_in {
			try!(write_value("StaffSignIn", &staff_sign_in.to_rfc3339(), writer));
		}

		if let Some(ref staff_sign_out) = self.staff_sign_out {
			try!(write_value("StaffSignOut", &staff_sign_out.to_rfc3339(), writer));
		}

		if let Some(ref staff_notes) = self.staff_notes {
			try!(write_value("StaffNotes", staff_notes, writer));
		}

		try!(writer.write(XmlEvent::end_element()));

		Ok(())
	}
}

impl WritableSchemaType for MonitoringSetType {
	fn write<W: Write>(&self, name: &str, writer: &mut EventWriter<W>) -> Result<(), EmitterError> {
		try!(writer.write(XmlEvent::start_element(name)));

		for monitor in &self.monitor {
			try!(write_value("Monitor", monitor.value(), writer));
		}

		try!(writer.write(XmlEvent::end_element()));

		Ok(())
	}
}

impl WritableSchemaType for AnesthesiaMethodSetType {
	fn write<W: Write>(&self, name: &str, writer: &mut EventWriter<W>) -> Result<(), EmitterError> {
		try!(writer.write(XmlEvent::start_element(name)));

		for anesthesia_method in &self.anesthesia_method {
			try!(anesthesia_method.write("AnesthesiaMethod", writer));
		}

		try!(writer.write(XmlEvent::end_element()));

		Ok(())
	}
}

impl WritableSchemaType for AnesthesiaMethodType {
	fn write<W: Write>(&self, name: &str, writer: &mut EventWriter<W>) -> Result<(), EmitterError> {
		try!(writer.write(XmlEvent::start_element(name)));

		try!(write_value("AnesthesiaCategory", self.anesthesia_category.value(), writer));

		if let Some(ref anesthesia_subcategory) = self.anesthesia_subcategory {
			try!(write_value("AnesthesiaSubCategory", anesthesia_subcategory.value(), writer));
		}

		try!(write_value("AnesthesiaStartTime", &self.anesthesia_start_time.to_rfc3339(), writer));
		try!(write_value("AnesthesiaEndTime", &self.anesthesia_end_time.to_rfc3339(), writer));

		if let Some(ref anesthesia_induction) = self.anesthesia_induction {
			try!(write_value("AnesthesiaInduction", anesthesia_induction.value(), writer));
		}

		if let Some(ref anesthesia_induction_start_time) = self.anesthesia_induction_start_time {
			try!(write_value("AnesthesiaInductionStartTime", &anesthesia_induction_start_time.to_rfc3339(), writer));
		}

		if let Some(ref anesthesia_maintenance) = self.anesthesia_maintenance {
			try!(write_value("AnesthesiaMaintenance", anesthesia_maintenance.value(), writer));
		}

		if let Some(ref anesthesia_notes) = self.anesthesia_notes {
			try!(write_value("AnesthesiaNotes", anesthesia_notes, writer));
		}

		try!(writer.write(XmlEvent::end_element()));

		Ok(())
	}
}

impl WritableSchemaType for AirwayManagementSetType {
	fn write<W: Write>(&self, name: &str, writer: &mut EventWriter<W>) -> Result<(), EmitterError> {
		try!(writer.write(XmlEvent::start_element(name)));

		for airway_management in &self.airway_management {
			try!(airway_management.write("AirwayManagement", writer));
		}

		try!(writer.write(XmlEvent::end_element()));

		Ok(())
	}
}

impl WritableSchemaType for AirwayManagementType {
	fn write<W: Write>(&self, name: &str, writer: &mut EventWriter<W>) -> Result<(), EmitterError> {
		try!(writer.write(XmlEvent::start_element(name)));

		try!(write_value("AirwayManagementMethod", self.airway_management_method.value(), writer));

		if let Some(ref airway_sub_management_method) = self.airway_sub_management_method {
			try!(write_value("AirwaySubManagementMethod", airway_sub_management_method.value(), writer));
		}

		try!(writer.write(XmlEvent::end_element()));

		Ok(())
	}
}

impl WritableSchemaType for CPTAnesSetType {
	fn write<W: Write>(&self, name: &str, writer: &mut EventWriter<W>) -> Result<(), EmitterError> {
		try!(writer.write(XmlEvent::start_element(name)));

		for cpt_anes in &self.cpt_anes {
			try!(cpt_anes.write("CPTAnes", writer));
		}

		try!(writer.write(XmlEvent::end_element()));

		Ok(())
	}
}

impl WritableSchemaType for CPTAnesType {
	fn write<W: Write>(&self, name: &str, writer: &mut EventWriter<W>) -> Result<(), EmitterError> {
		try!(writer.write(XmlEvent::start_element(name)));

		try!(write_value("CPTAnesValue", self.cpt_anes_value.value(), writer));

		if let Some(ref cpt_anes_modifier) = self.cpt_anes_modifier {
			try!(write_value("CPTAnesModifer", cpt_anes_modifier.value(), writer)); // Sic
		}

		if let Some(ref cpt_anes_description) = self.cpt_anes_description {
			try!(write_value("CPTAnesDescription", cpt_anes_description, writer));
		}

		try!(writer.write(XmlEvent::end_element()));

		Ok(())
	}
}

impl WritableSchemaType for PreOpType {
	fn write<W: Write>(&self, name: &str, writer: &mut EventWriter<W>) -> Result<(), EmitterError> {
		try!(writer.write(XmlEvent::start_element(name)));

		try!(write_value("Age", &self.age.to_string(), writer));

		if let Some(ref weight) = self.weight {
			try!(write_value("Weight", &weight.to_string(), writer));
		}

		if let Some(ref weight_in_kg) = self.weight_in_kg {
			try!(write_value("WeightInKg", &weight_in_kg.to_string(), writer));
		}

		if let Some(ref height) = self.height {
			try!(write_value("Height", &height.to_string(), writer));
		}

		if let Some(ref height_in_cm) = self.height_in_cm {
			try!(write_value("HeightInCm", &height_in_cm.to_string(), writer));
		}

		try!(write_value("ASAClass", self.asa_class.value(), writer));

		if let Some(ref pre_anesth_status) = self.pre_anesth_status {
			try!(write_value("PreAnesthStatus", pre_anesth_status.value(), writer));
		}

		if let Some(ref icd_set) = self.icd_set {
			try!(icd_set.write("ICDSet", writer));
		}

		if let Some(ref pre_risk_set) = self.pre_risk_set {
			try!(pre_risk_set.write("PreRiskSet", writer));
		}

		if let Some(ref pre_lab_set) = self.pre_lab_set {
			try!(pre_lab_set.write("PreLabSet", writer));
		}

		try!(writer.write(XmlEvent::end_element()));

		Ok(())
	}
}

impl WritableSchemaType for ICDSetType {
	fn write<W: Write>(&self, name: &str, writer: &mut EventWriter<W>) -> Result<(), EmitterError> {
		try!(writer.write(XmlEvent::start_element(name)));

		for icd in &self.icd {
			try!(icd.write("ICD", writer));
		}

		try!(writer.write(XmlEvent::end_element()));

		Ok(())
	}
}

impl WritableSchemaType for ICDType {
	fn write<W: Write>(&self, name: &str, writer: &mut EventWriter<W>) -> Result<(), EmitterError> {
		try!(writer.write(XmlEvent::start_element(name)));

		if let Some(ref icd_rank) = self.icd_rank {
			try!(write_value("ICDRank", &icd_rank.to_string(), writer));
		}

		try!(write_value("ICDValue", self.icd_value.value(), writer));
		try!(write_value("ICDVersion", self.icd_version.value(), writer));

		try!(writer.write(XmlEvent::end_element()));

		Ok(())
	}
}

impl WritableSchemaType for PreRiskSetType {
	fn write<W: Write>(&self, name: &str, writer: &mut EventWriter<W>) -> Result<(), EmitterError> {
		try!(writer.write(XmlEvent::start_element(name)));

		for pre_risk in &self.pre_risk {
			try!(pre_risk.write("PreRisk", writer));
		}

		try!(writer.write(XmlEvent::end_element()));

		Ok(())
	}
}

impl WritableSchemaType for PreOPRiskType {
	fn write<W: Write>(&self, name: &str, writer: &mut EventWriter<W>) -> Result<(), EmitterError> {
		try!(writer.write(XmlEvent::start_element(name)));

		if let Some(ref pre_op_risk_category) = self.pre_op_risk_category {
			try!(write_value("PreOPRiskCategory", pre_op_risk_category.value(), writer));
		}

		try!(write_value("PreOPRiskName", self.pre_op_risk_name.value(), writer));

		if let Some(ref pre_op_risk_notes) = self.pre_op_risk_notes {
			try!(write_value("PreOPRiskNotes", pre_op_risk_notes, writer));
		}

		try!(writer.write(XmlEvent::end_element()));

		Ok(())
	}
}

impl WritableSchemaType for PreLabDataSetType {
	fn write<W: Write>(&self, name: &str, writer: &mut EventWriter<W>) -> Result<(), EmitterError> {
		try!(writer.write(XmlEvent::start_element(name)));

		for pre_lab_data in &self.pre_lab_data {
			try!(pre_lab_data.write("PreLabData", writer));
		}

		try!(writer.write(XmlEvent::end_element()));

		Ok(())
	}
}

impl WritableSchemaType for LabDataType {
	fn write<W: Write>(&self, name: &str, writer: &mut EventWriter<W>) -> Result<(), EmitterError> {
		try!(writer.write(XmlEvent::start_element(name)));

		try!(write_value("LabName", self.lab_name.value(), writer));

		if let Some(ref lab_category_name) = self.lab_category_name {
			try!(write_value("LabCategoryName", lab_category_name.value(), writer));
		}

		try!(write_value("LabUnit", self.lab_unit.value(), writer));
		try!(write_value("LabValue", &self.lab_value, writer));

		if let Some(ref lab_value_text) = self.lab_value_text {
			try!(write_value("LabValueText", lab_value_text, writer));
		}

		if let Some(ref lab_range_high) = self.lab_range_high {
			try!(write_value("LabRangeHigh", lab_range_high, writer));
		}

		if let Some(ref lab_range_low) = self.lab_range_low {
			try!(write_value("LabRangeLow", lab_range_low, writer));
		}

		try!(write_value("LabDateTime", &self.lab_date_time.to_rfc3339(), writer));

		if let Some(ref lab_comments) = self.lab_comments {
			try!(write_value("LabComments", lab_comments, writer));
		}

		try!(writer.write(XmlEvent::end_element()));

		Ok(())
	}
}

impl WritableSchemaType for IntraOpType {
	fn write<W: Write>(&self, name: &str, writer: &mut EventWriter<W>) -> Result<(), EmitterError> {
		try!(writer.write(XmlEvent::start_element(name)));

		if let Some(ref medications_set) = self.medications_set {
			try!(medications_set.write("MedicationsSet", writer));
		}

		if let Some(ref monitoring_physiologic_set) = self.monitoring_physiologic_set {
			try!(monitoring_physiologic_set.write("MonitoringPhysiologicSet", writer));
		}

		if let Some(ref outputs_set) = self.outputs_set {
			try!(outputs_set.write("OutputsSet", writer));
		}

		try!(writer.write(XmlEvent::end_element()));

		Ok(())
	}
}

impl WritableSchemaType for MedicationsSetType {
	fn write<W: Write>(&self, name: &str, writer: &mut EventWriter<W>) -> Result<(), EmitterError> {
		try!(writer.write(XmlEvent::start_element(name)));

		for medication in &self.medication {
			try!(medication.write("Medication", writer));
		}

		try!(writer.write(XmlEvent::end_element()));

		Ok(())
	}
}

impl WritableSchemaType for MedicationType {
	fn write<W: Write>(&self, name: &str, writer: &mut EventWriter<W>) -> Result<(), EmitterError> {
		try!(writer.write(XmlEvent::start_element(name)));

		try!(write_value("MedicationName", &self.medication_name, writer));

		if let Some(ref medication_types) = self.medication_type {
			for medication_type in medication_types {
				try!(write_value("MedicationType", medication_type.value(), writer));
			}
		}

		if let Some(ref med_dose) = self.med_dose {
			try!(write_value("MedDose", &med_dose.to_string(), writer));
		}

		if let Some(ref dose_units) = self.dose_units {
			try!(write_value("DoseUnits", dose_units.value(), writer));
		}

		if let Some(ref dose_start) = self.dose_start {
			try!(write_value("DoseStart", &dose_start.to_rfc3339(), writer));
		}

		if let Some(ref dose_end) = self.dose_end {
			try!(write_value("DoseEnd", &dose_end.to_rfc3339(), writer));
		}

		if let Some(ref med_concentration) = self.med_concentration {
			try!(write_value("MedConcentration", &med_concentration.to_string(), writer));
		}

		if let Some(ref med_concentration_unit) = self.med_concentration_unit {
			try!(write_value("MedConcentrationUnit", med_concentration_unit.value(), writer));
		}

		if let Some(ref medication_routes) = self.medication_route {
			for medication_route in medication_routes {
				try!(write_value("MedicationRoute", medication_route.value(), writer));
			}
		}

		if let Some(ref mixture_medications) = self.mixture_medications {
			for mixture_medication in mixture_medications {
				try!(mixture_medication.write("MixtureMedication", writer));
			}
		}

		try!(writer.write(XmlEvent::end_element()));

		Ok(())
	}
}

impl WritableSchemaType for MixtureMedicationType {
	fn write<W: Write>(&self, name: &str, writer: &mut EventWriter<W>) -> Result<(), EmitterError> {
		try!(writer.write(XmlEvent::start_element(name)));

		try!(write_value("MixtureMedicationName", &self.mixture_medication_name, writer));

		if let Some(ref mixture_medication_types) = self.mixture_medication_type {
			for mixture_medication_type in mixture_medication_types {
				try!(write_value("MixtureMedicationType", mixture_medication_type.value(), writer));
			}
		}

		if let Some(ref mixture_med_dose) = self.mixture_med_dose {
			try!(write_value("MixtureMedDose", &mixture_med_dose.to_string(), writer));
		}

		if let Some(ref mixture_dose_units) = self.mixture_dose_units {
			try!(write_value("MixtureDoseUnits", mixture_dose_units.value(), writer));
		}

		if let Some(ref mixture_med_concentration) = self.mixture_med_concentration {
			try!(write_value("MixtureMedConcentration", &mixture_med_concentration.to_string(), writer));
		}

		if let Some(ref mixture_med_concentration_unit) = self.mixture_med_concentration_unit {
			try!(write_value("MixtureMedConcentrationUnit", mixture_med_concentration_unit.value(), writer));
		}

		try!(writer.write(XmlEvent::end_element()));

		Ok(())
	}
}

impl WritableSchemaType for MonitoringPhysiologicSetType {
	fn write<W: Write>(&self, name: &str, writer: &mut EventWriter<W>) -> Result<(), EmitterError> {
		try!(writer.write(XmlEvent::start_element(name)));

		for monitoring in &self.monitoring {
			try!(monitoring.write("Monitoring", writer));
		}

		try!(writer.write(XmlEvent::end_element()));

		Ok(())
	}
}

impl WritableSchemaType for MonitoringPhysiologicType {
	fn write<W: Write>(&self, name: &str, writer: &mut EventWriter<W>) -> Result<(), EmitterError> {
		try!(writer.write(XmlEvent::start_element(name)));

		try!(write_value("MonitoringName", self.monitoring_name.value(), writer));

		if let Some(ref monitoring_time) = self.monitoring_time {
			try!(write_value("MonitoringTime", &monitoring_time.to_rfc3339(), writer));
		}

		if let Some(ref monitoring_units) = self.monitoring_units {
			try!(write_value("MonitoringUnits", monitoring_units.value(), writer));
		}

		if let Some(ref monitoring_value_numeric) = self.monitoring_value_numeric {
			try!(write_value("MonitoringValueNumeric", &monitoring_value_numeric.to_string(), writer));
		}

		if let Some(ref monitoring_value_text) = self.monitoring_value_text {
			try!(write_value("MonitoringValueText", monitoring_value_text, writer));
		}

		if let Some(ref monitoring_source) = self.monitoring_source {
			try!(write_value("MonitoringSource", monitoring_source.value(), writer));
		}

		try!(writer.write(XmlEvent::end_element()));

		Ok(())
	}
}

impl WritableSchemaType for OutputsSetType {
	fn write<W: Write>(&self, name: &str, writer: &mut EventWriter<W>) -> Result<(), EmitterError> {
		try!(writer.write(XmlEvent::start_element(name)));

		for output_event in &self.output_event {
			try!(output_event.write("OutputEvent", writer));
		}

		try!(writer.write(XmlEvent::end_element()));

		Ok(())
	}
}

impl WritableSchemaType for OutputEventType {
	fn write<W: Write>(&self, name: &str, writer: &mut EventWriter<W>) -> Result<(), EmitterError> {
		try!(writer.write(XmlEvent::start_element(name)));

		try!(write_value("OutputEventName", self.output_event_name.value(), writer));

		if let Some(ref output_units) = self.output_units {
			try!(write_value("OutputUnits", output_units.value(), writer));
		}

		if let Some(ref output_value_numeric) = self.output_value_numeric {
			try!(write_value("OutputValueNumeric", &output_value_numeric.to_string(), writer));
		}

		if let Some(ref output_value_text) = self.output_value_text {
			try!(write_value("OutputValueText", output_value_text, writer));
		}

		if let Some(ref output_start_date_time) = self.output_start_date_time {
			try!(write_value("OutputStartDateTime", &output_start_date_time.to_rfc3339(), writer));
		}

		if let Some(ref output_end_date_time) = self.output_end_date_time {
			try!(write_value("OutputEndDateTime", &output_end_date_time.to_rfc3339(), writer));
		}

		try!(writer.write(XmlEvent::end_element()));

		Ok(())
	}
}

impl WritableSchemaType for PostOpType {
	fn write<W: Write>(&self, name: &str, writer: &mut EventWriter<W>) -> Result<(), EmitterError> {
		try!(writer.write(XmlEvent::start_element(name)));

		if let Some(ref post_op_disposition) = self.post_op_disposition {
			try!(write_value("PostOpDisposition", post_op_disposition.value(), writer));
		}

		if let Some(ref post_op_disp_date_time) = self.post_op_disp_date_time {
			try!(write_value("PostOpDispDateTime", &post_op_disp_date_time.to_rfc3339(), writer));
		}

		if let Some(ref post_op_discharge) = self.post_op_discharge {
			try!(write_value("PostOpDischarge", post_op_discharge.value(), writer));
		}

		if let Some(ref post_op_discharge_date_time) = self.post_op_discharge_date_time {
			try!(write_value("PostOpDischargeDateTime", &post_op_discharge_date_time.to_rfc3339(), writer));
		}

		if let Some(ref length_of_hospital_stay) = self.length_of_hospital_stay {
			try!(write_value("LengthOfHospitalStay", &length_of_hospital_stay.to_string(), writer));
		}

		for payment_method in &self.payment_method {
			try!(payment_method.write("PaymentMethod", writer));
		}

		if let Some(ref post_op_lab_set) = self.post_op_lab_set {
			try!(post_op_lab_set.write("PostOpLabSet", writer));
		}

		if let Some(ref icd_set) = self.icd_set {
			try!(icd_set.write("ICDSet", writer));
		}

		try!(writer.write(XmlEvent::end_element()));

		Ok(())
	}
}

impl WritableSchemaType for PaymentMethodType {
	fn write<W: Write>(&self, name: &str, writer: &mut EventWriter<W>) -> Result<(), EmitterError> {
		try!(writer.write(XmlEvent::start_element(name)));

		try!(write_value("PaymentCode", self.payment_code.value(), writer));

		if let Some(ref payment_description) = self.payment_description {
			try!(write_value("PaymentDescription", payment_description.value(), writer));
		}

		if let Some(ref payment_description_sec) = self.payment_description_sec {
			try!(write_value("PaymentDescriptionSec", payment_description_sec.value(), writer));
		}

		if let Some(ref payment_description_th) = self.payment_description_th {
			try!(write_value("PaymentDescriptionTh", payment_description_th.value(), writer));
		}

		try!(writer.write(XmlEvent::end_element()));

		Ok(())
	}
}

impl WritableSchemaType for PostOpLabSetType {
	fn write<W: Write>(&self, name: &str, writer: &mut EventWriter<W>) -> Result<(), EmitterError> {
		try!(writer.write(XmlEvent::start_element(name)));

		for post_lab_data in &self.post_lab_data {
			try!(post_lab_data.write("PostLabData", writer)); // minOccurs="1"
		}

		try!(writer.write(XmlEvent::end_element()));

		Ok(())
	}
}

impl WritableSchemaType for TimingMilestonesSetType {
	fn write<W: Write>(&self, name: &str, writer: &mut EventWriter<W>) -> Result<(), EmitterError> {
		try!(writer.write(XmlEvent::start_element(name)));

		for timing_milestone in &self.timing_milestone {
			try!(timing_milestone.write("TimingMilestone", writer));
		}

		try!(writer.write(XmlEvent::end_element()));

		Ok(())
	}
}

impl WritableSchemaType for TimingMilestoneType {
	fn write<W: Write>(&self, name: &str, writer: &mut EventWriter<W>) -> Result<(), EmitterError> {
		try!(writer.write(XmlEvent::start_element(name)));

		try!(write_value("TMType", self.tm_type.value(), writer));
		try!(write_value("TMStartTime", &self.tm_start_time.to_rfc3339(), writer));

		if let Some(ref tm_end_time) = self.tm_end_time {
			try!(write_value("TMEndTime", &tm_end_time.to_rfc3339(), writer));
		}


		try!(writer.write(XmlEvent::end_element()));

		Ok(())
	}
}

impl WritableSchemaType for OutcomesEventsType {
	fn write<W: Write>(&self, name: &str, writer: &mut EventWriter<W>) -> Result<(), EmitterError> {
		try!(writer.write(XmlEvent::start_element(name)));

		if let Some(ref ic_event_set) = self.ic_event_set {
			try!(ic_event_set.write("ICEventSet", writer));
		}

		if let Some(ref outcome_set) = self.outcome_set {
			try!(outcome_set.write("OutcomeSet", writer));
		}

		if let Some(ref qcdr_set) = self.qcdr_set {
			try!(qcdr_set.write("QCDRSet", writer));
		}

		try!(writer.write(XmlEvent::end_element()));

		Ok(())
	}
}

impl WritableSchemaType for ICEventSetType {
	fn write<W: Write>(&self, name: &str, writer: &mut EventWriter<W>) -> Result<(), EmitterError> {
		try!(writer.write(XmlEvent::start_element(name)));

		for ic_event in &self.ic_event {
			try!(ic_event.write("ICEvent", writer));
		}

		try!(writer.write(XmlEvent::end_element()));

		Ok(())
	}
}

impl WritableSchemaType for ICEventType {
	fn write<W: Write>(&self, name: &str, writer: &mut EventWriter<W>) -> Result<(), EmitterError> {
		try!(writer.write(XmlEvent::start_element(name)));

		if let Some(ref ic_event_time_date) = self.ic_event_time_date {
			try!(write_value("ICEventTimeDate", &ic_event_time_date.to_rfc3339(), writer));
		}

		if let Some(ref ic_category) = self.ic_category {
			try!(write_value("ICCategory", ic_category.value(), writer));
		}

		if let Some(ref ic_severity) = self.ic_severity {
			try!(write_value("ICSeverity", ic_severity, writer));
		}

		if let Some(ref ic_name) = self.ic_name {
			try!(write_value("ICName", ic_name, writer));
		}

		if let Some(ref ic_description) = self.ic_description {
			try!(write_value("ICDescription", ic_description, writer));
		}

		if let Some(ref ic_value) = self.ic_value {
			try!(write_value("ICValue", ic_value, writer));
		}

		if let Some(ref ic_notes) = self.ic_notes {
			try!(write_value("ICNotes", ic_notes, writer));
		}

		try!(writer.write(XmlEvent::end_element()));

		Ok(())
	}
}

impl WritableSchemaType for OutcomeSetSetType {
	fn write<W: Write>(&self, name: &str, writer: &mut EventWriter<W>) -> Result<(), EmitterError> {
		try!(writer.write(XmlEvent::start_element(name)));

		for outcome in &self.outcome {
			try!(outcome.write("Outcome", writer));
		}

		try!(writer.write(XmlEvent::end_element()));

		Ok(())
	}
}

impl WritableSchemaType for OutcomeCodeType {
	fn write<W: Write>(&self, name: &str, writer: &mut EventWriter<W>) -> Result<(), EmitterError> {
		try!(writer.write(XmlEvent::start_element(name)));

		try!(write_value("OutcomeID", &self.outcome_id.value().to_string(), writer));
		try!(write_value("OutcomeOccurred", &self.outcome_occurred.to_string(), writer));

		if let Some(ref outcome_time_stamp) = self.outcome_time_stamp {
			try!(write_value("OutcomeTimeStamp", &outcome_time_stamp.to_rfc3339(), writer));
		}

		if let Some(ref outcome_severity) = self.outcome_severity {
			try!(write_value("OutcomeSeverity", outcome_severity.value(), writer));
		}

		if let Some(ref outcome_time_frame) = self.outcome_time_frame {
			try!(write_value("OutcomeTimeFrame", outcome_time_frame.value(), writer));
		}

		try!(writer.write(XmlEvent::end_element()));

		Ok(())
	}
}

impl WritableSchemaType for QCDRSetTypeSet {
	fn write<W: Write>(&self, name: &str, writer: &mut EventWriter<W>) -> Result<(), EmitterError> {
		try!(writer.write(XmlEvent::start_element(name)));

		for qcdr in &self.qcdr {
			try!(qcdr.write("QCDR", writer));
		}

		try!(writer.write(XmlEvent::end_element()));

		Ok(())
	}
}

impl WritableSchemaType for QCDRSetType {
	fn write<W: Write>(&self, name: &str, writer: &mut EventWriter<W>) -> Result<(), EmitterError> {
		try!(writer.write(XmlEvent::start_element(name)));

		try!(write_value("QCDRMeasure", self.qcdr_measure.value(), writer));
		try!(write_value("QCDRCodeValue", self.qcdr_code_value.value(), writer));

		if let Some(ref qcdr_modifier) = self.qcdr_modifier {
			try!(write_value("QCDRModifier", qcdr_modifier.value(), writer));
		}

		try!(writer.write(XmlEvent::end_element()));

		Ok(())
	}
}

impl WritableSchemaType for AnesthesiaDetailsType {
	fn write<W: Write>(&self, name: &str, writer: &mut EventWriter<W>) -> Result<(), EmitterError> {
		try!(writer.write(XmlEvent::start_element(name)));

		if let Some(ref intake_output_set) = self.intake_output_set {
			try!(intake_output_set.write("IntakeOutputSet", writer));
		}

		if let Some(ref intubation_details) = self.intubation_details {
			try!(intubation_details.write("IntubationDetails", writer));
		}

		if let Some(ref anesthesia_details_set) = self.anesthesia_details_set {
			try!(anesthesia_details_set.write("AnesthesiaDetailsSet", writer));
		}

		if let Some(ref medications_total_set) = self.medications_total_set {
			try!(medications_total_set.write("MedicationsTotalSet", writer));
		}

		try!(writer.write(XmlEvent::end_element()));

		Ok(())
	}
}

impl WritableSchemaType for IntakeOutputSetType {
	fn write<W: Write>(&self, name: &str, writer: &mut EventWriter<W>) -> Result<(), EmitterError> {
		try!(writer.write(XmlEvent::start_element(name)));

		for intake_output_total in &self.intake_output_total {
			try!(intake_output_total.write("IntakeOutputTotal", writer));
		}

		try!(writer.write(XmlEvent::end_element()));

		Ok(())
	}
}

impl WritableSchemaType for IntakeOutputTotalType {
	fn write<W: Write>(&self, name: &str, writer: &mut EventWriter<W>) -> Result<(), EmitterError> {
		try!(writer.write(XmlEvent::start_element(name)));

		if let Some(ref intake_output_direction) = self.intake_output_direction {
			try!(write_value("IntakeOutputDirection", intake_output_direction.value(), writer));
		}

		if let Some(ref input_output_name) = self.input_output_name {
			try!(write_value("InputOutputName", input_output_name.value(), writer));
		}

		if let Some(ref output_units) = self.output_units {
			try!(write_value("OutputUnits", output_units.value(), writer));
		}

		if let Some(ref input_output_total) = self.input_output_total {
			try!(write_value("InputOutputTotal", &input_output_total.to_string(), writer));
		}

		if let Some(ref input_output_route) = self.input_output_route {
			try!(write_value("InputOutputRoute", input_output_route.value(), writer));
		}

		try!(writer.write(XmlEvent::end_element()));

		Ok(())
	}
}

impl WritableSchemaType for IntubationDetailsType {
	fn write<W: Write>(&self, name: &str, writer: &mut EventWriter<W>) -> Result<(), EmitterError> {
		try!(writer.write(XmlEvent::start_element(name)));

		if let Some(ref intubation_approach) = self.intubation_approach {
			try!(write_value("IntubationApproach", intubation_approach.value(), writer));
		}

		if let Some(ref intubation_attempts) = self.intubation_attempts {
			try!(write_value("IntubationAttempts", &intubation_attempts.to_string(), writer));
		}

		if let Some(ref tube_size) = self.tube_size {
			try!(write_value("TubeSize", &tube_size.to_string(), writer));
		}

		if let Some(ref tube_type) = self.tube_type {
			try!(write_value("TubeType", tube_type, writer));
		}

		if let Some(ref intubation_details_properties_set) = self.intubation_details_properties_set {
			try!(intubation_details_properties_set.write("IntubationDetailsPropertiesSet", writer));
		}

		try!(writer.write(XmlEvent::end_element()));

		Ok(())
	}
}

impl WritableSchemaType for IntubationDetailsPropertiesType {
	fn write<W: Write>(&self, name: &str, writer: &mut EventWriter<W>) -> Result<(), EmitterError> {
		try!(writer.write(XmlEvent::start_element(name)));

		for intubation_details_properties in &self.intubation_details_properties {
			try!(intubation_details_properties.write("IntubationDetailsProperties", writer));
		}

		try!(writer.write(XmlEvent::end_element()));

		Ok(())
	}
}

impl WritableSchemaType for IntubationDetailsPropertyType {
	fn write<W: Write>(&self, name: &str, writer: &mut EventWriter<W>) -> Result<(), EmitterError> {
		try!(writer.write(XmlEvent::start_element(name)));

		try!(write_value("IntubationProperty", &self.intubation_property, writer));

		if let Some(ref intubation_details_property_value) = self.intubation_details_property_value {
			try!(write_value("IntubationDetailsPropertyValue", intubation_details_property_value, writer));
		}

		try!(writer.write(XmlEvent::end_element()));

		Ok(())
	}
}

impl WritableSchemaType for AnesthesiaDetailsSetType {
	fn write<W: Write>(&self, name: &str, writer: &mut EventWriter<W>) -> Result<(), EmitterError> {
		try!(writer.write(XmlEvent::start_element(name)));

		for anesthesia_details_data in &self.anesthesia_details_data {
			try!(anesthesia_details_data.write("AnesthesiaDetailsData", writer));
		}

		try!(writer.write(XmlEvent::end_element()));

		Ok(())
	}
}

impl WritableSchemaType for AnesthesiaDetailsDataType {
	fn write<W: Write>(&self, name: &str, writer: &mut EventWriter<W>) -> Result<(), EmitterError> {
		try!(writer.write(XmlEvent::start_element(name)));

		try!(write_value("AnesthesiaDetailsCategory", self.anesthesia_details_category.value(), writer));

		if let Some(ref anesthesia_details_value) = self.anesthesia_details_value {
			try!(write_value("AnesthesiaDetailsValue", anesthesia_details_value, writer));
		}

		try!(writer.write(XmlEvent::end_element()));

		Ok(())
	}
}

impl WritableSchemaType for MedicationsTotalSetType {
	fn write<W: Write>(&self, name: &str, writer: &mut EventWriter<W>) -> Result<(), EmitterError> {
		try!(writer.write(XmlEvent::start_element(name)));

		for medications_total in &self.medications_totals {
			try!(medications_total.write("MedicationsTotal", writer));
		}

		try!(writer.write(XmlEvent::end_element()));

		Ok(())
	}
}
