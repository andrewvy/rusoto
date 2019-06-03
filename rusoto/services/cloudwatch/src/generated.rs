// =================================================================
//
//                           * WARNING *
//
//                    This file is generated!
//
//  Changes made to this file will be overwritten. If changes are
//  required to the generated code, the service_crategen project
//  must be updated to generate the changes.
//
// =================================================================

use std::error::Error;
use std::fmt;

#[allow(warnings)]
use futures::future;
use futures::Future;
use rusoto_core::credential::ProvideAwsCredentials;
use rusoto_core::region;
use rusoto_core::request::{BufferedHttpResponse, DispatchSignedRequest};
use rusoto_core::v2::{Dispatcher, Request, ServiceRequest};
use rusoto_core::{Client, RusotoError, RusotoFuture};

use rusoto_core::param::{Params, ServiceParams};
use rusoto_core::proto::xml::error::*;
use rusoto_core::proto::xml::util::{
    characters, deserialize_elements, end_element, find_start_element, peek_at_name, skip_tree,
    start_element,
};
use rusoto_core::proto::xml::util::{Next, Peek, XmlParseError, XmlResponse};
use rusoto_core::signature::SignedRequest;
use serde_urlencoded;
use std::str::FromStr;
use xml::reader::ParserConfig;
use xml::EventReader;

struct ActionsEnabledDeserializer;
impl ActionsEnabledDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<bool, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = bool::from_str(characters(stack)?.as_ref()).unwrap();
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct AlarmArnDeserializer;
impl AlarmArnDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct AlarmDescriptionDeserializer;
impl AlarmDescriptionDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>Represents the history of a specific alarm.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct AlarmHistoryItem {
    /// <p>The descriptive name for the alarm.</p>
    pub alarm_name: Option<String>,
    /// <p>Data about the alarm, in JSON format.</p>
    pub history_data: Option<String>,
    /// <p>The type of alarm history item.</p>
    pub history_item_type: Option<String>,
    /// <p>A summary of the alarm history, in text format.</p>
    pub history_summary: Option<String>,
    /// <p>The time stamp for the alarm history item.</p>
    pub timestamp: Option<String>,
}

