// ===============================================================================
// Authors: AFRL/RQQA
// Organization: Air Force Research Laboratory, Aerospace Systems Directorate, Power and Control Division
// 
// Copyright (c) 2017 Government of the United State of America, as represented by
// the Secretary of the Air Force.  No copyright is claimed in the United States under
// Title 17, U.S. Code.  All Other Rights Reserved.
// ===============================================================================

// This file was auto-created by LmcpGen. Modifications will be overwritten.

#![allow(non_camel_case_types)]

#[macro_use]
#[cfg(test)]
extern crate quickcheck;

#[macro_use]
mod avtas;
pub mod afrl;
pub mod uxas;


use avtas::lmcp::{Lmcp, StructInfo};
pub use avtas::lmcp::{Error, ErrorType, LmcpSubscription, SrcLoc};

#[derive(Debug)]
pub enum Message {
    UxasMessagesTaskAssignmentCoordinatorTask(uxas::messages::task::assignment_coordinator_task::AssignmentCoordinatorTask),
    UxasMessagesTaskRendezvousTask(uxas::messages::task::rendezvous_task::RendezvousTask),
    UxasMessagesTaskPlanningState(uxas::messages::task::planning_state::PlanningState),
    UxasMessagesTaskAssignmentCoordination(uxas::messages::task::assignment_coordination::AssignmentCoordination),
    UxasMessagesTaskCoordinatedAutomationRequest(uxas::messages::task::coordinated_automation_request::CoordinatedAutomationRequest),
    UxasMessagesTaskTaskAutomationRequest(uxas::messages::task::task_automation_request::TaskAutomationRequest),
    UxasMessagesTaskTaskAutomationResponse(uxas::messages::task::task_automation_response::TaskAutomationResponse),
    UxasMessagesTaskUniqueAutomationRequest(uxas::messages::task::unique_automation_request::UniqueAutomationRequest),
    UxasMessagesTaskUniqueAutomationResponse(uxas::messages::task::unique_automation_response::UniqueAutomationResponse),
    UxasMessagesTaskSensorFootprintRequests(uxas::messages::task::sensor_footprint_requests::SensorFootprintRequests),
    UxasMessagesTaskFootprintRequest(uxas::messages::task::footprint_request::FootprintRequest),
    UxasMessagesTaskSensorFootprint(uxas::messages::task::sensor_footprint::SensorFootprint),
    UxasMessagesTaskSensorFootprintResponse(uxas::messages::task::sensor_footprint_response::SensorFootprintResponse),
    UxasMessagesTaskTaskImplementationRequest(uxas::messages::task::task_implementation_request::TaskImplementationRequest),
    UxasMessagesTaskTaskImplementationResponse(uxas::messages::task::task_implementation_response::TaskImplementationResponse),
    UxasMessagesTaskAssignmentCostMatrix(uxas::messages::task::assignment_cost_matrix::AssignmentCostMatrix),
    UxasMessagesTaskTaskOptionCost(uxas::messages::task::task_option_cost::TaskOptionCost),
    UxasMessagesTaskTaskAssignment(uxas::messages::task::task_assignment::TaskAssignment),
    UxasMessagesTaskTaskAssignmentSummary(uxas::messages::task::task_assignment_summary::TaskAssignmentSummary),
    UxasMessagesTaskTaskOption(uxas::messages::task::task_option::TaskOption),
    UxasMessagesTaskTaskPlanOptions(uxas::messages::task::task_plan_options::TaskPlanOptions),
    UxasMessagesTaskTaskPause(uxas::messages::task::task_pause::TaskPause),
    UxasMessagesTaskTaskResume(uxas::messages::task::task_resume::TaskResume),
    UxasMessagesTaskTaskProgress(uxas::messages::task::task_progress::TaskProgress),
    UxasMessagesTaskTaskProgressRequest(uxas::messages::task::task_progress_request::TaskProgressRequest),
    UxasMessagesTaskTaskInitialized(uxas::messages::task::task_initialized::TaskInitialized),
    UxasMessagesTaskTaskActive(uxas::messages::task::task_active::TaskActive),
    UxasMessagesTaskTaskComplete(uxas::messages::task::task_complete::TaskComplete),
    UxasMessagesTaskCancelTask(uxas::messages::task::cancel_task::CancelTask),
    UxasMessagesUxnativeVideoRecord(uxas::messages::uxnative::video_record::VideoRecord),
    UxasMessagesUxnativeStartupComplete(uxas::messages::uxnative::startup_complete::StartupComplete),
    UxasMessagesUxnativeCreateNewService(uxas::messages::uxnative::create_new_service::CreateNewService),
    UxasMessagesUxnativeKillService(uxas::messages::uxnative::kill_service::KillService),
    UxasMessagesUxnativeIncrementWaypoint(uxas::messages::uxnative::increment_waypoint::IncrementWaypoint),
    UxasMessagesUxnativeSafeHeadingAction(uxas::messages::uxnative::safe_heading_action::SafeHeadingAction),
    UxasMessagesUxnativeEntityLocation(uxas::messages::uxnative::entity_location::EntityLocation),
    UxasMessagesUxnativeBandwidthTest(uxas::messages::uxnative::bandwidth_test::BandwidthTest),
    UxasMessagesUxnativeBandwidthReceiveReport(uxas::messages::uxnative::bandwidth_receive_report::BandwidthReceiveReport),
    UxasMessagesUxnativeSubTaskExecution(uxas::messages::uxnative::sub_task_execution::SubTaskExecution),
    UxasMessagesUxnativeSubTaskAssignment(uxas::messages::uxnative::sub_task_assignment::SubTaskAssignment),
    UxasMessagesUxnativeAutopilotKeepAlive(uxas::messages::uxnative::autopilot_keep_alive::AutopilotKeepAlive),
    UxasMessagesUxnativeOnboardStatusReport(uxas::messages::uxnative::onboard_status_report::OnboardStatusReport),
    UxasMessagesUxnativeEntityJoin(uxas::messages::uxnative::entity_join::EntityJoin),
    UxasMessagesUxnativeEntityExit(uxas::messages::uxnative::entity_exit::EntityExit),
    UxasMessagesUxnativeSimulationTimeStepAcknowledgement(uxas::messages::uxnative::simulation_time_step_acknowledgement::SimulationTimeStepAcknowledgement),
    UxasMessagesUxnativeSpeedOverrideAction(uxas::messages::uxnative::speed_override_action::SpeedOverrideAction),
    UxasMessagesRouteGraphNode(uxas::messages::route::graph_node::GraphNode),
    UxasMessagesRouteGraphEdge(uxas::messages::route::graph_edge::GraphEdge),
    UxasMessagesRouteGraphRegion(uxas::messages::route::graph_region::GraphRegion),
    UxasMessagesRouteRouteConstraints(uxas::messages::route::route_constraints::RouteConstraints),
    UxasMessagesRouteRouteRequest(uxas::messages::route::route_request::RouteRequest),
    UxasMessagesRouteRoutePlanRequest(uxas::messages::route::route_plan_request::RoutePlanRequest),
    UxasMessagesRouteRoutePlan(uxas::messages::route::route_plan::RoutePlan),
    UxasMessagesRouteRoutePlanResponse(uxas::messages::route::route_plan_response::RoutePlanResponse),
    UxasMessagesRouteRouteResponse(uxas::messages::route::route_response::RouteResponse),
    UxasMessagesRouteEgressRouteRequest(uxas::messages::route::egress_route_request::EgressRouteRequest),
    UxasMessagesRouteEgressRouteResponse(uxas::messages::route::egress_route_response::EgressRouteResponse),
    UxasMessagesRouteRoadPointsConstraints(uxas::messages::route::road_points_constraints::RoadPointsConstraints),
    UxasMessagesRouteRoadPointsRequest(uxas::messages::route::road_points_request::RoadPointsRequest),
    UxasMessagesRouteRoadPointsResponse(uxas::messages::route::road_points_response::RoadPointsResponse),
    AfrlVehiclesGroundVehicleConfiguration(afrl::vehicles::ground_vehicle_configuration::GroundVehicleConfiguration),
    AfrlVehiclesGroundVehicleState(afrl::vehicles::ground_vehicle_state::GroundVehicleState),
    AfrlVehiclesSurfaceVehicleConfiguration(afrl::vehicles::surface_vehicle_configuration::SurfaceVehicleConfiguration),
    AfrlVehiclesSurfaceVehicleState(afrl::vehicles::surface_vehicle_state::SurfaceVehicleState),
    AfrlVehiclesStationarySensorConfiguration(afrl::vehicles::stationary_sensor_configuration::StationarySensorConfiguration),
    AfrlVehiclesStationarySensorState(afrl::vehicles::stationary_sensor_state::StationarySensorState),
    AfrlCmasiAbstractGeometry(afrl::cmasi::abstract_geometry::AbstractGeometry),
    AfrlCmasiKeyValuePair(afrl::cmasi::key_value_pair::KeyValuePair),
    AfrlCmasiLocation3D(afrl::cmasi::location3d::Location3D),
    AfrlCmasiPayloadAction(afrl::cmasi::payload_action::PayloadAction),
    AfrlCmasiPayloadConfiguration(afrl::cmasi::payload_configuration::PayloadConfiguration),
    AfrlCmasiPayloadState(afrl::cmasi::payload_state::PayloadState),
    AfrlCmasiVehicleAction(afrl::cmasi::vehicle_action::VehicleAction),
    AfrlCmasiTask(afrl::cmasi::task::Task),
    AfrlCmasiSearchTask(afrl::cmasi::search_task::SearchTask),
    AfrlCmasiAbstractZone(afrl::cmasi::abstract_zone::AbstractZone),
    AfrlCmasiEntityConfiguration(afrl::cmasi::entity_configuration::EntityConfiguration),
    AfrlCmasiFlightProfile(afrl::cmasi::flight_profile::FlightProfile),
    AfrlCmasiAirVehicleConfiguration(afrl::cmasi::air_vehicle_configuration::AirVehicleConfiguration),
    AfrlCmasiEntityState(afrl::cmasi::entity_state::EntityState),
    AfrlCmasiAirVehicleState(afrl::cmasi::air_vehicle_state::AirVehicleState),
    AfrlCmasiWedge(afrl::cmasi::wedge::Wedge),
    AfrlCmasiAreaSearchTask(afrl::cmasi::area_search_task::AreaSearchTask),
    AfrlCmasiCameraAction(afrl::cmasi::camera_action::CameraAction),
    AfrlCmasiCameraConfiguration(afrl::cmasi::camera_configuration::CameraConfiguration),
    AfrlCmasiGimballedPayloadState(afrl::cmasi::gimballed_payload_state::GimballedPayloadState),
    AfrlCmasiCameraState(afrl::cmasi::camera_state::CameraState),
    AfrlCmasiCircle(afrl::cmasi::circle::Circle),
    AfrlCmasiGimbalAngleAction(afrl::cmasi::gimbal_angle_action::GimbalAngleAction),
    AfrlCmasiGimbalConfiguration(afrl::cmasi::gimbal_configuration::GimbalConfiguration),
    AfrlCmasiGimbalScanAction(afrl::cmasi::gimbal_scan_action::GimbalScanAction),
    AfrlCmasiGimbalStareAction(afrl::cmasi::gimbal_stare_action::GimbalStareAction),
    AfrlCmasiGimbalState(afrl::cmasi::gimbal_state::GimbalState),
    AfrlCmasiGoToWaypointAction(afrl::cmasi::go_to_waypoint_action::GoToWaypointAction),
    AfrlCmasiKeepInZone(afrl::cmasi::keep_in_zone::KeepInZone),
    AfrlCmasiKeepOutZone(afrl::cmasi::keep_out_zone::KeepOutZone),
    AfrlCmasiLineSearchTask(afrl::cmasi::line_search_task::LineSearchTask),
    AfrlCmasiNavigationAction(afrl::cmasi::navigation_action::NavigationAction),
    AfrlCmasiLoiterAction(afrl::cmasi::loiter_action::LoiterAction),
    AfrlCmasiLoiterTask(afrl::cmasi::loiter_task::LoiterTask),
    AfrlCmasiWaypoint(afrl::cmasi::waypoint::Waypoint),
    AfrlCmasiMissionCommand(afrl::cmasi::mission_command::MissionCommand),
    AfrlCmasiMustFlyTask(afrl::cmasi::must_fly_task::MustFlyTask),
    AfrlCmasiOperatorSignal(afrl::cmasi::operator_signal::OperatorSignal),
    AfrlCmasiOperatingRegion(afrl::cmasi::operating_region::OperatingRegion),
    AfrlCmasiAutomationRequest(afrl::cmasi::automation_request::AutomationRequest),
    AfrlCmasiPointSearchTask(afrl::cmasi::point_search_task::PointSearchTask),
    AfrlCmasiPolygon(afrl::cmasi::polygon::Polygon),
    AfrlCmasiRectangle(afrl::cmasi::rectangle::Rectangle),
    AfrlCmasiRemoveTasks(afrl::cmasi::remove_tasks::RemoveTasks),
    AfrlCmasiServiceStatus(afrl::cmasi::service_status::ServiceStatus),
    AfrlCmasiSessionStatus(afrl::cmasi::session_status::SessionStatus),
    AfrlCmasiVehicleActionCommand(afrl::cmasi::vehicle_action_command::VehicleActionCommand),
    AfrlCmasiVideoStreamAction(afrl::cmasi::video_stream_action::VideoStreamAction),
    AfrlCmasiVideoStreamConfiguration(afrl::cmasi::video_stream_configuration::VideoStreamConfiguration),
    AfrlCmasiVideoStreamState(afrl::cmasi::video_stream_state::VideoStreamState),
    AfrlCmasiAutomationResponse(afrl::cmasi::automation_response::AutomationResponse),
    AfrlCmasiRemoveZones(afrl::cmasi::remove_zones::RemoveZones),
    AfrlCmasiRemoveEntities(afrl::cmasi::remove_entities::RemoveEntities),
    AfrlCmasiFlightDirectorAction(afrl::cmasi::flight_director_action::FlightDirectorAction),
    AfrlCmasiWeatherReport(afrl::cmasi::weather_report::WeatherReport),
    AfrlCmasiFollowPathCommand(afrl::cmasi::follow_path_command::FollowPathCommand),
    AfrlCmasiPathWaypoint(afrl::cmasi::path_waypoint::PathWaypoint),
    AfrlCmasiStopMovementAction(afrl::cmasi::stop_movement_action::StopMovementAction),
    AfrlCmasiWaypointTransfer(afrl::cmasi::waypoint_transfer::WaypointTransfer),
    AfrlCmasiPayloadStowAction(afrl::cmasi::payload_stow_action::PayloadStowAction),
    AfrlImpactPowerConfiguration(afrl::impact::power_configuration::PowerConfiguration),
    AfrlImpactRadioConfiguration(afrl::impact::radio_configuration::RadioConfiguration),
    AfrlImpactRadioTowerConfiguration(afrl::impact::radio_tower_configuration::RadioTowerConfiguration),
    AfrlImpactRadioState(afrl::impact::radio_state::RadioState),
    AfrlImpactRadioTowerState(afrl::impact::radio_tower_state::RadioTowerState),
    AfrlImpactImpactPayloadConfiguration(afrl::impact::impact_payload_configuration::ImpactPayloadConfiguration),
    AfrlImpactDeployImpactPayload(afrl::impact::deploy_impact_payload::DeployImpactPayload),
    AfrlImpactPowerPlantState(afrl::impact::power_plant_state::PowerPlantState),
    AfrlImpactBatchRoutePlanRequest(afrl::impact::batch_route_plan_request::BatchRoutePlanRequest),
    AfrlImpactBatchRoutePlanResponse(afrl::impact::batch_route_plan_response::BatchRoutePlanResponse),
    AfrlImpactTaskTimingPair(afrl::impact::task_timing_pair::TaskTimingPair),
    AfrlImpactBatchSummaryRequest(afrl::impact::batch_summary_request::BatchSummaryRequest),
    AfrlImpactBatchSummaryResponse(afrl::impact::batch_summary_response::BatchSummaryResponse),
    AfrlImpactTaskSummary(afrl::impact::task_summary::TaskSummary),
    AfrlImpactVehicleSummary(afrl::impact::vehicle_summary::VehicleSummary),
    AfrlImpactSpeedAltPair(afrl::impact::speed_alt_pair::SpeedAltPair),
    AfrlImpactImpactAutomationRequest(afrl::impact::impact_automation_request::ImpactAutomationRequest),
    AfrlImpactImpactAutomationResponse(afrl::impact::impact_automation_response::ImpactAutomationResponse),
    AfrlImpactPointOfInterest(afrl::impact::point_of_interest::PointOfInterest),
    AfrlImpactLineOfInterest(afrl::impact::line_of_interest::LineOfInterest),
    AfrlImpactAreaOfInterest(afrl::impact::area_of_interest::AreaOfInterest),
    AfrlImpactImpactPointSearchTask(afrl::impact::impact_point_search_task::ImpactPointSearchTask),
    AfrlImpactPatternSearchTask(afrl::impact::pattern_search_task::PatternSearchTask),
    AfrlImpactAngledAreaSearchTask(afrl::impact::angled_area_search_task::AngledAreaSearchTask),
    AfrlImpactImpactLineSearchTask(afrl::impact::impact_line_search_task::ImpactLineSearchTask),
    AfrlImpactWatchTask(afrl::impact::watch_task::WatchTask),
    AfrlImpactMultiVehicleWatchTask(afrl::impact::multi_vehicle_watch_task::MultiVehicleWatchTask),
    AfrlImpactCommRelayTask(afrl::impact::comm_relay_task::CommRelayTask),
    AfrlImpactCordonTask(afrl::impact::cordon_task::CordonTask),
    AfrlImpactBlockadeTask(afrl::impact::blockade_task::BlockadeTask),
    AfrlImpactEscortTask(afrl::impact::escort_task::EscortTask),
    AfrlImpactConfigurationRequest(afrl::impact::configuration_request::ConfigurationRequest),
    AfrlImpactWaterReport(afrl::impact::water_report::WaterReport),
    AfrlImpactWaterZone(afrl::impact::water_zone::WaterZone),
    AfrlImpactPayloadDropTask(afrl::impact::payload_drop_task::PayloadDropTask),
    AfrlCmasiPerceiveEntityPerception(afrl::cmasi::perceive::entity_perception::EntityPerception),
    AfrlCmasiPerceiveTrackEntityAction(afrl::cmasi::perceive::track_entity_action::TrackEntityAction),
    AfrlCmasiPerceiveTrackEntityTask(afrl::cmasi::perceive::track_entity_task::TrackEntityTask),

}

