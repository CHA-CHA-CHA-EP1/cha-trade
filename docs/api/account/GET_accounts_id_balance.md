# GET /accounts/:id/balance

## Curl

```bash
curl http://localhost:8080/accounts/{id}/balance
```

## Response Body

```json
{
  "account_id": "uuid",
  "available_balance": 70000,
  "hold_balance": 30000,
  "total_balance": 100000
}
```

## Business Logic

### Validate Path Params
1. `:id` must be a valid UUID

### Steps
1. Query the `accounts` table by `id`
2. If not found, return 404
3. Compute `total_balance = available_balance + hold_balance`
4. Return all balance fields
