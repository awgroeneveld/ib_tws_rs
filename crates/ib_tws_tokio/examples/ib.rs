#[macro_use]
extern crate tracing;

use std::string::ToString;
use std::time::Duration;

use futures::{StreamExt, TryStreamExt};
use ib_tws_core::domain::{self, Contract};
use ib_tws_core::domain::market_data::{MarketDataType, TickByTickType};
use ib_tws_core::message::{request::*, Response};
use miette::IntoDiagnostic;

#[tokio::main]
async fn main() -> miette::Result<()> {
    tracing_subscriber::fmt::init();

    let client = {
        let port = std::env::args()
            .nth(1)
            .and_then(|p| p.parse::<u32>().ok())
            .unwrap_or(4001);
        let transport = ib_tws_tokio::Transport::connect(
            format!("127.0.0.1:{port}").parse().unwrap(),
            Duration::from_secs(5),
        )
        .await
        .into_diagnostic()?;
        ib_tws_core::AsyncClient::setup(transport, 1).await?
    };
    info!(version = client.server_version(), "connected to client");

    tokio::task::spawn(client.response_stream()
        .for_each(move |buf| async move {
            match buf {
                Response::ErrMsgMsg(msg) => warn!("{:#?}", msg),
                buf => trace!("{:#?}", buf),
            }
        }));

    let contract = Contract::new_cryptocurrency("ETH", "USD").unwrap();

    client.set_server_log_level(domain::misc::ServerLogLevel::Detail).await?;
    let response = client.request_contract_details(ReqContractDetails::new(contract.clone())).await?;
    info!(?response);
    // let response = client.request_market_depth_exchanges().await?;
    // info!(?response);
    client.request_market_data_type(MarketDataType::REALTIME).await?;
    let bars = client.request_historical_data(ReqHistoricalData::new(
        contract.clone(),
        "20221001 00:00:00 UTC".to_owned(),
        "1 D".to_owned(),
        "5 mins".to_owned(),
        "TRADES".to_owned(),
        0,
        2,
        false,
        vec![]
    )).await?;
    info!(?bars);
    client.request_tick_by_tick_data(ReqTickByTickData::new(
        contract.clone(),
        TickByTickType::AllLast,
        0,
        false,
    )).await?
        .try_for_each(move |response| async move {
            info!(?response);
            Ok(())
        })
        .await?;
    client.request_market_depth(ReqMktDepth::new(contract, 10, false, vec![])).await?
        .try_for_each(move |response| async move {
            info!(?response);
            Ok(())
        })
        .await?;

    Ok(())
}