impl Lmcp for Message {
    fn ser(&self, buf: &mut [u8]) -> Result<usize, Error> {
        match *self {
            Message::UxasMessagesTaskAssignmentCoordinatorTask(ref x) => x.ser(buf),
            Message::UxasMessagesTaskRendezvousTask(ref x) => x.ser(buf),
            Message::UxasMessagesTaskPlanningState(ref x) => x.ser(buf),
            Message::UxasMessagesTaskAssignmentCoordination(ref x) => x.ser(buf),
            Message::UxasMessagesTaskCoordinatedAutomationRequest(ref x) => x.ser(buf),
            Message::UxasMessagesTaskTaskAutomationRequest(ref x) => x.ser(buf),
            Message::UxasMessagesTaskTaskAutomationResponse(ref x) => x.ser(buf),
            Message::UxasMessagesTaskUniqueAutomationRequest(ref x) => x.ser(buf),
            Message::UxasMessagesTaskUniqueAutomationResponse(ref x) => x.ser(buf),
            Message::UxasMessagesTaskSensorFootprintRequests(ref x) => x.ser(buf),
            Message::UxasMessagesTaskFootprintRequest(ref x) => x.ser(buf),
            Message::UxasMessagesTaskSensorFootprint(ref x) => x.ser(buf),
            Message::UxasMessagesTaskSensorFootprintResponse(ref x) => x.ser(buf),
            Message::UxasMessagesTaskTaskImplementationRequest(ref x) => x.ser(buf),
            Message::UxasMessagesTaskTaskImplementationResponse(ref x) => x.ser(buf),
            Message::UxasMessagesTaskAssignmentCostMatrix(ref x) => x.ser(buf),
            Message::UxasMessagesTaskTaskOptionCost(ref x) => x.ser(buf),
            Message::UxasMessagesTaskTaskAssignment(ref x) => x.ser(buf),
            Message::UxasMessagesTaskTaskAssignmentSummary(ref x) => x.ser(buf),
            Message::UxasMessagesTaskTaskOption(ref x) => x.ser(buf),
            Message::UxasMessagesTaskTaskPlanOptions(ref x) => x.ser(buf),
            Message::UxasMessagesTaskTaskPause(ref x) => x.ser(buf),
            Message::UxasMessagesTaskTaskResume(ref x) => x.ser(buf),
            Message::UxasMessagesTaskTaskProgress(ref x) => x.ser(buf),
            Message::UxasMessagesTaskTaskProgressRequest(ref x) => x.ser(buf),
            Message::UxasMessagesTaskTaskInitialized(ref x) => x.ser(buf),
            Message::UxasMessagesTaskTaskActive(ref x) => x.ser(buf),
            Message::UxasMessagesTaskTaskComplete(ref x) => x.ser(buf),
            Message::UxasMessagesTaskCancelTask(ref x) => x.ser(buf),
            Message::UxasMessagesUxnativeVideoRecord(ref x) => x.ser(buf),
            Message::UxasMessagesUxnativeStartupComplete(ref x) => x.ser(buf),
            Message::UxasMessagesUxnativeCreateNewService(ref x) => x.ser(buf),
            Message::UxasMessagesUxnativeKillService(ref x) => x.ser(buf),
            Message::UxasMessagesUxnativeIncrementWaypoint(ref x) => x.ser(buf),
            Message::UxasMessagesUxnativeSafeHeadingAction(ref x) => x.ser(buf),
            Message::UxasMessagesUxnativeEntityLocation(ref x) => x.ser(buf),
            Message::UxasMessagesUxnativeBandwidthTest(ref x) => x.ser(buf),
            Message::UxasMessagesUxnativeBandwidthReceiveReport(ref x) => x.ser(buf),
            Message::UxasMessagesUxnativeSubTaskExecution(ref x) => x.ser(buf),
            Message::UxasMessagesUxnativeSubTaskAssignment(ref x) => x.ser(buf),
            Message::UxasMessagesUxnativeAutopilotKeepAlive(ref x) => x.ser(buf),
            Message::UxasMessagesUxnativeOnboardStatusReport(ref x) => x.ser(buf),
            Message::UxasMessagesUxnativeEntityJoin(ref x) => x.ser(buf),
            Message::UxasMessagesUxnativeEntityExit(ref x) => x.ser(buf),
            Message::UxasMessagesUxnativeSimulationTimeStepAcknowledgement(ref x) => x.ser(buf),
            Message::UxasMessagesUxnativeSpeedOverrideAction(ref x) => x.ser(buf),
            Message::UxasMessagesRouteGraphNode(ref x) => x.ser(buf),
            Message::UxasMessagesRouteGraphEdge(ref x) => x.ser(buf),
            Message::UxasMessagesRouteGraphRegion(ref x) => x.ser(buf),
            Message::UxasMessagesRouteRouteConstraints(ref x) => x.ser(buf),
            Message::UxasMessagesRouteRouteRequest(ref x) => x.ser(buf),
            Message::UxasMessagesRouteRoutePlanRequest(ref x) => x.ser(buf),
            Message::UxasMessagesRouteRoutePlan(ref x) => x.ser(buf),
            Message::UxasMessagesRouteRoutePlanResponse(ref x) => x.ser(buf),
            Message::UxasMessagesRouteRouteResponse(ref x) => x.ser(buf),
            Message::UxasMessagesRouteEgressRouteRequest(ref x) => x.ser(buf),
            Message::UxasMessagesRouteEgressRouteResponse(ref x) => x.ser(buf),
            Message::UxasMessagesRouteRoadPointsConstraints(ref x) => x.ser(buf),
            Message::UxasMessagesRouteRoadPointsRequest(ref x) => x.ser(buf),
            Message::UxasMessagesRouteRoadPointsResponse(ref x) => x.ser(buf),
            Message::AfrlVehiclesGroundVehicleConfiguration(ref x) => x.ser(buf),
            Message::AfrlVehiclesGroundVehicleState(ref x) => x.ser(buf),
            Message::AfrlVehiclesSurfaceVehicleConfiguration(ref x) => x.ser(buf),
            Message::AfrlVehiclesSurfaceVehicleState(ref x) => x.ser(buf),
            Message::AfrlVehiclesStationarySensorConfiguration(ref x) => x.ser(buf),
            Message::AfrlVehiclesStationarySensorState(ref x) => x.ser(buf),
            Message::AfrlCmasiAbstractGeometry(ref x) => x.ser(buf),
            Message::AfrlCmasiKeyValuePair(ref x) => x.ser(buf),
            Message::AfrlCmasiLocation3D(ref x) => x.ser(buf),
            Message::AfrlCmasiPayloadAction(ref x) => x.ser(buf),
            Message::AfrlCmasiPayloadConfiguration(ref x) => x.ser(buf),
            Message::AfrlCmasiPayloadState(ref x) => x.ser(buf),
            Message::AfrlCmasiVehicleAction(ref x) => x.ser(buf),
            Message::AfrlCmasiTask(ref x) => x.ser(buf),
            Message::AfrlCmasiSearchTask(ref x) => x.ser(buf),
            Message::AfrlCmasiAbstractZone(ref x) => x.ser(buf),
            Message::AfrlCmasiEntityConfiguration(ref x) => x.ser(buf),
            Message::AfrlCmasiFlightProfile(ref x) => x.ser(buf),
            Message::AfrlCmasiAirVehicleConfiguration(ref x) => x.ser(buf),
            Message::AfrlCmasiEntityState(ref x) => x.ser(buf),
            Message::AfrlCmasiAirVehicleState(ref x) => x.ser(buf),
            Message::AfrlCmasiWedge(ref x) => x.ser(buf),
            Message::AfrlCmasiAreaSearchTask(ref x) => x.ser(buf),
            Message::AfrlCmasiCameraAction(ref x) => x.ser(buf),
            Message::AfrlCmasiCameraConfiguration(ref x) => x.ser(buf),
            Message::AfrlCmasiGimballedPayloadState(ref x) => x.ser(buf),
            Message::AfrlCmasiCameraState(ref x) => x.ser(buf),
            Message::AfrlCmasiCircle(ref x) => x.ser(buf),
            Message::AfrlCmasiGimbalAngleAction(ref x) => x.ser(buf),
            Message::AfrlCmasiGimbalConfiguration(ref x) => x.ser(buf),
            Message::AfrlCmasiGimbalScanAction(ref x) => x.ser(buf),
            Message::AfrlCmasiGimbalStareAction(ref x) => x.ser(buf),
            Message::AfrlCmasiGimbalState(ref x) => x.ser(buf),
            Message::AfrlCmasiGoToWaypointAction(ref x) => x.ser(buf),
            Message::AfrlCmasiKeepInZone(ref x) => x.ser(buf),
            Message::AfrlCmasiKeepOutZone(ref x) => x.ser(buf),
            Message::AfrlCmasiLineSearchTask(ref x) => x.ser(buf),
            Message::AfrlCmasiNavigationAction(ref x) => x.ser(buf),
            Message::AfrlCmasiLoiterAction(ref x) => x.ser(buf),
            Message::AfrlCmasiLoiterTask(ref x) => x.ser(buf),
            Message::AfrlCmasiWaypoint(ref x) => x.ser(buf),
            Message::AfrlCmasiMissionCommand(ref x) => x.ser(buf),
            Message::AfrlCmasiMustFlyTask(ref x) => x.ser(buf),
            Message::AfrlCmasiOperatorSignal(ref x) => x.ser(buf),
            Message::AfrlCmasiOperatingRegion(ref x) => x.ser(buf),
            Message::AfrlCmasiAutomationRequest(ref x) => x.ser(buf),
            Message::AfrlCmasiPointSearchTask(ref x) => x.ser(buf),
            Message::AfrlCmasiPolygon(ref x) => x.ser(buf),
            Message::AfrlCmasiRectangle(ref x) => x.ser(buf),
            Message::AfrlCmasiRemoveTasks(ref x) => x.ser(buf),
            Message::AfrlCmasiServiceStatus(ref x) => x.ser(buf),
            Message::AfrlCmasiSessionStatus(ref x) => x.ser(buf),
            Message::AfrlCmasiVehicleActionCommand(ref x) => x.ser(buf),
            Message::AfrlCmasiVideoStreamAction(ref x) => x.ser(buf),
            Message::AfrlCmasiVideoStreamConfiguration(ref x) => x.ser(buf),
            Message::AfrlCmasiVideoStreamState(ref x) => x.ser(buf),
            Message::AfrlCmasiAutomationResponse(ref x) => x.ser(buf),
            Message::AfrlCmasiRemoveZones(ref x) => x.ser(buf),
            Message::AfrlCmasiRemoveEntities(ref x) => x.ser(buf),
            Message::AfrlCmasiFlightDirectorAction(ref x) => x.ser(buf),
            Message::AfrlCmasiWeatherReport(ref x) => x.ser(buf),
            Message::AfrlCmasiFollowPathCommand(ref x) => x.ser(buf),
            Message::AfrlCmasiPathWaypoint(ref x) => x.ser(buf),
            Message::AfrlCmasiStopMovementAction(ref x) => x.ser(buf),
            Message::AfrlCmasiWaypointTransfer(ref x) => x.ser(buf),
            Message::AfrlCmasiPayloadStowAction(ref x) => x.ser(buf),
            Message::AfrlImpactPowerConfiguration(ref x) => x.ser(buf),
            Message::AfrlImpactRadioConfiguration(ref x) => x.ser(buf),
            Message::AfrlImpactRadioTowerConfiguration(ref x) => x.ser(buf),
            Message::AfrlImpactRadioState(ref x) => x.ser(buf),
            Message::AfrlImpactRadioTowerState(ref x) => x.ser(buf),
            Message::AfrlImpactImpactPayloadConfiguration(ref x) => x.ser(buf),
            Message::AfrlImpactDeployImpactPayload(ref x) => x.ser(buf),
            Message::AfrlImpactPowerPlantState(ref x) => x.ser(buf),
            Message::AfrlImpactBatchRoutePlanRequest(ref x) => x.ser(buf),
            Message::AfrlImpactBatchRoutePlanResponse(ref x) => x.ser(buf),
            Message::AfrlImpactTaskTimingPair(ref x) => x.ser(buf),
            Message::AfrlImpactBatchSummaryRequest(ref x) => x.ser(buf),
            Message::AfrlImpactBatchSummaryResponse(ref x) => x.ser(buf),
            Message::AfrlImpactTaskSummary(ref x) => x.ser(buf),
            Message::AfrlImpactVehicleSummary(ref x) => x.ser(buf),
            Message::AfrlImpactSpeedAltPair(ref x) => x.ser(buf),
            Message::AfrlImpactImpactAutomationRequest(ref x) => x.ser(buf),
            Message::AfrlImpactImpactAutomationResponse(ref x) => x.ser(buf),
            Message::AfrlImpactPointOfInterest(ref x) => x.ser(buf),
            Message::AfrlImpactLineOfInterest(ref x) => x.ser(buf),
            Message::AfrlImpactAreaOfInterest(ref x) => x.ser(buf),
            Message::AfrlImpactImpactPointSearchTask(ref x) => x.ser(buf),
            Message::AfrlImpactPatternSearchTask(ref x) => x.ser(buf),
            Message::AfrlImpactAngledAreaSearchTask(ref x) => x.ser(buf),
            Message::AfrlImpactImpactLineSearchTask(ref x) => x.ser(buf),
            Message::AfrlImpactWatchTask(ref x) => x.ser(buf),
            Message::AfrlImpactMultiVehicleWatchTask(ref x) => x.ser(buf),
            Message::AfrlImpactCommRelayTask(ref x) => x.ser(buf),
            Message::AfrlImpactCordonTask(ref x) => x.ser(buf),
            Message::AfrlImpactBlockadeTask(ref x) => x.ser(buf),
            Message::AfrlImpactEscortTask(ref x) => x.ser(buf),
            Message::AfrlImpactConfigurationRequest(ref x) => x.ser(buf),
            Message::AfrlImpactWaterReport(ref x) => x.ser(buf),
            Message::AfrlImpactWaterZone(ref x) => x.ser(buf),
            Message::AfrlImpactPayloadDropTask(ref x) => x.ser(buf),
            Message::AfrlCmasiPerceiveEntityPerception(ref x) => x.ser(buf),
            Message::AfrlCmasiPerceiveTrackEntityAction(ref x) => x.ser(buf),
            Message::AfrlCmasiPerceiveTrackEntityTask(ref x) => x.ser(buf),

        }
    }

