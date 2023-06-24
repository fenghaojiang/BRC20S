use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct Transfer {
  // 10 letter identifier of the pool id，pool number not included
  #[serde(rename = "tid")]
  pub tick_id: String,

  // Ticker: 4-6 letter identifier of the brc20-s
  #[serde(rename = "tick")]
  pub tick: String,

  // Amount to transfer: States the amount of the brc20-s to transfer.
  #[serde(rename = "amt")]
  pub amount: String,
}

#[cfg(test)]
mod tests {
  use super::super::*;
  use super::*;

  #[test]
  fn test_serialize() {
    let obj = Transfer {
      tick_id: "tid".to_string(),
      tick: "tick".to_string(),
      amount: "amt".to_string(),
    };

    assert_eq!(
      serde_json::to_string(&obj).unwrap(),
      format!(
        r##"{{"tid":"{}","tick":"{}","amt":"{}"}}"##,
        obj.tick_id, obj.tick, obj.amount
      )
    )
  }

  #[test]
  fn test_deserialize() {
    let json_str = format!(
      r##"{{
        "p": "brc20-s",
        "op": "transfer",
        "tid": "tid",
        "tick": "tick",
        "amt": "amt"
      }}"##
    );

    let reuslt = deserialize_brc30(&json_str);

    assert!(!deserialize_brc30(&json_str).is_err());

    assert_eq!(
      deserialize_brc30(&json_str).unwrap(),
      Operation::Transfer(Transfer {
        tick: "tick".to_string(),
        tick_id: "tid".to_string(),
        amount: "amt".to_string(),
      })
    );
  }

  #[test]
  fn test_loss_require_key() {
    let json_str = format!(
      r##"{{
        "p": "brc20-s",
        "op": "transfer",
        "tick": "tick",
        "amt": "amt"
      }}"##
    );

    let reuslt = deserialize_brc30(&json_str);

    assert_eq!(
      deserialize_brc30(&json_str).unwrap_err(),
      JSONError::ParseOperationJsonError("missing field `pid`".to_string())
    );
  }

  #[test]
  fn test_duplicate_key() {
    let json_str = format!(
      r##"{{
        "p": "brc20-s",
        "op": "transfer",
        "tid": "tid",
        "tick": "tick-1",
        "tick": "tick-2",
        "amt": "amt"
      }}"##
    );
    assert_eq!(
      deserialize_brc30(&json_str).unwrap(),
      Operation::Transfer(Transfer {
        tick: "tick-2".to_string(),
        tick_id: "tid".to_string(),
        amount: "amt".to_string(),
      })
    );
  }
}