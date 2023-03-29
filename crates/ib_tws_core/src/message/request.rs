use std::collections::HashSet;

use crate::domain::{
    market_data::{GenericTick, MarketDataType, TickByTickType},
    misc::ServerLogLevel,
    *,
};

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum Request {
    Handshake(Handshake),
    StartApi(StartApi),
    CancelScannerSubscription(CancelScannerSubscription),
    ReqScannerParameters(ReqScannerParameters),
    ReqScannerSubscription(ReqScannerSubscription),
    ReqMktData(ReqMktData),
    CancelHistoricalData(CancelHistoricalData),
    CancelRealtimeBars(CancelRealtimeBars),
    ReqHistoricalData(ReqHistoricalData),
    ReqHeadTimestamp(ReqHeadTimestamp),
    CancelHeadTimestamp(CancelHeadTimestamp),
    ReqRealtimeBars(ReqRealtimeBars),
    ReqContractDetails(ReqContractDetails),
    ReqMktDepth(ReqMktDepth),
    CancelMktData(CancelMktData),
    CancelMktDepth(CancelMktDepth),
    ExerciseOptions(ExerciseOptions),
    PlaceOrder(PlaceOrder),
    ReqAccountUpdates(ReqAccountUpdates),
    ReqExecutions(ReqExecutions),
    CancelOrder(CacelOrder),
    ReqOpenOrders(ReqOpenOrders),
    ReqIds(ReqIds),
    ReqNewsBulletins(ReqNewsBulletins),
    CancelNewsBulletins(CancelNewsBulletins),
    SetServerLogLevel(SetServerLogLevel),
    ReqAutoOpenOrders(ReqAutoOpenOrders),
    ReqAllOpenOrders(ReqAllOpenOrders),
    ReqManagedAccts(ReqManagedAccts),
    RequestFA(RequestFA),
    ReplaceFA(ReplaceFA),
    ReqCurrentTime(ReqCurrentTime),
    ReqFundamentalData(ReqFundamentalData),
    CancelFundamentalData(CancelFundamentalData),
    CalculateImpliedVolatility(CalculateImpliedVolatility),
    CancelCalculateImpliedVolatility(CancelCalculateImpliedVolatility),
    CalculateOptionPrice(CalculateOptionPrice),
    CancelCalculateOptionPrice(CancelCalculateOptionPrice),
    ReqGlobalCancel(ReqGlobalCancel),
    ReqMarketDataType(ReqMarketDataType),
    ReqPositions(ReqPositions),
    ReqSecDefOptParams(ReqSecDefOptParams),
    ReqSoftDollarTiers(ReqSoftDollarTiers),
    CancelPositions(CancelPositions),
    ReqPositionsMulti(ReqPositionsMulti),
    CancelPositionsMulti(CancelPositionsMulti),
    CancelAccountUpdatesMulti(CancelAccountUpdatesMulti),
    ReqAccountUpdatesMulti(ReqAccountUpdatesMulti),
    ReqAccountSummary(ReqAccountSummary),
    CancelAccountSummary(CancelAccountSummary),
    VerifyRequest(VerifyRequest),
    VerifyMessage(VerifyMessage),
    VerfyAndAuthRequest(VerfyAndAuthRequest),
    VerifyAndAuthMessage(VerifyAndAuthMessage),
    QueryDisplayGroups(QueryDisplayGroups),
    SubscribeToGroupEvents(SubscribeToGroupEvents),
    UpdateDisplayGroup(UpdateDisplayGroup),
    UnsubscribeFromGroupEvents(UbsubscribeFromGroupEvents),
    MatchingSymbol(MatchingSymbol),
    ReqFamilyCodes(ReqFamilyCodes),
    ReqMktDepthExchanges(ReqMktDepthExchanges),
    ReqSmartComponents(ReqSmartComponents),
    ReqNewsProvider(ReqNewsProvider),
    ReqNewsArticle(ReqNewsArticle),
    ReqHistoricalNews(ReqHistoricalNews),
    ReqHistogramData(ReqHistogramData),
    CancelHistogramData(CancelHistogramData),
    ReqMarketRule(ReqMarketRule),
    ReqPnl(ReqPnl),
    CancelPnl(CancelPnl),
    ReqPnlSingle(ReqPnlSingle),
    CancelPnlSingle(CancelPnlSingle),
    ReqHistoricalTicks(ReqHistoricalTicks),
    ReqTickByTickData(ReqTickByTickData),
    CancelTickByTickData(CancelTickByTickData),
}

