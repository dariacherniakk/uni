# Smart Contract for Unis Discipline Management

1. [`factory`](./factory/) - a main contract for deployment.
2. [`token`](./token/) - a contract that represents each uni and stores its disciplines and matches

# Scenarios

## Initialization and working with the Factory contract

### Step 1: Build the contract

```bash
cargo build --target wasm32-unknown-unknown --profile testnet --features testnet
```

### Step 2: Deploy the factory contract

```bash
near dev-deploy target/wasm32-unknown-unknown/testnet/factory.wasm
```

### Step 3: Initialize the factory contract

```bash
near call $ACCOUNT new '{"owner_id": "'$OWNER_ACCOUNT'"}' --accountId $OWNER_ACCOUNT
```

### Step 4: Initialize the uni contract

```bash
near call $ACCOUNT add_uni '{"uni_name": "'$UNI_NAME'", "uni_owner_id": "'$UNI_OWNER_ID'"}' --accountId $OWNER_ACCOUNT --deposit 10 --gas 300000000000000
```

### Get factory owner

```bash
near view $ACCOUNT owner
```

### Get all unis contract ids

```bash
near view $ACCOUNT get_unis
```

## Working with the Uni contract

### Step 1: Add the discipline

```bash
near call $UNI_ACCOUNT add_discipline '{"discipline": "Math"}' --accountId $UNI_OWNER_ID
```

### Step 2: Add the discipline match

```bash
near call $UNI_ACCOUNT add_match '{"discipline_match": "SuperMath", "discipline": "Math"}' --accountId $UNI_OWNER_ID
```

### Delete the discipline match

```bash
near call $UNI_ACCOUNT delete_match '{"discipline_match": "SuperMath"}' --accountId $UNI_OWNER_ID
```

### Delete the discipline

```bash
near call $UNI_ACCOUNT delete_discipline '{"discipline": "Math"}' --accountId $UNI_OWNER_ID
```

### Get all the disciplines

```bash
near view $UNI_ACCOUNT get_disciplines
```

### Get all matches

```bash
near view $UNI_ACCOUNT get_all_matches
```

### Get all matches for the specific discipline

```bash
near view $UNI_ACCOUNT get_discipline_matches '{"discipline": "Math"}'
```

### Get discipline for the specific match

```bash
near view $UNI_ACCOUNT get_match '{"discipline_match": "SuperMath"}'
```