    fn size(&self) -> usize {
        match *self {
            Message::UxasMessagesTaskAssignmentCoordinatorTask(ref x) => x.size(),
            Message::UxasMessagesTaskRendezvousTask(ref x) => x.size(),
            Message::UxasMessagesTaskPlanningState(ref x) => x.size(),
            Message::UxasMessagesTaskAssignmentCoordination(ref x) => x.size(),
            Message::UxasMessagesTaskCoordinatedAutomationRequest(ref x) => x.size(),
            Message::UxasMessagesTaskTaskAutomationRequest(ref x) => x.size(),
            Message::UxasMessagesTaskTaskAutomationResponse(ref x) => x.size(),
            Message::UxasMessagesTaskUniqueAutomationRequest(ref x) => x.size(),
            Message::UxasMessagesTaskUniqueAutomationResponse(ref x) => x.size(),
            Message::UxasMessagesTaskSensorFootprintRequests(ref x) => x.size(),
            Message::UxasMessagesTaskFootprintRequest(ref x) => x.size(),
            Message::UxasMessagesTaskSensorFootprint(ref x) => x.size(),
            Message::UxasMessagesTaskSensorFootprintResponse(ref x) => x.size(),
            Message::UxasMessagesTaskTaskImplementationRequest(ref x) => x.size(),
            Message::UxasMessagesTaskTaskImplementationResponse(ref x) => x.size(),
            Message::UxasMessagesTaskAssignmentCostMatrix(ref x) => x.size(),
            Message::UxasMessagesTaskTaskOptionCost(ref x) => x.size(),
            Message::UxasMessagesTaskTaskAssignment(ref x) => x.size(),
            Message::UxasMessagesTaskTaskAssignmentSummary(ref x) => x.size(),
            Message::UxasMessagesTaskTaskOption(ref x) => x.size(),
            Message::UxasMessagesTaskTaskPlanOptions(ref x) => x.size(),
            Message::UxasMessagesTaskTaskPause(ref x) => x.size(),
            Message::UxasMessagesTaskTaskResume(ref x) => x.size(),
            Message::UxasMessagesTaskTaskProgress(ref x) => x.size(),
            Message::UxasMessagesTaskTaskProgressRequest(ref x) => x.size(),
            Message::UxasMessagesTaskTaskInitialized(ref x) => x.size(),
            Message::UxasMessagesTaskTaskActive(ref x) => x.size(),
            Message::UxasMessagesTaskTaskComplete(ref x) => x.size(),
            Message::UxasMessagesTaskCancelTask(ref x) => x.size(),
            Message::UxasMessagesUxnativeVideoRecord(ref x) => x.size(),
            Message::UxasMessagesUxnativeStartupComplete(ref x) => x.size(),
            Message::UxasMessagesUxnativeCreateNewService(ref x) => x.size(),
            Message::UxasMessagesUxnativeKillService(ref x) => x.size(),
            Message::UxasMessagesUxnativeIncrementWaypoint(ref x) => x.size(),
            Message::UxasMessagesUxnativeSafeHeadingAction(ref x) => x.size(),
            Message::UxasMessagesUxnativeEntityLocation(ref x) => x.size(),
            Message::UxasMessagesUxnativeBandwidthTest(ref x) => x.size(),
            Message::UxasMessagesUxnativeBandwidthReceiveReport(ref x) => x.size(),
            Message::UxasMessagesUxnativeSubTaskExecution(ref x) => x.size(),
            Message::UxasMessagesUxnativeSubTaskAssignment(ref x) => x.size(),
            Message::UxasMessagesUxnativeAutopilotKeepAlive(ref x) => x.size(),
            Message::UxasMessagesUxnativeOnboardStatusReport(ref x) => x.size(),
            Message::UxasMessagesUxnativeEntityJoin(ref x) => x.size(),
            Message::UxasMessagesUxnativeEntityExit(ref x) => x.size(),
            Message::UxasMessagesUxnativeSimulationTimeStepAcknowledgement(ref x) => x.size(),
            Message::UxasMessagesUxnativeSpeedOverrideAction(ref x) => x.size(),
            Message::UxasMessagesRouteGraphNode(ref x) => x.size(),
            Message::UxasMessagesRouteGraphEdge(ref x) => x.size(),
            Message::UxasMessagesRouteGraphRegion(ref x) => x.size(),
            Message::UxasMessagesRouteRouteConstraints(ref x) => x.size(),
            Message::UxasMessagesRouteRouteRequest(ref x) => x.size(),
            Message::UxasMessagesRouteRoutePlanRequest(ref x) => x.size(),
            Message::UxasMessagesRouteRoutePlan(ref x) => x.size(),
            Message::UxasMessagesRouteRoutePlanResponse(ref x) => x.size(),
            Message::UxasMessagesRouteRouteResponse(ref x) => x.size(),
            Message::UxasMessagesRouteEgressRouteRequest(ref x) => x.size(),
            Message::UxasMessagesRouteEgressRouteResponse(ref x) => x.size(),
            Message::UxasMessagesRouteRoadPointsConstraints(ref x) => x.size(),
            Message::UxasMessagesRouteRoadPointsRequest(ref x) => x.size(),
            Message::UxasMessagesRouteRoadPointsResponse(ref x) => x.size(),
            Message::AfrlVehiclesGroundVehicleConfiguration(ref x) => x.size(),
            Message::AfrlVehiclesGroundVehicleState(ref x) => x.size(),
            Message::AfrlVehiclesSurfaceVehicleConfiguration(ref x) => x.size(),
            Message::AfrlVehiclesSurfaceVehicleState(ref x) => x.size(),
            Message::AfrlVehiclesStationarySensorConfiguration(ref x) => x.size(),
            Message::AfrlVehiclesStationarySensorState(ref x) => x.size(),
            Message::AfrlCmasiAbstractGeometry(ref x) => x.size(),
            Message::AfrlCmasiKeyValuePair(ref x) => x.size(),
            Message::AfrlCmasiLocation3D(ref x) => x.size(),
            Message::AfrlCmasiPayloadAction(ref x) => x.size(),
            Message::AfrlCmasiPayloadConfiguration(ref x) => x.size(),
            Message::AfrlCmasiPayloadState(ref x) => x.size(),
            Message::AfrlCmasiVehicleAction(ref x) => x.size(),
            Message::AfrlCmasiTask(ref x) => x.size(),
            Message::AfrlCmasiSearchTask(ref x) => x.size(),
            Message::AfrlCmasiAbstractZone(ref x) => x.size(),
            Message::AfrlCmasiEntityConfiguration(ref x) => x.size(),
            Message::AfrlCmasiFlightProfile(ref x) => x.size(),
            Message::AfrlCmasiAirVehicleConfiguration(ref x) => x.size(),
            Message::AfrlCmasiEntityState(ref x) => x.size(),
            Message::AfrlCmasiAirVehicleState(ref x) => x.size(),
            Message::AfrlCmasiWedge(ref x) => x.size(),
            Message::AfrlCmasiAreaSearchTask(ref x) => x.size(),
            Message::AfrlCmasiCameraAction(ref x) => x.size(),
            Message::AfrlCmasiCameraConfiguration(ref x) => x.size(),
            Message::AfrlCmasiGimballedPayloadState(ref x) => x.size(),
            Message::AfrlCmasiCameraState(ref x) => x.size(),
            Message::AfrlCmasiCircle(ref x) => x.size(),
            Message::AfrlCmasiGimbalAngleAction(ref x) => x.size(),
            Message::AfrlCmasiGimbalConfiguration(ref x) => x.size(),
            Message::AfrlCmasiGimbalScanAction(ref x) => x.size(),
            Message::AfrlCmasiGimbalStareAction(ref x) => x.size(),
            Message::AfrlCmasiGimbalState(ref x) => x.size(),
            Message::AfrlCmasiGoToWaypointAction(ref x) => x.size(),
            Message::AfrlCmasiKeepInZone(ref x) => x.size(),
            Message::AfrlCmasiKeepOutZone(ref x) => x.size(),
            Message::AfrlCmasiLineSearchTask(ref x) => x.size(),
            Message::AfrlCmasiNavigationAction(ref x) => x.size(),
            Message::AfrlCmasiLoiterAction(ref x) => x.size(),
            Message::AfrlCmasiLoiterTask(ref x) => x.size(),
            Message::AfrlCmasiWaypoint(ref x) => x.size(),
            Message::AfrlCmasiMissionCommand(ref x) => x.size(),
            Message::AfrlCmasiMustFlyTask(ref x) => x.size(),
            Message::AfrlCmasiOperatorSignal(ref x) => x.size(),
            Message::AfrlCmasiOperatingRegion(ref x) => x.size(),
            Message::AfrlCmasiAutomationRequest(ref x) => x.size(),
            Message::AfrlCmasiPointSearchTask(ref x) => x.size(),
            Message::AfrlCmasiPolygon(ref x) => x.size(),
            Message::AfrlCmasiRectangle(ref x) => x.size(),
            Message::AfrlCmasiRemoveTasks(ref x) => x.size(),
            Message::AfrlCmasiServiceStatus(ref x) => x.size(),
            Message::AfrlCmasiSessionStatus(ref x) => x.size(),
            Message::AfrlCmasiVehicleActionCommand(ref x) => x.size(),
            Message::AfrlCmasiVideoStreamAction(ref x) => x.size(),
            Message::AfrlCmasiVideoStreamConfiguration(ref x) => x.size(),
            Message::AfrlCmasiVideoStreamState(ref x) => x.size(),
            Message::AfrlCmasiAutomationResponse(ref x) => x.size(),
            Message::AfrlCmasiRemoveZones(ref x) => x.size(),
            Message::AfrlCmasiRemoveEntities(ref x) => x.size(),
            Message::AfrlCmasiFlightDirectorAction(ref x) => x.size(),
            Message::AfrlCmasiWeatherReport(ref x) => x.size(),
            Message::AfrlCmasiFollowPathCommand(ref x) => x.size(),
            Message::AfrlCmasiPathWaypoint(ref x) => x.size(),
            Message::AfrlCmasiStopMovementAction(ref x) => x.size(),
            Message::AfrlCmasiWaypointTransfer(ref x) => x.size(),
            Message::AfrlCmasiPayloadStowAction(ref x) => x.size(),
            Message::AfrlImpactPowerConfiguration(ref x) => x.size(),
            Message::AfrlImpactRadioConfiguration(ref x) => x.size(),
            Message::AfrlImpactRadioTowerConfiguration(ref x) => x.size(),
            Message::AfrlImpactRadioState(ref x) => x.size(),
            Message::AfrlImpactRadioTowerState(ref x) => x.size(),
            Message::AfrlImpactImpactPayloadConfiguration(ref x) => x.size(),
            Message::AfrlImpactDeployImpactPayload(ref x) => x.size(),
            Message::AfrlImpactPowerPlantState(ref x) => x.size(),
            Message::AfrlImpactBatchRoutePlanRequest(ref x) => x.size(),
            Message::AfrlImpactBatchRoutePlanResponse(ref x) => x.size(),
            Message::AfrlImpactTaskTimingPair(ref x) => x.size(),
            Message::AfrlImpactBatchSummaryRequest(ref x) => x.size(),
            Message::AfrlImpactBatchSummaryResponse(ref x) => x.size(),
            Message::AfrlImpactTaskSummary(ref x) => x.size(),
            Message::AfrlImpactVehicleSummary(ref x) => x.size(),
            Message::AfrlImpactSpeedAltPair(ref x) => x.size(),
            Message::AfrlImpactImpactAutomationRequest(ref x) => x.size(),
            Message::AfrlImpactImpactAutomationResponse(ref x) => x.size(),
            Message::AfrlImpactPointOfInterest(ref x) => x.size(),
            Message::AfrlImpactLineOfInterest(ref x) => x.size(),
            Message::AfrlImpactAreaOfInterest(ref x) => x.size(),
            Message::AfrlImpactImpactPointSearchTask(ref x) => x.size(),
            Message::AfrlImpactPatternSearchTask(ref x) => x.size(),
            Message::AfrlImpactAngledAreaSearchTask(ref x) => x.size(),
            Message::AfrlImpactImpactLineSearchTask(ref x) => x.size(),
            Message::AfrlImpactWatchTask(ref x) => x.size(),
            Message::AfrlImpactMultiVehicleWatchTask(ref x) => x.size(),
            Message::AfrlImpactCommRelayTask(ref x) => x.size(),
            Message::AfrlImpactCordonTask(ref x) => x.size(),
            Message::AfrlImpactBlockadeTask(ref x) => x.size(),
            Message::AfrlImpactEscortTask(ref x) => x.size(),
            Message::AfrlImpactConfigurationRequest(ref x) => x.size(),
            Message::AfrlImpactWaterReport(ref x) => x.size(),
            Message::AfrlImpactWaterZone(ref x) => x.size(),
            Message::AfrlImpactPayloadDropTask(ref x) => x.size(),
            Message::AfrlCmasiPerceiveEntityPerception(ref x) => x.size(),
            Message::AfrlCmasiPerceiveTrackEntityAction(ref x) => x.size(),
            Message::AfrlCmasiPerceiveTrackEntityTask(ref x) => x.size(),

        }
    }