impl Request {
    pub(crate) fn set_request_id(&mut self, request_id: i32) {
        match self {
            Self::CancelScannerSubscription(msg) => msg.req_id = request_id,
            Self::ReqScannerSubscription(msg) => msg.req_id = request_id,
            Self::ReqMktData(msg) => msg.req_id = request_id,
            Self::CancelHistoricalData(msg) => msg.req_id = request_id,
            Self::CalculateImpliedVolatility(msg) => msg.req_id = request_id,
            Self::CalculateOptionPrice(msg) => msg.req_id = request_id,
            Self::CancelAccountSummary(msg) => msg.req_id = request_id,
            Self::CancelCalculateImpliedVolatility(msg) => msg.req_id = request_id,
            Self::CancelCalculateOptionPrice(msg) => msg.req_id = request_id,
            Self::CancelFundamentalData(msg) => msg.req_id = request_id,
            Self::CancelRealtimeBars(msg) => msg.req_id = request_id,
            Self::ReqHistoricalData(msg) => msg.req_id = request_id,
            Self::ReqHeadTimestamp(msg) => msg.req_id = request_id,
            Self::CancelHeadTimestamp(msg) => msg.req_id = request_id,
            Self::ReqRealtimeBars(msg) => msg.req_id = request_id,
            Self::ReqContractDetails(msg) => msg.req_id = request_id,
            Self::ReqMktDepth(msg) => msg.req_id = request_id,
            Self::CancelMktData(msg) => msg.req_id = request_id,
            Self::CancelMktDepth(msg) => msg.req_id = request_id,
            Self::ExerciseOptions(msg) => msg.req_id = request_id,
            Self::ReqExecutions(msg) => msg.req_id = request_id,
            Self::ReqAccountSummary(msg) => msg.req_id = request_id,
            Self::ReplaceFA(msg) => msg.req_id = request_id,
            Self::ReqFundamentalData(msg) => msg.req_id = request_id,
            Self::ReqSecDefOptParams(msg) => msg.req_id = request_id,
            Self::ReqSoftDollarTiers(msg) => msg.req_id = request_id,
            Self::MatchingSymbol(msg) => msg.req_id = request_id,
            Self::ReqSmartComponents(msg) => msg.req_id = request_id,
            Self::ReqPnl(msg) => msg.req_id = request_id,
            Self::CancelPnl(msg) => msg.req_id = request_id,
            Self::ReqPnlSingle(msg) => msg.req_id = request_id,
            Self::CancelPnlSingle(msg) => msg.req_id = request_id,
            Self::ReqHistoricalTicks(msg) => msg.req_id = request_id,
            // Self::ReqWshMetaData(msg) => msg.req_id = request_id,
            // Self::ReqUserInfo(msg) => msg.req_id = request_id,
            Self::CancelTickByTickData(msg) => msg.req_id = request_id,
            Self::ReqTickByTickData(msg) => msg.req_id = request_id,
            Self::QueryDisplayGroups(msg) => msg.req_id = request_id,
            Self::SubscribeToGroupEvents(msg) => msg.req_id = request_id,
            Self::UpdateDisplayGroup(msg) => msg.req_id = request_id,
            Self::UnsubscribeFromGroupEvents(msg) => msg.req_id = request_id,
            Self::ReqPositionsMulti(msg) => msg.req_id = request_id,
            Self::CancelPositionsMulti(msg) => msg.req_id = request_id,
            Self::ReqAccountUpdatesMulti(msg) => msg.req_id = request_id,
            Self::CancelAccountUpdatesMulti(msg) => msg.req_id = request_id,
            Self::ReqNewsArticle(msg) => msg.req_id = request_id,
            Self::ReqHistoricalNews(msg) => msg.req_id = request_id,
            _ => (),
        }
    }
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Handshake {
    pub min_version: i32,
    pub max_version: i32,
    pub option: Option<String>,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct StartApi {
    pub client_id: i32,
    pub optional_capabilities: String,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct CancelScannerSubscription {
    pub req_id: i32,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ReqScannerParameters {}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ReqScannerSubscription {
    pub req_id: i32,
    pub subscribe: ScannerSubscription,
    pub options: Vec<TagValue>,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ReqMktData {
    pub req_id: i32,
    pub contract: Contract,
    pub generic_tick_list: HashSet<GenericTick>,
    pub snapshot: bool,
    pub regulatory_snapshot: bool,
    pub mkt_data_options: Vec<TagValue>,
}

impl ReqMktData {
    #[must_use]
    pub fn new(
        contract: Contract,
        generic_tick_list: HashSet<GenericTick>,
        snapshot: bool,
        regulatory_snapshot: bool,
        mkt_data_options: Vec<TagValue>,
    ) -> Self {
        Self {
            req_id: 0,
            contract,
            generic_tick_list,
            snapshot,
            regulatory_snapshot,
            mkt_data_options,
        }
    }
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct CancelHistoricalData {
    pub req_id: i32,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct CancelRealtimeBars {
    pub req_id: i32,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ReqHistoricalData {
    pub req_id: i32,
    pub contract: Contract,
    pub end_date_time: String,
    pub duration_str: String,
    pub bar_size_setting: String,
    pub what_to_show: String,
    pub use_rth: i32,
    pub format_date: i32,
    pub keepup_to_date: bool,
    pub chart_options: Vec<TagValue>,
}

impl ReqHistoricalData {
    pub fn new(
        contract: Contract,
        end_date_time: String,
        duration_str: String,
        bar_size_setting: String,
        what_to_show: String,
        use_rth: i32,
        format_date: i32,
        keepup_to_date: bool,
        chart_options: Vec<TagValue>,
    ) -> Self {
        Self {
            req_id: 0,
            contract,
            end_date_time,
            duration_str,
            bar_size_setting,
            what_to_show,
            use_rth,
            format_date,
            keepup_to_date,
            chart_options,
        }
    }
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ReqHeadTimestamp {
    pub req_id: i32,
    pub contract: Contract,
    pub what_to_show: String,
    pub use_rth: i32,
    pub format_date: i32,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct CancelHeadTimestamp {
    pub req_id: i32,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ReqRealtimeBars {
    pub req_id: i32,
    pub contract: Contract,
    pub bar_size: i32,
    pub what_to_show: String,
    pub use_rth: bool,
    pub options: Vec<TagValue>,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ReqContractDetails {
    pub(crate) req_id: i32,
    pub contract: Contract,
}

impl ReqContractDetails {
    #[must_use]
    pub fn new(contract: Contract) -> Self {
        Self {
            req_id: 0,
            contract,
        }
    }
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ReqMktDepth {
    pub req_id: i32,
    pub contract: Contract,
    pub num_rows: i32,
    pub is_smart_depth: bool,
    pub options: Vec<TagValue>,
}

impl ReqMktDepth {
    #[must_use]
    pub fn new(
        contract: Contract,
        num_rows: i32,
        is_smart_depth: bool,
        options: Vec<TagValue>,
    ) -> Self {
        Self {
            req_id: 0,
            contract,
            num_rows,
            is_smart_depth,
            options,
        }
    }
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct CancelMktData {
    pub req_id: i32,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct CancelMktDepth {
    pub req_id: i32,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ExerciseOptions {
    pub req_id: i32,
    pub contract: Contract,
    pub exercise_action: i32,
    pub exercise_quantity: i32,
    pub account: String,
    pub overriden: i32,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct PlaceOrder {
    pub id: i32,
    pub contract: Contract,
    pub order: Order,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ReqAccountUpdates {
    pub subscribe: bool,
    pub acct_code: String,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ReqExecutions {
    pub req_id: i32,
    pub filter: ExecutionFilter,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct CacelOrder {
    pub id: i32,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ReqOpenOrders {}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ReqIds {
    pub num_ids: i32,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ReqNewsBulletins {
    pub all_msgs: bool,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct CancelNewsBulletins {}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct SetServerLogLevel {
    pub log_level: ServerLogLevel,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ReqAutoOpenOrders {
    pub auto_bind: bool,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ReqAllOpenOrders {}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ReqManagedAccts {}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct RequestFA {
    pub fa_data_type: i32,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ReplaceFA {
    pub req_id: i32,
    pub fa_data_type: i32,
    pub xml: String,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ReqCurrentTime {}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ReqFundamentalData {
    pub req_id: i32,
    pub contract: Contract,
    pub report_type: String,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct CancelFundamentalData {
    pub req_id: i32,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct CalculateImpliedVolatility {
    pub req_id: i32,
    pub contract: Contract,
    pub option_price: f64,
    pub under_price: f64,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct CancelCalculateImpliedVolatility {
    pub req_id: i32,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct CalculateOptionPrice {
    pub req_id: i32,
    pub contract: Contract,
    pub volatility: f64,
    pub under_price: f64,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct CancelCalculateOptionPrice {
    pub req_id: i32,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ReqGlobalCancel {}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ReqMarketDataType {
    pub market_data_type: MarketDataType,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ReqPositions {}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ReqSecDefOptParams {
    pub req_id: i32,
    pub underlying_symbol: String,
    pub fut_fop_exchange: String,
    pub underlying_sec_type: String,
    pub underlying_con_id: i32,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ReqSoftDollarTiers {
    pub req_id: i32,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct CancelPositions {}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ReqPositionsMulti {
    pub req_id: i32,
    pub account: String,
    pub model_code: String,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct CancelPositionsMulti {
    pub req_id: i32,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct CancelAccountUpdatesMulti {
    pub req_id: i32,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ReqAccountUpdatesMulti {
    pub req_id: i32,
    pub account: String,
    pub model_code: String,
    pub ledger_and_nlv: bool,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ReqAccountSummary {
    pub(crate) req_id: i32,
    pub group: String,
    pub tags: String,
}

impl ReqAccountSummary {
    pub fn new(group: String, tags: String) -> Self {
        Self {
            req_id: 0,
            group,
            tags,
        }
    }
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct CancelAccountSummary {
    pub req_id: i32,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct VerifyRequest {
    pub api_name: String,
    pub api_version: String,
    pub extra_auth: bool,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct VerifyMessage {
    pub api_data: String,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct VerfyAndAuthRequest {
    pub api_name: String,
    pub api_version: String,
    pub opaque_is_vkey: String,
    pub extra_auth: bool,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct VerifyAndAuthMessage {
    pub api_data: String,
    pub xyz_response: String,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct QueryDisplayGroups {
    pub req_id: i32,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct SubscribeToGroupEvents {
    pub req_id: i32,
    pub group_id: i32,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct UpdateDisplayGroup {
    pub req_id: i32,
    pub contract_info: String,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct UbsubscribeFromGroupEvents {
    pub req_id: i32,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct MatchingSymbol {
    pub req_id: i32,
    pub pattern: String,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ReqFamilyCodes {
    pub server_version: i32,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ReqMktDepthExchanges {}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ReqSmartComponents {
    pub req_id: i32,
    pub bbo_exchange: String,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ReqNewsProvider {
    pub server_version: i32,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ReqNewsArticle {
    pub req_id: i32,
    pub provider_code: String,
    pub article_id: String,
    pub options: Vec<TagValue>,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ReqHistoricalNews {
    pub req_id: i32,
    pub con_id: i32,
    pub provider_code: String,
    pub start_time: String,
    pub end_time: String,
    pub total_results: i32,
    pub options: Vec<TagValue>,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ReqHistogramData {
    pub req_id: i32,
    pub contract: Contract,
    pub use_rth: bool,
    pub time_period: String,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct CancelHistogramData {
    pub req_id: i32,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ReqMarketRule {
    pub market_rule_id: i32,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ReqPnl {
    pub req_id: i32,
    pub account: String,
    pub model_code: String,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct CancelPnl {
    pub req_id: i32,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ReqPnlSingle {
    pub req_id: i32,
    pub account: String,
    pub model_code: String,
    pub con_id: i32,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct CancelPnlSingle {
    pub req_id: i32,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ReqHistoricalTicks {
    pub req_id: i32,
    pub contract: Contract,
    pub start_time: String,
    pub end_time: String,
    pub num_of_ticks: i32,
    pub what_to_show: String,
    pub use_rth: i32,
    pub ignore_size: bool,
    pub options: Vec<TagValue>,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ReqTickByTickData {
    pub(crate) req_id: i32,
    pub contract: Contract,
    pub tick_type: TickByTickType,
    /// Number of ticks
    pub num_of_ticks: i32,
    /// Ignore size flag
    pub ignore_size: bool,
}

impl ReqTickByTickData {
    #[must_use]
    pub fn new(
        contract: Contract,
        tick_type: TickByTickType,
        num_of_ticks: i32,
        ignore_size: bool,
    ) -> Self {
        Self {
            req_id: 0,
            contract,
            tick_type,
            num_of_ticks,
            ignore_size,
        }
    }
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct CancelTickByTickData {
    pub req_id: i32,
}
