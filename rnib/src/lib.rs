// ==================================================================================
//   Copyright (c) 2022 Caurus
//
//   Licensed under the Apache License, Version 2.0 (the "License");
//   you may not use this file except in compliance with the License.
//   You may obtain a copy of the License at
//
//       http://www.apache.org/licenses/LICENSE-2.0
//
//   Unless required by applicable law or agreed to in writing, software
//   distributed under the License is distributed on an "AS IS" BASIS,
//   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//   See the License for the specific language governing permissions and
//   limitations under the License.
// ==================================================================================

pub mod entities {
    include!(concat!(env!("OUT_DIR"), "/entities.rs"));
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_additional_cell_information_struct_generated() {
        let _ = super::entities::AdditionalCellInformation::default();
        assert!(true);
    }

    #[test]
    fn test_cell_struct_generated() {
        let _ = super::entities::Cell::default();
        assert!(true);
    }

    #[test]
    fn test_cells_struct_generated() {
        let _ = super::entities::Cells::default();
        assert!(true);
    }

    #[test]
    fn test_served_cell_info_list_generated() {
        let _ = super::entities::ServedCellInfoList::default();
        assert!(true);
    }

    #[test]
    fn test_served_nr_cell_list_generated() {
        let _ = super::entities::ServedNrCellList::default();
        assert!(true);
    }

    #[test]
    fn test_e2node_component_config_generated() {
        let _ = super::entities::E2nodeComponentConfig::default();
        assert!(true);
    }

    #[test]
    fn test_e2node_component_iface_ng_generated() {
        let _ = super::entities::E2nodeComponentInterfaceNg::default();
        assert!(true);
    }

    #[test]
    fn test_e2node_component_iface_xn_generated() {
        let _ = super::entities::E2nodeComponentInterfaceXn::default();
        assert!(true);
    }

    #[test]
    fn test_e2node_component_iface_e1_generated() {
        let _ = super::entities::E2nodeComponentInterfaceE1::default();
        assert!(true);
    }

    #[test]
    fn test_e2node_component_iface_f1_generated() {
        let _ = super::entities::E2nodeComponentInterfaceF1::default();
        assert!(true);
    }

    #[test]
    fn test_e2node_component_iface_s1_generated() {
        let _ = super::entities::E2nodeComponentInterfaceF1::default();
        assert!(true);
    }

    #[test]
    fn test_e2node_component_iface_x2_generated() {
        let _ = super::entities::E2nodeComponentInterfaceX2::default();
        assert!(true);
    }

    #[test]
    fn test_global_enb_id_generated() {
        let _ = super::entities::GlobalEnbid::default();
        assert!(true);
    }

    #[test]
    fn test_global_engnb_id_generated() {
        let _ = super::entities::GlobalEngnbid::default();
        assert!(true);
    }

    #[test]
    fn test_global_gnb_id_generated() {
        let _ = super::entities::GlobalGnbid::default();
        assert!(true);
    }

    #[test]
    fn test_global_ng_enb_id_generated() {
        let _ = super::entities::GlobalNgenbid::default();
        assert!(true);
    }

    #[test]
    fn test_enb_generated() {
        let _ = super::entities::Enb::default();
        assert!(true);
    }

    #[test]
    fn test_served_cell_info_generated() {
        let _ = super::entities::ServedCellInfo::default();
        assert!(true);
    }

    #[test]
    fn test_choice_eutra_mode_generated() {
        let _ = super::entities::ChoiceEutraMode::default();
        assert!(true);
    }

    #[test]
    fn test_eutra_generated() {
        let _ = super::entities::Eutra::default();
        assert!(true);
    }

    #[test]
    fn test_neighbour_info_generated() {
        let _ = super::entities::NeighbourInformation::default();
        assert!(true);
    }

    #[test]
    fn test_mbsfn_subframe_generated() {
        let _ = super::entities::MbsfnSubframe::default();
        assert!(true);
    }

    #[test]
    fn test_prach_config_generated() {
        let _ = super::entities::PrachConfiguration::default();
        assert!(true);
    }

    #[test]
    fn test_tdd_info_generated() {
        let _ = super::entities::TddInfo::default();
        assert!(true);
    }

    #[test]
    fn test_addl_spec_subframe_info_generated() {
        let _ = super::entities::AdditionalSpecialSubframeInfo::default();
        assert!(true);
    }

    #[test]
    fn test_addl_spec_subframe_ext_info_generated() {
        let _ = super::entities::AdditionalSpecialSubframeExtensionInfo::default();
        assert!(true);
    }

    #[test]
    fn test_fdd_info_generated() {
        let _ = super::entities::FddInfo::default();
        assert!(true);
    }

    #[test]
    fn test_special_subframe_info_generated() {
        let _ = super::entities::SpecialSubframeInfo::default();
        assert!(true);
    }

    #[test]
    fn test_special_subframe_generated() {
        let _ = super::entities::SpecialSubframe::default();
        assert!(true);
    }

    #[test]
    fn test_addl_special_subframe_generated() {
        let _ = super::entities::AdditionalSpecialSubframe::default();
        assert!(true);
    }

    #[test]
    fn test_addl_special_subframe_patterns_generated() {
        let _ = super::entities::AdditionalSpecialSubframePatterns::default();
        assert!(true);
    }