    fn deser(buf: &[u8]) -> Result<(Message, usize), Error> {
        let (si, _) = StructInfo::deser(buf)?; // TODO: support null structs or get rid of them
        match (si.series, si.struct_ty) {
            (6149757930721443840, 1) => {
                let (s, i) = uxas::messages::task::assignment_coordinator_task::AssignmentCoordinatorTask::deser(buf)?;
                Ok((Message::UxasMessagesTaskAssignmentCoordinatorTask(s), i))
            }
            (6149757930721443840, 2) => {
                let (s, i) = uxas::messages::task::rendezvous_task::RendezvousTask::deser(buf)?;
                Ok((Message::UxasMessagesTaskRendezvousTask(s), i))
            }
            (6149757930721443840, 3) => {
                let (s, i) = uxas::messages::task::planning_state::PlanningState::deser(buf)?;
                Ok((Message::UxasMessagesTaskPlanningState(s), i))
            }
            (6149757930721443840, 4) => {
                let (s, i) = uxas::messages::task::assignment_coordination::AssignmentCoordination::deser(buf)?;
                Ok((Message::UxasMessagesTaskAssignmentCoordination(s), i))
            }
            (6149757930721443840, 5) => {
                let (s, i) = uxas::messages::task::coordinated_automation_request::CoordinatedAutomationRequest::deser(buf)?;
                Ok((Message::UxasMessagesTaskCoordinatedAutomationRequest(s), i))
            }
            (6149757930721443840, 6) => {
                let (s, i) = uxas::messages::task::task_automation_request::TaskAutomationRequest::deser(buf)?;
                Ok((Message::UxasMessagesTaskTaskAutomationRequest(s), i))
            }
            (6149757930721443840, 7) => {
                let (s, i) = uxas::messages::task::task_automation_response::TaskAutomationResponse::deser(buf)?;
                Ok((Message::UxasMessagesTaskTaskAutomationResponse(s), i))
            }
            (6149757930721443840, 8) => {
                let (s, i) = uxas::messages::task::unique_automation_request::UniqueAutomationRequest::deser(buf)?;
                Ok((Message::UxasMessagesTaskUniqueAutomationRequest(s), i))
            }
            (6149757930721443840, 9) => {
                let (s, i) = uxas::messages::task::unique_automation_response::UniqueAutomationResponse::deser(buf)?;
                Ok((Message::UxasMessagesTaskUniqueAutomationResponse(s), i))
            }
            (6149757930721443840, 10) => {
                let (s, i) = uxas::messages::task::sensor_footprint_requests::SensorFootprintRequests::deser(buf)?;
                Ok((Message::UxasMessagesTaskSensorFootprintRequests(s), i))
            }
            (6149757930721443840, 11) => {
                let (s, i) = uxas::messages::task::footprint_request::FootprintRequest::deser(buf)?;
                Ok((Message::UxasMessagesTaskFootprintRequest(s), i))
            }
            (6149757930721443840, 12) => {
                let (s, i) = uxas::messages::task::sensor_footprint::SensorFootprint::deser(buf)?;
                Ok((Message::UxasMessagesTaskSensorFootprint(s), i))
            }
            (6149757930721443840, 13) => {
                let (s, i) = uxas::messages::task::sensor_footprint_response::SensorFootprintResponse::deser(buf)?;
                Ok((Message::UxasMessagesTaskSensorFootprintResponse(s), i))
            }
            (6149757930721443840, 14) => {
                let (s, i) = uxas::messages::task::task_implementation_request::TaskImplementationRequest::deser(buf)?;
                Ok((Message::UxasMessagesTaskTaskImplementationRequest(s), i))
            }
            (6149757930721443840, 15) => {
                let (s, i) = uxas::messages::task::task_implementation_response::TaskImplementationResponse::deser(buf)?;
                Ok((Message::UxasMessagesTaskTaskImplementationResponse(s), i))
            }
            (6149757930721443840, 16) => {
                let (s, i) = uxas::messages::task::assignment_cost_matrix::AssignmentCostMatrix::deser(buf)?;
                Ok((Message::UxasMessagesTaskAssignmentCostMatrix(s), i))
            }
            (6149757930721443840, 17) => {
                let (s, i) = uxas::messages::task::task_option_cost::TaskOptionCost::deser(buf)?;
                Ok((Message::UxasMessagesTaskTaskOptionCost(s), i))
            }
            (6149757930721443840, 18) => {
                let (s, i) = uxas::messages::task::task_assignment::TaskAssignment::deser(buf)?;
                Ok((Message::UxasMessagesTaskTaskAssignment(s), i))
            }
            (6149757930721443840, 19) => {
                let (s, i) = uxas::messages::task::task_assignment_summary::TaskAssignmentSummary::deser(buf)?;
                Ok((Message::UxasMessagesTaskTaskAssignmentSummary(s), i))
            }
            (6149757930721443840, 20) => {
                let (s, i) = uxas::messages::task::task_option::TaskOption::deser(buf)?;
                Ok((Message::UxasMessagesTaskTaskOption(s), i))
            }
            (6149757930721443840, 21) => {
                let (s, i) = uxas::messages::task::task_plan_options::TaskPlanOptions::deser(buf)?;
                Ok((Message::UxasMessagesTaskTaskPlanOptions(s), i))
            }
            (6149757930721443840, 22) => {
                let (s, i) = uxas::messages::task::task_pause::TaskPause::deser(buf)?;
                Ok((Message::UxasMessagesTaskTaskPause(s), i))
            }
            (6149757930721443840, 23) => {
                let (s, i) = uxas::messages::task::task_resume::TaskResume::deser(buf)?;
                Ok((Message::UxasMessagesTaskTaskResume(s), i))
            }
            (6149757930721443840, 24) => {
                let (s, i) = uxas::messages::task::task_progress::TaskProgress::deser(buf)?;
                Ok((Message::UxasMessagesTaskTaskProgress(s), i))
            }
            (6149757930721443840, 25) => {
                let (s, i) = uxas::messages::task::task_progress_request::TaskProgressRequest::deser(buf)?;
                Ok((Message::UxasMessagesTaskTaskProgressRequest(s), i))
            }
            (6149757930721443840, 26) => {
                let (s, i) = uxas::messages::task::task_initialized::TaskInitialized::deser(buf)?;
                Ok((Message::UxasMessagesTaskTaskInitialized(s), i))
            }
            (6149757930721443840, 27) => {
                let (s, i) = uxas::messages::task::task_active::TaskActive::deser(buf)?;
                Ok((Message::UxasMessagesTaskTaskActive(s), i))
            }
            (6149757930721443840, 28) => {
                let (s, i) = uxas::messages::task::task_complete::TaskComplete::deser(buf)?;
                Ok((Message::UxasMessagesTaskTaskComplete(s), i))
            }
            (6149757930721443840, 29) => {
                let (s, i) = uxas::messages::task::cancel_task::CancelTask::deser(buf)?;
                Ok((Message::UxasMessagesTaskCancelTask(s), i))
            }
            (6149751333668345413, 1) => {
                let (s, i) = uxas::messages::uxnative::video_record::VideoRecord::deser(buf)?;
                Ok((Message::UxasMessagesUxnativeVideoRecord(s), i))
            }
            (6149751333668345413, 2) => {
                let (s, i) = uxas::messages::uxnative::startup_complete::StartupComplete::deser(buf)?;
                Ok((Message::UxasMessagesUxnativeStartupComplete(s), i))
            }
            (6149751333668345413, 3) => {
                let (s, i) = uxas::messages::uxnative::create_new_service::CreateNewService::deser(buf)?;
                Ok((Message::UxasMessagesUxnativeCreateNewService(s), i))
            }
            (6149751333668345413, 4) => {
                let (s, i) = uxas::messages::uxnative::kill_service::KillService::deser(buf)?;
                Ok((Message::UxasMessagesUxnativeKillService(s), i))
            }
            (6149751333668345413, 5) => {
                let (s, i) = uxas::messages::uxnative::increment_waypoint::IncrementWaypoint::deser(buf)?;
                Ok((Message::UxasMessagesUxnativeIncrementWaypoint(s), i))
            }
            (6149751333668345413, 6) => {
                let (s, i) = uxas::messages::uxnative::safe_heading_action::SafeHeadingAction::deser(buf)?;
                Ok((Message::UxasMessagesUxnativeSafeHeadingAction(s), i))
            }
            (6149751333668345413, 7) => {
                let (s, i) = uxas::messages::uxnative::entity_location::EntityLocation::deser(buf)?;
                Ok((Message::UxasMessagesUxnativeEntityLocation(s), i))
            }
            (6149751333668345413, 8) => {
                let (s, i) = uxas::messages::uxnative::bandwidth_test::BandwidthTest::deser(buf)?;
                Ok((Message::UxasMessagesUxnativeBandwidthTest(s), i))
            }
            (6149751333668345413, 9) => {
                let (s, i) = uxas::messages::uxnative::bandwidth_receive_report::BandwidthReceiveReport::deser(buf)?;
                Ok((Message::UxasMessagesUxnativeBandwidthReceiveReport(s), i))
            }
            (6149751333668345413, 10) => {
                let (s, i) = uxas::messages::uxnative::sub_task_execution::SubTaskExecution::deser(buf)?;
                Ok((Message::UxasMessagesUxnativeSubTaskExecution(s), i))
            }
            (6149751333668345413, 11) => {
                let (s, i) = uxas::messages::uxnative::sub_task_assignment::SubTaskAssignment::deser(buf)?;
                Ok((Message::UxasMessagesUxnativeSubTaskAssignment(s), i))
            }
            (6149751333668345413, 12) => {
                let (s, i) = uxas::messages::uxnative::autopilot_keep_alive::AutopilotKeepAlive::deser(buf)?;
                Ok((Message::UxasMessagesUxnativeAutopilotKeepAlive(s), i))
            }
            (6149751333668345413, 13) => {
                let (s, i) = uxas::messages::uxnative::onboard_status_report::OnboardStatusReport::deser(buf)?;
                Ok((Message::UxasMessagesUxnativeOnboardStatusReport(s), i))
            }
            (6149751333668345413, 14) => {
                let (s, i) = uxas::messages::uxnative::entity_join::EntityJoin::deser(buf)?;
                Ok((Message::UxasMessagesUxnativeEntityJoin(s), i))
            }
            (6149751333668345413, 15) => {
                let (s, i) = uxas::messages::uxnative::entity_exit::EntityExit::deser(buf)?;
                Ok((Message::UxasMessagesUxnativeEntityExit(s), i))
            }
            (6149751333668345413, 16) => {
                let (s, i) = uxas::messages::uxnative::simulation_time_step_acknowledgement::SimulationTimeStepAcknowledgement::deser(buf)?;
                Ok((Message::UxasMessagesUxnativeSimulationTimeStepAcknowledgement(s), i))
            }
            (6149751333668345413, 17) => {
                let (s, i) = uxas::messages::uxnative::speed_override_action::SpeedOverrideAction::deser(buf)?;
                Ok((Message::UxasMessagesUxnativeSpeedOverrideAction(s), i))
            }
            (5931053054693474304, 1) => {
                let (s, i) = uxas::messages::route::graph_node::GraphNode::deser(buf)?;
                Ok((Message::UxasMessagesRouteGraphNode(s), i))
            }
            (5931053054693474304, 2) => {
                let (s, i) = uxas::messages::route::graph_edge::GraphEdge::deser(buf)?;
                Ok((Message::UxasMessagesRouteGraphEdge(s), i))
            }
            (5931053054693474304, 3) => {
                let (s, i) = uxas::messages::route::graph_region::GraphRegion::deser(buf)?;
                Ok((Message::UxasMessagesRouteGraphRegion(s), i))
            }
            (5931053054693474304, 4) => {
                let (s, i) = uxas::messages::route::route_constraints::RouteConstraints::deser(buf)?;
                Ok((Message::UxasMessagesRouteRouteConstraints(s), i))
            }
            (5931053054693474304, 5) => {
                let (s, i) = uxas::messages::route::route_request::RouteRequest::deser(buf)?;
                Ok((Message::UxasMessagesRouteRouteRequest(s), i))
            }
            (5931053054693474304, 6) => {
                let (s, i) = uxas::messages::route::route_plan_request::RoutePlanRequest::deser(buf)?;
                Ok((Message::UxasMessagesRouteRoutePlanRequest(s), i))
            }
            (5931053054693474304, 7) => {
                let (s, i) = uxas::messages::route::route_plan::RoutePlan::deser(buf)?;
                Ok((Message::UxasMessagesRouteRoutePlan(s), i))
            }
            (5931053054693474304, 8) => {
                let (s, i) = uxas::messages::route::route_plan_response::RoutePlanResponse::deser(buf)?;
                Ok((Message::UxasMessagesRouteRoutePlanResponse(s), i))
            }
            (5931053054693474304, 9) => {
                let (s, i) = uxas::messages::route::route_response::RouteResponse::deser(buf)?;
                Ok((Message::UxasMessagesRouteRouteResponse(s), i))
            }
            (5931053054693474304, 10) => {
                let (s, i) = uxas::messages::route::egress_route_request::EgressRouteRequest::deser(buf)?;
                Ok((Message::UxasMessagesRouteEgressRouteRequest(s), i))
            }
            (5931053054693474304, 11) => {
                let (s, i) = uxas::messages::route::egress_route_response::EgressRouteResponse::deser(buf)?;
                Ok((Message::UxasMessagesRouteEgressRouteResponse(s), i))
            }
            (5931053054693474304, 12) => {
                let (s, i) = uxas::messages::route::road_points_constraints::RoadPointsConstraints::deser(buf)?;
                Ok((Message::UxasMessagesRouteRoadPointsConstraints(s), i))
            }
            (5931053054693474304, 13) => {
                let (s, i) = uxas::messages::route::road_points_request::RoadPointsRequest::deser(buf)?;
                Ok((Message::UxasMessagesRouteRoadPointsRequest(s), i))
            }
            (5931053054693474304, 14) => {
                let (s, i) = uxas::messages::route::road_points_response::RoadPointsResponse::deser(buf)?;
                Ok((Message::UxasMessagesRouteRoadPointsResponse(s), i))
            }
            (6216454340153722195, 1) => {
                let (s, i) = afrl::vehicles::ground_vehicle_configuration::GroundVehicleConfiguration::deser(buf)?;
                Ok((Message::AfrlVehiclesGroundVehicleConfiguration(s), i))
            }
            (6216454340153722195, 2) => {
                let (s, i) = afrl::vehicles::ground_vehicle_state::GroundVehicleState::deser(buf)?;
                Ok((Message::AfrlVehiclesGroundVehicleState(s), i))
            }
            (6216454340153722195, 3) => {
                let (s, i) = afrl::vehicles::surface_vehicle_configuration::SurfaceVehicleConfiguration::deser(buf)?;
                Ok((Message::AfrlVehiclesSurfaceVehicleConfiguration(s), i))
            }
            (6216454340153722195, 4) => {
                let (s, i) = afrl::vehicles::surface_vehicle_state::SurfaceVehicleState::deser(buf)?;
                Ok((Message::AfrlVehiclesSurfaceVehicleState(s), i))
            }
            (6216454340153722195, 5) => {
                let (s, i) = afrl::vehicles::stationary_sensor_configuration::StationarySensorConfiguration::deser(buf)?;
                Ok((Message::AfrlVehiclesStationarySensorConfiguration(s), i))
            }
            (6216454340153722195, 6) => {
                let (s, i) = afrl::vehicles::stationary_sensor_state::StationarySensorState::deser(buf)?;
                Ok((Message::AfrlVehiclesStationarySensorState(s), i))
            }
            (4849604199710720000, 1) => {
                let (s, i) = afrl::cmasi::abstract_geometry::AbstractGeometry::deser(buf)?;
                Ok((Message::AfrlCmasiAbstractGeometry(s), i))
            }
            (4849604199710720000, 2) => {
                let (s, i) = afrl::cmasi::key_value_pair::KeyValuePair::deser(buf)?;
                Ok((Message::AfrlCmasiKeyValuePair(s), i))
            }
            (4849604199710720000, 3) => {
                let (s, i) = afrl::cmasi::location3d::Location3D::deser(buf)?;
                Ok((Message::AfrlCmasiLocation3D(s), i))
            }
            (4849604199710720000, 4) => {
                let (s, i) = afrl::cmasi::payload_action::PayloadAction::deser(buf)?;
                Ok((Message::AfrlCmasiPayloadAction(s), i))
            }
            (4849604199710720000, 5) => {
                let (s, i) = afrl::cmasi::payload_configuration::PayloadConfiguration::deser(buf)?;
                Ok((Message::AfrlCmasiPayloadConfiguration(s), i))
            }
            (4849604199710720000, 6) => {
                let (s, i) = afrl::cmasi::payload_state::PayloadState::deser(buf)?;
                Ok((Message::AfrlCmasiPayloadState(s), i))
            }
            (4849604199710720000, 7) => {
                let (s, i) = afrl::cmasi::vehicle_action::VehicleAction::deser(buf)?;
                Ok((Message::AfrlCmasiVehicleAction(s), i))
            }
            (4849604199710720000, 8) => {
                let (s, i) = afrl::cmasi::task::Task::deser(buf)?;
                Ok((Message::AfrlCmasiTask(s), i))
            }
            (4849604199710720000, 9) => {
                let (s, i) = afrl::cmasi::search_task::SearchTask::deser(buf)?;
                Ok((Message::AfrlCmasiSearchTask(s), i))
            }
            (4849604199710720000, 10) => {
                let (s, i) = afrl::cmasi::abstract_zone::AbstractZone::deser(buf)?;
                Ok((Message::AfrlCmasiAbstractZone(s), i))
            }
            (4849604199710720000, 11) => {
                let (s, i) = afrl::cmasi::entity_configuration::EntityConfiguration::deser(buf)?;
                Ok((Message::AfrlCmasiEntityConfiguration(s), i))
            }
            (4849604199710720000, 12) => {
                let (s, i) = afrl::cmasi::flight_profile::FlightProfile::deser(buf)?;
                Ok((Message::AfrlCmasiFlightProfile(s), i))
            }
            (4849604199710720000, 13) => {
                let (s, i) = afrl::cmasi::air_vehicle_configuration::AirVehicleConfiguration::deser(buf)?;
                Ok((Message::AfrlCmasiAirVehicleConfiguration(s), i))
            }
            (4849604199710720000, 14) => {
                let (s, i) = afrl::cmasi::entity_state::EntityState::deser(buf)?;
                Ok((Message::AfrlCmasiEntityState(s), i))
            }
            (4849604199710720000, 15) => {
                let (s, i) = afrl::cmasi::air_vehicle_state::AirVehicleState::deser(buf)?;
                Ok((Message::AfrlCmasiAirVehicleState(s), i))
            }
            (4849604199710720000, 16) => {
                let (s, i) = afrl::cmasi::wedge::Wedge::deser(buf)?;
                Ok((Message::AfrlCmasiWedge(s), i))
            }
            (4849604199710720000, 17) => {
                let (s, i) = afrl::cmasi::area_search_task::AreaSearchTask::deser(buf)?;
                Ok((Message::AfrlCmasiAreaSearchTask(s), i))
            }
            (4849604199710720000, 18) => {
                let (s, i) = afrl::cmasi::camera_action::CameraAction::deser(buf)?;
                Ok((Message::AfrlCmasiCameraAction(s), i))
            }
            (4849604199710720000, 19) => {
                let (s, i) = afrl::cmasi::camera_configuration::CameraConfiguration::deser(buf)?;
                Ok((Message::AfrlCmasiCameraConfiguration(s), i))
            }
            (4849604199710720000, 20) => {
                let (s, i) = afrl::cmasi::gimballed_payload_state::GimballedPayloadState::deser(buf)?;
                Ok((Message::AfrlCmasiGimballedPayloadState(s), i))
            }
            (4849604199710720000, 21) => {
                let (s, i) = afrl::cmasi::camera_state::CameraState::deser(buf)?;
                Ok((Message::AfrlCmasiCameraState(s), i))
            }
            (4849604199710720000, 22) => {
                let (s, i) = afrl::cmasi::circle::Circle::deser(buf)?;
                Ok((Message::AfrlCmasiCircle(s), i))
            }
            (4849604199710720000, 23) => {
                let (s, i) = afrl::cmasi::gimbal_angle_action::GimbalAngleAction::deser(buf)?;
                Ok((Message::AfrlCmasiGimbalAngleAction(s), i))
            }
            (4849604199710720000, 24) => {
                let (s, i) = afrl::cmasi::gimbal_configuration::GimbalConfiguration::deser(buf)?;
                Ok((Message::AfrlCmasiGimbalConfiguration(s), i))
            }
            (4849604199710720000, 25) => {
                let (s, i) = afrl::cmasi::gimbal_scan_action::GimbalScanAction::deser(buf)?;
                Ok((Message::AfrlCmasiGimbalScanAction(s), i))
            }
            (4849604199710720000, 26) => {
                let (s, i) = afrl::cmasi::gimbal_stare_action::GimbalStareAction::deser(buf)?;
                Ok((Message::AfrlCmasiGimbalStareAction(s), i))
            }
            (4849604199710720000, 27) => {
                let (s, i) = afrl::cmasi::gimbal_state::GimbalState::deser(buf)?;
                Ok((Message::AfrlCmasiGimbalState(s), i))
            }
            (4849604199710720000, 28) => {
                let (s, i) = afrl::cmasi::go_to_waypoint_action::GoToWaypointAction::deser(buf)?;
                Ok((Message::AfrlCmasiGoToWaypointAction(s), i))
            }
            (4849604199710720000, 29) => {
                let (s, i) = afrl::cmasi::keep_in_zone::KeepInZone::deser(buf)?;
                Ok((Message::AfrlCmasiKeepInZone(s), i))
            }
            (4849604199710720000, 30) => {
                let (s, i) = afrl::cmasi::keep_out_zone::KeepOutZone::deser(buf)?;
                Ok((Message::AfrlCmasiKeepOutZone(s), i))
            }
            (4849604199710720000, 31) => {
                let (s, i) = afrl::cmasi::line_search_task::LineSearchTask::deser(buf)?;
                Ok((Message::AfrlCmasiLineSearchTask(s), i))
            }
            (4849604199710720000, 32) => {
                let (s, i) = afrl::cmasi::navigation_action::NavigationAction::deser(buf)?;
                Ok((Message::AfrlCmasiNavigationAction(s), i))
            }
            (4849604199710720000, 33) => {
                let (s, i) = afrl::cmasi::loiter_action::LoiterAction::deser(buf)?;
                Ok((Message::AfrlCmasiLoiterAction(s), i))
            }
            (4849604199710720000, 34) => {
                let (s, i) = afrl::cmasi::loiter_task::LoiterTask::deser(buf)?;
                Ok((Message::AfrlCmasiLoiterTask(s), i))
            }
            (4849604199710720000, 35) => {
                let (s, i) = afrl::cmasi::waypoint::Waypoint::deser(buf)?;
                Ok((Message::AfrlCmasiWaypoint(s), i))
            }
            (4849604199710720000, 36) => {
                let (s, i) = afrl::cmasi::mission_command::MissionCommand::deser(buf)?;
                Ok((Message::AfrlCmasiMissionCommand(s), i))
            }
            (4849604199710720000, 37) => {
                let (s, i) = afrl::cmasi::must_fly_task::MustFlyTask::deser(buf)?;
                Ok((Message::AfrlCmasiMustFlyTask(s), i))
            }
            (4849604199710720000, 38) => {
                let (s, i) = afrl::cmasi::operator_signal::OperatorSignal::deser(buf)?;
                Ok((Message::AfrlCmasiOperatorSignal(s), i))
            }
            (4849604199710720000, 39) => {
                let (s, i) = afrl::cmasi::operating_region::OperatingRegion::deser(buf)?;
                Ok((Message::AfrlCmasiOperatingRegion(s), i))
            }
            (4849604199710720000, 40) => {
                let (s, i) = afrl::cmasi::automation_request::AutomationRequest::deser(buf)?;
                Ok((Message::AfrlCmasiAutomationRequest(s), i))
            }
            (4849604199710720000, 41) => {
                let (s, i) = afrl::cmasi::point_search_task::PointSearchTask::deser(buf)?;
                Ok((Message::AfrlCmasiPointSearchTask(s), i))
            }
            (4849604199710720000, 42) => {
                let (s, i) = afrl::cmasi::polygon::Polygon::deser(buf)?;
                Ok((Message::AfrlCmasiPolygon(s), i))
            }
            (4849604199710720000, 43) => {
                let (s, i) = afrl::cmasi::rectangle::Rectangle::deser(buf)?;
                Ok((Message::AfrlCmasiRectangle(s), i))
            }
            (4849604199710720000, 44) => {
                let (s, i) = afrl::cmasi::remove_tasks::RemoveTasks::deser(buf)?;
                Ok((Message::AfrlCmasiRemoveTasks(s), i))
            }
            (4849604199710720000, 45) => {
                let (s, i) = afrl::cmasi::service_status::ServiceStatus::deser(buf)?;
                Ok((Message::AfrlCmasiServiceStatus(s), i))
            }
            (4849604199710720000, 46) => {
                let (s, i) = afrl::cmasi::session_status::SessionStatus::deser(buf)?;
                Ok((Message::AfrlCmasiSessionStatus(s), i))
            }
            (4849604199710720000, 47) => {
                let (s, i) = afrl::cmasi::vehicle_action_command::VehicleActionCommand::deser(buf)?;
                Ok((Message::AfrlCmasiVehicleActionCommand(s), i))
            }
            (4849604199710720000, 48) => {
                let (s, i) = afrl::cmasi::video_stream_action::VideoStreamAction::deser(buf)?;
                Ok((Message::AfrlCmasiVideoStreamAction(s), i))
            }
            (4849604199710720000, 49) => {
                let (s, i) = afrl::cmasi::video_stream_configuration::VideoStreamConfiguration::deser(buf)?;
                Ok((Message::AfrlCmasiVideoStreamConfiguration(s), i))
            }
            (4849604199710720000, 50) => {
                let (s, i) = afrl::cmasi::video_stream_state::VideoStreamState::deser(buf)?;
                Ok((Message::AfrlCmasiVideoStreamState(s), i))
            }
            (4849604199710720000, 51) => {
                let (s, i) = afrl::cmasi::automation_response::AutomationResponse::deser(buf)?;
                Ok((Message::AfrlCmasiAutomationResponse(s), i))
            }
            (4849604199710720000, 52) => {
                let (s, i) = afrl::cmasi::remove_zones::RemoveZones::deser(buf)?;
                Ok((Message::AfrlCmasiRemoveZones(s), i))
            }
            (4849604199710720000, 53) => {
                let (s, i) = afrl::cmasi::remove_entities::RemoveEntities::deser(buf)?;
                Ok((Message::AfrlCmasiRemoveEntities(s), i))
            }
            (4849604199710720000, 54) => {
                let (s, i) = afrl::cmasi::flight_director_action::FlightDirectorAction::deser(buf)?;
                Ok((Message::AfrlCmasiFlightDirectorAction(s), i))
            }
            (4849604199710720000, 55) => {
                let (s, i) = afrl::cmasi::weather_report::WeatherReport::deser(buf)?;
                Ok((Message::AfrlCmasiWeatherReport(s), i))
            }
            (4849604199710720000, 56) => {
                let (s, i) = afrl::cmasi::follow_path_command::FollowPathCommand::deser(buf)?;
                Ok((Message::AfrlCmasiFollowPathCommand(s), i))
            }
            (4849604199710720000, 57) => {
                let (s, i) = afrl::cmasi::path_waypoint::PathWaypoint::deser(buf)?;
                Ok((Message::AfrlCmasiPathWaypoint(s), i))
            }
            (4849604199710720000, 58) => {
                let (s, i) = afrl::cmasi::stop_movement_action::StopMovementAction::deser(buf)?;
                Ok((Message::AfrlCmasiStopMovementAction(s), i))
            }
            (4849604199710720000, 59) => {
                let (s, i) = afrl::cmasi::waypoint_transfer::WaypointTransfer::deser(buf)?;
                Ok((Message::AfrlCmasiWaypointTransfer(s), i))
            }
            (4849604199710720000, 60) => {
                let (s, i) = afrl::cmasi::payload_stow_action::PayloadStowAction::deser(buf)?;
                Ok((Message::AfrlCmasiPayloadStowAction(s), i))
            }
            (5281966179208134656, 1) => {
                let (s, i) = afrl::impact::power_configuration::PowerConfiguration::deser(buf)?;
                Ok((Message::AfrlImpactPowerConfiguration(s), i))
            }
            (5281966179208134656, 2) => {
                let (s, i) = afrl::impact::radio_configuration::RadioConfiguration::deser(buf)?;
                Ok((Message::AfrlImpactRadioConfiguration(s), i))
            }
            (5281966179208134656, 3) => {
                let (s, i) = afrl::impact::radio_tower_configuration::RadioTowerConfiguration::deser(buf)?;
                Ok((Message::AfrlImpactRadioTowerConfiguration(s), i))
            }
            (5281966179208134656, 4) => {
                let (s, i) = afrl::impact::radio_state::RadioState::deser(buf)?;
                Ok((Message::AfrlImpactRadioState(s), i))
            }
            (5281966179208134656, 5) => {
                let (s, i) = afrl::impact::radio_tower_state::RadioTowerState::deser(buf)?;
                Ok((Message::AfrlImpactRadioTowerState(s), i))
            }
            (5281966179208134656, 6) => {
                let (s, i) = afrl::impact::impact_payload_configuration::ImpactPayloadConfiguration::deser(buf)?;
                Ok((Message::AfrlImpactImpactPayloadConfiguration(s), i))
            }
            (5281966179208134656, 7) => {
                let (s, i) = afrl::impact::deploy_impact_payload::DeployImpactPayload::deser(buf)?;
                Ok((Message::AfrlImpactDeployImpactPayload(s), i))
            }
            (5281966179208134656, 8) => {
                let (s, i) = afrl::impact::power_plant_state::PowerPlantState::deser(buf)?;
                Ok((Message::AfrlImpactPowerPlantState(s), i))
            }
            (5281966179208134656, 9) => {
                let (s, i) = afrl::impact::batch_route_plan_request::BatchRoutePlanRequest::deser(buf)?;
                Ok((Message::AfrlImpactBatchRoutePlanRequest(s), i))
            }
            (5281966179208134656, 10) => {
                let (s, i) = afrl::impact::batch_route_plan_response::BatchRoutePlanResponse::deser(buf)?;
                Ok((Message::AfrlImpactBatchRoutePlanResponse(s), i))
            }
            (5281966179208134656, 11) => {
                let (s, i) = afrl::impact::task_timing_pair::TaskTimingPair::deser(buf)?;
                Ok((Message::AfrlImpactTaskTimingPair(s), i))
            }
            (5281966179208134656, 12) => {
                let (s, i) = afrl::impact::batch_summary_request::BatchSummaryRequest::deser(buf)?;
                Ok((Message::AfrlImpactBatchSummaryRequest(s), i))
            }
            (5281966179208134656, 13) => {
                let (s, i) = afrl::impact::batch_summary_response::BatchSummaryResponse::deser(buf)?;
                Ok((Message::AfrlImpactBatchSummaryResponse(s), i))
            }
            (5281966179208134656, 14) => {
                let (s, i) = afrl::impact::task_summary::TaskSummary::deser(buf)?;
                Ok((Message::AfrlImpactTaskSummary(s), i))
            }
            (5281966179208134656, 15) => {
                let (s, i) = afrl::impact::vehicle_summary::VehicleSummary::deser(buf)?;
                Ok((Message::AfrlImpactVehicleSummary(s), i))
            }
            (5281966179208134656, 16) => {
                let (s, i) = afrl::impact::speed_alt_pair::SpeedAltPair::deser(buf)?;
                Ok((Message::AfrlImpactSpeedAltPair(s), i))
            }
            (5281966179208134656, 17) => {
                let (s, i) = afrl::impact::impact_automation_request::ImpactAutomationRequest::deser(buf)?;
                Ok((Message::AfrlImpactImpactAutomationRequest(s), i))
            }
            (5281966179208134656, 18) => {
                let (s, i) = afrl::impact::impact_automation_response::ImpactAutomationResponse::deser(buf)?;
                Ok((Message::AfrlImpactImpactAutomationResponse(s), i))
            }
            (5281966179208134656, 19) => {
                let (s, i) = afrl::impact::point_of_interest::PointOfInterest::deser(buf)?;
                Ok((Message::AfrlImpactPointOfInterest(s), i))
            }
            (5281966179208134656, 20) => {
                let (s, i) = afrl::impact::line_of_interest::LineOfInterest::deser(buf)?;
                Ok((Message::AfrlImpactLineOfInterest(s), i))
            }
            (5281966179208134656, 21) => {
                let (s, i) = afrl::impact::area_of_interest::AreaOfInterest::deser(buf)?;
                Ok((Message::AfrlImpactAreaOfInterest(s), i))
            }
            (5281966179208134656, 22) => {
                let (s, i) = afrl::impact::impact_point_search_task::ImpactPointSearchTask::deser(buf)?;
                Ok((Message::AfrlImpactImpactPointSearchTask(s), i))
            }
            (5281966179208134656, 23) => {
                let (s, i) = afrl::impact::pattern_search_task::PatternSearchTask::deser(buf)?;
                Ok((Message::AfrlImpactPatternSearchTask(s), i))
            }
            (5281966179208134656, 24) => {
                let (s, i) = afrl::impact::angled_area_search_task::AngledAreaSearchTask::deser(buf)?;
                Ok((Message::AfrlImpactAngledAreaSearchTask(s), i))
            }
            (5281966179208134656, 25) => {
                let (s, i) = afrl::impact::impact_line_search_task::ImpactLineSearchTask::deser(buf)?;
                Ok((Message::AfrlImpactImpactLineSearchTask(s), i))
            }
            (5281966179208134656, 26) => {
                let (s, i) = afrl::impact::watch_task::WatchTask::deser(buf)?;
                Ok((Message::AfrlImpactWatchTask(s), i))
            }
            (5281966179208134656, 27) => {
                let (s, i) = afrl::impact::multi_vehicle_watch_task::MultiVehicleWatchTask::deser(buf)?;
                Ok((Message::AfrlImpactMultiVehicleWatchTask(s), i))
            }
            (5281966179208134656, 28) => {
                let (s, i) = afrl::impact::comm_relay_task::CommRelayTask::deser(buf)?;
                Ok((Message::AfrlImpactCommRelayTask(s), i))
            }
            (5281966179208134656, 29) => {
                let (s, i) = afrl::impact::cordon_task::CordonTask::deser(buf)?;
                Ok((Message::AfrlImpactCordonTask(s), i))
            }
            (5281966179208134656, 30) => {
                let (s, i) = afrl::impact::blockade_task::BlockadeTask::deser(buf)?;
                Ok((Message::AfrlImpactBlockadeTask(s), i))
            }
            (5281966179208134656, 31) => {
                let (s, i) = afrl::impact::escort_task::EscortTask::deser(buf)?;
                Ok((Message::AfrlImpactEscortTask(s), i))
            }
            (5281966179208134656, 32) => {
                let (s, i) = afrl::impact::configuration_request::ConfigurationRequest::deser(buf)?;
                Ok((Message::AfrlImpactConfigurationRequest(s), i))
            }
            (5281966179208134656, 33) => {
                let (s, i) = afrl::impact::water_report::WaterReport::deser(buf)?;
                Ok((Message::AfrlImpactWaterReport(s), i))
            }
            (5281966179208134656, 34) => {
                let (s, i) = afrl::impact::water_zone::WaterZone::deser(buf)?;
                Ok((Message::AfrlImpactWaterZone(s), i))
            }
            (5281966179208134656, 35) => {
                let (s, i) = afrl::impact::payload_drop_task::PayloadDropTask::deser(buf)?;
                Ok((Message::AfrlImpactPayloadDropTask(s), i))
            }
            (5784119745305990725, 1) => {
                let (s, i) = afrl::cmasi::perceive::entity_perception::EntityPerception::deser(buf)?;
                Ok((Message::AfrlCmasiPerceiveEntityPerception(s), i))
            }
            (5784119745305990725, 2) => {
                let (s, i) = afrl::cmasi::perceive::track_entity_action::TrackEntityAction::deser(buf)?;
                Ok((Message::AfrlCmasiPerceiveTrackEntityAction(s), i))
            }
            (5784119745305990725, 3) => {
                let (s, i) = afrl::cmasi::perceive::track_entity_task::TrackEntityTask::deser(buf)?;
                Ok((Message::AfrlCmasiPerceiveTrackEntityTask(s), i))
            }

            _ => Err(error!(ErrorType::UnknownStruct(si)))
        }
    }
}

