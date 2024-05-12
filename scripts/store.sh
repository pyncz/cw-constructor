# Process vars
## required vars
WASM_PATH="$1";
if [ -z $WASM_PATH ]; then
  echo "Wasm file to store is not provided!";
  exit 1;
fi

if [ -z $ACCOUNT ]; then
  echo "ACCOUNT is not provided!";
  exit 1;
fi

## optional vars
: "${CHAIN_ID:='constantine-3'}";
if [ $CHAIN_ID == 'archway-1' ]; then 
    RPC='https://rpc.mainnet.archway.io:443';
elif [ $CHAIN_ID == 'titus-2' ]; then
    RPC='https://rpc.titus.archway.io:443';
elif [ $CHAIN_ID == 'constantine-3' ]; then
    RPC='https://rpc.constantine.archway.io:443';
else
  echo "Invalid CHAIN_ID!";
  exit 1;
fi

: "${GAS_ADJUSTMENT:=1.4}";
: "${DRY_RUN:=0}";

# Store code to get code_id
GAS_PRICES=$(archwayd query rewards estimate-fees 1 --node $RPC --output json | jq -r '.gas_unit_price | (.amount + .denom)');
if [ $DRY_RUN == 1 ]; then
  archwayd tx wasm store $WASM_PATH --from $ACCOUNT --node $RPC --gas-prices $GAS_PRICES --gas-adjustment $GAS_ADJUSTMENT -y --output json -b sync --dry-run;
else
  RES=$(archwayd tx wasm store $WASM_PATH --from $ACCOUNT --node $RPC --gas auto --gas-prices $GAS_PRICES --gas-adjustment $GAS_ADJUSTMENT -y --output json -b sync);
  HASH=$(echo $RES | jq -r '.txhash');

  echo $RES | jq;
  # CODE_ID=$(echo $RES | jq -r '.logs[0].events[] | select(.type=="store_code") | .attributes[] | select(.key=="code_id") | .value');

  # TX=$(archwayd query tx $HASH);
  # CODE_ID=$(echo $TX | jq -r '.logs[0].events[] | select(.type=="store_code") | .attributes[] | select(.key=="code_id") | .value');

  # Log the results
  # echo "code_id:";
  # echo $CODE_ID;

  # echo "instantiated contracts:";
  # archwayd query wasm list-contract-by-code $CODE_ID --node $RPC --output json | jq;
fi