struct AlarmHistoryItemDeserializer;
impl AlarmHistoryItemDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<AlarmHistoryItem, XmlParseError> {
        deserialize_elements::<_, AlarmHistoryItem, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "AlarmName" => {
                    obj.alarm_name = Some(AlarmNameDeserializer::deserialize("AlarmName", stack)?);
                }
                "HistoryData" => {
                    obj.history_data =
                        Some(HistoryDataDeserializer::deserialize("HistoryData", stack)?);
                }
                "HistoryItemType" => {
                    obj.history_item_type = Some(HistoryItemTypeDeserializer::deserialize(
                        "HistoryItemType",
                        stack,
                    )?);
                }
                "HistorySummary" => {
                    obj.history_summary = Some(HistorySummaryDeserializer::deserialize(
                        "HistorySummary",
                        stack,
                    )?);
                }
                "Timestamp" => {
                    obj.timestamp = Some(TimestampDeserializer::deserialize("Timestamp", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
struct AlarmHistoryItemsDeserializer;
impl AlarmHistoryItemsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<AlarmHistoryItem>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(AlarmHistoryItemDeserializer::deserialize("member", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
struct AlarmNameDeserializer;
impl AlarmNameDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}

/// Serialize `AlarmNames` contents to a `SignedRequest`.
struct AlarmNamesSerializer;
impl AlarmNamesSerializer {
    fn serialize(params: &mut impl ServiceParams, name: &str, obj: &Vec<String>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

struct ComparisonOperatorDeserializer;
impl ComparisonOperatorDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}

/// Serialize `Counts` contents to a `SignedRequest`.
struct CountsSerializer;
impl CountsSerializer {
    fn serialize(params: &mut impl ServiceParams, name: &str, obj: &Vec<f64>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

struct DashboardArnDeserializer;
impl DashboardArnDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct DashboardBodyDeserializer;
impl DashboardBodyDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct DashboardEntriesDeserializer;
impl DashboardEntriesDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<DashboardEntry>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(DashboardEntryDeserializer::deserialize("member", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
/// <p>Represents a specific dashboard.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DashboardEntry {
    /// <p>The Amazon Resource Name (ARN) of the dashboard.</p>
    pub dashboard_arn: Option<String>,
    /// <p>The name of the dashboard.</p>
    pub dashboard_name: Option<String>,
    /// <p>The time stamp of when the dashboard was last modified, either by an API call or through the console. This number is expressed as the number of milliseconds since Jan 1, 1970 00:00:00 UTC.</p>
    pub last_modified: Option<String>,
    /// <p>The size of the dashboard, in bytes.</p>
    pub size: Option<i64>,
}

struct DashboardEntryDeserializer;
impl DashboardEntryDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DashboardEntry, XmlParseError> {
        deserialize_elements::<_, DashboardEntry, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "DashboardArn" => {
                    obj.dashboard_arn = Some(DashboardArnDeserializer::deserialize(
                        "DashboardArn",
                        stack,
                    )?);
                }
                "DashboardName" => {
                    obj.dashboard_name = Some(DashboardNameDeserializer::deserialize(
                        "DashboardName",
                        stack,
                    )?);
                }
                "LastModified" => {
                    obj.last_modified = Some(LastModifiedDeserializer::deserialize(
                        "LastModified",
                        stack,
                    )?);
                }
                "Size" => {
                    obj.size = Some(SizeDeserializer::deserialize("Size", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
struct DashboardNameDeserializer;
impl DashboardNameDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}

/// Serialize `DashboardNames` contents to a `SignedRequest`.
struct DashboardNamesSerializer;
impl DashboardNamesSerializer {
    fn serialize(params: &mut impl ServiceParams, name: &str, obj: &Vec<String>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

/// <p>An error or warning for the operation.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DashboardValidationMessage {
    /// <p>The data path related to the message.</p>
    pub data_path: Option<String>,
    /// <p>A message describing the error or warning.</p>
    pub message: Option<String>,
}

struct DashboardValidationMessageDeserializer;
impl DashboardValidationMessageDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DashboardValidationMessage, XmlParseError> {
        deserialize_elements::<_, DashboardValidationMessage, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "DataPath" => {
                        obj.data_path = Some(DataPathDeserializer::deserialize("DataPath", stack)?);
                    }
                    "Message" => {
                        obj.message = Some(MessageDeserializer::deserialize("Message", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
struct DashboardValidationMessagesDeserializer;
impl DashboardValidationMessagesDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<DashboardValidationMessage>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(DashboardValidationMessageDeserializer::deserialize(
                    "member", stack,
                )?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
struct DataPathDeserializer;
impl DataPathDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>Encapsulates the statistical data that CloudWatch computes from metric data.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Datapoint {
    /// <p>The average of the metric values that correspond to the data point.</p>
    pub average: Option<f64>,
    /// <p>The percentile statistic for the data point.</p>
    pub extended_statistics: Option<::std::collections::HashMap<String, f64>>,
    /// <p>The maximum metric value for the data point.</p>
    pub maximum: Option<f64>,
    /// <p>The minimum metric value for the data point.</p>
    pub minimum: Option<f64>,
    /// <p>The number of metric values that contributed to the aggregate value of this data point.</p>
    pub sample_count: Option<f64>,
    /// <p>The sum of the metric values for the data point.</p>
    pub sum: Option<f64>,
    /// <p>The time stamp used for the data point.</p>
    pub timestamp: Option<String>,
    /// <p>The standard unit for the data point.</p>
    pub unit: Option<String>,
}

struct DatapointDeserializer;
impl DatapointDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Datapoint, XmlParseError> {
        deserialize_elements::<_, Datapoint, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Average" => {
                    obj.average = Some(DatapointValueDeserializer::deserialize("Average", stack)?);
                }
                "ExtendedStatistics" => {
                    obj.extended_statistics = Some(DatapointValueMapDeserializer::deserialize(
                        "ExtendedStatistics",
                        stack,
                    )?);
                }
                "Maximum" => {
                    obj.maximum = Some(DatapointValueDeserializer::deserialize("Maximum", stack)?);
                }
                "Minimum" => {
                    obj.minimum = Some(DatapointValueDeserializer::deserialize("Minimum", stack)?);
                }
                "SampleCount" => {
                    obj.sample_count = Some(DatapointValueDeserializer::deserialize(
                        "SampleCount",
                        stack,
                    )?);
                }
                "Sum" => {
                    obj.sum = Some(DatapointValueDeserializer::deserialize("Sum", stack)?);
                }
                "Timestamp" => {
                    obj.timestamp = Some(TimestampDeserializer::deserialize("Timestamp", stack)?);
                }
                "Unit" => {
                    obj.unit = Some(StandardUnitDeserializer::deserialize("Unit", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
struct DatapointValueDeserializer;
impl DatapointValueDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<f64, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = f64::from_str(characters(stack)?.as_ref()).unwrap();
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct DatapointValueMapDeserializer;
impl DatapointValueMapDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<::std::collections::HashMap<String, f64>, XmlParseError> {
        start_element(tag_name, stack)?;

        let mut obj = ::std::collections::HashMap::new();

        while peek_at_name(stack)? == "entry" {
            start_element("entry", stack)?;
            let key = ExtendedStatisticDeserializer::deserialize("key", stack)?;
            let value = DatapointValueDeserializer::deserialize("value", stack)?;
            obj.insert(key, value);
            end_element("entry", stack)?;
        }

        end_element(tag_name, stack)?;
        Ok(obj)
    }
}
struct DatapointValuesDeserializer;
impl DatapointValuesDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<f64>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(DatapointValueDeserializer::deserialize("member", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
struct DatapointsDeserializer;
impl DatapointsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<Datapoint>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(DatapointDeserializer::deserialize("member", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
struct DatapointsToAlarmDeserializer;
impl DatapointsToAlarmDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<i64, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = i64::from_str(characters(stack)?.as_ref()).unwrap();
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteAlarmsRequest {
    /// <p>The alarms to be deleted.</p>
    pub alarm_names: Vec<String>,
}

/// Serialize `DeleteAlarmsRequest` contents to a `SignedRequest`.
struct DeleteAlarmsRequestSerializer;
impl DeleteAlarmsRequestSerializer {
    fn serialize(params: &mut impl ServiceParams, name: &str, obj: &DeleteAlarmsRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        AlarmNamesSerializer::serialize(
            params,
            &format!("{}{}", prefix, "AlarmNames"),
            &obj.alarm_names,
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteAlarmsResponse {}

struct DeleteAlarmsResponseDeserializer;
impl DeleteAlarmsResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DeleteAlarmsResponse, XmlParseError> {
        Ok(DeleteAlarmsResponse::default())
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteDashboardsRequest {
    /// <p>The dashboards to be deleted. This parameter is required.</p>
    pub dashboard_names: Vec<String>,
}

/// Serialize `DeleteDashboardsRequest` contents to a `SignedRequest`.
struct DeleteDashboardsRequestSerializer;
impl DeleteDashboardsRequestSerializer {
    fn serialize(params: &mut impl ServiceParams, name: &str, obj: &DeleteDashboardsRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        DashboardNamesSerializer::serialize(
            params,
            &format!("{}{}", prefix, "DashboardNames"),
            &obj.dashboard_names,
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct DeleteDashboardsResponse {}

struct DeleteDashboardsResponseDeserializer;
impl DeleteDashboardsResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DeleteDashboardsResponse, XmlParseError> {
        start_element(tag_name, stack)?;

        let obj = DeleteDashboardsResponse::default();

        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeAlarmHistoryRequest {
    /// <p>The name of the alarm.</p>
    pub alarm_name: Option<String>,
    /// <p>The ending date to retrieve alarm history.</p>
    pub end_date: Option<String>,
    /// <p>The type of alarm histories to retrieve.</p>
    pub history_item_type: Option<String>,
    /// <p>The maximum number of alarm history records to retrieve.</p>
    pub max_records: Option<i64>,
    /// <p>The token returned by a previous call to indicate that there is more data available.</p>
    pub next_token: Option<String>,
    /// <p>The starting date to retrieve alarm history.</p>
    pub start_date: Option<String>,
}

/// Serialize `DescribeAlarmHistoryRequest` contents to a `SignedRequest`.
struct DescribeAlarmHistoryRequestSerializer;
impl DescribeAlarmHistoryRequestSerializer {
    fn serialize(params: &mut impl ServiceParams, name: &str, obj: &DescribeAlarmHistoryRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.alarm_name {
            params.put(&format!("{}{}", prefix, "AlarmName"), &field_value);
        }
        if let Some(ref field_value) = obj.end_date {
            params.put(&format!("{}{}", prefix, "EndDate"), &field_value);
        }
        if let Some(ref field_value) = obj.history_item_type {
            params.put(&format!("{}{}", prefix, "HistoryItemType"), &field_value);
        }
        if let Some(ref field_value) = obj.max_records {
            params.put(&format!("{}{}", prefix, "MaxRecords"), &field_value);
        }
        if let Some(ref field_value) = obj.next_token {
            params.put(&format!("{}{}", prefix, "NextToken"), &field_value);
        }
        if let Some(ref field_value) = obj.start_date {
            params.put(&format!("{}{}", prefix, "StartDate"), &field_value);
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeAlarmHistoryResponse {
    /// <p>The alarm histories, in JSON format.</p>
    pub alarm_history_items: Option<Vec<AlarmHistoryItem>>,
    /// <p>The token that marks the start of the next batch of returned results.</p>
    pub next_token: Option<String>,
}

struct DescribeAlarmHistoryResponseDeserializer;
impl DescribeAlarmHistoryResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DescribeAlarmHistoryResponse, XmlParseError> {
        deserialize_elements::<_, DescribeAlarmHistoryResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "AlarmHistoryItems" => {
                        obj.alarm_history_items.get_or_insert(vec![]).extend(
                            AlarmHistoryItemsDeserializer::deserialize("AlarmHistoryItems", stack)?,
                        );
                    }
                    "NextToken" => {
                        obj.next_token =
                            Some(NextTokenDeserializer::deserialize("NextToken", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeAlarmsForMetricRequest {
    /// <p>The dimensions associated with the metric. If the metric has any associated dimensions, you must specify them in order for the call to succeed.</p>
    pub dimensions: Option<Vec<Dimension>>,
    /// <p>The percentile statistic for the metric. Specify a value between p0.0 and p100.</p>
    pub extended_statistic: Option<String>,
    /// <p>The name of the metric.</p>
    pub metric_name: String,
    /// <p>The namespace of the metric.</p>
    pub namespace: String,
    /// <p>The period, in seconds, over which the statistic is applied.</p>
    pub period: Option<i64>,
    /// <p>The statistic for the metric, other than percentiles. For percentile statistics, use <code>ExtendedStatistics</code>.</p>
    pub statistic: Option<String>,
    /// <p>The unit for the metric.</p>
    pub unit: Option<String>,
}

/// Serialize `DescribeAlarmsForMetricRequest` contents to a `SignedRequest`.
struct DescribeAlarmsForMetricRequestSerializer;
impl DescribeAlarmsForMetricRequestSerializer {
    fn serialize(
        params: &mut impl ServiceParams,
        name: &str,
        obj: &DescribeAlarmsForMetricRequest,
    ) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.dimensions {
            DimensionsSerializer::serialize(
                params,
                &format!("{}{}", prefix, "Dimensions"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.extended_statistic {
            params.put(&format!("{}{}", prefix, "ExtendedStatistic"), &field_value);
        }
        params.put(&format!("{}{}", prefix, "MetricName"), &obj.metric_name);
        params.put(&format!("{}{}", prefix, "Namespace"), &obj.namespace);
        if let Some(ref field_value) = obj.period {
            params.put(&format!("{}{}", prefix, "Period"), &field_value);
        }
        if let Some(ref field_value) = obj.statistic {
            params.put(&format!("{}{}", prefix, "Statistic"), &field_value);
        }
        if let Some(ref field_value) = obj.unit {
            params.put(&format!("{}{}", prefix, "Unit"), &field_value);
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeAlarmsForMetricResponse {
    /// <p>The information for each alarm with the specified metric.</p>
    pub metric_alarms: Option<Vec<MetricAlarm>>,
}

struct DescribeAlarmsForMetricResponseDeserializer;
impl DescribeAlarmsForMetricResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DescribeAlarmsForMetricResponse, XmlParseError> {
        deserialize_elements::<_, DescribeAlarmsForMetricResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "MetricAlarms" => {
                        obj.metric_alarms.get_or_insert(vec![]).extend(
                            MetricAlarmsDeserializer::deserialize("MetricAlarms", stack)?,
                        );
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeAlarmsRequest {
    /// <p>The action name prefix.</p>
    pub action_prefix: Option<String>,
    /// <p>The alarm name prefix. If this parameter is specified, you cannot specify <code>AlarmNames</code>.</p>
    pub alarm_name_prefix: Option<String>,
    /// <p>The names of the alarms.</p>
    pub alarm_names: Option<Vec<String>>,
    /// <p>The maximum number of alarm descriptions to retrieve.</p>
    pub max_records: Option<i64>,
    /// <p>The token returned by a previous call to indicate that there is more data available.</p>
    pub next_token: Option<String>,
    /// <p>The state value to be used in matching alarms.</p>
    pub state_value: Option<String>,
}

/// Serialize `DescribeAlarmsRequest` contents to a `SignedRequest`.
struct DescribeAlarmsRequestSerializer;
impl DescribeAlarmsRequestSerializer {
    fn serialize(params: &mut impl ServiceParams, name: &str, obj: &DescribeAlarmsRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.action_prefix {
            params.put(&format!("{}{}", prefix, "ActionPrefix"), &field_value);
        }
        if let Some(ref field_value) = obj.alarm_name_prefix {
            params.put(&format!("{}{}", prefix, "AlarmNamePrefix"), &field_value);
        }
        if let Some(ref field_value) = obj.alarm_names {
            AlarmNamesSerializer::serialize(
                params,
                &format!("{}{}", prefix, "AlarmNames"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.max_records {
            params.put(&format!("{}{}", prefix, "MaxRecords"), &field_value);
        }
        if let Some(ref field_value) = obj.next_token {
            params.put(&format!("{}{}", prefix, "NextToken"), &field_value);
        }
        if let Some(ref field_value) = obj.state_value {
            params.put(&format!("{}{}", prefix, "StateValue"), &field_value);
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct DescribeAlarmsResponse {
    /// <p>The information for the specified alarms.</p>
    pub metric_alarms: Option<Vec<MetricAlarm>>,
    /// <p>The token that marks the start of the next batch of returned results.</p>
    pub next_token: Option<String>,
}

struct DescribeAlarmsResponseDeserializer;
impl DescribeAlarmsResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DescribeAlarmsResponse, XmlParseError> {
        deserialize_elements::<_, DescribeAlarmsResponse, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "MetricAlarms" => {
                    obj.metric_alarms.get_or_insert(vec![]).extend(
                        MetricAlarmsDeserializer::deserialize("MetricAlarms", stack)?,
                    );
                }
                "NextToken" => {
                    obj.next_token = Some(NextTokenDeserializer::deserialize("NextToken", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
/// <p>Expands the identity of a metric.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Dimension {
    /// <p>The name of the dimension.</p>
    pub name: String,
    /// <p>The value representing the dimension measurement.</p>
    pub value: String,
}

struct DimensionDeserializer;
impl DimensionDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Dimension, XmlParseError> {
        deserialize_elements::<_, Dimension, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Name" => {
                    obj.name = DimensionNameDeserializer::deserialize("Name", stack)?;
                }
                "Value" => {
                    obj.value = DimensionValueDeserializer::deserialize("Value", stack)?;
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

/// Serialize `Dimension` contents to a `SignedRequest`.
struct DimensionSerializer;
impl DimensionSerializer {
    fn serialize(params: &mut impl ServiceParams, name: &str, obj: &Dimension) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(&format!("{}{}", prefix, "Name"), &obj.name);
        params.put(&format!("{}{}", prefix, "Value"), &obj.value);
    }
}

/// <p>Represents filters for a dimension.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct DimensionFilter {
    /// <p>The dimension name to be matched.</p>
    pub name: String,
    /// <p>The value of the dimension to be matched.</p>
    pub value: Option<String>,
}

/// Serialize `DimensionFilter` contents to a `SignedRequest`.
struct DimensionFilterSerializer;
impl DimensionFilterSerializer {
    fn serialize(params: &mut impl ServiceParams, name: &str, obj: &DimensionFilter) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(&format!("{}{}", prefix, "Name"), &obj.name);
        if let Some(ref field_value) = obj.value {
            params.put(&format!("{}{}", prefix, "Value"), &field_value);
        }
    }
}

/// Serialize `DimensionFilters` contents to a `SignedRequest`.
struct DimensionFiltersSerializer;
impl DimensionFiltersSerializer {
    fn serialize(params: &mut impl ServiceParams, name: &str, obj: &Vec<DimensionFilter>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            DimensionFilterSerializer::serialize(params, &key, obj);
        }
    }
}

struct DimensionNameDeserializer;
impl DimensionNameDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct DimensionValueDeserializer;
impl DimensionValueDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct DimensionsDeserializer;
impl DimensionsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<Dimension>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(DimensionDeserializer::deserialize("member", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}

/// Serialize `Dimensions` contents to a `SignedRequest`.
struct DimensionsSerializer;
impl DimensionsSerializer {
    fn serialize(params: &mut impl ServiceParams, name: &str, obj: &Vec<Dimension>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            DimensionSerializer::serialize(params, &key, obj);
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct DisableAlarmActionsRequest {
    /// <p>The names of the alarms.</p>
    pub alarm_names: Vec<String>,
}

/// Serialize `DisableAlarmActionsRequest` contents to a `SignedRequest`.
struct DisableAlarmActionsRequestSerializer;
impl DisableAlarmActionsRequestSerializer {
    fn serialize(params: &mut impl ServiceParams, name: &str, obj: &DisableAlarmActionsRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        AlarmNamesSerializer::serialize(
            params,
            &format!("{}{}", prefix, "AlarmNames"),
            &obj.alarm_names,
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct DisableAlarmActionsResponse {}

struct DisableAlarmActionsResponseDeserializer;
impl DisableAlarmActionsResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DisableAlarmActionsResponse, XmlParseError> {
        Ok(DisableAlarmActionsResponse::default())
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct EnableAlarmActionsRequest {
    /// <p>The names of the alarms.</p>
    pub alarm_names: Vec<String>,
}

/// Serialize `EnableAlarmActionsRequest` contents to a `SignedRequest`.
struct EnableAlarmActionsRequestSerializer;
impl EnableAlarmActionsRequestSerializer {
    fn serialize(params: &mut impl ServiceParams, name: &str, obj: &EnableAlarmActionsRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        AlarmNamesSerializer::serialize(
            params,
            &format!("{}{}", prefix, "AlarmNames"),
            &obj.alarm_names,
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct EnableAlarmActionsResponse {}

struct EnableAlarmActionsResponseDeserializer;
impl EnableAlarmActionsResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<EnableAlarmActionsResponse, XmlParseError> {
        Ok(EnableAlarmActionsResponse::default())
    }
}
struct EvaluateLowSampleCountPercentileDeserializer;
impl EvaluateLowSampleCountPercentileDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct EvaluationPeriodsDeserializer;
impl EvaluationPeriodsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<i64, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = i64::from_str(characters(stack)?.as_ref()).unwrap();
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct ExtendedStatisticDeserializer;
impl ExtendedStatisticDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}

/// Serialize `ExtendedStatistics` contents to a `SignedRequest`.
struct ExtendedStatisticsSerializer;
impl ExtendedStatisticsSerializer {
    fn serialize(params: &mut impl ServiceParams, name: &str, obj: &Vec<String>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetDashboardRequest {
    /// <p>The name of the dashboard to be described.</p>
    pub dashboard_name: String,
}

/// Serialize `GetDashboardRequest` contents to a `SignedRequest`.
struct GetDashboardRequestSerializer;
impl GetDashboardRequestSerializer {
    fn serialize(params: &mut impl ServiceParams, name: &str, obj: &GetDashboardRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "DashboardName"),
            &obj.dashboard_name,
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetDashboardResponse {
    /// <p>The Amazon Resource Name (ARN) of the dashboard.</p>
    pub dashboard_arn: Option<String>,
    /// <p>The detailed information about the dashboard, including what widgets are included and their location on the dashboard. For more information about the <code>DashboardBody</code> syntax, see <a>CloudWatch-Dashboard-Body-Structure</a>. </p>
    pub dashboard_body: Option<String>,
    /// <p>The name of the dashboard.</p>
    pub dashboard_name: Option<String>,
}

struct GetDashboardResponseDeserializer;
impl GetDashboardResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetDashboardResponse, XmlParseError> {
        deserialize_elements::<_, GetDashboardResponse, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "DashboardArn" => {
                    obj.dashboard_arn = Some(DashboardArnDeserializer::deserialize(
                        "DashboardArn",
                        stack,
                    )?);
                }
                "DashboardBody" => {
                    obj.dashboard_body = Some(DashboardBodyDeserializer::deserialize(
                        "DashboardBody",
                        stack,
                    )?);
                }
                "DashboardName" => {
                    obj.dashboard_name = Some(DashboardNameDeserializer::deserialize(
                        "DashboardName",
                        stack,
                    )?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetMetricDataRequest {
    /// <p>The time stamp indicating the latest data to be returned.</p> <p>For better performance, specify <code>StartTime</code> and <code>EndTime</code> values that align with the value of the metric's <code>Period</code> and sync up with the beginning and end of an hour. For example, if the <code>Period</code> of a metric is 5 minutes, specifying 12:05 or 12:30 as <code>EndTime</code> can get a faster response from CloudWatch than setting 12:07 or 12:29 as the <code>EndTime</code>.</p>
    pub end_time: String,
    /// <p>The maximum number of data points the request should return before paginating. If you omit this, the default of 100,800 is used.</p>
    pub max_datapoints: Option<i64>,
    /// <p>The metric queries to be returned. A single <code>GetMetricData</code> call can include as many as 100 <code>MetricDataQuery</code> structures. Each of these structures can specify either a metric to retrieve, or a math expression to perform on retrieved data. </p>
    pub metric_data_queries: Vec<MetricDataQuery>,
    /// <p>Include this value, if it was returned by the previous call, to get the next set of data points.</p>
    pub next_token: Option<String>,
    /// <p>The order in which data points should be returned. <code>TimestampDescending</code> returns the newest data first and paginates when the <code>MaxDatapoints</code> limit is reached. <code>TimestampAscending</code> returns the oldest data first and paginates when the <code>MaxDatapoints</code> limit is reached.</p>
    pub scan_by: Option<String>,
    /// <p>The time stamp indicating the earliest data to be returned.</p> <p>For better performance, specify <code>StartTime</code> and <code>EndTime</code> values that align with the value of the metric's <code>Period</code> and sync up with the beginning and end of an hour. For example, if the <code>Period</code> of a metric is 5 minutes, specifying 12:05 or 12:30 as <code>StartTime</code> can get a faster response from CloudWatch than setting 12:07 or 12:29 as the <code>StartTime</code>.</p>
    pub start_time: String,
}

/// Serialize `GetMetricDataRequest` contents to a `SignedRequest`.
struct GetMetricDataRequestSerializer;
impl GetMetricDataRequestSerializer {
    fn serialize(params: &mut impl ServiceParams, name: &str, obj: &GetMetricDataRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(&format!("{}{}", prefix, "EndTime"), &obj.end_time);
        if let Some(ref field_value) = obj.max_datapoints {
            params.put(&format!("{}{}", prefix, "MaxDatapoints"), &field_value);
        }
        MetricDataQueriesSerializer::serialize(
            params,
            &format!("{}{}", prefix, "MetricDataQueries"),
            &obj.metric_data_queries,
        );
        if let Some(ref field_value) = obj.next_token {
            params.put(&format!("{}{}", prefix, "NextToken"), &field_value);
        }
        if let Some(ref field_value) = obj.scan_by {
            params.put(&format!("{}{}", prefix, "ScanBy"), &field_value);
        }
        params.put(&format!("{}{}", prefix, "StartTime"), &obj.start_time);
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetMetricDataResponse {
    /// <p>Contains a message about this <code>GetMetricData</code> operation, if the operation results in such a message. An example of a message that may be returned is <code>Maximum number of allowed metrics exceeded</code>. If there is a message, as much of the operation as possible is still executed.</p> <p>A message appears here only if it is related to the global <code>GetMetricData</code> operation. Any message about a specific metric returned by the operation appears in the <code>MetricDataResult</code> object returned for that metric.</p>
    pub messages: Option<Vec<MessageData>>,
    /// <p>The metrics that are returned, including the metric name, namespace, and dimensions.</p>
    pub metric_data_results: Option<Vec<MetricDataResult>>,
    /// <p>A token that marks the next batch of returned results.</p>
    pub next_token: Option<String>,
}

struct GetMetricDataResponseDeserializer;
impl GetMetricDataResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetMetricDataResponse, XmlParseError> {
        deserialize_elements::<_, GetMetricDataResponse, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Messages" => {
                    obj.messages.get_or_insert(vec![]).extend(
                        MetricDataResultMessagesDeserializer::deserialize("Messages", stack)?,
                    );
                }
                "MetricDataResults" => {
                    obj.metric_data_results.get_or_insert(vec![]).extend(
                        MetricDataResultsDeserializer::deserialize("MetricDataResults", stack)?,
                    );
                }
                "NextToken" => {
                    obj.next_token = Some(NextTokenDeserializer::deserialize("NextToken", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetMetricStatisticsRequest {
    /// <p>The dimensions. If the metric contains multiple dimensions, you must include a value for each dimension. CloudWatch treats each unique combination of dimensions as a separate metric. If a specific combination of dimensions was not published, you can't retrieve statistics for it. You must specify the same dimensions that were used when the metrics were created. For an example, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/cloudwatch_concepts.html#dimension-combinations">Dimension Combinations</a> in the <i>Amazon CloudWatch User Guide</i>. For more information about specifying dimensions, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/publishingMetrics.html">Publishing Metrics</a> in the <i>Amazon CloudWatch User Guide</i>.</p>
    pub dimensions: Option<Vec<Dimension>>,
    /// <p>The time stamp that determines the last data point to return.</p> <p>The value specified is exclusive; results include data points up to the specified time stamp. The time stamp must be in ISO 8601 UTC format (for example, 2016-10-10T23:00:00Z).</p>
    pub end_time: String,
    /// <p>The percentile statistics. Specify values between p0.0 and p100. When calling <code>GetMetricStatistics</code>, you must specify either <code>Statistics</code> or <code>ExtendedStatistics</code>, but not both. Percentile statistics are not available for metrics when any of the metric values are negative numbers.</p>
    pub extended_statistics: Option<Vec<String>>,
    /// <p>The name of the metric, with or without spaces.</p>
    pub metric_name: String,
    /// <p>The namespace of the metric, with or without spaces.</p>
    pub namespace: String,
    /// <p><p>The granularity, in seconds, of the returned data points. For metrics with regular resolution, a period can be as short as one minute (60 seconds) and must be a multiple of 60. For high-resolution metrics that are collected at intervals of less than one minute, the period can be 1, 5, 10, 30, 60, or any multiple of 60. High-resolution metrics are those metrics stored by a <code>PutMetricData</code> call that includes a <code>StorageResolution</code> of 1 second.</p> <p>If the <code>StartTime</code> parameter specifies a time stamp that is greater than 3 hours ago, you must specify the period as follows or no data points in that time range is returned:</p> <ul> <li> <p>Start time between 3 hours and 15 days ago - Use a multiple of 60 seconds (1 minute).</p> </li> <li> <p>Start time between 15 and 63 days ago - Use a multiple of 300 seconds (5 minutes).</p> </li> <li> <p>Start time greater than 63 days ago - Use a multiple of 3600 seconds (1 hour).</p> </li> </ul></p>
    pub period: i64,
    /// <p>The time stamp that determines the first data point to return. Start times are evaluated relative to the time that CloudWatch receives the request.</p> <p>The value specified is inclusive; results include data points with the specified time stamp. The time stamp must be in ISO 8601 UTC format (for example, 2016-10-03T23:00:00Z).</p> <p>CloudWatch rounds the specified time stamp as follows:</p> <ul> <li> <p>Start time less than 15 days ago - Round down to the nearest whole minute. For example, 12:32:34 is rounded down to 12:32:00.</p> </li> <li> <p>Start time between 15 and 63 days ago - Round down to the nearest 5-minute clock interval. For example, 12:32:34 is rounded down to 12:30:00.</p> </li> <li> <p>Start time greater than 63 days ago - Round down to the nearest 1-hour clock interval. For example, 12:32:34 is rounded down to 12:00:00.</p> </li> </ul> <p>If you set <code>Period</code> to 5, 10, or 30, the start time of your request is rounded down to the nearest time that corresponds to even 5-, 10-, or 30-second divisions of a minute. For example, if you make a query at (HH:mm:ss) 01:05:23 for the previous 10-second period, the start time of your request is rounded down and you receive data from 01:05:10 to 01:05:20. If you make a query at 15:07:17 for the previous 5 minutes of data, using a period of 5 seconds, you receive data timestamped between 15:02:15 and 15:07:15. </p>
    pub start_time: String,
    /// <p>The metric statistics, other than percentile. For percentile statistics, use <code>ExtendedStatistics</code>. When calling <code>GetMetricStatistics</code>, you must specify either <code>Statistics</code> or <code>ExtendedStatistics</code>, but not both.</p>
    pub statistics: Option<Vec<String>>,
    /// <p>The unit for a given metric. Metrics may be reported in multiple units. Not supplying a unit results in all units being returned. If you specify only a unit that the metric does not report, the results of the call are null.</p>
    pub unit: Option<String>,
}

/// Serialize `GetMetricStatisticsRequest` contents to a `SignedRequest`.
struct GetMetricStatisticsRequestSerializer;
impl GetMetricStatisticsRequestSerializer {
    fn serialize(params: &mut impl ServiceParams, name: &str, obj: &GetMetricStatisticsRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.dimensions {
            DimensionsSerializer::serialize(
                params,
                &format!("{}{}", prefix, "Dimensions"),
                field_value,
            );
        }
        params.put(&format!("{}{}", prefix, "EndTime"), &obj.end_time);
        if let Some(ref field_value) = obj.extended_statistics {
            ExtendedStatisticsSerializer::serialize(
                params,
                &format!("{}{}", prefix, "ExtendedStatistics"),
                field_value,
            );
        }
        params.put(&format!("{}{}", prefix, "MetricName"), &obj.metric_name);
        params.put(&format!("{}{}", prefix, "Namespace"), &obj.namespace);
        params.put(&format!("{}{}", prefix, "Period"), &obj.period);
        params.put(&format!("{}{}", prefix, "StartTime"), &obj.start_time);
        if let Some(ref field_value) = obj.statistics {
            StatisticsSerializer::serialize(
                params,
                &format!("{}{}", prefix, "Statistics"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.unit {
            params.put(&format!("{}{}", prefix, "Unit"), &field_value);
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetMetricStatisticsResponse {
    /// <p>The data points for the specified metric.</p>
    pub datapoints: Option<Vec<Datapoint>>,
    /// <p>A label for the specified metric.</p>
    pub label: Option<String>,
}

struct GetMetricStatisticsResponseDeserializer;
impl GetMetricStatisticsResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetMetricStatisticsResponse, XmlParseError> {
        deserialize_elements::<_, GetMetricStatisticsResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Datapoints" => {
                        obj.datapoints
                            .get_or_insert(vec![])
                            .extend(DatapointsDeserializer::deserialize("Datapoints", stack)?);
                    }
                    "Label" => {
                        obj.label = Some(MetricLabelDeserializer::deserialize("Label", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetMetricWidgetImageRequest {
    /// <p>A JSON string that defines the bitmap graph to be retrieved. The string includes the metrics to include in the graph, statistics, annotations, title, axis limits, and so on. You can include only one <code>MetricWidget</code> parameter in each <code>GetMetricWidgetImage</code> call.</p> <p>For more information about the syntax of <code>MetricWidget</code> see <a>CloudWatch-Metric-Widget-Structure</a>.</p> <p>If any metric on the graph could not load all the requested data points, an orange triangle with an exclamation point appears next to the graph legend.</p>
    pub metric_widget: String,
    /// <p>The format of the resulting image. Only PNG images are supported.</p> <p>The default is <code>png</code>. If you specify <code>png</code>, the API returns an HTTP response with the content-type set to <code>text/xml</code>. The image data is in a <code>MetricWidgetImage</code> field. For example:</p> <p> <code> &lt;GetMetricWidgetImageResponse xmlns=&lt;URLstring&gt;&gt;</code> </p> <p> <code> &lt;GetMetricWidgetImageResult&gt;</code> </p> <p> <code> &lt;MetricWidgetImage&gt;</code> </p> <p> <code> iVBORw0KGgoAAAANSUhEUgAAAlgAAAGQEAYAAAAip...</code> </p> <p> <code> &lt;/MetricWidgetImage&gt;</code> </p> <p> <code> &lt;/GetMetricWidgetImageResult&gt;</code> </p> <p> <code> &lt;ResponseMetadata&gt;</code> </p> <p> <code> &lt;RequestId&gt;6f0d4192-4d42-11e8-82c1-f539a07e0e3b&lt;/RequestId&gt;</code> </p> <p> <code> &lt;/ResponseMetadata&gt;</code> </p> <p> <code>&lt;/GetMetricWidgetImageResponse&gt;</code> </p> <p>The <code>image/png</code> setting is intended only for custom HTTP requests. For most use cases, and all actions using an AWS SDK, you should use <code>png</code>. If you specify <code>image/png</code>, the HTTP response has a content-type set to <code>image/png</code>, and the body of the response is a PNG image. </p>
    pub output_format: Option<String>,
}

/// Serialize `GetMetricWidgetImageRequest` contents to a `SignedRequest`.
struct GetMetricWidgetImageRequestSerializer;
impl GetMetricWidgetImageRequestSerializer {
    fn serialize(params: &mut impl ServiceParams, name: &str, obj: &GetMetricWidgetImageRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(&format!("{}{}", prefix, "MetricWidget"), &obj.metric_widget);
        if let Some(ref field_value) = obj.output_format {
            params.put(&format!("{}{}", prefix, "OutputFormat"), &field_value);
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetMetricWidgetImageResponse {
    /// <p>The image of the graph, in the output format specified.</p>
    pub metric_widget_image: Option<bytes::Bytes>,
}

struct GetMetricWidgetImageResponseDeserializer;
impl GetMetricWidgetImageResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GetMetricWidgetImageResponse, XmlParseError> {
        deserialize_elements::<_, GetMetricWidgetImageResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "MetricWidgetImage" => {
                        obj.metric_widget_image = Some(MetricWidgetImageDeserializer::deserialize(
                            "MetricWidgetImage",
                            stack,
                        )?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
struct HistoryDataDeserializer;
impl HistoryDataDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct HistoryItemTypeDeserializer;
impl HistoryItemTypeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct HistorySummaryDeserializer;
impl HistorySummaryDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct LastModifiedDeserializer;
impl LastModifiedDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListDashboardsRequest {
    /// <p>If you specify this parameter, only the dashboards with names starting with the specified string are listed. The maximum length is 255, and valid characters are A-Z, a-z, 0-9, ".", "-", and "_". </p>
    pub dashboard_name_prefix: Option<String>,
    /// <p>The token returned by a previous call to indicate that there is more data available.</p>
    pub next_token: Option<String>,
}

/// Serialize `ListDashboardsRequest` contents to a `SignedRequest`.
struct ListDashboardsRequestSerializer;
impl ListDashboardsRequestSerializer {
    fn serialize(params: &mut impl ServiceParams, name: &str, obj: &ListDashboardsRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.dashboard_name_prefix {
            params.put(
                &format!("{}{}", prefix, "DashboardNamePrefix"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.next_token {
            params.put(&format!("{}{}", prefix, "NextToken"), &field_value);
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListDashboardsResponse {
    /// <p>The list of matching dashboards.</p>
    pub dashboard_entries: Option<Vec<DashboardEntry>>,
    /// <p>The token that marks the start of the next batch of returned results.</p>
    pub next_token: Option<String>,
}

struct ListDashboardsResponseDeserializer;
impl ListDashboardsResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListDashboardsResponse, XmlParseError> {
        deserialize_elements::<_, ListDashboardsResponse, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "DashboardEntries" => {
                    obj.dashboard_entries.get_or_insert(vec![]).extend(
                        DashboardEntriesDeserializer::deserialize("DashboardEntries", stack)?,
                    );
                }
                "NextToken" => {
                    obj.next_token = Some(NextTokenDeserializer::deserialize("NextToken", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListMetricsRequest {
    /// <p>The dimensions to filter against.</p>
    pub dimensions: Option<Vec<DimensionFilter>>,
    /// <p>The name of the metric to filter against.</p>
    pub metric_name: Option<String>,
    /// <p>The namespace to filter against.</p>
    pub namespace: Option<String>,
    /// <p>The token returned by a previous call to indicate that there is more data available.</p>
    pub next_token: Option<String>,
}

/// Serialize `ListMetricsRequest` contents to a `SignedRequest`.
struct ListMetricsRequestSerializer;
impl ListMetricsRequestSerializer {
    fn serialize(params: &mut impl ServiceParams, name: &str, obj: &ListMetricsRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.dimensions {
            DimensionFiltersSerializer::serialize(
                params,
                &format!("{}{}", prefix, "Dimensions"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.metric_name {
            params.put(&format!("{}{}", prefix, "MetricName"), &field_value);
        }
        if let Some(ref field_value) = obj.namespace {
            params.put(&format!("{}{}", prefix, "Namespace"), &field_value);
        }
        if let Some(ref field_value) = obj.next_token {
            params.put(&format!("{}{}", prefix, "NextToken"), &field_value);
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListMetricsResponse {
    /// <p>The metrics.</p>
    pub metrics: Option<Vec<Metric>>,
    /// <p>The token that marks the start of the next batch of returned results.</p>
    pub next_token: Option<String>,
}

struct ListMetricsResponseDeserializer;
impl ListMetricsResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListMetricsResponse, XmlParseError> {
        deserialize_elements::<_, ListMetricsResponse, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Metrics" => {
                    obj.metrics
                        .get_or_insert(vec![])
                        .extend(MetricsDeserializer::deserialize("Metrics", stack)?);
                }
                "NextToken" => {
                    obj.next_token = Some(NextTokenDeserializer::deserialize("NextToken", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListTagsForResourceRequest {
    /// <p>The ARN of the CloudWatch resource that you want to view tags for. For more information on ARN format, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html#arn-syntax-cloudwatch">Example ARNs</a> in the <i>Amazon Web Services General Reference</i>.</p>
    pub resource_arn: String,
}

/// Serialize `ListTagsForResourceRequest` contents to a `SignedRequest`.
struct ListTagsForResourceRequestSerializer;
impl ListTagsForResourceRequestSerializer {
    fn serialize(params: &mut impl ServiceParams, name: &str, obj: &ListTagsForResourceRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(&format!("{}{}", prefix, "ResourceARN"), &obj.resource_arn);
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListTagsForResourceResponse {
    /// <p>The list of tag keys and values associated with the resource you specified.</p>
    pub tags: Option<Vec<Tag>>,
}

struct ListTagsForResourceResponseDeserializer;
impl ListTagsForResourceResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ListTagsForResourceResponse, XmlParseError> {
        deserialize_elements::<_, ListTagsForResourceResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Tags" => {
                        obj.tags
                            .get_or_insert(vec![])
                            .extend(TagListDeserializer::deserialize("Tags", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
struct MessageDeserializer;
impl MessageDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>A message returned by the <code>GetMetricData</code>API, including a code and a description.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct MessageData {
    /// <p>The error code or status code associated with the message.</p>
    pub code: Option<String>,
    /// <p>The message text.</p>
    pub value: Option<String>,
}

struct MessageDataDeserializer;
impl MessageDataDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<MessageData, XmlParseError> {
        deserialize_elements::<_, MessageData, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Code" => {
                    obj.code = Some(MessageDataCodeDeserializer::deserialize("Code", stack)?);
                }
                "Value" => {
                    obj.value = Some(MessageDataValueDeserializer::deserialize("Value", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
struct MessageDataCodeDeserializer;
impl MessageDataCodeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct MessageDataValueDeserializer;
impl MessageDataValueDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>Represents a specific metric.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Metric {
    /// <p>The dimensions for the metric.</p>
    pub dimensions: Option<Vec<Dimension>>,
    /// <p>The name of the metric. This is a required field.</p>
    pub metric_name: Option<String>,
    /// <p>The namespace of the metric.</p>
    pub namespace: Option<String>,
}

struct MetricDeserializer;
impl MetricDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Metric, XmlParseError> {
        deserialize_elements::<_, Metric, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Dimensions" => {
                    obj.dimensions
                        .get_or_insert(vec![])
                        .extend(DimensionsDeserializer::deserialize("Dimensions", stack)?);
                }
                "MetricName" => {
                    obj.metric_name =
                        Some(MetricNameDeserializer::deserialize("MetricName", stack)?);
                }
                "Namespace" => {
                    obj.namespace = Some(NamespaceDeserializer::deserialize("Namespace", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

/// Serialize `Metric` contents to a `SignedRequest`.
struct MetricSerializer;
impl MetricSerializer {
    fn serialize(params: &mut impl ServiceParams, name: &str, obj: &Metric) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.dimensions {
            DimensionsSerializer::serialize(
                params,
                &format!("{}{}", prefix, "Dimensions"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.metric_name {
            params.put(&format!("{}{}", prefix, "MetricName"), &field_value);
        }
        if let Some(ref field_value) = obj.namespace {
            params.put(&format!("{}{}", prefix, "Namespace"), &field_value);
        }
    }
}

/// <p>Represents an alarm.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct MetricAlarm {
    /// <p>Indicates whether actions should be executed during any changes to the alarm state.</p>
    pub actions_enabled: Option<bool>,
    /// <p>The actions to execute when this alarm transitions to the <code>ALARM</code> state from any other state. Each action is specified as an Amazon Resource Name (ARN).</p>
    pub alarm_actions: Option<Vec<String>>,
    /// <p>The Amazon Resource Name (ARN) of the alarm.</p>
    pub alarm_arn: Option<String>,
    /// <p>The time stamp of the last update to the alarm configuration.</p>
    pub alarm_configuration_updated_timestamp: Option<String>,
    /// <p>The description of the alarm.</p>
    pub alarm_description: Option<String>,
    /// <p>The name of the alarm.</p>
    pub alarm_name: Option<String>,
    /// <p>The arithmetic operation to use when comparing the specified statistic and threshold. The specified statistic value is used as the first operand.</p>
    pub comparison_operator: Option<String>,
    /// <p>The number of datapoints that must be breaching to trigger the alarm.</p>
    pub datapoints_to_alarm: Option<i64>,
    /// <p>The dimensions for the metric associated with the alarm.</p>
    pub dimensions: Option<Vec<Dimension>>,
    /// <p>Used only for alarms based on percentiles. If <code>ignore</code>, the alarm state does not change during periods with too few data points to be statistically significant. If <code>evaluate</code> or this parameter is not used, the alarm is always evaluated and possibly changes state no matter how many data points are available.</p>
    pub evaluate_low_sample_count_percentile: Option<String>,
    /// <p>The number of periods over which data is compared to the specified threshold.</p>
    pub evaluation_periods: Option<i64>,
    /// <p>The percentile statistic for the metric associated with the alarm. Specify a value between p0.0 and p100.</p>
    pub extended_statistic: Option<String>,
    /// <p>The actions to execute when this alarm transitions to the <code>INSUFFICIENT_DATA</code> state from any other state. Each action is specified as an Amazon Resource Name (ARN).</p>
    pub insufficient_data_actions: Option<Vec<String>>,
    /// <p>The name of the metric associated with the alarm.</p>
    pub metric_name: Option<String>,
    /// <p><p/></p>
    pub metrics: Option<Vec<MetricDataQuery>>,
    /// <p>The namespace of the metric associated with the alarm.</p>
    pub namespace: Option<String>,
    /// <p>The actions to execute when this alarm transitions to the <code>OK</code> state from any other state. Each action is specified as an Amazon Resource Name (ARN).</p>
    pub ok_actions: Option<Vec<String>>,
    /// <p>The period, in seconds, over which the statistic is applied.</p>
    pub period: Option<i64>,
    /// <p>An explanation for the alarm state, in text format.</p>
    pub state_reason: Option<String>,
    /// <p>An explanation for the alarm state, in JSON format.</p>
    pub state_reason_data: Option<String>,
    /// <p>The time stamp of the last update to the alarm state.</p>
    pub state_updated_timestamp: Option<String>,
    /// <p>The state value for the alarm.</p>
    pub state_value: Option<String>,
    /// <p>The statistic for the metric associated with the alarm, other than percentile. For percentile statistics, use <code>ExtendedStatistic</code>.</p>
    pub statistic: Option<String>,
    /// <p>The value to compare with the specified statistic.</p>
    pub threshold: Option<f64>,
    /// <p>Sets how this alarm is to handle missing data points. If this parameter is omitted, the default behavior of <code>missing</code> is used.</p>
    pub treat_missing_data: Option<String>,
    /// <p>The unit of the metric associated with the alarm.</p>
    pub unit: Option<String>,
}

struct MetricAlarmDeserializer;
impl MetricAlarmDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<MetricAlarm, XmlParseError> {
        deserialize_elements::<_, MetricAlarm, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "ActionsEnabled" => {
                    obj.actions_enabled = Some(ActionsEnabledDeserializer::deserialize(
                        "ActionsEnabled",
                        stack,
                    )?);
                }
                "AlarmActions" => {
                    obj.alarm_actions.get_or_insert(vec![]).extend(
                        ResourceListDeserializer::deserialize("AlarmActions", stack)?,
                    );
                }
                "AlarmArn" => {
                    obj.alarm_arn = Some(AlarmArnDeserializer::deserialize("AlarmArn", stack)?);
                }
                "AlarmConfigurationUpdatedTimestamp" => {
                    obj.alarm_configuration_updated_timestamp =
                        Some(TimestampDeserializer::deserialize(
                            "AlarmConfigurationUpdatedTimestamp",
                            stack,
                        )?);
                }
                "AlarmDescription" => {
                    obj.alarm_description = Some(AlarmDescriptionDeserializer::deserialize(
                        "AlarmDescription",
                        stack,
                    )?);
                }
                "AlarmName" => {
                    obj.alarm_name = Some(AlarmNameDeserializer::deserialize("AlarmName", stack)?);
                }
                "ComparisonOperator" => {
                    obj.comparison_operator = Some(ComparisonOperatorDeserializer::deserialize(
                        "ComparisonOperator",
                        stack,
                    )?);
                }
                "DatapointsToAlarm" => {
                    obj.datapoints_to_alarm = Some(DatapointsToAlarmDeserializer::deserialize(
                        "DatapointsToAlarm",
                        stack,
                    )?);
                }
                "Dimensions" => {
                    obj.dimensions
                        .get_or_insert(vec![])
                        .extend(DimensionsDeserializer::deserialize("Dimensions", stack)?);
                }
                "EvaluateLowSampleCountPercentile" => {
                    obj.evaluate_low_sample_count_percentile =
                        Some(EvaluateLowSampleCountPercentileDeserializer::deserialize(
                            "EvaluateLowSampleCountPercentile",
                            stack,
                        )?);
                }
                "EvaluationPeriods" => {
                    obj.evaluation_periods = Some(EvaluationPeriodsDeserializer::deserialize(
                        "EvaluationPeriods",
                        stack,
                    )?);
                }
                "ExtendedStatistic" => {
                    obj.extended_statistic = Some(ExtendedStatisticDeserializer::deserialize(
                        "ExtendedStatistic",
                        stack,
                    )?);
                }
                "InsufficientDataActions" => {
                    obj.insufficient_data_actions.get_or_insert(vec![]).extend(
                        ResourceListDeserializer::deserialize("InsufficientDataActions", stack)?,
                    );
                }
                "MetricName" => {
                    obj.metric_name =
                        Some(MetricNameDeserializer::deserialize("MetricName", stack)?);
                }
                "Metrics" => {
                    obj.metrics.get_or_insert(vec![]).extend(
                        MetricDataQueriesDeserializer::deserialize("Metrics", stack)?,
                    );
                }
                "Namespace" => {
                    obj.namespace = Some(NamespaceDeserializer::deserialize("Namespace", stack)?);
                }
                "OKActions" => {
                    obj.ok_actions
                        .get_or_insert(vec![])
                        .extend(ResourceListDeserializer::deserialize("OKActions", stack)?);
                }
                "Period" => {
                    obj.period = Some(PeriodDeserializer::deserialize("Period", stack)?);
                }
                "StateReason" => {
                    obj.state_reason =
                        Some(StateReasonDeserializer::deserialize("StateReason", stack)?);
                }
                "StateReasonData" => {
                    obj.state_reason_data = Some(StateReasonDataDeserializer::deserialize(
                        "StateReasonData",
                        stack,
                    )?);
                }
                "StateUpdatedTimestamp" => {
                    obj.state_updated_timestamp = Some(TimestampDeserializer::deserialize(
                        "StateUpdatedTimestamp",
                        stack,
                    )?);
                }
                "StateValue" => {
                    obj.state_value =
                        Some(StateValueDeserializer::deserialize("StateValue", stack)?);
                }
                "Statistic" => {
                    obj.statistic = Some(StatisticDeserializer::deserialize("Statistic", stack)?);
                }
                "Threshold" => {
                    obj.threshold = Some(ThresholdDeserializer::deserialize("Threshold", stack)?);
                }
                "TreatMissingData" => {
                    obj.treat_missing_data = Some(TreatMissingDataDeserializer::deserialize(
                        "TreatMissingData",
                        stack,
                    )?);
                }
                "Unit" => {
                    obj.unit = Some(StandardUnitDeserializer::deserialize("Unit", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
struct MetricAlarmsDeserializer;
impl MetricAlarmsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<MetricAlarm>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(MetricAlarmDeserializer::deserialize("member", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}

/// Serialize `MetricData` contents to a `SignedRequest`.
struct MetricDataSerializer;
impl MetricDataSerializer {
    fn serialize(params: &mut impl ServiceParams, name: &str, obj: &Vec<MetricDatum>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            MetricDatumSerializer::serialize(params, &key, obj);
        }
    }
}

struct MetricDataQueriesDeserializer;
impl MetricDataQueriesDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<MetricDataQuery>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(MetricDataQueryDeserializer::deserialize("member", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}

/// Serialize `MetricDataQueries` contents to a `SignedRequest`.
struct MetricDataQueriesSerializer;
impl MetricDataQueriesSerializer {
    fn serialize(params: &mut impl ServiceParams, name: &str, obj: &Vec<MetricDataQuery>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            MetricDataQuerySerializer::serialize(params, &key, obj);
        }
    }
}

/// <p>This structure is used in both <code>GetMetricData</code> and <code>PutMetricAlarm</code>. The supported use of this structure is different for those two operations.</p> <p>When used in <code>GetMetricData</code>, it indicates the metric data to return, and whether this call is just retrieving a batch set of data for one metric, or is performing a math expression on metric data. A single <code>GetMetricData</code> call can include up to 100 <code>MetricDataQuery</code> structures.</p> <p>When used in <code>PutMetricAlarm</code>, it enables you to create an alarm based on a metric math expression. Each <code>MetricDataQuery</code> in the array specifies either a metric to retrieve, or a math expression to be performed on retrieved metrics. A single <code>PutMetricAlarm</code> call can include up to 20 <code>MetricDataQuery</code> structures in the array. The 20 structures can include as many as 10 structures that contain a <code>MetricStat</code> parameter to retrieve a metric, and as many as 10 structures that contain the <code>Expression</code> parameter to perform a math expression. Of those <code>Expression</code> structures, one must have <code>True</code> as the value for <code>ReturnData</code>. The result of this expression is the value the alarm watches.</p> <p>Any expression used in a <code>PutMetricAlarm</code> operation must return a single time series. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/using-metric-math.html#metric-math-syntax">Metric Math Syntax and Functions</a> in the <i>Amazon CloudWatch User Guide</i>.</p> <p>Some of the parameters of this structure also have different uses whether you are using this structure in a <code>GetMetricData</code> operation or a <code>PutMetricAlarm</code> operation. These differences are explained in the following parameter list.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct MetricDataQuery {
    /// <p>The math expression to be performed on the returned data, if this object is performing a math expression. This expression can use the <code>Id</code> of the other metrics to refer to those metrics, and can also use the <code>Id</code> of other expressions to use the result of those expressions. For more information about metric math expressions, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/using-metric-math.html#metric-math-syntax">Metric Math Syntax and Functions</a> in the <i>Amazon CloudWatch User Guide</i>.</p> <p>Within each MetricDataQuery object, you must specify either <code>Expression</code> or <code>MetricStat</code> but not both.</p>
    pub expression: Option<String>,
    /// <p>A short name used to tie this object to the results in the response. This name must be unique within a single call to <code>GetMetricData</code>. If you are performing math expressions on this set of data, this name represents that data and can serve as a variable in the mathematical expression. The valid characters are letters, numbers, and underscore. The first character must be a lowercase letter.</p>
    pub id: String,
    /// <p>A human-readable label for this metric or expression. This is especially useful if this is an expression, so that you know what the value represents. If the metric or expression is shown in a CloudWatch dashboard widget, the label is shown. If Label is omitted, CloudWatch generates a default.</p>
    pub label: Option<String>,
    /// <p>The metric to be returned, along with statistics, period, and units. Use this parameter only if this object is retrieving a metric and not performing a math expression on returned data.</p> <p>Within one MetricDataQuery object, you must specify either <code>Expression</code> or <code>MetricStat</code> but not both.</p>
    pub metric_stat: Option<MetricStat>,
    /// <p>When used in <code>GetMetricData</code>, this option indicates whether to return the timestamps and raw data values of this metric. If you are performing this call just to do math expressions and do not also need the raw data returned, you can specify <code>False</code>. If you omit this, the default of <code>True</code> is used.</p> <p>When used in <code>PutMetricAlarm</code>, specify <code>True</code> for the one expression result to use as the alarm. For all other metrics and expressions in the same <code>PutMetricAlarm</code> operation, specify <code>ReturnData</code> as False.</p>
    pub return_data: Option<bool>,
}

struct MetricDataQueryDeserializer;
impl MetricDataQueryDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<MetricDataQuery, XmlParseError> {
        deserialize_elements::<_, MetricDataQuery, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Expression" => {
                    obj.expression = Some(MetricExpressionDeserializer::deserialize(
                        "Expression",
                        stack,
                    )?);
                }
                "Id" => {
                    obj.id = MetricIdDeserializer::deserialize("Id", stack)?;
                }
                "Label" => {
                    obj.label = Some(MetricLabelDeserializer::deserialize("Label", stack)?);
                }
                "MetricStat" => {
                    obj.metric_stat =
                        Some(MetricStatDeserializer::deserialize("MetricStat", stack)?);
                }
                "ReturnData" => {
                    obj.return_data =
                        Some(ReturnDataDeserializer::deserialize("ReturnData", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

/// Serialize `MetricDataQuery` contents to a `SignedRequest`.
struct MetricDataQuerySerializer;
impl MetricDataQuerySerializer {
    fn serialize(params: &mut impl ServiceParams, name: &str, obj: &MetricDataQuery) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.expression {
            params.put(&format!("{}{}", prefix, "Expression"), &field_value);
        }
        params.put(&format!("{}{}", prefix, "Id"), &obj.id);
        if let Some(ref field_value) = obj.label {
            params.put(&format!("{}{}", prefix, "Label"), &field_value);
        }
        if let Some(ref field_value) = obj.metric_stat {
            MetricStatSerializer::serialize(
                params,
                &format!("{}{}", prefix, "MetricStat"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.return_data {
            params.put(&format!("{}{}", prefix, "ReturnData"), &field_value);
        }
    }
}

/// <p>A <code>GetMetricData</code> call returns an array of <code>MetricDataResult</code> structures. Each of these structures includes the data points for that metric, along with the timestamps of those data points and other identifying information.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct MetricDataResult {
    /// <p>The short name you specified to represent this metric.</p>
    pub id: Option<String>,
    /// <p>The human-readable label associated with the data.</p>
    pub label: Option<String>,
    /// <p>A list of messages with additional information about the data returned.</p>
    pub messages: Option<Vec<MessageData>>,
    /// <p>The status of the returned data. <code>Complete</code> indicates that all data points in the requested time range were returned. <code>PartialData</code> means that an incomplete set of data points were returned. You can use the <code>NextToken</code> value that was returned and repeat your request to get more data points. <code>NextToken</code> is not returned if you are performing a math expression. <code>InternalError</code> indicates that an error occurred. Retry your request using <code>NextToken</code>, if present.</p>
    pub status_code: Option<String>,
    /// <p>The timestamps for the data points, formatted in Unix timestamp format. The number of timestamps always matches the number of values and the value for Timestamps[x] is Values[x].</p>
    pub timestamps: Option<Vec<String>>,
    /// <p>The data points for the metric corresponding to <code>Timestamps</code>. The number of values always matches the number of timestamps and the timestamp for Values[x] is Timestamps[x].</p>
    pub values: Option<Vec<f64>>,
}

struct MetricDataResultDeserializer;
impl MetricDataResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<MetricDataResult, XmlParseError> {
        deserialize_elements::<_, MetricDataResult, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Id" => {
                    obj.id = Some(MetricIdDeserializer::deserialize("Id", stack)?);
                }
                "Label" => {
                    obj.label = Some(MetricLabelDeserializer::deserialize("Label", stack)?);
                }
                "Messages" => {
                    obj.messages.get_or_insert(vec![]).extend(
                        MetricDataResultMessagesDeserializer::deserialize("Messages", stack)?,
                    );
                }
                "StatusCode" => {
                    obj.status_code =
                        Some(StatusCodeDeserializer::deserialize("StatusCode", stack)?);
                }
                "Timestamps" => {
                    obj.timestamps
                        .get_or_insert(vec![])
                        .extend(TimestampsDeserializer::deserialize("Timestamps", stack)?);
                }
                "Values" => {
                    obj.values
                        .get_or_insert(vec![])
                        .extend(DatapointValuesDeserializer::deserialize("Values", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
struct MetricDataResultMessagesDeserializer;
impl MetricDataResultMessagesDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<MessageData>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(MessageDataDeserializer::deserialize("member", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
struct MetricDataResultsDeserializer;
impl MetricDataResultsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<MetricDataResult>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(MetricDataResultDeserializer::deserialize("member", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
/// <p>Encapsulates the information sent to either create a metric or add new values to be aggregated into an existing metric.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct MetricDatum {
    /// <p>Array of numbers that is used along with the <code>Values</code> array. Each number in the <code>Count</code> array is the number of times the corresponding value in the <code>Values</code> array occurred during the period. </p> <p>If you omit the <code>Counts</code> array, the default of 1 is used as the value for each count. If you include a <code>Counts</code> array, it must include the same amount of values as the <code>Values</code> array.</p>
    pub counts: Option<Vec<f64>>,
    /// <p>The dimensions associated with the metric.</p>
    pub dimensions: Option<Vec<Dimension>>,
    /// <p>The name of the metric.</p>
    pub metric_name: String,
    /// <p>The statistical values for the metric.</p>
    pub statistic_values: Option<StatisticSet>,
    /// <p>Valid values are 1 and 60. Setting this to 1 specifies this metric as a high-resolution metric, so that CloudWatch stores the metric with sub-minute resolution down to one second. Setting this to 60 specifies this metric as a regular-resolution metric, which CloudWatch stores at 1-minute resolution. Currently, high resolution is available only for custom metrics. For more information about high-resolution metrics, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/publishingMetrics.html#high-resolution-metrics">High-Resolution Metrics</a> in the <i>Amazon CloudWatch User Guide</i>. </p> <p>This field is optional, if you do not specify it the default of 60 is used.</p>
    pub storage_resolution: Option<i64>,
    /// <p>The time the metric data was received, expressed as the number of milliseconds since Jan 1, 1970 00:00:00 UTC.</p>
    pub timestamp: Option<String>,
    /// <p>The unit of the metric.</p>
    pub unit: Option<String>,
    /// <p>The value for the metric.</p> <p>Although the parameter accepts numbers of type Double, CloudWatch rejects values that are either too small or too large. Values must be in the range of 8.515920e-109 to 1.174271e+108 (Base 10) or 2e-360 to 2e360 (Base 2). In addition, special values (for example, NaN, +Infinity, -Infinity) are not supported.</p>
    pub value: Option<f64>,
    /// <p>Array of numbers representing the values for the metric during the period. Each unique value is listed just once in this array, and the corresponding number in the <code>Counts</code> array specifies the number of times that value occurred during the period. You can include up to 150 unique values in each <code>PutMetricData</code> action that specifies a <code>Values</code> array.</p> <p>Although the <code>Values</code> array accepts numbers of type <code>Double</code>, CloudWatch rejects values that are either too small or too large. Values must be in the range of 8.515920e-109 to 1.174271e+108 (Base 10) or 2e-360 to 2e360 (Base 2). In addition, special values (for example, NaN, +Infinity, -Infinity) are not supported.</p>
    pub values: Option<Vec<f64>>,
}

/// Serialize `MetricDatum` contents to a `SignedRequest`.
struct MetricDatumSerializer;
impl MetricDatumSerializer {
    fn serialize(params: &mut impl ServiceParams, name: &str, obj: &MetricDatum) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.counts {
            CountsSerializer::serialize(params, &format!("{}{}", prefix, "Counts"), field_value);
        }
        if let Some(ref field_value) = obj.dimensions {
            DimensionsSerializer::serialize(
                params,
                &format!("{}{}", prefix, "Dimensions"),
                field_value,
            );
        }
        params.put(&format!("{}{}", prefix, "MetricName"), &obj.metric_name);
        if let Some(ref field_value) = obj.statistic_values {
            StatisticSetSerializer::serialize(
                params,
                &format!("{}{}", prefix, "StatisticValues"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.storage_resolution {
            params.put(&format!("{}{}", prefix, "StorageResolution"), &field_value);
        }
        if let Some(ref field_value) = obj.timestamp {
            params.put(&format!("{}{}", prefix, "Timestamp"), &field_value);
        }
        if let Some(ref field_value) = obj.unit {
            params.put(&format!("{}{}", prefix, "Unit"), &field_value);
        }
        if let Some(ref field_value) = obj.value {
            params.put(&format!("{}{}", prefix, "Value"), &field_value);
        }
        if let Some(ref field_value) = obj.values {
            ValuesSerializer::serialize(params, &format!("{}{}", prefix, "Values"), field_value);
        }
    }
}

struct MetricExpressionDeserializer;
impl MetricExpressionDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct MetricIdDeserializer;
impl MetricIdDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct MetricLabelDeserializer;
impl MetricLabelDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct MetricNameDeserializer;
impl MetricNameDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>This structure defines the metric to be returned, along with the statistics, period, and units.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct MetricStat {
    /// <p>The metric to return, including the metric name, namespace, and dimensions.</p>
    pub metric: Metric,
    /// <p>The period, in seconds, to use when retrieving the metric.</p>
    pub period: i64,
    /// <p>The statistic to return. It can include any CloudWatch statistic or extended statistic.</p>
    pub stat: String,
    /// <p>The unit to use for the returned data points.</p>
    pub unit: Option<String>,
}

struct MetricStatDeserializer;
impl MetricStatDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<MetricStat, XmlParseError> {
        deserialize_elements::<_, MetricStat, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Metric" => {
                    obj.metric = MetricDeserializer::deserialize("Metric", stack)?;
                }
                "Period" => {
                    obj.period = PeriodDeserializer::deserialize("Period", stack)?;
                }
                "Stat" => {
                    obj.stat = StatDeserializer::deserialize("Stat", stack)?;
                }
                "Unit" => {
                    obj.unit = Some(StandardUnitDeserializer::deserialize("Unit", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

/// Serialize `MetricStat` contents to a `SignedRequest`.
struct MetricStatSerializer;
impl MetricStatSerializer {
    fn serialize(params: &mut impl ServiceParams, name: &str, obj: &MetricStat) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        MetricSerializer::serialize(params, &format!("{}{}", prefix, "Metric"), &obj.metric);
        params.put(&format!("{}{}", prefix, "Period"), &obj.period);
        params.put(&format!("{}{}", prefix, "Stat"), &obj.stat);
        if let Some(ref field_value) = obj.unit {
            params.put(&format!("{}{}", prefix, "Unit"), &field_value);
        }
    }
}

struct MetricWidgetImageDeserializer;
impl MetricWidgetImageDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<bytes::Bytes, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?.into();
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct MetricsDeserializer;
impl MetricsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<Metric>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(MetricDeserializer::deserialize("member", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
struct NamespaceDeserializer;
impl NamespaceDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct NextTokenDeserializer;
impl NextTokenDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct PeriodDeserializer;
impl PeriodDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<i64, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = i64::from_str(characters(stack)?.as_ref()).unwrap();
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct PutDashboardRequest {
    /// <p>The detailed information about the dashboard in JSON format, including the widgets to include and their location on the dashboard. This parameter is required.</p> <p>For more information about the syntax, see <a>CloudWatch-Dashboard-Body-Structure</a>.</p>
    pub dashboard_body: String,
    /// <p>The name of the dashboard. If a dashboard with this name already exists, this call modifies that dashboard, replacing its current contents. Otherwise, a new dashboard is created. The maximum length is 255, and valid characters are A-Z, a-z, 0-9, "-", and "_". This parameter is required.</p>
    pub dashboard_name: String,
}

/// Serialize `PutDashboardRequest` contents to a `SignedRequest`.
struct PutDashboardRequestSerializer;
impl PutDashboardRequestSerializer {
    fn serialize(params: &mut impl ServiceParams, name: &str, obj: &PutDashboardRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "DashboardBody"),
            &obj.dashboard_body,
        );
        params.put(
            &format!("{}{}", prefix, "DashboardName"),
            &obj.dashboard_name,
        );
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct PutDashboardResponse {
    /// <p>If the input for <code>PutDashboard</code> was correct and the dashboard was successfully created or modified, this result is empty.</p> <p>If this result includes only warning messages, then the input was valid enough for the dashboard to be created or modified, but some elements of the dashboard may not render.</p> <p>If this result includes error messages, the input was not valid and the operation failed.</p>
    pub dashboard_validation_messages: Option<Vec<DashboardValidationMessage>>,
}

struct PutDashboardResponseDeserializer;
impl PutDashboardResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<PutDashboardResponse, XmlParseError> {
        deserialize_elements::<_, PutDashboardResponse, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "DashboardValidationMessages" => {
                    obj.dashboard_validation_messages
                        .get_or_insert(vec![])
                        .extend(DashboardValidationMessagesDeserializer::deserialize(
                            "DashboardValidationMessages",
                            stack,
                        )?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct PutMetricAlarmRequest {
    /// <p>Indicates whether actions should be executed during any changes to the alarm state. The default is TRUE.</p>
    pub actions_enabled: Option<bool>,
    /// <p>The actions to execute when this alarm transitions to the <code>ALARM</code> state from any other state. Each action is specified as an Amazon Resource Name (ARN).</p> <p>Valid Values: <code>arn:aws:automate:<i>region</i>:ec2:stop</code> | <code>arn:aws:automate:<i>region</i>:ec2:terminate</code> | <code>arn:aws:automate:<i>region</i>:ec2:recover</code> | <code>arn:aws:automate:<i>region</i>:ec2:reboot</code> | <code>arn:aws:sns:<i>region</i>:<i>account-id</i>:<i>sns-topic-name</i> </code> | <code>arn:aws:autoscaling:<i>region</i>:<i>account-id</i>:scalingPolicy:<i>policy-id</i>autoScalingGroupName/<i>group-friendly-name</i>:policyName/<i>policy-friendly-name</i> </code> </p> <p>Valid Values (for use with IAM roles): <code>arn:aws:swf:<i>region</i>:<i>account-id</i>:action/actions/AWS_EC2.InstanceId.Stop/1.0</code> | <code>arn:aws:swf:<i>region</i>:<i>account-id</i>:action/actions/AWS_EC2.InstanceId.Terminate/1.0</code> | <code>arn:aws:swf:<i>region</i>:<i>account-id</i>:action/actions/AWS_EC2.InstanceId.Reboot/1.0</code> </p>
    pub alarm_actions: Option<Vec<String>>,
    /// <p>The description for the alarm.</p>
    pub alarm_description: Option<String>,
    /// <p>The name for the alarm. This name must be unique within your AWS account.</p>
    pub alarm_name: String,
    /// <p> The arithmetic operation to use when comparing the specified statistic and threshold. The specified statistic value is used as the first operand.</p>
    pub comparison_operator: String,
    /// <p>The number of datapoints that must be breaching to trigger the alarm. This is used only if you are setting an "M out of N" alarm. In that case, this value is the M. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/AlarmThatSendsEmail.html#alarm-evaluation">Evaluating an Alarm</a> in the <i>Amazon CloudWatch User Guide</i>.</p>
    pub datapoints_to_alarm: Option<i64>,
    /// <p>The dimensions for the metric specified in <code>MetricName</code>.</p>
    pub dimensions: Option<Vec<Dimension>>,
    /// <p> Used only for alarms based on percentiles. If you specify <code>ignore</code>, the alarm state does not change during periods with too few data points to be statistically significant. If you specify <code>evaluate</code> or omit this parameter, the alarm is always evaluated and possibly changes state no matter how many data points are available. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/AlarmThatSendsEmail.html#percentiles-with-low-samples">Percentile-Based CloudWatch Alarms and Low Data Samples</a>.</p> <p>Valid Values: <code>evaluate | ignore</code> </p>
    pub evaluate_low_sample_count_percentile: Option<String>,
    /// <p>The number of periods over which data is compared to the specified threshold. If you are setting an alarm that requires that a number of consecutive data points be breaching to trigger the alarm, this value specifies that number. If you are setting an "M out of N" alarm, this value is the N.</p> <p>An alarm's total current evaluation period can be no longer than one day, so this number multiplied by <code>Period</code> cannot be more than 86,400 seconds.</p>
    pub evaluation_periods: i64,
    /// <p>The percentile statistic for the metric specified in <code>MetricName</code>. Specify a value between p0.0 and p100. When you call <code>PutMetricAlarm</code> and specify a <code>MetricName</code>, you must specify either <code>Statistic</code> or <code>ExtendedStatistic,</code> but not both.</p>
    pub extended_statistic: Option<String>,
    /// <p>The actions to execute when this alarm transitions to the <code>INSUFFICIENT_DATA</code> state from any other state. Each action is specified as an Amazon Resource Name (ARN).</p> <p>Valid Values: <code>arn:aws:automate:<i>region</i>:ec2:stop</code> | <code>arn:aws:automate:<i>region</i>:ec2:terminate</code> | <code>arn:aws:automate:<i>region</i>:ec2:recover</code> | <code>arn:aws:automate:<i>region</i>:ec2:reboot</code> | <code>arn:aws:sns:<i>region</i>:<i>account-id</i>:<i>sns-topic-name</i> </code> | <code>arn:aws:autoscaling:<i>region</i>:<i>account-id</i>:scalingPolicy:<i>policy-id</i>autoScalingGroupName/<i>group-friendly-name</i>:policyName/<i>policy-friendly-name</i> </code> </p> <p>Valid Values (for use with IAM roles): <code>&gt;arn:aws:swf:<i>region</i>:<i>account-id</i>:action/actions/AWS_EC2.InstanceId.Stop/1.0</code> | <code>arn:aws:swf:<i>region</i>:<i>account-id</i>:action/actions/AWS_EC2.InstanceId.Terminate/1.0</code> | <code>arn:aws:swf:<i>region</i>:<i>account-id</i>:action/actions/AWS_EC2.InstanceId.Reboot/1.0</code> </p>
    pub insufficient_data_actions: Option<Vec<String>>,
    /// <p>The name for the metric associated with the alarm.</p> <p>If you are creating an alarm based on a math expression, you cannot specify this parameter, or any of the <code>Dimensions</code>, <code>Period</code>, <code>Namespace</code>, <code>Statistic</code>, or <code>ExtendedStatistic</code> parameters. Instead, you specify all this information in the <code>Metrics</code> array.</p>
    pub metric_name: Option<String>,
    /// <p>An array of <code>MetricDataQuery</code> structures that enable you to create an alarm based on the result of a metric math expression. Each item in the <code>Metrics</code> array either retrieves a metric or performs a math expression.</p> <p>One item in the <code>Metrics</code> array is the expression that the alarm watches. You designate this expression by setting <code>ReturnValue</code> to true for this object in the array. For more information, see <a>MetricDataQuery</a>.</p> <p>If you use the <code>Metrics</code> parameter, you cannot include the <code>MetricName</code>, <code>Dimensions</code>, <code>Period</code>, <code>Namespace</code>, <code>Statistic</code>, or <code>ExtendedStatistic</code> parameters of <code>PutMetricAlarm</code> in the same operation. Instead, you retrieve the metrics you are using in your math expression as part of the <code>Metrics</code> array.</p>
    pub metrics: Option<Vec<MetricDataQuery>>,
    /// <p>The namespace for the metric associated specified in <code>MetricName</code>.</p>
    pub namespace: Option<String>,
    /// <p>The actions to execute when this alarm transitions to an <code>OK</code> state from any other state. Each action is specified as an Amazon Resource Name (ARN).</p> <p>Valid Values: <code>arn:aws:automate:<i>region</i>:ec2:stop</code> | <code>arn:aws:automate:<i>region</i>:ec2:terminate</code> | <code>arn:aws:automate:<i>region</i>:ec2:recover</code> | <code>arn:aws:automate:<i>region</i>:ec2:reboot</code> | <code>arn:aws:sns:<i>region</i>:<i>account-id</i>:<i>sns-topic-name</i> </code> | <code>arn:aws:autoscaling:<i>region</i>:<i>account-id</i>:scalingPolicy:<i>policy-id</i>autoScalingGroupName/<i>group-friendly-name</i>:policyName/<i>policy-friendly-name</i> </code> </p> <p>Valid Values (for use with IAM roles): <code>arn:aws:swf:<i>region</i>:<i>account-id</i>:action/actions/AWS_EC2.InstanceId.Stop/1.0</code> | <code>arn:aws:swf:<i>region</i>:<i>account-id</i>:action/actions/AWS_EC2.InstanceId.Terminate/1.0</code> | <code>arn:aws:swf:<i>region</i>:<i>account-id</i>:action/actions/AWS_EC2.InstanceId.Reboot/1.0</code> </p>
    pub ok_actions: Option<Vec<String>>,
    /// <p>The length, in seconds, used each time the metric specified in <code>MetricName</code> is evaluated. Valid values are 10, 30, and any multiple of 60.</p> <p>Be sure to specify 10 or 30 only for metrics that are stored by a <code>PutMetricData</code> call with a <code>StorageResolution</code> of 1. If you specify a period of 10 or 30 for a metric that does not have sub-minute resolution, the alarm still attempts to gather data at the period rate that you specify. In this case, it does not receive data for the attempts that do not correspond to a one-minute data resolution, and the alarm may often lapse into INSUFFICENT_DATA status. Specifying 10 or 30 also sets this alarm as a high-resolution alarm, which has a higher charge than other alarms. For more information about pricing, see <a href="https://aws.amazon.com/cloudwatch/pricing/">Amazon CloudWatch Pricing</a>.</p> <p>An alarm's total current evaluation period can be no longer than one day, so <code>Period</code> multiplied by <code>EvaluationPeriods</code> cannot be more than 86,400 seconds.</p>
    pub period: Option<i64>,
    /// <p>The statistic for the metric specified in <code>MetricName</code>, other than percentile. For percentile statistics, use <code>ExtendedStatistic</code>. When you call <code>PutMetricAlarm</code> and specify a <code>MetricName</code>, you must specify either <code>Statistic</code> or <code>ExtendedStatistic,</code> but not both.</p>
    pub statistic: Option<String>,
    /// <p>A list of key-value pairs to associate with the alarm. You can associate as many as 50 tags with an alarm.</p> <p>Tags can help you organize and categorize your resources. You can also use them to scope user permissions, by granting a user permission to access or change only resources with certain tag values.</p>
    pub tags: Option<Vec<Tag>>,
    /// <p>The value against which the specified statistic is compared.</p>
    pub threshold: f64,
    /// <p> Sets how this alarm is to handle missing data points. If <code>TreatMissingData</code> is omitted, the default behavior of <code>missing</code> is used. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/AlarmThatSendsEmail.html#alarms-and-missing-data">Configuring How CloudWatch Alarms Treats Missing Data</a>.</p> <p>Valid Values: <code>breaching | notBreaching | ignore | missing</code> </p>
    pub treat_missing_data: Option<String>,
    /// <p>The unit of measure for the statistic. For example, the units for the Amazon EC2 NetworkIn metric are Bytes because NetworkIn tracks the number of bytes that an instance receives on all network interfaces. You can also specify a unit when you create a custom metric. Units help provide conceptual meaning to your data. Metric data points that specify a unit of measure, such as Percent, are aggregated separately.</p> <p>If you specify a unit, you must use a unit that is appropriate for the metric. Otherwise, the CloudWatch alarm can get stuck in the <code>INSUFFICIENT DATA</code> state. </p>
    pub unit: Option<String>,
}

/// Serialize `PutMetricAlarmRequest` contents to a `SignedRequest`.
struct PutMetricAlarmRequestSerializer;
impl PutMetricAlarmRequestSerializer {
    fn serialize(params: &mut impl ServiceParams, name: &str, obj: &PutMetricAlarmRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.actions_enabled {
            params.put(&format!("{}{}", prefix, "ActionsEnabled"), &field_value);
        }
        if let Some(ref field_value) = obj.alarm_actions {
            ResourceListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "AlarmActions"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.alarm_description {
            params.put(&format!("{}{}", prefix, "AlarmDescription"), &field_value);
        }
        params.put(&format!("{}{}", prefix, "AlarmName"), &obj.alarm_name);
        params.put(
            &format!("{}{}", prefix, "ComparisonOperator"),
            &obj.comparison_operator,
        );
        if let Some(ref field_value) = obj.datapoints_to_alarm {
            params.put(&format!("{}{}", prefix, "DatapointsToAlarm"), &field_value);
        }
        if let Some(ref field_value) = obj.dimensions {
            DimensionsSerializer::serialize(
                params,
                &format!("{}{}", prefix, "Dimensions"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.evaluate_low_sample_count_percentile {
            params.put(
                &format!("{}{}", prefix, "EvaluateLowSampleCountPercentile"),
                &field_value,
            );
        }
        params.put(
            &format!("{}{}", prefix, "EvaluationPeriods"),
            &obj.evaluation_periods,
        );
        if let Some(ref field_value) = obj.extended_statistic {
            params.put(&format!("{}{}", prefix, "ExtendedStatistic"), &field_value);
        }
        if let Some(ref field_value) = obj.insufficient_data_actions {
            ResourceListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "InsufficientDataActions"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.metric_name {
            params.put(&format!("{}{}", prefix, "MetricName"), &field_value);
        }
        if let Some(ref field_value) = obj.metrics {
            MetricDataQueriesSerializer::serialize(
                params,
                &format!("{}{}", prefix, "Metrics"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.namespace {
            params.put(&format!("{}{}", prefix, "Namespace"), &field_value);
        }
        if let Some(ref field_value) = obj.ok_actions {
            ResourceListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "OKActions"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.period {
            params.put(&format!("{}{}", prefix, "Period"), &field_value);
        }
        if let Some(ref field_value) = obj.statistic {
            params.put(&format!("{}{}", prefix, "Statistic"), &field_value);
        }
        if let Some(ref field_value) = obj.tags {
            TagListSerializer::serialize(params, &format!("{}{}", prefix, "Tags"), field_value);
        }
        params.put(&format!("{}{}", prefix, "Threshold"), &obj.threshold);
        if let Some(ref field_value) = obj.treat_missing_data {
            params.put(&format!("{}{}", prefix, "TreatMissingData"), &field_value);
        }
        if let Some(ref field_value) = obj.unit {
            params.put(&format!("{}{}", prefix, "Unit"), &field_value);
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct PutMetricAlarmResponse {}

struct PutMetricAlarmResponseDeserializer;
impl PutMetricAlarmResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<PutMetricAlarmResponse, XmlParseError> {
        Ok(PutMetricAlarmResponse::default())
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct PutMetricDataRequest {
    /// <p>The data for the metric. The array can include no more than 20 metrics per call.</p>
    pub metric_data: Vec<MetricDatum>,
    /// <p>The namespace for the metric data.</p> <p>You cannot specify a namespace that begins with "AWS/". Namespaces that begin with "AWS/" are reserved for use by Amazon Web Services products.</p>
    pub namespace: String,
}

/// Serialize `PutMetricDataRequest` contents to a `SignedRequest`.
struct PutMetricDataRequestSerializer;
impl PutMetricDataRequestSerializer {
    fn serialize(params: &mut impl ServiceParams, name: &str, obj: &PutMetricDataRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        MetricDataSerializer::serialize(
            params,
            &format!("{}{}", prefix, "MetricData"),
            &obj.metric_data,
        );
        params.put(&format!("{}{}", prefix, "Namespace"), &obj.namespace);
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct PutMetricDataResponse {}

struct PutMetricDataResponseDeserializer;
impl PutMetricDataResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<PutMetricDataResponse, XmlParseError> {
        Ok(PutMetricDataResponse::default())
    }
}
struct ResourceListDeserializer;
impl ResourceListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<String>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(ResourceNameDeserializer::deserialize("member", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}

/// Serialize `ResourceList` contents to a `SignedRequest`.
struct ResourceListSerializer;
impl ResourceListSerializer {
    fn serialize(params: &mut impl ServiceParams, name: &str, obj: &Vec<String>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

struct ResourceNameDeserializer;
impl ResourceNameDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct ReturnDataDeserializer;
impl ReturnDataDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<bool, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = bool::from_str(characters(stack)?.as_ref()).unwrap();
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SetAlarmStateRequest {
    /// <p>The name for the alarm. This name must be unique within the AWS account. The maximum length is 255 characters.</p>
    pub alarm_name: String,
    /// <p>The reason that this alarm is set to this specific state, in text format.</p>
    pub state_reason: String,
    /// <p>The reason that this alarm is set to this specific state, in JSON format.</p>
    pub state_reason_data: Option<String>,
    /// <p>The value of the state.</p>
    pub state_value: String,
}

/// Serialize `SetAlarmStateRequest` contents to a `SignedRequest`.
struct SetAlarmStateRequestSerializer;
impl SetAlarmStateRequestSerializer {
    fn serialize(params: &mut impl ServiceParams, name: &str, obj: &SetAlarmStateRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(&format!("{}{}", prefix, "AlarmName"), &obj.alarm_name);
        params.put(&format!("{}{}", prefix, "StateReason"), &obj.state_reason);
        if let Some(ref field_value) = obj.state_reason_data {
            params.put(&format!("{}{}", prefix, "StateReasonData"), &field_value);
        }
        params.put(&format!("{}{}", prefix, "StateValue"), &obj.state_value);
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct SetAlarmStateResponse {}

struct SetAlarmStateResponseDeserializer;
impl SetAlarmStateResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<SetAlarmStateResponse, XmlParseError> {
        Ok(SetAlarmStateResponse::default())
    }
}
struct SizeDeserializer;
impl SizeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<i64, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = i64::from_str(characters(stack)?.as_ref()).unwrap();
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct StandardUnitDeserializer;
impl StandardUnitDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct StatDeserializer;
impl StatDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct StateReasonDeserializer;
impl StateReasonDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct StateReasonDataDeserializer;
impl StateReasonDataDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct StateValueDeserializer;
impl StateValueDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct StatisticDeserializer;
impl StatisticDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>Represents a set of statistics that describes a specific metric. </p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct StatisticSet {
    /// <p>The maximum value of the sample set.</p>
    pub maximum: f64,
    /// <p>The minimum value of the sample set.</p>
    pub minimum: f64,
    /// <p>The number of samples used for the statistic set.</p>
    pub sample_count: f64,
    /// <p>The sum of values for the sample set.</p>
    pub sum: f64,
}

/// Serialize `StatisticSet` contents to a `SignedRequest`.
struct StatisticSetSerializer;
impl StatisticSetSerializer {
    fn serialize(params: &mut impl ServiceParams, name: &str, obj: &StatisticSet) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(&format!("{}{}", prefix, "Maximum"), &obj.maximum);
        params.put(&format!("{}{}", prefix, "Minimum"), &obj.minimum);
        params.put(&format!("{}{}", prefix, "SampleCount"), &obj.sample_count);
        params.put(&format!("{}{}", prefix, "Sum"), &obj.sum);
    }
}

/// Serialize `Statistics` contents to a `SignedRequest`.
struct StatisticsSerializer;
impl StatisticsSerializer {
    fn serialize(params: &mut impl ServiceParams, name: &str, obj: &Vec<String>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

struct StatusCodeDeserializer;
impl StatusCodeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
/// <p>A key-value pair associated with a CloudWatch resource.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Tag {
    /// <p>A string that you can use to assign a value. The combination of tag keys and values can help you organize and categorize your resources.</p>
    pub key: String,
    /// <p>The value for the specified tag key.</p>
    pub value: String,
}

struct TagDeserializer;
impl TagDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Tag, XmlParseError> {
        deserialize_elements::<_, Tag, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Key" => {
                    obj.key = TagKeyDeserializer::deserialize("Key", stack)?;
                }
                "Value" => {
                    obj.value = TagValueDeserializer::deserialize("Value", stack)?;
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

/// Serialize `Tag` contents to a `SignedRequest`.
struct TagSerializer;
impl TagSerializer {
    fn serialize(params: &mut impl ServiceParams, name: &str, obj: &Tag) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(&format!("{}{}", prefix, "Key"), &obj.key);
        params.put(&format!("{}{}", prefix, "Value"), &obj.value);
    }
}

struct TagKeyDeserializer;
impl TagKeyDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}

/// Serialize `TagKeyList` contents to a `SignedRequest`.
struct TagKeyListSerializer;
impl TagKeyListSerializer {
    fn serialize(params: &mut impl ServiceParams, name: &str, obj: &Vec<String>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

struct TagListDeserializer;
impl TagListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<Tag>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(TagDeserializer::deserialize("member", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}

/// Serialize `TagList` contents to a `SignedRequest`.
struct TagListSerializer;
impl TagListSerializer {
    fn serialize(params: &mut impl ServiceParams, name: &str, obj: &Vec<Tag>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            TagSerializer::serialize(params, &key, obj);
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct TagResourceRequest {
    /// <p>The ARN of the CloudWatch resource that you're adding tags to. For more information on ARN format, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html#arn-syntax-cloudwatch">Example ARNs</a> in the <i>Amazon Web Services General Reference</i>.</p>
    pub resource_arn: String,
    /// <p>The list of key-value pairs to associate with the resource.</p>
    pub tags: Vec<Tag>,
}

/// Serialize `TagResourceRequest` contents to a `SignedRequest`.
struct TagResourceRequestSerializer;
impl TagResourceRequestSerializer {
    fn serialize(params: &mut impl ServiceParams, name: &str, obj: &TagResourceRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(&format!("{}{}", prefix, "ResourceARN"), &obj.resource_arn);
        TagListSerializer::serialize(params, &format!("{}{}", prefix, "Tags"), &obj.tags);
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct TagResourceResponse {}

struct TagResourceResponseDeserializer;
impl TagResourceResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<TagResourceResponse, XmlParseError> {
        start_element(tag_name, stack)?;

        let obj = TagResourceResponse::default();

        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct TagValueDeserializer;
impl TagValueDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct ThresholdDeserializer;
impl ThresholdDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<f64, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = f64::from_str(characters(stack)?.as_ref()).unwrap();
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct TimestampDeserializer;
impl TimestampDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
struct TimestampsDeserializer;
impl TimestampsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<String>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(TimestampDeserializer::deserialize("member", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
struct TreatMissingDataDeserializer;
impl TreatMissingDataDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        start_element(tag_name, stack)?;
        let obj = characters(stack)?;
        end_element(tag_name, stack)?;

        Ok(obj)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct UntagResourceRequest {
    /// <p>The ARN of the CloudWatch resource that you're removing tags from. For more information on ARN format, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html#arn-syntax-cloudwatch">Example ARNs</a> in the <i>Amazon Web Services General Reference</i>.</p>
    pub resource_arn: String,
    /// <p>The list of tag keys to remove from the resource.</p>
    pub tag_keys: Vec<String>,
}

/// Serialize `UntagResourceRequest` contents to a `SignedRequest`.
struct UntagResourceRequestSerializer;
impl UntagResourceRequestSerializer {
    fn serialize(params: &mut impl ServiceParams, name: &str, obj: &UntagResourceRequest) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(&format!("{}{}", prefix, "ResourceARN"), &obj.resource_arn);
        TagKeyListSerializer::serialize(params, &format!("{}{}", prefix, "TagKeys"), &obj.tag_keys);
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct UntagResourceResponse {}

struct UntagResourceResponseDeserializer;
impl UntagResourceResponseDeserializer {
    #[allow(unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<UntagResourceResponse, XmlParseError> {
        start_element(tag_name, stack)?;

        let obj = UntagResourceResponse::default();

        end_element(tag_name, stack)?;

        Ok(obj)
    }
}

/// Serialize `Values` contents to a `SignedRequest`.
struct ValuesSerializer;
impl ValuesSerializer {
    fn serialize(params: &mut impl ServiceParams, name: &str, obj: &Vec<f64>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

/// Errors returned by DeleteAlarms
#[derive(Debug, PartialEq)]
pub enum DeleteAlarmsError {
    /// <p>The named resource does not exist.</p>
    ResourceNotFound(String),
}

impl DeleteAlarmsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteAlarmsError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "ResourceNotFound" => {
                        return RusotoError::Service(DeleteAlarmsError::ResourceNotFound(
                            parsed_error.message,
                        ))
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DeleteAlarmsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteAlarmsError {
    fn description(&self) -> &str {
        match *self {
            DeleteAlarmsError::ResourceNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteDashboards
#[derive(Debug, PartialEq)]
pub enum DeleteDashboardsError {
    /// <p>The specified dashboard does not exist.</p>
    DashboardNotFoundError(String),
    /// <p>Request processing has failed due to some unknown error, exception, or failure.</p>
    InternalServiceFault(String),
    /// <p>The value of an input parameter is bad or out-of-range.</p>
    InvalidParameterValue(String),
}

impl DeleteDashboardsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteDashboardsError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "ResourceNotFound" => {
                        return RusotoError::Service(DeleteDashboardsError::DashboardNotFoundError(
                            parsed_error.message,
                        ))
                    }
                    "InternalServiceError" => {
                        return RusotoError::Service(DeleteDashboardsError::InternalServiceFault(
                            parsed_error.message,
                        ))
                    }
                    "InvalidParameterValue" => {
                        return RusotoError::Service(DeleteDashboardsError::InvalidParameterValue(
                            parsed_error.message,
                        ))
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DeleteDashboardsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteDashboardsError {
    fn description(&self) -> &str {
        match *self {
            DeleteDashboardsError::DashboardNotFoundError(ref cause) => cause,
            DeleteDashboardsError::InternalServiceFault(ref cause) => cause,
            DeleteDashboardsError::InvalidParameterValue(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeAlarmHistory
#[derive(Debug, PartialEq)]
pub enum DescribeAlarmHistoryError {
    /// <p>The next token specified is invalid.</p>
    InvalidNextToken(String),
}

impl DescribeAlarmHistoryError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeAlarmHistoryError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "InvalidNextToken" => {
                        return RusotoError::Service(DescribeAlarmHistoryError::InvalidNextToken(
                            parsed_error.message,
                        ))
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DescribeAlarmHistoryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeAlarmHistoryError {
    fn description(&self) -> &str {
        match *self {
            DescribeAlarmHistoryError::InvalidNextToken(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeAlarms
#[derive(Debug, PartialEq)]
pub enum DescribeAlarmsError {
    /// <p>The next token specified is invalid.</p>
    InvalidNextToken(String),
}

impl DescribeAlarmsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeAlarmsError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "InvalidNextToken" => {
                        return RusotoError::Service(DescribeAlarmsError::InvalidNextToken(
                            parsed_error.message,
                        ))
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DescribeAlarmsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeAlarmsError {
    fn description(&self) -> &str {
        match *self {
            DescribeAlarmsError::InvalidNextToken(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeAlarmsForMetric
#[derive(Debug, PartialEq)]
pub enum DescribeAlarmsForMetricError {}

impl DescribeAlarmsForMetricError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeAlarmsForMetricError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DescribeAlarmsForMetricError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeAlarmsForMetricError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by DisableAlarmActions
#[derive(Debug, PartialEq)]
pub enum DisableAlarmActionsError {}

impl DisableAlarmActionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DisableAlarmActionsError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DisableAlarmActionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DisableAlarmActionsError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by EnableAlarmActions
#[derive(Debug, PartialEq)]
pub enum EnableAlarmActionsError {}

impl EnableAlarmActionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<EnableAlarmActionsError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for EnableAlarmActionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for EnableAlarmActionsError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by GetDashboard
#[derive(Debug, PartialEq)]
pub enum GetDashboardError {
    /// <p>The specified dashboard does not exist.</p>
    DashboardNotFoundError(String),
    /// <p>Request processing has failed due to some unknown error, exception, or failure.</p>
    InternalServiceFault(String),
    /// <p>The value of an input parameter is bad or out-of-range.</p>
    InvalidParameterValue(String),
}

impl GetDashboardError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetDashboardError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "ResourceNotFound" => {
                        return RusotoError::Service(GetDashboardError::DashboardNotFoundError(
                            parsed_error.message,
                        ))
                    }
                    "InternalServiceError" => {
                        return RusotoError::Service(GetDashboardError::InternalServiceFault(
                            parsed_error.message,
                        ))
                    }
                    "InvalidParameterValue" => {
                        return RusotoError::Service(GetDashboardError::InvalidParameterValue(
                            parsed_error.message,
                        ))
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for GetDashboardError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetDashboardError {
    fn description(&self) -> &str {
        match *self {
            GetDashboardError::DashboardNotFoundError(ref cause) => cause,
            GetDashboardError::InternalServiceFault(ref cause) => cause,
            GetDashboardError::InvalidParameterValue(ref cause) => cause,
        }
    }
}
/// Errors returned by GetMetricData
#[derive(Debug, PartialEq)]
pub enum GetMetricDataError {
    /// <p>The next token specified is invalid.</p>
    InvalidNextToken(String),
}

impl GetMetricDataError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetMetricDataError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "InvalidNextToken" => {
                        return RusotoError::Service(GetMetricDataError::InvalidNextToken(
                            parsed_error.message,
                        ))
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for GetMetricDataError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetMetricDataError {
    fn description(&self) -> &str {
        match *self {
            GetMetricDataError::InvalidNextToken(ref cause) => cause,
        }
    }
}
/// Errors returned by GetMetricStatistics
#[derive(Debug, PartialEq)]
pub enum GetMetricStatisticsError {
    /// <p>Request processing has failed due to some unknown error, exception, or failure.</p>
    InternalServiceFault(String),
    /// <p>Parameters were used together that cannot be used together.</p>
    InvalidParameterCombination(String),
    /// <p>The value of an input parameter is bad or out-of-range.</p>
    InvalidParameterValue(String),
    /// <p>An input parameter that is required is missing.</p>
    MissingRequiredParameter(String),
}

impl GetMetricStatisticsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetMetricStatisticsError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "InternalServiceError" => {
                        return RusotoError::Service(
                            GetMetricStatisticsError::InternalServiceFault(parsed_error.message),
                        )
                    }
                    "InvalidParameterCombination" => {
                        return RusotoError::Service(
                            GetMetricStatisticsError::InvalidParameterCombination(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidParameterValue" => {
                        return RusotoError::Service(
                            GetMetricStatisticsError::InvalidParameterValue(parsed_error.message),
                        )
                    }
                    "MissingParameter" => {
                        return RusotoError::Service(
                            GetMetricStatisticsError::MissingRequiredParameter(
                                parsed_error.message,
                            ),
                        )
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for GetMetricStatisticsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetMetricStatisticsError {
    fn description(&self) -> &str {
        match *self {
            GetMetricStatisticsError::InternalServiceFault(ref cause) => cause,
            GetMetricStatisticsError::InvalidParameterCombination(ref cause) => cause,
            GetMetricStatisticsError::InvalidParameterValue(ref cause) => cause,
            GetMetricStatisticsError::MissingRequiredParameter(ref cause) => cause,
        }
    }
}
/// Errors returned by GetMetricWidgetImage
#[derive(Debug, PartialEq)]
pub enum GetMetricWidgetImageError {}

impl GetMetricWidgetImageError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetMetricWidgetImageError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for GetMetricWidgetImageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetMetricWidgetImageError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by ListDashboards
#[derive(Debug, PartialEq)]
pub enum ListDashboardsError {
    /// <p>Request processing has failed due to some unknown error, exception, or failure.</p>
    InternalServiceFault(String),
    /// <p>The value of an input parameter is bad or out-of-range.</p>
    InvalidParameterValue(String),
}

impl ListDashboardsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListDashboardsError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "InternalServiceError" => {
                        return RusotoError::Service(ListDashboardsError::InternalServiceFault(
                            parsed_error.message,
                        ))
                    }
                    "InvalidParameterValue" => {
                        return RusotoError::Service(ListDashboardsError::InvalidParameterValue(
                            parsed_error.message,
                        ))
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for ListDashboardsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListDashboardsError {
    fn description(&self) -> &str {
        match *self {
            ListDashboardsError::InternalServiceFault(ref cause) => cause,
            ListDashboardsError::InvalidParameterValue(ref cause) => cause,
        }
    }
}
/// Errors returned by ListMetrics
#[derive(Debug, PartialEq)]
pub enum ListMetricsError {
    /// <p>Request processing has failed due to some unknown error, exception, or failure.</p>
    InternalServiceFault(String),
    /// <p>The value of an input parameter is bad or out-of-range.</p>
    InvalidParameterValue(String),
}

impl ListMetricsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListMetricsError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "InternalServiceError" => {
                        return RusotoError::Service(ListMetricsError::InternalServiceFault(
                            parsed_error.message,
                        ))
                    }
                    "InvalidParameterValue" => {
                        return RusotoError::Service(ListMetricsError::InvalidParameterValue(
                            parsed_error.message,
                        ))
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for ListMetricsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListMetricsError {
    fn description(&self) -> &str {
        match *self {
            ListMetricsError::InternalServiceFault(ref cause) => cause,
            ListMetricsError::InvalidParameterValue(ref cause) => cause,
        }
    }
}
/// Errors returned by ListTagsForResource
#[derive(Debug, PartialEq)]
pub enum ListTagsForResourceError {
    /// <p>Request processing has failed due to some unknown error, exception, or failure.</p>
    InternalServiceFault(String),
    /// <p>The value of an input parameter is bad or out-of-range.</p>
    InvalidParameterValue(String),
    /// <p>The named resource does not exist.</p>
    ResourceNotFound(String),
}

impl ListTagsForResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTagsForResourceError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "InternalServiceError" => {
                        return RusotoError::Service(
                            ListTagsForResourceError::InternalServiceFault(parsed_error.message),
                        )
                    }
                    "InvalidParameterValue" => {
                        return RusotoError::Service(
                            ListTagsForResourceError::InvalidParameterValue(parsed_error.message),
                        )
                    }
                    "ResourceNotFoundException" => {
                        return RusotoError::Service(ListTagsForResourceError::ResourceNotFound(
                            parsed_error.message,
                        ))
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for ListTagsForResourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListTagsForResourceError {
    fn description(&self) -> &str {
        match *self {
            ListTagsForResourceError::InternalServiceFault(ref cause) => cause,
            ListTagsForResourceError::InvalidParameterValue(ref cause) => cause,
            ListTagsForResourceError::ResourceNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by PutDashboard
#[derive(Debug, PartialEq)]
pub enum PutDashboardError {
    /// <p>Some part of the dashboard data is invalid.</p>
    DashboardInvalidInputError(String),
    /// <p>Request processing has failed due to some unknown error, exception, or failure.</p>
    InternalServiceFault(String),
}

impl PutDashboardError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutDashboardError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "InvalidParameterInput" => {
                        return RusotoError::Service(PutDashboardError::DashboardInvalidInputError(
                            parsed_error.message,
                        ))
                    }
                    "InternalServiceError" => {
                        return RusotoError::Service(PutDashboardError::InternalServiceFault(
                            parsed_error.message,
                        ))
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for PutDashboardError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutDashboardError {
    fn description(&self) -> &str {
        match *self {
            PutDashboardError::DashboardInvalidInputError(ref cause) => cause,
            PutDashboardError::InternalServiceFault(ref cause) => cause,
        }
    }
}
/// Errors returned by PutMetricAlarm
#[derive(Debug, PartialEq)]
pub enum PutMetricAlarmError {
    /// <p>The quota for alarms for this customer has already been reached.</p>
    LimitExceededFault(String),
}

impl PutMetricAlarmError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutMetricAlarmError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "LimitExceeded" => {
                        return RusotoError::Service(PutMetricAlarmError::LimitExceededFault(
                            parsed_error.message,
                        ))
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for PutMetricAlarmError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutMetricAlarmError {
    fn description(&self) -> &str {
        match *self {
            PutMetricAlarmError::LimitExceededFault(ref cause) => cause,
        }
    }
}
/// Errors returned by PutMetricData
#[derive(Debug, PartialEq)]
pub enum PutMetricDataError {
    /// <p>Request processing has failed due to some unknown error, exception, or failure.</p>
    InternalServiceFault(String),
    /// <p>Parameters were used together that cannot be used together.</p>
    InvalidParameterCombination(String),
    /// <p>The value of an input parameter is bad or out-of-range.</p>
    InvalidParameterValue(String),
    /// <p>An input parameter that is required is missing.</p>
    MissingRequiredParameter(String),
}

impl PutMetricDataError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutMetricDataError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "InternalServiceError" => {
                        return RusotoError::Service(PutMetricDataError::InternalServiceFault(
                            parsed_error.message,
                        ))
                    }
                    "InvalidParameterCombination" => {
                        return RusotoError::Service(
                            PutMetricDataError::InvalidParameterCombination(parsed_error.message),
                        )
                    }
                    "InvalidParameterValue" => {
                        return RusotoError::Service(PutMetricDataError::InvalidParameterValue(
                            parsed_error.message,
                        ))
                    }
                    "MissingParameter" => {
                        return RusotoError::Service(PutMetricDataError::MissingRequiredParameter(
                            parsed_error.message,
                        ))
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for PutMetricDataError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutMetricDataError {
    fn description(&self) -> &str {
        match *self {
            PutMetricDataError::InternalServiceFault(ref cause) => cause,
            PutMetricDataError::InvalidParameterCombination(ref cause) => cause,
            PutMetricDataError::InvalidParameterValue(ref cause) => cause,
            PutMetricDataError::MissingRequiredParameter(ref cause) => cause,
        }
    }
}
/// Errors returned by SetAlarmState
#[derive(Debug, PartialEq)]
pub enum SetAlarmStateError {
    /// <p>Data was not syntactically valid JSON.</p>
    InvalidFormatFault(String),
    /// <p>The named resource does not exist.</p>
    ResourceNotFound(String),
}

impl SetAlarmStateError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<SetAlarmStateError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "InvalidFormat" => {
                        return RusotoError::Service(SetAlarmStateError::InvalidFormatFault(
                            parsed_error.message,
                        ))
                    }
                    "ResourceNotFound" => {
                        return RusotoError::Service(SetAlarmStateError::ResourceNotFound(
                            parsed_error.message,
                        ))
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for SetAlarmStateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for SetAlarmStateError {
    fn description(&self) -> &str {
        match *self {
            SetAlarmStateError::InvalidFormatFault(ref cause) => cause,
            SetAlarmStateError::ResourceNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by TagResource
#[derive(Debug, PartialEq)]
pub enum TagResourceError {
    /// <p>More than one process tried to modify a resource at the same time.</p>
    ConcurrentModification(String),
    /// <p>Request processing has failed due to some unknown error, exception, or failure.</p>
    InternalServiceFault(String),
    /// <p>The value of an input parameter is bad or out-of-range.</p>
    InvalidParameterValue(String),
    /// <p>The named resource does not exist.</p>
    ResourceNotFound(String),
}

impl TagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TagResourceError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "ConcurrentModificationException" => {
                        return RusotoError::Service(TagResourceError::ConcurrentModification(
                            parsed_error.message,
                        ))
                    }
                    "InternalServiceError" => {
                        return RusotoError::Service(TagResourceError::InternalServiceFault(
                            parsed_error.message,
                        ))
                    }
                    "InvalidParameterValue" => {
                        return RusotoError::Service(TagResourceError::InvalidParameterValue(
                            parsed_error.message,
                        ))
                    }
                    "ResourceNotFoundException" => {
                        return RusotoError::Service(TagResourceError::ResourceNotFound(
                            parsed_error.message,
                        ))
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for TagResourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for TagResourceError {
    fn description(&self) -> &str {
        match *self {
            TagResourceError::ConcurrentModification(ref cause) => cause,
            TagResourceError::InternalServiceFault(ref cause) => cause,
            TagResourceError::InvalidParameterValue(ref cause) => cause,
            TagResourceError::ResourceNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by UntagResource
#[derive(Debug, PartialEq)]
pub enum UntagResourceError {
    /// <p>More than one process tried to modify a resource at the same time.</p>
    ConcurrentModification(String),
    /// <p>Request processing has failed due to some unknown error, exception, or failure.</p>
    InternalServiceFault(String),
    /// <p>The value of an input parameter is bad or out-of-range.</p>
    InvalidParameterValue(String),
    /// <p>The named resource does not exist.</p>
    ResourceNotFound(String),
}

impl UntagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UntagResourceError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "ConcurrentModificationException" => {
                        return RusotoError::Service(UntagResourceError::ConcurrentModification(
                            parsed_error.message,
                        ))
                    }
                    "InternalServiceError" => {
                        return RusotoError::Service(UntagResourceError::InternalServiceFault(
                            parsed_error.message,
                        ))
                    }
                    "InvalidParameterValue" => {
                        return RusotoError::Service(UntagResourceError::InvalidParameterValue(
                            parsed_error.message,
                        ))
                    }
                    "ResourceNotFoundException" => {
                        return RusotoError::Service(UntagResourceError::ResourceNotFound(
                            parsed_error.message,
                        ))
                    }
                    _ => {}
                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for UntagResourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UntagResourceError {
    fn description(&self) -> &str {
        match *self {
            UntagResourceError::ConcurrentModification(ref cause) => cause,
            UntagResourceError::InternalServiceFault(ref cause) => cause,
            UntagResourceError::InvalidParameterValue(ref cause) => cause,
            UntagResourceError::ResourceNotFound(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the CloudWatch API. CloudWatch clients implement this trait.
pub trait CloudWatch {
    /// <p>Deletes the specified alarms. In the event of an error, no alarms are deleted.</p>
    fn delete_alarms(&self, input: DeleteAlarmsRequest) -> Request<DeleteAlarmsRequest>;

    /// <p>Deletes all dashboards that you specify. You may specify up to 100 dashboards to delete. If there is an error during this call, no dashboards are deleted.</p>
    fn delete_dashboards(&self, input: DeleteDashboardsRequest)
        -> Request<DeleteDashboardsRequest>;

    /// <p>Retrieves the history for the specified alarm. You can filter the results by date range or item type. If an alarm name is not specified, the histories for all alarms are returned.</p> <p>CloudWatch retains the history of an alarm even if you delete the alarm.</p>
    fn describe_alarm_history(
        &self,
        input: DescribeAlarmHistoryRequest,
    ) -> Request<DescribeAlarmHistoryRequest>;

    /// <p>Retrieves the specified alarms. If no alarms are specified, all alarms are returned. Alarms can be retrieved by using only a prefix for the alarm name, the alarm state, or a prefix for any action.</p>
    fn describe_alarms(&self, input: DescribeAlarmsRequest) -> Request<DescribeAlarmsRequest>;

    /// <p>Retrieves the alarms for the specified metric. To filter the results, specify a statistic, period, or unit.</p>
    fn describe_alarms_for_metric(
        &self,
        input: DescribeAlarmsForMetricRequest,
    ) -> Request<DescribeAlarmsForMetricRequest>;

    /// <p>Disables the actions for the specified alarms. When an alarm's actions are disabled, the alarm actions do not execute when the alarm state changes.</p>
    fn disable_alarm_actions(
        &self,
        input: DisableAlarmActionsRequest,
    ) -> Request<DisableAlarmActionsRequest>;

    /// <p>Enables the actions for the specified alarms.</p>
    fn enable_alarm_actions(
        &self,
        input: EnableAlarmActionsRequest,
    ) -> Request<EnableAlarmActionsRequest>;

    /// <p>Displays the details of the dashboard that you specify.</p> <p>To copy an existing dashboard, use <code>GetDashboard</code>, and then use the data returned within <code>DashboardBody</code> as the template for the new dashboard when you call <code>PutDashboard</code> to create the copy.</p>
    fn get_dashboard(&self, input: GetDashboardRequest) -> Request<GetDashboardRequest>;

    /// <p>You can use the <code>GetMetricData</code> API to retrieve as many as 100 different metrics in a single request, with a total of as many as 100,800 datapoints. You can also optionally perform math expressions on the values of the returned statistics, to create new time series that represent new insights into your data. For example, using Lambda metrics, you could divide the Errors metric by the Invocations metric to get an error rate time series. For more information about metric math expressions, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/using-metric-math.html#metric-math-syntax">Metric Math Syntax and Functions</a> in the <i>Amazon CloudWatch User Guide</i>.</p> <p>Calls to the <code>GetMetricData</code> API have a different pricing structure than calls to <code>GetMetricStatistics</code>. For more information about pricing, see <a href="https://aws.amazon.com/cloudwatch/pricing/">Amazon CloudWatch Pricing</a>.</p> <p>Amazon CloudWatch retains metric data as follows:</p> <ul> <li> <p>Data points with a period of less than 60 seconds are available for 3 hours. These data points are high-resolution metrics and are available only for custom metrics that have been defined with a <code>StorageResolution</code> of 1.</p> </li> <li> <p>Data points with a period of 60 seconds (1-minute) are available for 15 days.</p> </li> <li> <p>Data points with a period of 300 seconds (5-minute) are available for 63 days.</p> </li> <li> <p>Data points with a period of 3600 seconds (1 hour) are available for 455 days (15 months).</p> </li> </ul> <p>Data points that are initially published with a shorter period are aggregated together for long-term storage. For example, if you collect data using a period of 1 minute, the data remains available for 15 days with 1-minute resolution. After 15 days, this data is still available, but is aggregated and retrievable only with a resolution of 5 minutes. After 63 days, the data is further aggregated and is available with a resolution of 1 hour.</p>
    fn get_metric_data(&self, input: GetMetricDataRequest) -> Request<GetMetricDataRequest>;

    /// <p>Gets statistics for the specified metric.</p> <p>The maximum number of data points returned from a single call is 1,440. If you request more than 1,440 data points, CloudWatch returns an error. To reduce the number of data points, you can narrow the specified time range and make multiple requests across adjacent time ranges, or you can increase the specified period. Data points are not returned in chronological order.</p> <p>CloudWatch aggregates data points based on the length of the period that you specify. For example, if you request statistics with a one-hour period, CloudWatch aggregates all data points with time stamps that fall within each one-hour period. Therefore, the number of values aggregated by CloudWatch is larger than the number of data points returned.</p> <p>CloudWatch needs raw data points to calculate percentile statistics. If you publish data using a statistic set instead, you can only retrieve percentile statistics for this data if one of the following conditions is true:</p> <ul> <li> <p>The SampleCount value of the statistic set is 1.</p> </li> <li> <p>The Min and the Max values of the statistic set are equal.</p> </li> </ul> <p>Percentile statistics are not available for metrics when any of the metric values are negative numbers.</p> <p>Amazon CloudWatch retains metric data as follows:</p> <ul> <li> <p>Data points with a period of less than 60 seconds are available for 3 hours. These data points are high-resolution metrics and are available only for custom metrics that have been defined with a <code>StorageResolution</code> of 1.</p> </li> <li> <p>Data points with a period of 60 seconds (1-minute) are available for 15 days.</p> </li> <li> <p>Data points with a period of 300 seconds (5-minute) are available for 63 days.</p> </li> <li> <p>Data points with a period of 3600 seconds (1 hour) are available for 455 days (15 months).</p> </li> </ul> <p>Data points that are initially published with a shorter period are aggregated together for long-term storage. For example, if you collect data using a period of 1 minute, the data remains available for 15 days with 1-minute resolution. After 15 days, this data is still available, but is aggregated and retrievable only with a resolution of 5 minutes. After 63 days, the data is further aggregated and is available with a resolution of 1 hour.</p> <p>CloudWatch started retaining 5-minute and 1-hour metric data as of July 9, 2016.</p> <p>For information about metrics and dimensions supported by AWS services, see the <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/CW_Support_For_AWS.html">Amazon CloudWatch Metrics and Dimensions Reference</a> in the <i>Amazon CloudWatch User Guide</i>.</p>
    fn get_metric_statistics(
        &self,
        input: GetMetricStatisticsRequest,
    ) -> Request<GetMetricStatisticsRequest>;

    /// <p><p>You can use the <code>GetMetricWidgetImage</code> API to retrieve a snapshot graph of one or more Amazon CloudWatch metrics as a bitmap image. You can then embed this image into your services and products, such as wiki pages, reports, and documents. You could also retrieve images regularly, such as every minute, and create your own custom live dashboard.</p> <p>The graph you retrieve can include all CloudWatch metric graph features, including metric math and horizontal and vertical annotations.</p> <p>There is a limit of 20 transactions per second for this API. Each <code>GetMetricWidgetImage</code> action has the following limits:</p> <ul> <li> <p>As many as 100 metrics in the graph.</p> </li> <li> <p>Up to 100 KB uncompressed payload.</p> </li> </ul></p>
    fn get_metric_widget_image(
        &self,
        input: GetMetricWidgetImageRequest,
    ) -> Request<GetMetricWidgetImageRequest>;

    /// <p>Returns a list of the dashboards for your account. If you include <code>DashboardNamePrefix</code>, only those dashboards with names starting with the prefix are listed. Otherwise, all dashboards in your account are listed. </p> <p> <code>ListDashboards</code> returns up to 1000 results on one page. If there are more than 1000 dashboards, you can call <code>ListDashboards</code> again and include the value you received for <code>NextToken</code> in the first call, to receive the next 1000 results.</p>
    fn list_dashboards(&self, input: ListDashboardsRequest) -> Request<ListDashboardsRequest>;

    /// <p>List the specified metrics. You can use the returned metrics with <a>GetMetricData</a> or <a>GetMetricStatistics</a> to obtain statistical data.</p> <p>Up to 500 results are returned for any one call. To retrieve additional results, use the returned token with subsequent calls.</p> <p>After you create a metric, allow up to fifteen minutes before the metric appears. Statistics about the metric, however, are available sooner using <a>GetMetricData</a> or <a>GetMetricStatistics</a>.</p>
    fn list_metrics(&self, input: ListMetricsRequest) -> Request<ListMetricsRequest>;

    /// <p>Displays the tags associated with a CloudWatch resource. Alarms support tagging.</p>
    fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Request<ListTagsForResourceRequest>;

    /// <p>Creates a dashboard if it does not already exist, or updates an existing dashboard. If you update a dashboard, the entire contents are replaced with what you specify here.</p> <p>There is no limit to the number of dashboards in your account. All dashboards in your account are global, not region-specific.</p> <p>A simple way to create a dashboard using <code>PutDashboard</code> is to copy an existing dashboard. To copy an existing dashboard using the console, you can load the dashboard and then use the View/edit source command in the Actions menu to display the JSON block for that dashboard. Another way to copy a dashboard is to use <code>GetDashboard</code>, and then use the data returned within <code>DashboardBody</code> as the template for the new dashboard when you call <code>PutDashboard</code>.</p> <p>When you create a dashboard with <code>PutDashboard</code>, a good practice is to add a text widget at the top of the dashboard with a message that the dashboard was created by script and should not be changed in the console. This message could also point console users to the location of the <code>DashboardBody</code> script or the CloudFormation template used to create the dashboard.</p>
    fn put_dashboard(&self, input: PutDashboardRequest) -> Request<PutDashboardRequest>;

    /// <p>Creates or updates an alarm and associates it with the specified metric or metric math expression.</p> <p>When this operation creates an alarm, the alarm state is immediately set to <code>INSUFFICIENT_DATA</code>. The alarm is then evaluated and its state is set appropriately. Any actions associated with the new state are then executed.</p> <p>When you update an existing alarm, its state is left unchanged, but the update completely overwrites the previous configuration of the alarm.</p> <p>If you are an IAM user, you must have Amazon EC2 permissions for some alarm operations:</p> <ul> <li> <p> <code>iam:CreateServiceLinkedRole</code> for all alarms with EC2 actions</p> </li> <li> <p> <code>ec2:DescribeInstanceStatus</code> and <code>ec2:DescribeInstances</code> for all alarms on EC2 instance status metrics</p> </li> <li> <p> <code>ec2:StopInstances</code> for alarms with stop actions</p> </li> <li> <p> <code>ec2:TerminateInstances</code> for alarms with terminate actions</p> </li> <li> <p>No specific permissions are needed for alarms with recover actions</p> </li> </ul> <p>If you have read/write permissions for Amazon CloudWatch but not for Amazon EC2, you can still create an alarm, but the stop or terminate actions are not performed. However, if you are later granted the required permissions, the alarm actions that you created earlier are performed.</p> <p>If you are using an IAM role (for example, an EC2 instance profile), you cannot stop or terminate the instance using alarm actions. However, you can still see the alarm state and perform any other actions such as Amazon SNS notifications or Auto Scaling policies.</p> <p>If you are using temporary security credentials granted using AWS STS, you cannot stop or terminate an EC2 instance using alarm actions.</p> <p>The first time you create an alarm in the AWS Management Console, the CLI, or by using the PutMetricAlarm API, CloudWatch creates the necessary service-linked role for you. The service-linked role is called <code>AWSServiceRoleForCloudWatchEvents</code>. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_roles_terms-and-concepts.html#iam-term-service-linked-role">AWS service-linked role</a>.</p>
    fn put_metric_alarm(&self, input: PutMetricAlarmRequest) -> Request<PutMetricAlarmRequest>;

    /// <p><p>Publishes metric data points to Amazon CloudWatch. CloudWatch associates the data points with the specified metric. If the specified metric does not exist, CloudWatch creates the metric. When CloudWatch creates a metric, it can take up to fifteen minutes for the metric to appear in calls to <a>ListMetrics</a>.</p> <p>You can publish either individual data points in the <code>Value</code> field, or arrays of values and the number of times each value occurred during the period by using the <code>Values</code> and <code>Counts</code> fields in the <code>MetricDatum</code> structure. Using the <code>Values</code> and <code>Counts</code> method enables you to publish up to 150 values per metric with one <code>PutMetricData</code> request, and supports retrieving percentile statistics on this data.</p> <p>Each <code>PutMetricData</code> request is limited to 40 KB in size for HTTP POST requests. You can send a payload compressed by gzip. Each request is also limited to no more than 20 different metrics.</p> <p>Although the <code>Value</code> parameter accepts numbers of type <code>Double</code>, CloudWatch rejects values that are either too small or too large. Values must be in the range of 8.515920e-109 to 1.174271e+108 (Base 10) or 2e-360 to 2e360 (Base 2). In addition, special values (for example, NaN, +Infinity, -Infinity) are not supported.</p> <p>You can use up to 10 dimensions per metric to further clarify what data the metric collects. Each dimension consists of a Name and Value pair. For more information about specifying dimensions, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/publishingMetrics.html">Publishing Metrics</a> in the <i>Amazon CloudWatch User Guide</i>.</p> <p>Data points with time stamps from 24 hours ago or longer can take at least 48 hours to become available for <a>GetMetricData</a> or <a>GetMetricStatistics</a> from the time they are submitted.</p> <p>CloudWatch needs raw data points to calculate percentile statistics. If you publish data using a statistic set instead, you can only retrieve percentile statistics for this data if one of the following conditions is true:</p> <ul> <li> <p>The <code>SampleCount</code> value of the statistic set is 1 and <code>Min</code>, <code>Max</code>, and <code>Sum</code> are all equal.</p> </li> <li> <p>The <code>Min</code> and <code>Max</code> are equal, and <code>Sum</code> is equal to <code>Min</code> multiplied by <code>SampleCount</code>.</p> </li> </ul></p>
    fn put_metric_data(&self, input: PutMetricDataRequest) -> Request<PutMetricDataRequest>;

    /// <p>Temporarily sets the state of an alarm for testing purposes. When the updated state differs from the previous value, the action configured for the appropriate state is invoked. For example, if your alarm is configured to send an Amazon SNS message when an alarm is triggered, temporarily changing the alarm state to <code>ALARM</code> sends an SNS message. The alarm returns to its actual state (often within seconds). Because the alarm state change happens quickly, it is typically only visible in the alarm's <b>History</b> tab in the Amazon CloudWatch console or through <a>DescribeAlarmHistory</a>.</p>
    fn set_alarm_state(&self, input: SetAlarmStateRequest) -> Request<SetAlarmStateRequest>;

    /// <p>Assigns one or more tags (key-value pairs) to the specified CloudWatch resource. Tags can help you organize and categorize your resources. You can also use them to scope user permissions, by granting a user permission to access or change only resources with certain tag values. In CloudWatch, alarms can be tagged.</p> <p>Tags don't have any semantic meaning to AWS and are interpreted strictly as strings of characters.</p> <p>You can use the <code>TagResource</code> action with a resource that already has tags. If you specify a new tag key for the resource, this tag is appended to the list of tags associated with the resource. If you specify a tag key that is already associated with the resource, the new tag value that you specify replaces the previous value for that tag.</p> <p>You can associate as many as 50 tags with a resource.</p>
    fn tag_resource(&self, input: TagResourceRequest) -> Request<TagResourceRequest>;

    /// <p>Removes one or more tags from the specified resource.</p>
    fn untag_resource(&self, input: UntagResourceRequest) -> Request<UntagResourceRequest>;
}
/// A client for the CloudWatch API.
#[derive(Clone)]
pub struct CloudWatchClient {
    client: Client,
    region: region::Region,
}

impl CloudWatchClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> CloudWatchClient {
        CloudWatchClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> CloudWatchClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        P::Future: Send,
        D: DispatchSignedRequest + Send + Sync + 'static,
        D::Future: Send,
    {
        CloudWatchClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }
}

impl CloudWatch for CloudWatchClient {
    /// <p>Deletes the specified alarms. In the event of an error, no alarms are deleted.</p>
    fn delete_alarms(&self, input: DeleteAlarmsRequest) -> Request<DeleteAlarmsRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Deletes all dashboards that you specify. You may specify up to 100 dashboards to delete. If there is an error during this call, no dashboards are deleted.</p>
    fn delete_dashboards(
        &self,
        input: DeleteDashboardsRequest,
    ) -> Request<DeleteDashboardsRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Retrieves the history for the specified alarm. You can filter the results by date range or item type. If an alarm name is not specified, the histories for all alarms are returned.</p> <p>CloudWatch retains the history of an alarm even if you delete the alarm.</p>
    fn describe_alarm_history(
        &self,
        input: DescribeAlarmHistoryRequest,
    ) -> Request<DescribeAlarmHistoryRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Retrieves the specified alarms. If no alarms are specified, all alarms are returned. Alarms can be retrieved by using only a prefix for the alarm name, the alarm state, or a prefix for any action.</p>
    fn describe_alarms(&self, input: DescribeAlarmsRequest) -> Request<DescribeAlarmsRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Retrieves the alarms for the specified metric. To filter the results, specify a statistic, period, or unit.</p>
    fn describe_alarms_for_metric(
        &self,
        input: DescribeAlarmsForMetricRequest,
    ) -> Request<DescribeAlarmsForMetricRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Disables the actions for the specified alarms. When an alarm's actions are disabled, the alarm actions do not execute when the alarm state changes.</p>
    fn disable_alarm_actions(
        &self,
        input: DisableAlarmActionsRequest,
    ) -> Request<DisableAlarmActionsRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Enables the actions for the specified alarms.</p>
    fn enable_alarm_actions(
        &self,
        input: EnableAlarmActionsRequest,
    ) -> Request<EnableAlarmActionsRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Displays the details of the dashboard that you specify.</p> <p>To copy an existing dashboard, use <code>GetDashboard</code>, and then use the data returned within <code>DashboardBody</code> as the template for the new dashboard when you call <code>PutDashboard</code> to create the copy.</p>
    fn get_dashboard(&self, input: GetDashboardRequest) -> Request<GetDashboardRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>You can use the <code>GetMetricData</code> API to retrieve as many as 100 different metrics in a single request, with a total of as many as 100,800 datapoints. You can also optionally perform math expressions on the values of the returned statistics, to create new time series that represent new insights into your data. For example, using Lambda metrics, you could divide the Errors metric by the Invocations metric to get an error rate time series. For more information about metric math expressions, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/using-metric-math.html#metric-math-syntax">Metric Math Syntax and Functions</a> in the <i>Amazon CloudWatch User Guide</i>.</p> <p>Calls to the <code>GetMetricData</code> API have a different pricing structure than calls to <code>GetMetricStatistics</code>. For more information about pricing, see <a href="https://aws.amazon.com/cloudwatch/pricing/">Amazon CloudWatch Pricing</a>.</p> <p>Amazon CloudWatch retains metric data as follows:</p> <ul> <li> <p>Data points with a period of less than 60 seconds are available for 3 hours. These data points are high-resolution metrics and are available only for custom metrics that have been defined with a <code>StorageResolution</code> of 1.</p> </li> <li> <p>Data points with a period of 60 seconds (1-minute) are available for 15 days.</p> </li> <li> <p>Data points with a period of 300 seconds (5-minute) are available for 63 days.</p> </li> <li> <p>Data points with a period of 3600 seconds (1 hour) are available for 455 days (15 months).</p> </li> </ul> <p>Data points that are initially published with a shorter period are aggregated together for long-term storage. For example, if you collect data using a period of 1 minute, the data remains available for 15 days with 1-minute resolution. After 15 days, this data is still available, but is aggregated and retrievable only with a resolution of 5 minutes. After 63 days, the data is further aggregated and is available with a resolution of 1 hour.</p>
    fn get_metric_data(&self, input: GetMetricDataRequest) -> Request<GetMetricDataRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Gets statistics for the specified metric.</p> <p>The maximum number of data points returned from a single call is 1,440. If you request more than 1,440 data points, CloudWatch returns an error. To reduce the number of data points, you can narrow the specified time range and make multiple requests across adjacent time ranges, or you can increase the specified period. Data points are not returned in chronological order.</p> <p>CloudWatch aggregates data points based on the length of the period that you specify. For example, if you request statistics with a one-hour period, CloudWatch aggregates all data points with time stamps that fall within each one-hour period. Therefore, the number of values aggregated by CloudWatch is larger than the number of data points returned.</p> <p>CloudWatch needs raw data points to calculate percentile statistics. If you publish data using a statistic set instead, you can only retrieve percentile statistics for this data if one of the following conditions is true:</p> <ul> <li> <p>The SampleCount value of the statistic set is 1.</p> </li> <li> <p>The Min and the Max values of the statistic set are equal.</p> </li> </ul> <p>Percentile statistics are not available for metrics when any of the metric values are negative numbers.</p> <p>Amazon CloudWatch retains metric data as follows:</p> <ul> <li> <p>Data points with a period of less than 60 seconds are available for 3 hours. These data points are high-resolution metrics and are available only for custom metrics that have been defined with a <code>StorageResolution</code> of 1.</p> </li> <li> <p>Data points with a period of 60 seconds (1-minute) are available for 15 days.</p> </li> <li> <p>Data points with a period of 300 seconds (5-minute) are available for 63 days.</p> </li> <li> <p>Data points with a period of 3600 seconds (1 hour) are available for 455 days (15 months).</p> </li> </ul> <p>Data points that are initially published with a shorter period are aggregated together for long-term storage. For example, if you collect data using a period of 1 minute, the data remains available for 15 days with 1-minute resolution. After 15 days, this data is still available, but is aggregated and retrievable only with a resolution of 5 minutes. After 63 days, the data is further aggregated and is available with a resolution of 1 hour.</p> <p>CloudWatch started retaining 5-minute and 1-hour metric data as of July 9, 2016.</p> <p>For information about metrics and dimensions supported by AWS services, see the <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/CW_Support_For_AWS.html">Amazon CloudWatch Metrics and Dimensions Reference</a> in the <i>Amazon CloudWatch User Guide</i>.</p>
    fn get_metric_statistics(
        &self,
        input: GetMetricStatisticsRequest,
    ) -> Request<GetMetricStatisticsRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p><p>You can use the <code>GetMetricWidgetImage</code> API to retrieve a snapshot graph of one or more Amazon CloudWatch metrics as a bitmap image. You can then embed this image into your services and products, such as wiki pages, reports, and documents. You could also retrieve images regularly, such as every minute, and create your own custom live dashboard.</p> <p>The graph you retrieve can include all CloudWatch metric graph features, including metric math and horizontal and vertical annotations.</p> <p>There is a limit of 20 transactions per second for this API. Each <code>GetMetricWidgetImage</code> action has the following limits:</p> <ul> <li> <p>As many as 100 metrics in the graph.</p> </li> <li> <p>Up to 100 KB uncompressed payload.</p> </li> </ul></p>
    fn get_metric_widget_image(
        &self,
        input: GetMetricWidgetImageRequest,
    ) -> Request<GetMetricWidgetImageRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Returns a list of the dashboards for your account. If you include <code>DashboardNamePrefix</code>, only those dashboards with names starting with the prefix are listed. Otherwise, all dashboards in your account are listed. </p> <p> <code>ListDashboards</code> returns up to 1000 results on one page. If there are more than 1000 dashboards, you can call <code>ListDashboards</code> again and include the value you received for <code>NextToken</code> in the first call, to receive the next 1000 results.</p>
    fn list_dashboards(&self, input: ListDashboardsRequest) -> Request<ListDashboardsRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>List the specified metrics. You can use the returned metrics with <a>GetMetricData</a> or <a>GetMetricStatistics</a> to obtain statistical data.</p> <p>Up to 500 results are returned for any one call. To retrieve additional results, use the returned token with subsequent calls.</p> <p>After you create a metric, allow up to fifteen minutes before the metric appears. Statistics about the metric, however, are available sooner using <a>GetMetricData</a> or <a>GetMetricStatistics</a>.</p>
    fn list_metrics(&self, input: ListMetricsRequest) -> Request<ListMetricsRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Displays the tags associated with a CloudWatch resource. Alarms support tagging.</p>
    fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Request<ListTagsForResourceRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Creates a dashboard if it does not already exist, or updates an existing dashboard. If you update a dashboard, the entire contents are replaced with what you specify here.</p> <p>There is no limit to the number of dashboards in your account. All dashboards in your account are global, not region-specific.</p> <p>A simple way to create a dashboard using <code>PutDashboard</code> is to copy an existing dashboard. To copy an existing dashboard using the console, you can load the dashboard and then use the View/edit source command in the Actions menu to display the JSON block for that dashboard. Another way to copy a dashboard is to use <code>GetDashboard</code>, and then use the data returned within <code>DashboardBody</code> as the template for the new dashboard when you call <code>PutDashboard</code>.</p> <p>When you create a dashboard with <code>PutDashboard</code>, a good practice is to add a text widget at the top of the dashboard with a message that the dashboard was created by script and should not be changed in the console. This message could also point console users to the location of the <code>DashboardBody</code> script or the CloudFormation template used to create the dashboard.</p>
    fn put_dashboard(&self, input: PutDashboardRequest) -> Request<PutDashboardRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Creates or updates an alarm and associates it with the specified metric or metric math expression.</p> <p>When this operation creates an alarm, the alarm state is immediately set to <code>INSUFFICIENT_DATA</code>. The alarm is then evaluated and its state is set appropriately. Any actions associated with the new state are then executed.</p> <p>When you update an existing alarm, its state is left unchanged, but the update completely overwrites the previous configuration of the alarm.</p> <p>If you are an IAM user, you must have Amazon EC2 permissions for some alarm operations:</p> <ul> <li> <p> <code>iam:CreateServiceLinkedRole</code> for all alarms with EC2 actions</p> </li> <li> <p> <code>ec2:DescribeInstanceStatus</code> and <code>ec2:DescribeInstances</code> for all alarms on EC2 instance status metrics</p> </li> <li> <p> <code>ec2:StopInstances</code> for alarms with stop actions</p> </li> <li> <p> <code>ec2:TerminateInstances</code> for alarms with terminate actions</p> </li> <li> <p>No specific permissions are needed for alarms with recover actions</p> </li> </ul> <p>If you have read/write permissions for Amazon CloudWatch but not for Amazon EC2, you can still create an alarm, but the stop or terminate actions are not performed. However, if you are later granted the required permissions, the alarm actions that you created earlier are performed.</p> <p>If you are using an IAM role (for example, an EC2 instance profile), you cannot stop or terminate the instance using alarm actions. However, you can still see the alarm state and perform any other actions such as Amazon SNS notifications or Auto Scaling policies.</p> <p>If you are using temporary security credentials granted using AWS STS, you cannot stop or terminate an EC2 instance using alarm actions.</p> <p>The first time you create an alarm in the AWS Management Console, the CLI, or by using the PutMetricAlarm API, CloudWatch creates the necessary service-linked role for you. The service-linked role is called <code>AWSServiceRoleForCloudWatchEvents</code>. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_roles_terms-and-concepts.html#iam-term-service-linked-role">AWS service-linked role</a>.</p>
    fn put_metric_alarm(&self, input: PutMetricAlarmRequest) -> Request<PutMetricAlarmRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p><p>Publishes metric data points to Amazon CloudWatch. CloudWatch associates the data points with the specified metric. If the specified metric does not exist, CloudWatch creates the metric. When CloudWatch creates a metric, it can take up to fifteen minutes for the metric to appear in calls to <a>ListMetrics</a>.</p> <p>You can publish either individual data points in the <code>Value</code> field, or arrays of values and the number of times each value occurred during the period by using the <code>Values</code> and <code>Counts</code> fields in the <code>MetricDatum</code> structure. Using the <code>Values</code> and <code>Counts</code> method enables you to publish up to 150 values per metric with one <code>PutMetricData</code> request, and supports retrieving percentile statistics on this data.</p> <p>Each <code>PutMetricData</code> request is limited to 40 KB in size for HTTP POST requests. You can send a payload compressed by gzip. Each request is also limited to no more than 20 different metrics.</p> <p>Although the <code>Value</code> parameter accepts numbers of type <code>Double</code>, CloudWatch rejects values that are either too small or too large. Values must be in the range of 8.515920e-109 to 1.174271e+108 (Base 10) or 2e-360 to 2e360 (Base 2). In addition, special values (for example, NaN, +Infinity, -Infinity) are not supported.</p> <p>You can use up to 10 dimensions per metric to further clarify what data the metric collects. Each dimension consists of a Name and Value pair. For more information about specifying dimensions, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/publishingMetrics.html">Publishing Metrics</a> in the <i>Amazon CloudWatch User Guide</i>.</p> <p>Data points with time stamps from 24 hours ago or longer can take at least 48 hours to become available for <a>GetMetricData</a> or <a>GetMetricStatistics</a> from the time they are submitted.</p> <p>CloudWatch needs raw data points to calculate percentile statistics. If you publish data using a statistic set instead, you can only retrieve percentile statistics for this data if one of the following conditions is true:</p> <ul> <li> <p>The <code>SampleCount</code> value of the statistic set is 1 and <code>Min</code>, <code>Max</code>, and <code>Sum</code> are all equal.</p> </li> <li> <p>The <code>Min</code> and <code>Max</code> are equal, and <code>Sum</code> is equal to <code>Min</code> multiplied by <code>SampleCount</code>.</p> </li> </ul></p>
    fn put_metric_data(&self, input: PutMetricDataRequest) -> Request<PutMetricDataRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Temporarily sets the state of an alarm for testing purposes. When the updated state differs from the previous value, the action configured for the appropriate state is invoked. For example, if your alarm is configured to send an Amazon SNS message when an alarm is triggered, temporarily changing the alarm state to <code>ALARM</code> sends an SNS message. The alarm returns to its actual state (often within seconds). Because the alarm state change happens quickly, it is typically only visible in the alarm's <b>History</b> tab in the Amazon CloudWatch console or through <a>DescribeAlarmHistory</a>.</p>
    fn set_alarm_state(&self, input: SetAlarmStateRequest) -> Request<SetAlarmStateRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Assigns one or more tags (key-value pairs) to the specified CloudWatch resource. Tags can help you organize and categorize your resources. You can also use them to scope user permissions, by granting a user permission to access or change only resources with certain tag values. In CloudWatch, alarms can be tagged.</p> <p>Tags don't have any semantic meaning to AWS and are interpreted strictly as strings of characters.</p> <p>You can use the <code>TagResource</code> action with a resource that already has tags. If you specify a new tag key for the resource, this tag is appended to the list of tags associated with the resource. If you specify a tag key that is already associated with the resource, the new tag value that you specify replaces the previous value for that tag.</p> <p>You can associate as many as 50 tags with a resource.</p>
    fn tag_resource(&self, input: TagResourceRequest) -> Request<TagResourceRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Removes one or more tags from the specified resource.</p>
    fn untag_resource(&self, input: UntagResourceRequest) -> Request<UntagResourceRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }
}

impl ServiceRequest for DeleteAlarmsRequest {
    type Output = DeleteAlarmsResponse;
    type Error = DeleteAlarmsError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "monitoring", region, "/");
        let mut params = Params::new();

        params.put("Action", "DeleteAlarms");
        params.put("Version", "2010-08-01");
        DeleteAlarmsRequestSerializer::serialize(&mut params, "", &self);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteAlarmsError::from_response(response))),
                );
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = DeleteAlarmsResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result = DeleteAlarmsResponseDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack,
                    )?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for DeleteDashboardsRequest {
    type Output = DeleteDashboardsResponse;
    type Error = DeleteDashboardsError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "monitoring", region, "/");
        let mut params = Params::new();

        params.put("Action", "DeleteDashboards");
        params.put("Version", "2010-08-01");
        DeleteDashboardsRequestSerializer::serialize(&mut params, "", &self);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteDashboardsError::from_response(response))),
                );
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = DeleteDashboardsResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    start_element(&actual_tag_name, &mut stack)?;
                    result = DeleteDashboardsResponseDeserializer::deserialize(
                        "DeleteDashboardsResult",
                        &mut stack,
                    )?;
                    skip_tree(&mut stack);
                    end_element(&actual_tag_name, &mut stack)?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for DescribeAlarmHistoryRequest {
    type Output = DescribeAlarmHistoryResponse;
    type Error = DescribeAlarmHistoryError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "monitoring", region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeAlarmHistory");
        params.put("Version", "2010-08-01");
        DescribeAlarmHistoryRequestSerializer::serialize(&mut params, "", &self);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DescribeAlarmHistoryError::from_response(response))
                    }),
                );
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = DescribeAlarmHistoryResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    start_element(&actual_tag_name, &mut stack)?;
                    result = DescribeAlarmHistoryResponseDeserializer::deserialize(
                        "DescribeAlarmHistoryResult",
                        &mut stack,
                    )?;
                    skip_tree(&mut stack);
                    end_element(&actual_tag_name, &mut stack)?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for DescribeAlarmsRequest {
    type Output = DescribeAlarmsResponse;
    type Error = DescribeAlarmsError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "monitoring", region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeAlarms");
        params.put("Version", "2010-08-01");
        DescribeAlarmsRequestSerializer::serialize(&mut params, "", &self);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DescribeAlarmsError::from_response(response))),
                );
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = DescribeAlarmsResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    start_element(&actual_tag_name, &mut stack)?;
                    result = DescribeAlarmsResponseDeserializer::deserialize(
                        "DescribeAlarmsResult",
                        &mut stack,
                    )?;
                    skip_tree(&mut stack);
                    end_element(&actual_tag_name, &mut stack)?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for DescribeAlarmsForMetricRequest {
    type Output = DescribeAlarmsForMetricResponse;
    type Error = DescribeAlarmsForMetricError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "monitoring", region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeAlarmsForMetric");
        params.put("Version", "2010-08-01");
        DescribeAlarmsForMetricRequestSerializer::serialize(&mut params, "", &self);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeAlarmsForMetricError::from_response(response))
                }));
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = DescribeAlarmsForMetricResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    start_element(&actual_tag_name, &mut stack)?;
                    result = DescribeAlarmsForMetricResponseDeserializer::deserialize(
                        "DescribeAlarmsForMetricResult",
                        &mut stack,
                    )?;
                    skip_tree(&mut stack);
                    end_element(&actual_tag_name, &mut stack)?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for DisableAlarmActionsRequest {
    type Output = DisableAlarmActionsResponse;
    type Error = DisableAlarmActionsError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "monitoring", region, "/");
        let mut params = Params::new();

        params.put("Action", "DisableAlarmActions");
        params.put("Version", "2010-08-01");
        DisableAlarmActionsRequestSerializer::serialize(&mut params, "", &self);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DisableAlarmActionsError::from_response(response))
                    }),
                );
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = DisableAlarmActionsResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result = DisableAlarmActionsResponseDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack,
                    )?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for EnableAlarmActionsRequest {
    type Output = EnableAlarmActionsResponse;
    type Error = EnableAlarmActionsError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "monitoring", region, "/");
        let mut params = Params::new();

        params.put("Action", "EnableAlarmActions");
        params.put("Version", "2010-08-01");
        EnableAlarmActionsRequestSerializer::serialize(&mut params, "", &self);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(EnableAlarmActionsError::from_response(response))),
                );
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = EnableAlarmActionsResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result = EnableAlarmActionsResponseDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack,
                    )?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for GetDashboardRequest {
    type Output = GetDashboardResponse;
    type Error = GetDashboardError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "monitoring", region, "/");
        let mut params = Params::new();

        params.put("Action", "GetDashboard");
        params.put("Version", "2010-08-01");
        GetDashboardRequestSerializer::serialize(&mut params, "", &self);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetDashboardError::from_response(response))),
                );
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = GetDashboardResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    start_element(&actual_tag_name, &mut stack)?;
                    result = GetDashboardResponseDeserializer::deserialize(
                        "GetDashboardResult",
                        &mut stack,
                    )?;
                    skip_tree(&mut stack);
                    end_element(&actual_tag_name, &mut stack)?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for GetMetricDataRequest {
    type Output = GetMetricDataResponse;
    type Error = GetMetricDataError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "monitoring", region, "/");
        let mut params = Params::new();

        params.put("Action", "GetMetricData");
        params.put("Version", "2010-08-01");
        GetMetricDataRequestSerializer::serialize(&mut params, "", &self);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetMetricDataError::from_response(response))),
                );
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = GetMetricDataResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    start_element(&actual_tag_name, &mut stack)?;
                    result = GetMetricDataResponseDeserializer::deserialize(
                        "GetMetricDataResult",
                        &mut stack,
                    )?;
                    skip_tree(&mut stack);
                    end_element(&actual_tag_name, &mut stack)?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for GetMetricStatisticsRequest {
    type Output = GetMetricStatisticsResponse;
    type Error = GetMetricStatisticsError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "monitoring", region, "/");
        let mut params = Params::new();

        params.put("Action", "GetMetricStatistics");
        params.put("Version", "2010-08-01");
        GetMetricStatisticsRequestSerializer::serialize(&mut params, "", &self);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(GetMetricStatisticsError::from_response(response))
                    }),
                );
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = GetMetricStatisticsResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    start_element(&actual_tag_name, &mut stack)?;
                    result = GetMetricStatisticsResponseDeserializer::deserialize(
                        "GetMetricStatisticsResult",
                        &mut stack,
                    )?;
                    skip_tree(&mut stack);
                    end_element(&actual_tag_name, &mut stack)?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for GetMetricWidgetImageRequest {
    type Output = GetMetricWidgetImageResponse;
    type Error = GetMetricWidgetImageError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "monitoring", region, "/");
        let mut params = Params::new();

        params.put("Action", "GetMetricWidgetImage");
        params.put("Version", "2010-08-01");
        GetMetricWidgetImageRequestSerializer::serialize(&mut params, "", &self);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(GetMetricWidgetImageError::from_response(response))
                    }),
                );
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = GetMetricWidgetImageResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    start_element(&actual_tag_name, &mut stack)?;
                    result = GetMetricWidgetImageResponseDeserializer::deserialize(
                        "GetMetricWidgetImageResult",
                        &mut stack,
                    )?;
                    skip_tree(&mut stack);
                    end_element(&actual_tag_name, &mut stack)?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for ListDashboardsRequest {
    type Output = ListDashboardsResponse;
    type Error = ListDashboardsError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "monitoring", region, "/");
        let mut params = Params::new();

        params.put("Action", "ListDashboards");
        params.put("Version", "2010-08-01");
        ListDashboardsRequestSerializer::serialize(&mut params, "", &self);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListDashboardsError::from_response(response))),
                );
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = ListDashboardsResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    start_element(&actual_tag_name, &mut stack)?;
                    result = ListDashboardsResponseDeserializer::deserialize(
                        "ListDashboardsResult",
                        &mut stack,
                    )?;
                    skip_tree(&mut stack);
                    end_element(&actual_tag_name, &mut stack)?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for ListMetricsRequest {
    type Output = ListMetricsResponse;
    type Error = ListMetricsError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "monitoring", region, "/");
        let mut params = Params::new();

        params.put("Action", "ListMetrics");
        params.put("Version", "2010-08-01");
        ListMetricsRequestSerializer::serialize(&mut params, "", &self);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListMetricsError::from_response(response))),
                );
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = ListMetricsResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    start_element(&actual_tag_name, &mut stack)?;
                    result = ListMetricsResponseDeserializer::deserialize(
                        "ListMetricsResult",
                        &mut stack,
                    )?;
                    skip_tree(&mut stack);
                    end_element(&actual_tag_name, &mut stack)?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for ListTagsForResourceRequest {
    type Output = ListTagsForResourceResponse;
    type Error = ListTagsForResourceError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "monitoring", region, "/");
        let mut params = Params::new();

        params.put("Action", "ListTagsForResource");
        params.put("Version", "2010-08-01");
        ListTagsForResourceRequestSerializer::serialize(&mut params, "", &self);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(ListTagsForResourceError::from_response(response))
                    }),
                );
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = ListTagsForResourceResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    start_element(&actual_tag_name, &mut stack)?;
                    result = ListTagsForResourceResponseDeserializer::deserialize(
                        "ListTagsForResourceResult",
                        &mut stack,
                    )?;
                    skip_tree(&mut stack);
                    end_element(&actual_tag_name, &mut stack)?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for PutDashboardRequest {
    type Output = PutDashboardResponse;
    type Error = PutDashboardError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "monitoring", region, "/");
        let mut params = Params::new();

        params.put("Action", "PutDashboard");
        params.put("Version", "2010-08-01");
        PutDashboardRequestSerializer::serialize(&mut params, "", &self);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(PutDashboardError::from_response(response))),
                );
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = PutDashboardResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    start_element(&actual_tag_name, &mut stack)?;
                    result = PutDashboardResponseDeserializer::deserialize(
                        "PutDashboardResult",
                        &mut stack,
                    )?;
                    skip_tree(&mut stack);
                    end_element(&actual_tag_name, &mut stack)?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for PutMetricAlarmRequest {
    type Output = PutMetricAlarmResponse;
    type Error = PutMetricAlarmError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "monitoring", region, "/");
        let mut params = Params::new();

        params.put("Action", "PutMetricAlarm");
        params.put("Version", "2010-08-01");
        PutMetricAlarmRequestSerializer::serialize(&mut params, "", &self);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(PutMetricAlarmError::from_response(response))),
                );
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = PutMetricAlarmResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result = PutMetricAlarmResponseDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack,
                    )?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for PutMetricDataRequest {
    type Output = PutMetricDataResponse;
    type Error = PutMetricDataError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "monitoring", region, "/");
        let mut params = Params::new();

        params.put("Action", "PutMetricData");
        params.put("Version", "2010-08-01");
        PutMetricDataRequestSerializer::serialize(&mut params, "", &self);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(PutMetricDataError::from_response(response))),
                );
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = PutMetricDataResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result = PutMetricDataResponseDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack,
                    )?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for SetAlarmStateRequest {
    type Output = SetAlarmStateResponse;
    type Error = SetAlarmStateError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "monitoring", region, "/");
        let mut params = Params::new();

        params.put("Action", "SetAlarmState");
        params.put("Version", "2010-08-01");
        SetAlarmStateRequestSerializer::serialize(&mut params, "", &self);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(SetAlarmStateError::from_response(response))),
                );
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = SetAlarmStateResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    result = SetAlarmStateResponseDeserializer::deserialize(
                        &actual_tag_name,
                        &mut stack,
                    )?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for TagResourceRequest {
    type Output = TagResourceResponse;
    type Error = TagResourceError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "monitoring", region, "/");
        let mut params = Params::new();

        params.put("Action", "TagResource");
        params.put("Version", "2010-08-01");
        TagResourceRequestSerializer::serialize(&mut params, "", &self);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(TagResourceError::from_response(response))),
                );
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = TagResourceResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    start_element(&actual_tag_name, &mut stack)?;
                    result = TagResourceResponseDeserializer::deserialize(
                        "TagResourceResult",
                        &mut stack,
                    )?;
                    skip_tree(&mut stack);
                    end_element(&actual_tag_name, &mut stack)?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }
}

impl ServiceRequest for UntagResourceRequest {
    type Output = UntagResourceResponse;
    type Error = UntagResourceError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "monitoring", region, "/");
        let mut params = Params::new();

        params.put("Action", "UntagResource");
        params.put("Version", "2010-08-01");
        UntagResourceRequestSerializer::serialize(&mut params, "", &self);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        dispatcher.dispatch(request, |response| {
            if !response.status.is_success() {
                return Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UntagResourceError::from_response(response))),
                );
            }

            Box::new(response.buffer().from_err().and_then(move |response| {
                let result;

                if response.body.is_empty() {
                    result = UntagResourceResponse::default();
                } else {
                    let reader = EventReader::new_with_config(
                        response.body.as_ref(),
                        ParserConfig::new().trim_whitespace(true),
                    );
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = peek_at_name(&mut stack)?;
                    start_element(&actual_tag_name, &mut stack)?;
                    result = UntagResourceResponseDeserializer::deserialize(
                        "UntagResourceResult",
                        &mut stack,
                    )?;
                    skip_tree(&mut stack);
                    end_element(&actual_tag_name, &mut stack)?;
                }
                // parse non-payload
                Ok(result)
            }))
        })
    }
}

#[cfg(test)]
mod protocol_tests {

    extern crate rusoto_mock;

    use self::rusoto_mock::*;
    use super::*;
    use rusoto_core::Region as rusoto_region;

    #[test]
    fn test_parse_error_cloudwatch_describe_alarm_history() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/error",
            "cloudwatch-describe-alarm-history.xml",
        );
        let mock = MockRequestDispatcher::with_status(400).with_body(&mock_response);
        let client =
            CloudWatchClient::new_with(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = DescribeAlarmHistoryRequest::default();
        let result = client.describe_alarm_history(request).sync();
        assert!(!result.is_ok(), "parse error: {:?}", result);
    }

    #[test]
    fn test_parse_valid_cloudwatch_describe_alarm_history() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "cloudwatch-describe-alarm-history.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client =
            CloudWatchClient::new_with(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = DescribeAlarmHistoryRequest::default();
        let result = client.describe_alarm_history(request).sync();
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[test]
    fn test_parse_valid_cloudwatch_describe_alarms() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "cloudwatch-describe-alarms.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client =
            CloudWatchClient::new_with(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = DescribeAlarmsRequest::default();
        let result = client.describe_alarms(request).sync();
        assert!(result.is_ok(), "parse error: {:?}", result);
    }

    #[test]
    fn test_parse_valid_cloudwatch_list_metrics() {
        let mock_response = MockResponseReader::read_response(
            "test_resources/generated/valid",
            "cloudwatch-list-metrics.xml",
        );
        let mock = MockRequestDispatcher::with_status(200).with_body(&mock_response);
        let client =
            CloudWatchClient::new_with(mock, MockCredentialsProvider, rusoto_region::UsEast1);
        let request = ListMetricsRequest::default();
        let result = client.list_metrics(request).sync();
        assert!(result.is_ok(), "parse error: {:?}", result);
    }
}