impl Message {
    pub fn subscription(&self) -> &'static str {
        match *self {
            Message::UxasMessagesTaskAssignmentCoordinatorTask(_) => uxas::messages::task::assignment_coordinator_task::AssignmentCoordinatorTask::subscription(),
            Message::UxasMessagesTaskRendezvousTask(_) => uxas::messages::task::rendezvous_task::RendezvousTask::subscription(),
            Message::UxasMessagesTaskPlanningState(_) => uxas::messages::task::planning_state::PlanningState::subscription(),
            Message::UxasMessagesTaskAssignmentCoordination(_) => uxas::messages::task::assignment_coordination::AssignmentCoordination::subscription(),
            Message::UxasMessagesTaskCoordinatedAutomationRequest(_) => uxas::messages::task::coordinated_automation_request::CoordinatedAutomationRequest::subscription(),
            Message::UxasMessagesTaskTaskAutomationRequest(_) => uxas::messages::task::task_automation_request::TaskAutomationRequest::subscription(),
            Message::UxasMessagesTaskTaskAutomationResponse(_) => uxas::messages::task::task_automation_response::TaskAutomationResponse::subscription(),
            Message::UxasMessagesTaskUniqueAutomationRequest(_) => uxas::messages::task::unique_automation_request::UniqueAutomationRequest::subscription(),
            Message::UxasMessagesTaskUniqueAutomationResponse(_) => uxas::messages::task::unique_automation_response::UniqueAutomationResponse::subscription(),
            Message::UxasMessagesTaskSensorFootprintRequests(_) => uxas::messages::task::sensor_footprint_requests::SensorFootprintRequests::subscription(),
            Message::UxasMessagesTaskFootprintRequest(_) => uxas::messages::task::footprint_request::FootprintRequest::subscription(),
            Message::UxasMessagesTaskSensorFootprint(_) => uxas::messages::task::sensor_footprint::SensorFootprint::subscription(),
            Message::UxasMessagesTaskSensorFootprintResponse(_) => uxas::messages::task::sensor_footprint_response::SensorFootprintResponse::subscription(),
            Message::UxasMessagesTaskTaskImplementationRequest(_) => uxas::messages::task::task_implementation_request::TaskImplementationRequest::subscription(),
            Message::UxasMessagesTaskTaskImplementationResponse(_) => uxas::messages::task::task_implementation_response::TaskImplementationResponse::subscription(),
            Message::UxasMessagesTaskAssignmentCostMatrix(_) => uxas::messages::task::assignment_cost_matrix::AssignmentCostMatrix::subscription(),
            Message::UxasMessagesTaskTaskOptionCost(_) => uxas::messages::task::task_option_cost::TaskOptionCost::subscription(),
            Message::UxasMessagesTaskTaskAssignment(_) => uxas::messages::task::task_assignment::TaskAssignment::subscription(),
            Message::UxasMessagesTaskTaskAssignmentSummary(_) => uxas::messages::task::task_assignment_summary::TaskAssignmentSummary::subscription(),
            Message::UxasMessagesTaskTaskOption(_) => uxas::messages::task::task_option::TaskOption::subscription(),
            Message::UxasMessagesTaskTaskPlanOptions(_) => uxas::messages::task::task_plan_options::TaskPlanOptions::subscription(),
            Message::UxasMessagesTaskTaskPause(_) => uxas::messages::task::task_pause::TaskPause::subscription(),
            Message::UxasMessagesTaskTaskResume(_) => uxas::messages::task::task_resume::TaskResume::subscription(),
            Message::UxasMessagesTaskTaskProgress(_) => uxas::messages::task::task_progress::TaskProgress::subscription(),
            Message::UxasMessagesTaskTaskProgressRequest(_) => uxas::messages::task::task_progress_request::TaskProgressRequest::subscription(),
            Message::UxasMessagesTaskTaskInitialized(_) => uxas::messages::task::task_initialized::TaskInitialized::subscription(),
            Message::UxasMessagesTaskTaskActive(_) => uxas::messages::task::task_active::TaskActive::subscription(),
            Message::UxasMessagesTaskTaskComplete(_) => uxas::messages::task::task_complete::TaskComplete::subscription(),
            Message::UxasMessagesTaskCancelTask(_) => uxas::messages::task::cancel_task::CancelTask::subscription(),
            Message::UxasMessagesUxnativeVideoRecord(_) => uxas::messages::uxnative::video_record::VideoRecord::subscription(),
            Message::UxasMessagesUxnativeStartupComplete(_) => uxas::messages::uxnative::startup_complete::StartupComplete::subscription(),
            Message::UxasMessagesUxnativeCreateNewService(_) => uxas::messages::uxnative::create_new_service::CreateNewService::subscription(),
            Message::UxasMessagesUxnativeKillService(_) => uxas::messages::uxnative::kill_service::KillService::subscription(),
            Message::UxasMessagesUxnativeIncrementWaypoint(_) => uxas::messages::uxnative::increment_waypoint::IncrementWaypoint::subscription(),
            Message::UxasMessagesUxnativeSafeHeadingAction(_) => uxas::messages::uxnative::safe_heading_action::SafeHeadingAction::subscription(),
            Message::UxasMessagesUxnativeEntityLocation(_) => uxas::messages::uxnative::entity_location::EntityLocation::subscription(),
            Message::UxasMessagesUxnativeBandwidthTest(_) => uxas::messages::uxnative::bandwidth_test::BandwidthTest::subscription(),
            Message::UxasMessagesUxnativeBandwidthReceiveReport(_) => uxas::messages::uxnative::bandwidth_receive_report::BandwidthReceiveReport::subscription(),
            Message::UxasMessagesUxnativeSubTaskExecution(_) => uxas::messages::uxnative::sub_task_execution::SubTaskExecution::subscription(),
            Message::UxasMessagesUxnativeSubTaskAssignment(_) => uxas::messages::uxnative::sub_task_assignment::SubTaskAssignment::subscription(),
            Message::UxasMessagesUxnativeAutopilotKeepAlive(_) => uxas::messages::uxnative::autopilot_keep_alive::AutopilotKeepAlive::subscription(),
            Message::UxasMessagesUxnativeOnboardStatusReport(_) => uxas::messages::uxnative::onboard_status_report::OnboardStatusReport::subscription(),
            Message::UxasMessagesUxnativeEntityJoin(_) => uxas::messages::uxnative::entity_join::EntityJoin::subscription(),
            Message::UxasMessagesUxnativeEntityExit(_) => uxas::messages::uxnative::entity_exit::EntityExit::subscription(),
            Message::UxasMessagesUxnativeSimulationTimeStepAcknowledgement(_) => uxas::messages::uxnative::simulation_time_step_acknowledgement::SimulationTimeStepAcknowledgement::subscription(),
            Message::UxasMessagesUxnativeSpeedOverrideAction(_) => uxas::messages::uxnative::speed_override_action::SpeedOverrideAction::subscription(),
            Message::UxasMessagesRouteGraphNode(_) => uxas::messages::route::graph_node::GraphNode::subscription(),
            Message::UxasMessagesRouteGraphEdge(_) => uxas::messages::route::graph_edge::GraphEdge::subscription(),
            Message::UxasMessagesRouteGraphRegion(_) => uxas::messages::route::graph_region::GraphRegion::subscription(),
            Message::UxasMessagesRouteRouteConstraints(_) => uxas::messages::route::route_constraints::RouteConstraints::subscription(),
            Message::UxasMessagesRouteRouteRequest(_) => uxas::messages::route::route_request::RouteRequest::subscription(),
            Message::UxasMessagesRouteRoutePlanRequest(_) => uxas::messages::route::route_plan_request::RoutePlanRequest::subscription(),
            Message::UxasMessagesRouteRoutePlan(_) => uxas::messages::route::route_plan::RoutePlan::subscription(),
            Message::UxasMessagesRouteRoutePlanResponse(_) => uxas::messages::route::route_plan_response::RoutePlanResponse::subscription(),
            Message::UxasMessagesRouteRouteResponse(_) => uxas::messages::route::route_response::RouteResponse::subscription(),
            Message::UxasMessagesRouteEgressRouteRequest(_) => uxas::messages::route::egress_route_request::EgressRouteRequest::subscription(),
            Message::UxasMessagesRouteEgressRouteResponse(_) => uxas::messages::route::egress_route_response::EgressRouteResponse::subscription(),
            Message::UxasMessagesRouteRoadPointsConstraints(_) => uxas::messages::route::road_points_constraints::RoadPointsConstraints::subscription(),
            Message::UxasMessagesRouteRoadPointsRequest(_) => uxas::messages::route::road_points_request::RoadPointsRequest::subscription(),
            Message::UxasMessagesRouteRoadPointsResponse(_) => uxas::messages::route::road_points_response::RoadPointsResponse::subscription(),
            Message::AfrlVehiclesGroundVehicleConfiguration(_) => afrl::vehicles::ground_vehicle_configuration::GroundVehicleConfiguration::subscription(),
            Message::AfrlVehiclesGroundVehicleState(_) => afrl::vehicles::ground_vehicle_state::GroundVehicleState::subscription(),
            Message::AfrlVehiclesSurfaceVehicleConfiguration(_) => afrl::vehicles::surface_vehicle_configuration::SurfaceVehicleConfiguration::subscription(),
            Message::AfrlVehiclesSurfaceVehicleState(_) => afrl::vehicles::surface_vehicle_state::SurfaceVehicleState::subscription(),
            Message::AfrlVehiclesStationarySensorConfiguration(_) => afrl::vehicles::stationary_sensor_configuration::StationarySensorConfiguration::subscription(),
            Message::AfrlVehiclesStationarySensorState(_) => afrl::vehicles::stationary_sensor_state::StationarySensorState::subscription(),
            Message::AfrlCmasiAbstractGeometry(_) => afrl::cmasi::abstract_geometry::AbstractGeometry::subscription(),
            Message::AfrlCmasiKeyValuePair(_) => afrl::cmasi::key_value_pair::KeyValuePair::subscription(),
            Message::AfrlCmasiLocation3D(_) => afrl::cmasi::location3d::Location3D::subscription(),
            Message::AfrlCmasiPayloadAction(_) => afrl::cmasi::payload_action::PayloadAction::subscription(),
            Message::AfrlCmasiPayloadConfiguration(_) => afrl::cmasi::payload_configuration::PayloadConfiguration::subscription(),
            Message::AfrlCmasiPayloadState(_) => afrl::cmasi::payload_state::PayloadState::subscription(),
            Message::AfrlCmasiVehicleAction(_) => afrl::cmasi::vehicle_action::VehicleAction::subscription(),
            Message::AfrlCmasiTask(_) => afrl::cmasi::task::Task::subscription(),
            Message::AfrlCmasiSearchTask(_) => afrl::cmasi::search_task::SearchTask::subscription(),
            Message::AfrlCmasiAbstractZone(_) => afrl::cmasi::abstract_zone::AbstractZone::subscription(),
            Message::AfrlCmasiEntityConfiguration(_) => afrl::cmasi::entity_configuration::EntityConfiguration::subscription(),
            Message::AfrlCmasiFlightProfile(_) => afrl::cmasi::flight_profile::FlightProfile::subscription(),
            Message::AfrlCmasiAirVehicleConfiguration(_) => afrl::cmasi::air_vehicle_configuration::AirVehicleConfiguration::subscription(),
            Message::AfrlCmasiEntityState(_) => afrl::cmasi::entity_state::EntityState::subscription(),
            Message::AfrlCmasiAirVehicleState(_) => afrl::cmasi::air_vehicle_state::AirVehicleState::subscription(),
            Message::AfrlCmasiWedge(_) => afrl::cmasi::wedge::Wedge::subscription(),
            Message::AfrlCmasiAreaSearchTask(_) => afrl::cmasi::area_search_task::AreaSearchTask::subscription(),
            Message::AfrlCmasiCameraAction(_) => afrl::cmasi::camera_action::CameraAction::subscription(),
            Message::AfrlCmasiCameraConfiguration(_) => afrl::cmasi::camera_configuration::CameraConfiguration::subscription(),
            Message::AfrlCmasiGimballedPayloadState(_) => afrl::cmasi::gimballed_payload_state::GimballedPayloadState::subscription(),
            Message::AfrlCmasiCameraState(_) => afrl::cmasi::camera_state::CameraState::subscription(),
            Message::AfrlCmasiCircle(_) => afrl::cmasi::circle::Circle::subscription(),
            Message::AfrlCmasiGimbalAngleAction(_) => afrl::cmasi::gimbal_angle_action::GimbalAngleAction::subscription(),
            Message::AfrlCmasiGimbalConfiguration(_) => afrl::cmasi::gimbal_configuration::GimbalConfiguration::subscription(),
            Message::AfrlCmasiGimbalScanAction(_) => afrl::cmasi::gimbal_scan_action::GimbalScanAction::subscription(),
            Message::AfrlCmasiGimbalStareAction(_) => afrl::cmasi::gimbal_stare_action::GimbalStareAction::subscription(),
            Message::AfrlCmasiGimbalState(_) => afrl::cmasi::gimbal_state::GimbalState::subscription(),
            Message::AfrlCmasiGoToWaypointAction(_) => afrl::cmasi::go_to_waypoint_action::GoToWaypointAction::subscription(),
            Message::AfrlCmasiKeepInZone(_) => afrl::cmasi::keep_in_zone::KeepInZone::subscription(),
            Message::AfrlCmasiKeepOutZone(_) => afrl::cmasi::keep_out_zone::KeepOutZone::subscription(),
            Message::AfrlCmasiLineSearchTask(_) => afrl::cmasi::line_search_task::LineSearchTask::subscription(),
            Message::AfrlCmasiNavigationAction(_) => afrl::cmasi::navigation_action::NavigationAction::subscription(),
            Message::AfrlCmasiLoiterAction(_) => afrl::cmasi::loiter_action::LoiterAction::subscription(),
            Message::AfrlCmasiLoiterTask(_) => afrl::cmasi::loiter_task::LoiterTask::subscription(),
            Message::AfrlCmasiWaypoint(_) => afrl::cmasi::waypoint::Waypoint::subscription(),
            Message::AfrlCmasiMissionCommand(_) => afrl::cmasi::mission_command::MissionCommand::subscription(),
            Message::AfrlCmasiMustFlyTask(_) => afrl::cmasi::must_fly_task::MustFlyTask::subscription(),
            Message::AfrlCmasiOperatorSignal(_) => afrl::cmasi::operator_signal::OperatorSignal::subscription(),
            Message::AfrlCmasiOperatingRegion(_) => afrl::cmasi::operating_region::OperatingRegion::subscription(),
            Message::AfrlCmasiAutomationRequest(_) => afrl::cmasi::automation_request::AutomationRequest::subscription(),
            Message::AfrlCmasiPointSearchTask(_) => afrl::cmasi::point_search_task::PointSearchTask::subscription(),
            Message::AfrlCmasiPolygon(_) => afrl::cmasi::polygon::Polygon::subscription(),
            Message::AfrlCmasiRectangle(_) => afrl::cmasi::rectangle::Rectangle::subscription(),
            Message::AfrlCmasiRemoveTasks(_) => afrl::cmasi::remove_tasks::RemoveTasks::subscription(),
            Message::AfrlCmasiServiceStatus(_) => afrl::cmasi::service_status::ServiceStatus::subscription(),
            Message::AfrlCmasiSessionStatus(_) => afrl::cmasi::session_status::SessionStatus::subscription(),
            Message::AfrlCmasiVehicleActionCommand(_) => afrl::cmasi::vehicle_action_command::VehicleActionCommand::subscription(),
            Message::AfrlCmasiVideoStreamAction(_) => afrl::cmasi::video_stream_action::VideoStreamAction::subscription(),
            Message::AfrlCmasiVideoStreamConfiguration(_) => afrl::cmasi::video_stream_configuration::VideoStreamConfiguration::subscription(),
            Message::AfrlCmasiVideoStreamState(_) => afrl::cmasi::video_stream_state::VideoStreamState::subscription(),
            Message::AfrlCmasiAutomationResponse(_) => afrl::cmasi::automation_response::AutomationResponse::subscription(),
            Message::AfrlCmasiRemoveZones(_) => afrl::cmasi::remove_zones::RemoveZones::subscription(),
            Message::AfrlCmasiRemoveEntities(_) => afrl::cmasi::remove_entities::RemoveEntities::subscription(),
            Message::AfrlCmasiFlightDirectorAction(_) => afrl::cmasi::flight_director_action::FlightDirectorAction::subscription(),
            Message::AfrlCmasiWeatherReport(_) => afrl::cmasi::weather_report::WeatherReport::subscription(),
            Message::AfrlCmasiFollowPathCommand(_) => afrl::cmasi::follow_path_command::FollowPathCommand::subscription(),
            Message::AfrlCmasiPathWaypoint(_) => afrl::cmasi::path_waypoint::PathWaypoint::subscription(),
            Message::AfrlCmasiStopMovementAction(_) => afrl::cmasi::stop_movement_action::StopMovementAction::subscription(),
            Message::AfrlCmasiWaypointTransfer(_) => afrl::cmasi::waypoint_transfer::WaypointTransfer::subscription(),
            Message::AfrlCmasiPayloadStowAction(_) => afrl::cmasi::payload_stow_action::PayloadStowAction::subscription(),
            Message::AfrlImpactPowerConfiguration(_) => afrl::impact::power_configuration::PowerConfiguration::subscription(),
            Message::AfrlImpactRadioConfiguration(_) => afrl::impact::radio_configuration::RadioConfiguration::subscription(),
            Message::AfrlImpactRadioTowerConfiguration(_) => afrl::impact::radio_tower_configuration::RadioTowerConfiguration::subscription(),
            Message::AfrlImpactRadioState(_) => afrl::impact::radio_state::RadioState::subscription(),
            Message::AfrlImpactRadioTowerState(_) => afrl::impact::radio_tower_state::RadioTowerState::subscription(),
            Message::AfrlImpactImpactPayloadConfiguration(_) => afrl::impact::impact_payload_configuration::ImpactPayloadConfiguration::subscription(),
            Message::AfrlImpactDeployImpactPayload(_) => afrl::impact::deploy_impact_payload::DeployImpactPayload::subscription(),
            Message::AfrlImpactPowerPlantState(_) => afrl::impact::power_plant_state::PowerPlantState::subscription(),
            Message::AfrlImpactBatchRoutePlanRequest(_) => afrl::impact::batch_route_plan_request::BatchRoutePlanRequest::subscription(),
            Message::AfrlImpactBatchRoutePlanResponse(_) => afrl::impact::batch_route_plan_response::BatchRoutePlanResponse::subscription(),
            Message::AfrlImpactTaskTimingPair(_) => afrl::impact::task_timing_pair::TaskTimingPair::subscription(),
            Message::AfrlImpactBatchSummaryRequest(_) => afrl::impact::batch_summary_request::BatchSummaryRequest::subscription(),
            Message::AfrlImpactBatchSummaryResponse(_) => afrl::impact::batch_summary_response::BatchSummaryResponse::subscription(),
            Message::AfrlImpactTaskSummary(_) => afrl::impact::task_summary::TaskSummary::subscription(),
            Message::AfrlImpactVehicleSummary(_) => afrl::impact::vehicle_summary::VehicleSummary::subscription(),
            Message::AfrlImpactSpeedAltPair(_) => afrl::impact::speed_alt_pair::SpeedAltPair::subscription(),
            Message::AfrlImpactImpactAutomationRequest(_) => afrl::impact::impact_automation_request::ImpactAutomationRequest::subscription(),
            Message::AfrlImpactImpactAutomationResponse(_) => afrl::impact::impact_automation_response::ImpactAutomationResponse::subscription(),
            Message::AfrlImpactPointOfInterest(_) => afrl::impact::point_of_interest::PointOfInterest::subscription(),
            Message::AfrlImpactLineOfInterest(_) => afrl::impact::line_of_interest::LineOfInterest::subscription(),
            Message::AfrlImpactAreaOfInterest(_) => afrl::impact::area_of_interest::AreaOfInterest::subscription(),
            Message::AfrlImpactImpactPointSearchTask(_) => afrl::impact::impact_point_search_task::ImpactPointSearchTask::subscription(),
            Message::AfrlImpactPatternSearchTask(_) => afrl::impact::pattern_search_task::PatternSearchTask::subscription(),
            Message::AfrlImpactAngledAreaSearchTask(_) => afrl::impact::angled_area_search_task::AngledAreaSearchTask::subscription(),
            Message::AfrlImpactImpactLineSearchTask(_) => afrl::impact::impact_line_search_task::ImpactLineSearchTask::subscription(),
            Message::AfrlImpactWatchTask(_) => afrl::impact::watch_task::WatchTask::subscription(),
            Message::AfrlImpactMultiVehicleWatchTask(_) => afrl::impact::multi_vehicle_watch_task::MultiVehicleWatchTask::subscription(),
            Message::AfrlImpactCommRelayTask(_) => afrl::impact::comm_relay_task::CommRelayTask::subscription(),
            Message::AfrlImpactCordonTask(_) => afrl::impact::cordon_task::CordonTask::subscription(),
            Message::AfrlImpactBlockadeTask(_) => afrl::impact::blockade_task::BlockadeTask::subscription(),
            Message::AfrlImpactEscortTask(_) => afrl::impact::escort_task::EscortTask::subscription(),
            Message::AfrlImpactConfigurationRequest(_) => afrl::impact::configuration_request::ConfigurationRequest::subscription(),
            Message::AfrlImpactWaterReport(_) => afrl::impact::water_report::WaterReport::subscription(),
            Message::AfrlImpactWaterZone(_) => afrl::impact::water_zone::WaterZone::subscription(),
            Message::AfrlImpactPayloadDropTask(_) => afrl::impact::payload_drop_task::PayloadDropTask::subscription(),
            Message::AfrlCmasiPerceiveEntityPerception(_) => afrl::cmasi::perceive::entity_perception::EntityPerception::subscription(),
            Message::AfrlCmasiPerceiveTrackEntityAction(_) => afrl::cmasi::perceive::track_entity_action::TrackEntityAction::subscription(),
            Message::AfrlCmasiPerceiveTrackEntityTask(_) => afrl::cmasi::perceive::track_entity_task::TrackEntityTask::subscription(),

        }
    }

    pub fn deser(buf: &[u8]) -> Result<Option<Message>, Error> {
        let h = get!(buf.get(0..4));
        if h[0] != 76 || h[1] != 77 || h[2] != 67 || h[3] != 80 {
            return Err(error!(ErrorType::InvalidLmcpHeader));
        }

        let r = get!(buf.get(4..8));
        let (size, _) = u32::deser(r)?;

        if (size + 12) as usize > buf.len() {
            // 12 = 8 for header + 4 for checksum
            return Err(error!(ErrorType::NotEnoughBytes));
        }
        let h = get!(buf.get(8..));
        let (si, _) = StructInfo::deser(h)?;
        let ch = get!(buf.get(buf.len() - 4..buf.len()));
        let (_, _): (u32, _) = u32::deser(ch)?; // TODO: checksum
        if si.exist == 0 {
            Ok(None)
        } else {
            let (msg, _) = Lmcp::deser(h)?;
            Ok(Some(msg))
        }
    }

    pub fn ser(&self, buf: &mut [u8]) -> Result<usize, Error> {
        let size = Lmcp::size(self);
        if size > (u32::max_value() as usize) {
            return Err(error!(ErrorType::MessageTooLarge));
        }

        {
            let r = get!(buf.get_mut(0..4));
            r[0] = 76;
            r[1] = 77;
            r[2] = 67;
            r[3] = 80;
        }
        {
            let r = get!(buf.get_mut(4..));
            (size as u32).ser(r)?;
        }
        {
            let r = get!(buf.get_mut(8..));
            let wr = Lmcp::ser(self, r)?;
            if wr != size {
                return Err(error!(ErrorType::MessageSizeMismatch));
            }
        }
        {
            // TODO: optionally implement checksum
            let r = get!(buf.get_mut(size + 8..size + 12));
            r[0] = 0;
            r[1] = 0;
            r[2] = 0;
            r[3] = 0;
        }

        Ok(size + 12)
    }

    pub fn size(&self) -> usize {
        Lmcp::size(self) + 12
    }
}
