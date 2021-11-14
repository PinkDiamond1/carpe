import { getLocalProofs, getTowerChainView } from "./miner_invoke";

export function healthCheck() {
  console.log("healthcheck");
  getTowerChainView();
  getLocalProofs();
}


/// get cpu usage

/// get latest file in local

/// get on chain state