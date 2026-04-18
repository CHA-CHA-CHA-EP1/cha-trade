# POST /accounts

## Curl

```bash
curl -X POST http://localhost:8080/accounts \
  -H "Content-Type: application/json" \
  -d '{
    "first_name": "John",
    "last_name": "Doe",
    "email": "john@example.com",
    "password": "plaintext_password"
  }'
```

## Request Body

```json
{
  "first_name": "John",
  "last_name": "Doe",
  "email": "john@example.com",
  "password": "plaintext_password"
}
```

## Response Body

```json
{
  "id": "uuid",
  "user_id": "uuid",
  "available_balance": 100000,
  "hold_balance": 0,
  "created_at": "2024-01-01T00:00:00Z",
  "updated_at": "2024-01-01T00:00:00Z"
}
```

## Business Logic

### Validate Request Body
1. `first_name` must not be empty
2. `last_name` must not be empty
3. `email` must not be empty and must be a valid email format
4. `password` must not be empty and must be at least 8 characters

### Steps
1. Check that `email` is not already registered in the `users` table → if exists, return 409
2. Begin database transaction
3. Hash the password (bcrypt/argon2)
4. Encrypt `first_name` and `last_name` with AES-256
5. Insert a new row in the `users` table
6. Insert a new row in the `accounts` table with the new `user_id` and default balance of 100,000
7. Commit transaction — if any step fails, rollback everything
8. Return the newly created account