    #[test]
    fn test_gnb_generated() {
        let _ = super::entities::Gnb::default();
        assert!(true);
    }

    #[test]
    fn test_served_nr_cell_generated() {
        let _ = super::entities::ServedNrCell::default();
        assert!(true);
    }

    #[test]
    fn test_served_nr_cell_info_generated() {
        let _ = super::entities::ServedNrCellInformation::default();
        assert!(true);
    }

    #[test]
    fn test_nr_generated() {
        let _ = super::entities::Nr::default();
        assert!(true);
    }

    #[test]
    fn test_nr_freq_info_generated() {
        let _ = super::entities::NrFrequencyInfo::default();
        assert!(true);
    }

    #[test]
    fn test_freq_band_item_generated() {
        let _ = super::entities::FrequencyBandItem::default();
        assert!(true);
    }

    #[test]
    fn test_nr_transmission_bandwidth_generated() {
        let _ = super::entities::NrTransmissionBandwidth::default();
        assert!(true);
    }

    #[test]
    fn test_nr_neighbour_info_generated() {
        let _ = super::entities::NrNeighbourInformation::default();
        assert!(true);
    }

    #[test]
    fn test_global_nb_id_generated() {
        let _ = super::entities::GlobalNbId::default();
        assert!(true);
    }

    #[test]
    fn test_nb_identity_generated() {
        let _ = super::entities::NbIdentity::default();
        assert!(true);
    }

    #[test]
    fn test_nodeb_info_generated() {
        let _ = super::entities::NodebInfo::default();
        assert!(true);
    }

    #[test]
    fn test_e2_app_protocol_generated() {
        let _ = super::entities::E2ApplicationProtocol::default();
        assert!(true);
    }

    #[test]
    fn test_node_generated() {
        let _ = super::entities::Node::default();
        assert!(true);
    }

    #[test]
    fn test_failure_generated() {
        let _ = super::entities::Failure::default();
        assert!(true);
    }

    #[test]
    fn test_ran_function_generated() {
        let _ = super::entities::RanFunction::default();
        assert!(true);
    }

    #[test]
    fn test_ran_load_info_generated() {
        let _ = super::entities::RanLoadInformation::default();
        assert!(true);
    }

    #[test]
    fn test_cell_load_info_generated() {
        let _ = super::entities::CellLoadInformation::default();
        assert!(true);
    }

    #[test]
    fn test_ul_interference_ol_indication_generated() {
        let _ = super::entities::UlInterferenceOverloadIndication::default();
        assert!(true);
    }

    #[test]
    fn test_ul_high_interference_info_generated() {
        let _ = super::entities::UlHighInterferenceInformation::default();
        assert!(true);
    }

    #[test]
    fn test_relative_narrow_band_txpower_generated() {
        let _ = super::entities::RelativeNarrowbandTxPower::default();
        assert!(true);
    }

    #[test]
    fn test_enhanced_rntp_generated() {
        let _ = super::entities::EnhancedRntp::default();
        assert!(true);
    }

    #[test]
    fn test_start_time_generated() {
        let _ = super::entities::StartTime::default();
        assert!(true);
    }

    #[test]
    fn test_abs_information_generated() {
        let _ = super::entities::AbsInformation::default();
        assert!(true);
    }

    #[test]
    fn test_invoke_indication_generated() {
        let _ = super::entities::InvokeIndication::default();
        assert!(true);
    }

    #[test]
    fn test_extended_ul_interference_ol_info() {
        let _ = super::entities::ExtendedUlInterferenceOverloadInfo::default();
        assert!(true);
    }

    #[test]
    fn test_comp_information_generated() {
        let _ = super::entities::CompInformation::default();
        assert!(true);
    }

    #[test]
    fn test_comp_information_item_generated() {
        let _ = super::entities::CompInformationItem::default();
        assert!(true);
    }

    #[test]
    fn test_comp_hypothesis_set_generated() {
        let _ = super::entities::CompHypothesisSet::default();
        assert!(true);
    }

    #[test]
    fn test_naics_state_generated() {
        let _ = super::entities::NaicsState::default();
        assert!(true);
    }

    #[test]
    fn test_dynamic_dl_xmission_info_generated() {
        let _ = super::entities::DynamicDlTransmissionInformation::default();
        assert!(true);
    }

    #[test]
    fn test_setup_failure_generated() {
        let _ = super::entities::SetupFailure::default();
        assert!(true);
    }

    #[test]
    fn test_radio_nw_layer_generated() {
        let _ = super::entities::RadioNetworkLayer::default();
        assert!(true);
    }

    #[test]
    fn test_tranport_layer_generated() {
        let _ = super::entities::TransportLayer::default();
        assert!(true);
    }

    #[test]
    fn test_protocol_generated() {
        let _ = super::entities::Protocol::default();
        assert!(true);
    }

    #[test]
    fn test_miscellaneous_generated() {
        let _ = super::entities::Miscellaneous::default();
        assert!(true);
    }

    #[test]
    fn test_criticality_diagnostics_generated() {
        let _ = super::entities::CriticalityDiagnostics::default();
        assert!(true);
    }

    #[test]
    fn test_ie_criticality_diagnostic_generated() {
        let _ = super::entities::InformationElementCriticalityDiagnostic::default();
        assert!(true);
    }
}
