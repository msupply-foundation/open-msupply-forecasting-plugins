use gql::Api;
use reqwest::{self, Url};
use serde::Deserialize;
use serde_json::json;
use tabled::{Table, Tabled};
mod gql;

#[macro_export]
macro_rules! assert_variant {
    ($e:expr, $matches:pat => $result:expr) => {
         match $e {
            $matches=> $result,
            _ => panic!("expected {}", stringify!($matches:pat => $result:expr))
        }
    }
}

pub const INSERT: &'static str = r#"
mutation Insert(
  $storeId: String!
  $input: InsertProgramRequestRequisitionInput!
) {
  root: insertProgramRequestRequisition(input: $input, storeId: $storeId) {
    __typename
    ... on RequisitionNode {
      id
    }
    ... on InsertProgramRequestRequisitionError {
      error {
        __typename
        description
      }
    }
  }
}
"#;

pub const LINES: &'static str = r#"
query Lines($storeId: String!, $id: String!) {
  root: requisition(id: $id, storeId: $storeId) {
    __typename
    ... on RequisitionNode {
      id
      lines {
        nodes {
          id
          itemId
          averageMonthlyConsumption
          itemName
          additionInUnits
          availableStockOnHand
          incomingUnits
          outgoingUnits
          lossInUnits
          initialStockOnHandUnits
          suggestedQuantity
        }
      }
    }
  }
}
"#;

pub const DELETE: &'static str = r#"
mutation Delete($storeId: String!, $id: String!) {
  root: deleteRequestRequisition(storeId: $storeId, input: { id: $id }) {
    __typename
    ... on DeleteResponse {
      id
    }
    ... on DeleteRequestRequisitionError {
      error {
        __typename
        description
      }
    }
  }
}
"#;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("loggin in");
    let api = Api::new_with_token(Url::parse("http://localhost:8000")?, "admin", "pass").await?;

    println!("creating requisition");
    let requisition_id = "request_requisition";
    let variables = json!({
      "storeId": "AD3356BD52886E4AA65C096335B8C4C7",
      "input": {
        "id":requisition_id,
        "programOrderTypeId": "C74322977EA5704B8C192C904B2093D4AE2C43C63C33E749A249BC7E1B6E36D6Ordinaire",
        "otherPartyId": "F8D91DEBDF779243AB5DC1B8C3EB6FD6",
        "periodId": "F479B1BCE3AD4945AFF86B55AD59917E"
      }
    });

    let _ = api.gql(INSERT, variables, Some("RequisitionNode")).await?;

    println!("getting lines");
    let variables = json!({
      "storeId": "AD3356BD52886E4AA65C096335B8C4C7",
      "id": requisition_id});

    let result = api.gql(LINES, variables, Some("RequisitionNode")).await?;
    #[derive(Tabled, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct Lines {
        item_id: String,
        item_name: String,
        average_monthly_consumption: f64,
        initial_stock_on_hand_units: f64,
        incoming_units: f64,
        outgoing_units: f64,
        #[tabled(skip)]
        loss_in_units: f64,
        #[tabled(skip)]
        addition_in_units: f64,
        available_stock_on_hand: f64,
        #[serde(default)]
        inventory_adjustment: f64,
        suggested_quantity: f64,
    }

    let lines: Vec<Lines> = serde_json::from_value(result["lines"]["nodes"].clone()).unwrap();
    let mut lines: Vec<Lines> = lines
        .into_iter()
        .map(|line| Lines {
            inventory_adjustment: line.addition_in_units + line.loss_in_units,
            ..line
        })
        .collect();
    lines.sort_by(|a, b| {
        a.average_monthly_consumption
            .partial_cmp(&b.average_monthly_consumption)
            .unwrap()
    });

    println!("{}", Table::new(&lines));

    let variables = json!({
      "storeId": "AD3356BD52886E4AA65C096335B8C4C7",
      "id": requisition_id});

    let _ = api.gql(DELETE, variables, Some("DeleteResponse")).await?;

    Ok(())
}
